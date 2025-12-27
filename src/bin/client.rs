use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("0.0.0.0:3000")?;
    // stream.write_all(b"Hello from client").unwrap();

    let mut reader = stream.try_clone()?;

    // main thread ve bu thread'in ikisinde de loop attığımız joinhandle muhabbetine gerek kalmıyor.
    // zaten main loop quit ettiğinde hepsi çöksün gitsin
    thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    println!("Server says {}", String::from_utf8_lossy(&buffer[..n]));
                }
                Err(_e) => break,
            }
        }
    });

    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("input required");

        // println!("{}", input.trim());

        if input.trim() == "quit" {
            break;
        }

        stream.write_all(input.trim().as_bytes()).unwrap();
    }

    Ok(())
}

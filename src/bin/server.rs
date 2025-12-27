use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // println!("{stream:#?}");
    let peer = stream.peer_addr().unwrap();
    println!("New Connection: {}", peer);

    let mut buffer = [0; 512];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };
        println!(
            "Message from {}: {}",
            peer,
            String::from_utf8_lossy(&buffer[..bytes_read])
        );
        stream.write_all(b"Message received").unwrap();
    }

    println!("Client {} disconnected", peer);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000")?;
    println!("Server running at 127.0.0.1:3000");

    // accept connections and process them serially
    for stream in listener.incoming() {
        let stream_clone = stream?.try_clone()?;
        thread::spawn(|| {
            handle_client(stream_clone).unwrap();
        });
    }

    Ok(())
}

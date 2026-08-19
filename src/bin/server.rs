use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    // println!("{stream:#?}");
    let peer = stream.peer_addr()?;
    println!("New Connection: {}", peer);

    let mut buffer = [0u8; 1024];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Read error from {peer}: {e}");
                break;
            }
        };
        println!(
            "Message from {}: {}",
            peer,
            String::from_utf8_lossy(&buffer[..bytes_read])
        );
        stream.write_all(b"Message received")?;
    }

    println!("Client {} disconnected", peer);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000")?;
    println!("Server running at 127.0.0.1:3000");

    // Accept connections and handle each client in a separate thread
    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            if let Err(e) = handle_connection(stream) {
                eprintln!("Handle connection error: {e}")
            }
        });
    }

    Ok(())
}

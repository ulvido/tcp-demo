use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

type PeersType = Arc<Mutex<HashMap<SocketAddr, TcpStream>>>;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000")?;
    println!("Server running at 127.0.0.1:3000");

    let peers: PeersType = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        let stream = stream?;
        let addr = stream.peer_addr()?;
        let peers_clone = Arc::clone(&peers);

        // İstemciyi hashmap'e ekle (.expect kaldırıldı)
        {
            let mut peers_guard = peers.lock().expect("Peers lock error");
            peers_guard.insert(addr, stream.try_clone()?);
        }

        // peers_clone içeri aktarıldı
        thread::spawn(move || {
            if let Err(e) = handle_connection(stream, addr, peers_clone) {
                eprintln!("Handle connection error: {e}");
            }
        });
    }

    Ok(())
}

fn handle_connection(
    mut stream: TcpStream,
    addr: SocketAddr,
    peers: PeersType,
) -> std::io::Result<()> {
    println!("New Connection: {}", addr);

    let mut buffer = [0u8; 1024];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Read error from {addr}: {e}");
                break;
            }
        };

        println!(
            "Message from {}: {}",
            addr,
            String::from_utf8_lossy(&buffer[..bytes_read])
        );

        // Doğrudan HashMap üzerinde döngü (vektör yok, klonlama yok)
        {
            let mut peers_lock = peers.lock().expect("Peers lock error");

            for (peer_addr, peer_stream) in peers_lock.iter_mut() {
                if *peer_addr != addr {
                    // Yazma hatası olursa diğer istemcileri etkilememesi için unwrap/try (?) yapılmaz
                    let _ = peer_stream.write_all(
                        format!(
                            "{}: {}",
                            addr,
                            String::from_utf8_lossy(&buffer[..bytes_read])
                        )
                        .as_bytes(),
                    );
                }
            }
        }
    }

    println!("Client {} disconnected", addr);

    // Ayrılan istemciyi listeden sil
    {
        let mut peers_lock = peers.lock().expect("Peers lock error");
        if peers_lock.remove(&addr).is_some() {
            println!("Client {} removed from peers list", addr);
        }
    }

    Ok(())
}

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening on port 8080...");
    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("New client connected: {:?}",
                 stream.peer_addr()?);
        let mut buf = [0; 1024];
        loop {
            let bytes_read = stream.read(&mut buf)?;
            if bytes_read == 0 {
                println!("Client disconnected");
                break;
            }
            stream.write_all(&buf[..bytes_read])?;
        }
    }
    Ok(())
}

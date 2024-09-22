use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::str;

const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 12000;

fn main() -> io::Result<()> {
    let server_addr = SocketAddrV4::new(IP, PORT);
    let server_listener = TcpListener::bind(server_addr)?;

    println!("Server listening on port {PORT}");

    loop {
        println!("Waiting for connections...");
        let (stream, client_addr) = server_listener.accept()?;
        println!("Client connected! Client address: {}", client_addr);
        handle_client_connection(stream, client_addr)?;
    }
}

fn handle_client_connection(mut stream: TcpStream, client_addr: SocketAddr) -> io::Result<()> {
    let mut buffer: [u8; 1024] = [0; 1024];

    loop {
        println!("Waiting for messages from {client_addr}...");
        let bytes = stream.read(&mut buffer)?;

        if bytes == 0 {
            println!("Client disconnected!");
            return Ok(());
        }

        match str::from_utf8(&buffer[..bytes]) {
            Ok(message) => {
                println!("Message received: {message}");
                let modified_message = message.to_string().to_uppercase();
                stream.write_all(modified_message.as_bytes())?;
            }
            Err(e) => println!("Error: {e}"),
        }
    }
}

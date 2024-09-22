use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    let mut client_socket = TcpStream::connect("127.0.0.1:12000")?;
    let mut buffer = [0_u8; 1024];

    loop {
        let message = get_input("Input lowercase sentence: ")?;

        if message.eq("exit") {
            return Ok(());
        }

        client_socket.write_all(message.as_bytes())?;

        let bytes = client_socket.read(&mut buffer)?;

        match str::from_utf8(&buffer[..bytes]) {
            Ok(message) => println!("From server: {message}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}

fn get_input(console_message: &str) -> io::Result<String> {
    let mut input = String::new();

    print!("{console_message}");

    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    Ok(input.trim_end().to_string())
}

use std::io::{self, Read, Write};
use std::net::{TcpListener,TcpStream, Ipv4Addr};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut data = vec![0; 4096];

    loop {
        let len = stream.read(&mut data)?;
        if len == 0 {
            return Ok(())
        }
        let mut stdout = io::stdout();
        stdout.write_all(&data[..len])?;
        stdout.flush()?;
    }
}

fn main() -> Result<()> {
    let addr = (Ipv4Addr::UNSPECIFIED, 8765);
    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        let _ = handle_connection(stream?);
    }
    Ok(())
}

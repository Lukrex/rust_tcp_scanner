use std::io;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

fn main() -> io::Result<()> {
    let host = "127.0.0.1";
    let start_port = 1;
    let end_port = 1024;

    println!("Scanning {}:{}-{}", host, start_port, end_port);

    for port in start_port..=end_port {
        //try every port
        match scan_port(host, port) {
            Ok(true) => println!("Port {} is open!", port),
            Ok(false) => (), //if port not open, do nothing
            Err(e) => eprint!("Scan unsuccessful on port {}: {}", port, e),
        }
    }

    Ok(())
}

fn scan_port(host: &str, port: u16) -> io::Result<bool> {
    let socket: SocketAddr = format!("{}:{}", host, port) //create target address
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    match TcpStream::connect_timeout(&socket, Duration::from_millis(1_000)) {
        Ok(_) => Ok(true), //if conencted to address for 1 second, port open
        Err(e) => {
            //otherwise Error
            match e.kind() {
                io::ErrorKind::ConnectionRefused => Ok(false), //port closed
                io::ErrorKind::TimedOut => Ok(false),          //timeout = closed
                _ => Err(e),                                   //other errors
            }
        }
    }
}

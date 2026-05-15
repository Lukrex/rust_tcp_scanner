use std::io;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut input_host = String::new();
    let start_port = 1;
    let end_port = 1024;

    println!("Enter IP to scan: ");

    io::stdin()
        .read_line(&mut input_host)
        .expect("Error occurred when entering IP");

    let host = input_host.trim();

    println!("Scanning {}:{}-{}", host, start_port, end_port);

    for port in start_port..=end_port {
        //try every port
        match scan_port(host, port) {
            Ok(true) => println!("Port {} is open!", port),
            Ok(false) => println!("Port {} is closed...", port),
            Err(e) => eprintln!("Scan unsuccessful on port {}: {}", port, e),
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

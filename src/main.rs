use std::io;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input_host = String::new();
    let mut input_port = String::new();
    let mut input_end_port = String::new();

    println!("Enter IP to scan: ");
    io::stdin()
        .read_line(&mut input_host)
        .expect("Error occurred when entering IP");

    let host = &input_host.trim().to_string();

    println!("Enter first port to scan from [1]: ");
    io::stdin()
        .read_line(&mut input_port)
        .expect("Error occurred when entering first port");
    let start_port: u16 = if input_port.trim().is_empty() {
        1
    } else {
        input_port.trim().parse().unwrap_or(1)
    };

    println!("Enter last port to scan until [1024]: ");
    io::stdin()
        .read_line(&mut input_end_port)
        .expect("Error occurred when entering last port");
    let end_port: u16 = if input_end_port.trim().is_empty() {
        1024
    } else {
        input_end_port.trim().parse().unwrap_or(1024)
    };

    println!("Scanning {}:{}-{}", host, start_port, end_port);

    let mut tasks = Vec::new();

    for port in start_port..=end_port {
        let host_clone = host.clone();

        let task = tokio::spawn(async move {
            if scan_port(&host_clone, port).await {
                println!("Port {} is open!", port);
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await;
    }
    println!("Scan finished.");
    Ok(())
}

async fn scan_port(host: &str, port: u16) -> bool {
    let socket: SocketAddr = match format!("{}:{}", host, port).parse() {
        Ok(addr) => addr,
        Err(_) => return false,
    };

    let connection_result =
        timeout(Duration::from_millis(1_000), TcpStream::connect(&socket)).await;

    match connection_result {
        Ok(Ok(_stream)) => true,        //got connection before timeout
        Ok(Err(_)) => false,            //rejected connection or network error
        Err(_timeout_elapsed) => false, //timeout reached, no answer
    }
}

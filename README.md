# Rust TCP Port Scanner

A simple, asynchronous TCP port scanner written in Rust using **Tokio**.
It scans a given IP address across a user-defined port range and reports open ports.

## Features

* Async port scanning using Tokio
* User-defined IP and port range
* Fast concurrent scanning (one task per port)
* Built-in timeout handling
* Simple CLI interface

## How It Works

The scanner attempts to establish a TCP connection to each port in the specified range.
If a connection succeeds within the timeout window, the port is reported as open.

## Requirements

* Rust (stable)
* Cargo

## Dependencies

This project uses:

* [`tokio`](https://crates.io/crates/tokio) — async runtime

Add it to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

## Usage

Clone the repository and run:

```bash
git clone https://github.com/your-username/rust-port-scanner.git
cd rust-port-scanner
cargo run
```

## Example

```
Enter IP to scan:
192.168.1.1

Enter first port to scan from [1]:
1

Enter last port to scan until [1024]:
1000

Scanning 192.168.1.1:1-1000
Port 22 is open!
Port 80 is open!
Scan finished.
```

## How It Works Internally

For each port in the range:

* A Tokio task is spawned
* The scanner tries to connect using `TcpStream::connect`
* Each attempt is wrapped in a 1-second timeout
* Successful connections indicate open ports

## Limitations

* No service detection (only open/closed)
* No rate limiting (can overwhelm system/network for large ranges)
* IPv4 only (no explicit IPv6 handling)
* Accuracy depends on firewall behavior and timeouts

## Future Improvements

* Add service/banner grabbing
* Add thread limiting / concurrency control
* Add CIDR range support
* Add CLI flags instead of interactive input
* Export results to file (JSON / TXT)

## Disclaimer

This tool is for **educational and authorized security testing only**.
Do not scan networks or systems you do not own or have permission to test.

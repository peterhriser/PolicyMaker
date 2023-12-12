use serde_json::Value;
use std::env;
use std::net::UdpSocket;

mod csm;
mod iam;

fn main() -> std::io::Result<()> {
    // Read the host and port from environment variables with defaults
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port_str = env::var("PORT").unwrap_or_else(|_| "31000".to_string());
    let port: u16 = port_str.parse().expect("Invalid port number");

    // Bind to the UDP socket on the specified host and port
    let socket = UdpSocket::bind(format!("{}:{}", host, port))?;

    println!("Listening for UDP traffic on {}:{}", host, port);

    // Create a buffer to store incoming data
    let mut buf = [0; 65536];
    loop {
        // Receive data from the socket
        let (num_bytes, peer_addr) = socket.recv_from(&mut buf)?;

        // Convert the received bytes to a string and parse as JSON
        let received_data = String::from_utf8_lossy(&buf[0..num_bytes]);
        match serde_json::from_str::<Value>(&received_data) {
            Ok(json) => {
                let pretty_string = serde_json::to_string_pretty(&json).unwrap();
                println!("{}", pretty_string);
            }
            Err(err) => {
                eprintln!(
                    "Error parsing JSON from {}: {}, {}",
                    peer_addr, err, &received_data
                );
            }
        }
    }
}

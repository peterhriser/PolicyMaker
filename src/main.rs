use std::net::UdpSocket;
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread::sleep;
use std::{env, process::exit};

use crate::csm::ApiCall;
use tracing::{debug, info};
use tracing_subscriber;
mod csm;
mod iam;

fn main() -> std::io::Result<()> {
    // Read the host and port from environment variables with defaults
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port_str = env::var("PORT").unwrap_or_else(|_| "31000".to_string());
    let port: u16 = port_str.parse().expect("Invalid port number");

    // Bind to the UDP socket on the specified host and port
    let socket = UdpSocket::bind(format!("{}:{}", host, port))?;
    socket.set_read_timeout(Some(std::time::Duration::from_millis(100)))?;

    println!("Listening for UDP traffic on {}:{}", host, port);

    // Create a buffer to store incoming data
    let mut buf = [0; 65536];
    let mut policy_builder = iam::PolicyBuilder::new();

    let (tx, rx) = channel();
    ctrlc::set_handler(move || {
        println!("Received Ctrl-C. Exiting.");
        tx.send(()).expect("Could not send signal on channel.")
    })
    .expect("Error setting Ctrl-C handler");
    loop {
        // Receive data from the socket
        let (num_bytes, peer_addr) = match socket.recv_from(&mut buf) {
            Ok((num_bytes, peer_addr)) => (num_bytes, peer_addr),
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                sleep(std::time::Duration::from_millis(50));
                continue;
            }
            Err(err) => Err(err)?,
        };

        // Convert the received bytes to a string and parse as JSON
        let received_data = String::from_utf8_lossy(&buf[0..num_bytes]);
        let received_data = received_data.trim();
        let recieved_data_value = serde_json::to_string_pretty(
            &serde_json::from_str::<serde_json::Value>(&received_data).unwrap(),
        )
        .unwrap();
        match serde_json::from_str::<ApiCall>(&received_data) {
            Ok(json) => {
                // print the CSM API call
                debug!("{}", recieved_data_value);
                let _ = &policy_builder.add_api_call(&json);
            }
            Err(err) => {
                eprintln!(
                    "Error parsing JSON from {}: {}, {}",
                    peer_addr, err, &received_data
                );
            }
        }
        match rx.recv_timeout(std::time::Duration::from_millis(1000)) {
            Ok(_) => {
                info!("Received Ctrl-C. Exiting.");
                let policy = policy_builder.build();
                let policy = serde_json::to_string_pretty(&policy).unwrap();
                println!("{}", policy);
                exit(0)
            }
            Err(RecvTimeoutError::Timeout) => (),
            Err(err) => {
                eprintln!("Error receiving signal: {}", err);
                exit(1)
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{csm::ApiCall, iam::Statement};

    fn generate_api_call() -> ApiCall {
        let json = r#"{
  "Api": "ListObjectsV2",
  "AttemptCount": 1,
  "ClientId": "",
  "FinalHttpStatusCode": 200,
  "Latency": 356,
  "MaxRetriesExceeded": 0,
  "Region": "us-west-2",
  "Service": "S3",
  "Timestamp": 1702364837492,
  "Type": "ApiCall",
  "UserAgent": "aws-cli/2.13.28 Python/3.11.6 Linux/6.2.6-76060206-generic exe/x86_64.pop.22 prompt/off command/s3.ls",
  "Version": 1
}"#;
        serde_json::from_str::<ApiCall>(&json).unwrap()
    }
    #[test]
    fn from_api_call_to_statement() {
        let api_call = generate_api_call();
        let statement: Statement = Statement::from(&api_call);
        eprintln!("{:#?}", statement);
        assert_eq!(statement.effect, "Allow");
        assert_eq!(
            statement.action.into_iter().next().unwrap().as_str(),
            "s3:ListBucket"
        );
        assert_eq!(statement.resource[0], "*");
    }
}

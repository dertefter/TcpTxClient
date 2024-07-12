use std::net::TcpStream;
use std::io::{self, Read, Write};
const ADDRESS: &str = "127.0.0.1:8080";

fn tx_to_bytes(input_string: &str) -> Vec<u8> {
    let b = input_string.replace(" ", "").to_string().into_bytes();
    b
}

fn bytes_to_tx(input_bytes: &[u8], size: usize) -> String {
    String::from_utf8_lossy(&input_bytes[..size]).to_string().replace(" ", "")
}


fn main() {
    // Try to connect
    let mut stream = loop {
        println!("Trying to connect...");
        match TcpStream::connect(ADDRESS) {
            Ok(new_stream) => {
                println!("SUCCESSFUL CONNECTION!");
                break new_stream;
            }
            Err(_) => {
                println!("Connection failed...");
            }
        }
    };

    loop {
        println!("1 - Send data");
        println!("0 - Exit");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input: u32 = user_input.trim().parse().unwrap();
        match user_input {
            1 => {
                println!("Write some data:");
                let mut data_to_send = String::new();
                io::stdin()
                    .read_line(&mut data_to_send)
                    .expect("Failed to read line");
                let data_to_send = tx_to_bytes(&data_to_send);
                match stream.write(&*data_to_send) {
                    Ok(_) => {
                        println!("Data send successful!");

                    },
                    Err(_) => {
                        println!("Data send ERROR...");
                        continue;
                    },
                }
                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(n) if n == 0 => {
                        println!("Server closed connection.");
                        break;
                    }
                    Ok(n) => {
                        let response = bytes_to_tx(&buffer, n);
                        println!("Received from server: {}", response);
                    }
                    Err(_) => println!("Failed to read from server."),
                }
            }
            0 => break,
            _ => println!("Don't know what to do... :("),
        }
    }
}

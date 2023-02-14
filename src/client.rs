use std::io::{Write, Read, BufRead};
use std::net::TcpStream;

fn main() {

    let mut buf = [0 as u8; 512];
    loop {

        match TcpStream::connect("0.0.0.0:9999"){
            Ok(mut stream) => {
                
                stream.read(&mut buf);
                let s = match std::str::from_utf8(&buf) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                println!("{}", s);
    
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
    
                stream.write(input.as_bytes());
    
            },
            Err(_e) => {
                println!("Failed to connect");
            }
        }

    }

}

use std::io::{Write, Read, BufRead};
use std::net::TcpStream;

fn main() {

    let mut buf = [0 as u8; 512];

    loop {
        match TcpStream::connect("127.0.0.1:9999"){
            Ok(mut stream) => {
    
                loop {
                    stream.read(&mut buf);
                    
                    let mut s = match std::str::from_utf8(&buf) {
                        Ok(v) => v,
                        Err(e) => "",
                    };
    
                    println!("{}", s);
    
                    if s.contains("# sua vez"){
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
    
                        stream.write(input.as_bytes()).unwrap();
                        break;
                    }
    
                }
    
            },
            Err(_e) => {
                println!("Failed to connect");
            }
        }
    }   
    
}

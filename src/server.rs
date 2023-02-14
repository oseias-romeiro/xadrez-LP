use std::fmt::format;
use std::io::{Write, Read, BufRead};
use std::net::TcpListener;


mod xadrez;
use xadrez::tabuleiro::Tabuleiro as Tabuleiro;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9999").unwrap();
    println!("server listening...");

    let mut tab = Tabuleiro::tabuleiro_inicial();

    let mut msg = String::new();

    for stream in listener.incoming() {

        msg = tab.str_tabuleiro();
        
        let mut stream = stream.unwrap();
        let mut buf = [0 as u8; 512];

        stream.write(msg.as_bytes()).unwrap();

        stream.read(&mut buf);

        let mut s = match std::str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        if s.contains(" -> ") {
            let coord = get_coord(s.to_string());
    
            let res:bool = tab.mover(coord.0,coord.1);
    
        }else {
            println!("Entrada inválida");
        }
    }
}

fn get_coord(mut input: String) -> ((usize, usize), (usize, usize)) {

    while input.len() > 10 {
        input.pop();
    }

    let coord: Vec<&str> = input.split(" -> ").collect();
        
    let coord_peca: Vec<&str> = coord[0].split(',').collect();
    let coord_vai: Vec<&str> = coord[1].split(',').collect();

    let vec : Vec<char> = coord_vai[1].to_string().chars().collect();
    
    return (
        (
            coord_peca[0].to_string().parse().unwrap(),
            coord_peca[1].to_string().parse().unwrap()
        ),
        (
            coord_vai[0].to_string().parse().unwrap(),
            coord_vai[1].to_string().parse().unwrap()
        )
    );
}


use std::fmt::format;
use std::io::{Write, Read, BufRead};
use std::net::TcpListener;


mod xadrez;
use xadrez::tabuleiro::Tabuleiro as Tabuleiro;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();
    println!("server listening...");

    let mut tab = Tabuleiro::tabuleiro_inicial();
    let mut msg = String::new();
    let mut valid_move = "";
    let mut buf = [0 as u8; 512];

    for stream in listener.incoming() {

        let mut stream = stream.unwrap();

        // vez do server
        msg = tab.str_tabuleiro();
        println!("# sua vez");
        println!("{}", msg);

        stream.write(b"# vez do adversario\n").unwrap();
        stream.write(msg.as_bytes()).unwrap();

        valid_move = "";
        loop {

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.contains(" -> ") && input.contains(',') {
                let coord = get_coord(input);

                valid_move = tab.mover(coord.0,coord.1);

                if valid_move != "Movimento invalido!" {
                    break;
                }else {
                    println!("{}", valid_move);
                }

            }else {
                println!("Entrada invalida");
            }
        }

        if valid_move == "Xeque mate!"  {
            stream.write(valid_move.as_bytes()).unwrap();
            println!("{}", valid_move);
            break
        }

        // vez do cliente
        msg = tab.str_tabuleiro();
        println!("# vez do adversario");
        println!("{}", msg);

        stream.write(b"\n# sua vez\n").unwrap();
        stream.write(msg.as_bytes()).unwrap();

        valid_move = "";
        loop  {

            stream.read(&mut buf);

            let mut s = match std::str::from_utf8(&buf) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            if s.contains(" -> ") && s.contains(',') {
                let coord = get_coord(s.to_string());
        
                valid_move = tab.mover(coord.0,coord.1);

                if valid_move != "Movimento invalido!" {                    
                    break;
                }else {
                    stream.write(valid_move.as_bytes()).unwrap();
                }
        
            }else {
                stream.write(b"Entrada invalida\n").unwrap();
            }
        }

        if valid_move == "Xeque mate!"  {
            stream.write(valid_move.as_bytes()).unwrap();
            println!("{}", valid_move);
            break
        }
        buf = [0 as u8; 512];
        
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


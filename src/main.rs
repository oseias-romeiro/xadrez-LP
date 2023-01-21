<<<<<<< HEAD
struct Posicao{
    x: u32,
    y: u32,
}
impl Posicao {
    fn new(x:u32, y:u32) -> Self {
        Self {
            x,
            y,
        }
    }
}


#[derive(Debug)]
enum Cor {
    Branco,
    Preto,
}

#[derive(Debug)]
enum Peca {
    Cavalo(Cor),
    Peao(Cor),
    Dama(Cor),
    Rei(Cor),
    Torre(Cor),
    Bispo(Cor),
    Vazio,
}

impl Peca {
    fn representacao_visual(&self)-> &str{
        match self {
            Peca::Torre(Cor::Preto) => "\u{2656}",
            Peca::Cavalo(Cor::Preto) => "\u{2658}",
            Peca::Bispo(Cor::Preto) => "\u{2657}",
            Peca::Dama(Cor::Preto) => "\u{2655}",
            Peca::Rei(Cor::Preto) => "\u{2654}",
            Peca::Peao(Cor::Preto) => "\u{2659}",
            Peca::Torre(Cor::Branco) => "\u{265C}",
            Peca::Cavalo(Cor::Branco) => "\u{265E}",
            Peca::Bispo(Cor::Branco) => "\u{265D}",
            Peca::Dama(Cor::Branco) => "\u{265B}",
            Peca::Rei(Cor::Branco) => "\u{265A}",
            Peca::Peao(Cor::Branco) => "\u{265F}",
            Peca::Vazio => " ",
        }
    }

}

struct Tabuleiro {
    mapa: [[Peca;8];8],
}


impl Tabuleiro {
    fn tabuleiro_inicial() -> Self {

        Self {  
            mapa: [
                [Peca::Torre(Cor::Preto), Peca::Cavalo(Cor::Preto), Peca:: Bispo(Cor::Preto), Peca::Dama(Cor::Preto), Peca::Rei(Cor::Preto), 
                 Peca:: Bispo(Cor::Preto), Peca::Cavalo(Cor::Preto), Peca::Torre(Cor::Preto)],
                [Peca::Peao(Cor::Preto), Peca::Peao(Cor::Preto), Peca::Peao(Cor::Preto), Peca::Peao(Cor::Preto),
                 Peca::Peao(Cor::Preto), Peca::Peao(Cor::Preto), Peca::Peao(Cor::Preto), Peca::Peao(Cor::Preto)],

                [Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio],
                [Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio],
                [Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio],
                [Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio, Peca::Vazio],
                
                [Peca::Peao(Cor::Branco), Peca::Peao(Cor::Branco), Peca::Peao(Cor::Branco), Peca::Peao(Cor::Branco),
                 Peca::Peao(Cor::Branco), Peca::Peao(Cor::Branco), Peca::Peao(Cor::Branco), Peca::Peao(Cor::Branco)],

                [Peca::Torre(Cor::Branco), Peca::Cavalo(Cor::Branco), Peca:: Bispo(Cor::Branco), Peca::Dama(Cor::Branco),
                 Peca::Rei(Cor::Branco), Peca:: Bispo(Cor::Branco), Peca::Cavalo(Cor::Branco), Peca::Torre(Cor::Branco)],
            ],
        }
    }

    fn imprime_tabuleiro(&self) {
        for linha in &self.mapa {
            println!("-----------------");
            print!("|");
            for peca in linha {
                print!("{}", peca.representacao_visual());

                print!("|");
            }
            println!();
        }
        println!("-----------------");

    }
}



fn main() {
    let mut tab = Tabuleiro::tabuleiro_inicial();
    tab.imprime_tabuleiro();
}

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


#[derive(Debug, Clone, Copy)]
enum Cor {
    Branco,
    Preto,
}

#[derive(Debug, Clone, Copy)]
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

pub struct Tabuleiro {
    mapa: [[Peca;8];8],
}


impl Tabuleiro {

    pub fn tabuleiro_inicial() -> Self {

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

    pub fn imprime_tabuleiro(&self) {
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

    // TODO: captura
    pub fn valid_captura(&self) -> bool {return true}

    // TODO: definir as regras de movimento das outras pecas
    fn valid_moves(&self, coord_peca:(usize, usize), _coord_vai:(usize, usize)) -> bool {

        /* regras:
         * impede movimentar uma peca vazia
         * se o destino da peça não for vazia aplica a captura
        */

        if 
            &self.mapa[coord_peca.0][coord_peca.1].representacao_visual() !=  &" "
        {
            // destino já tem uma peça
            if &self.mapa[_coord_vai.0][_coord_vai.1].representacao_visual() !=  &" " {
                // verifica se a captura é valida
                if self.valid_captura() {
                    // captura a peça
                    return true;
                }
            }else {
                return true;
            }
        }
        return false;
    }

    pub fn mover(&mut self, coord_peca:(usize, usize), _coord_vai:(usize, usize)) {

        if self.valid_moves(coord_peca, _coord_vai) {
            self.mapa[_coord_vai.0][_coord_vai.1] = self.mapa[coord_peca.0][coord_peca.1];
            self.mapa[coord_peca.0][coord_peca.1] = Peca::Vazio;
        }else {
            println!("Movimento não aceito")
        }
    }
    
}

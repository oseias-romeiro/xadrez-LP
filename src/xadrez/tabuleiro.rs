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


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cor {
    Branco,
    Preto,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn str_tabuleiro(&self) -> String {
        let mut tab = "".to_string();
        for linha in &self.mapa {
            tab += "-----------------\n";
            tab += "|";
            for peca in linha {
                tab += peca.representacao_visual();

                tab += "|";
            }
            tab += "\n";
        }
        tab += "-----------------\n";
        return tab;
    }

    // valida movimentos de um pião
    pub fn valid_move_peao(&self, coord_peca:(usize, usize), coord_vai:(usize, usize), captura_mode:bool, cor:Cor) -> bool {

        let d_x = (coord_vai.0 as i8) - (coord_peca.0 as i8);
        let d_y = (coord_vai.1 as i8) - (coord_peca.1 as i8);

        // apenas movimentos para frente
        if cor == Cor::Preto && d_x < 0 {
            return false;
        }else if cor == Cor::Branco && d_x > 0 {
            return false;
        }

        let diff_x = i8::abs(d_x) as usize;
        let diff_y = i8::abs(d_y) as usize;

        // na captura o movimento pode ser de apenas uma casa pra frente na diagonal
        if captura_mode {
            return diff_x == 1 && diff_y == 1;
        }

        // primeiro movimento de um pião pode ser de uma/duas casas para frente
        if cor == Cor::Preto && coord_peca.0 == 1 || cor == Cor::Branco && coord_peca.0 == 6 {
            return (diff_x == 1 || diff_x == 2) && diff_y == 0
        }

        // apenas um movimento para frente é valido
        return diff_x == 1 && diff_y == 0
    }
    // valida movimentos do rei
    pub fn valid_move_rei(&self, coord_peca:(usize, usize), coord_vai:(usize, usize)) -> bool {
        let diff_x = (coord_peca.0).abs_diff(coord_vai.0);
        let diff_y = (coord_peca.1).abs_diff(coord_vai.1);

        // movimento de 1 casa para qualer direcao
        return diff_x <= 1 && diff_y <= 1;
    }

    // TODO: definir as regras de movimento das outras pecas
    fn valid_moves(&self, coord_peca:(usize, usize), coord_vai:(usize, usize)) -> bool {

        /* regras:
         * impede movimentar uma peca vazia
         * se o destino da peça não for vazia aplica a captura
        */

        // modo captura ativado se o destino da peça ja tem alguem
        let captura_mode = &self.mapa[coord_vai.0][coord_vai.1].representacao_visual() !=  &" ";

        return match &self.mapa[coord_peca.0][coord_peca.1] {
            &Peca::Cavalo(_) => false,
            &Peca::Bispo(_) => false,
            &Peca::Torre(_) => false,
            &Peca::Dama(_) => false,
            &Peca::Rei(_) => self.valid_move_rei(coord_peca, coord_vai),
            &Peca::Peao(Cor::Branco) => self.valid_move_peao(coord_peca, coord_vai, captura_mode, Cor::Branco),
            &Peca::Peao(Cor::Preto) => self.valid_move_peao(coord_peca, coord_vai, captura_mode, Cor::Preto),
            &Peca::Vazio => false,
        };
        
    }

    pub fn mover(&mut self, coord_peca:(usize, usize), coord_vai:(usize, usize)) -> &str {

        if self.valid_moves(coord_peca, coord_vai) {

            if self.mapa[coord_vai.0][coord_vai.1] == Peca::Rei(Cor::Branco) ||
                self.mapa[coord_vai.0][coord_vai.1] == Peca::Rei(Cor::Preto)
            {
                return "Xeque mate!"; // para o jogo
            }

            self.mapa[coord_vai.0][coord_vai.1] = self.mapa[coord_peca.0][coord_peca.1];
            self.mapa[coord_peca.0][coord_peca.1] = Peca::Vazio;

            return "aceito";

        }else {

            return "Movimento invalido!";
        }
        
    }
    
}

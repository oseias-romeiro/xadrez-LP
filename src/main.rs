mod xadrez;

use xadrez::tabuleiro::Tabuleiro as Tabuleiro;

fn main() {
    // TODO: interação com o usuario
    let mut tab = Tabuleiro::tabuleiro_inicial();
    tab.imprime_tabuleiro();
    
    // exemplo de movimentos de peoes validos
    tab.mover((1,0),(3,0));
    tab.mover((6,1),(5,1));

    tab.imprime_tabuleiro();
}

mod xadrez;

use xadrez::tabuleiro::Tabuleiro as Tabuleiro;

fn main() {
    // TODO: interação com o usuario
    let mut tab = Tabuleiro::tabuleiro_inicial();
    tab.imprime_tabuleiro();
    
    // exemplo de movimento valido
    tab.mover((1,0),(3,0));
    tab.mover((0,6),(2,5)); // teste movimento do cavalo
    
    tab.imprime_tabuleiro();
}

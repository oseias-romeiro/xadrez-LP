mod xadrez;

use xadrez::tabuleiro::Tabuleiro as Tabuleiro;

fn main() {

    let mut tab = Tabuleiro::tabuleiro_inicial();

    loop {
        tab.imprime_tabuleiro();


        // origemX,origemY -> destinoX,destinoY
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.pop();

        if input.contains(" -> ") {
            let coord = get_coord(input);

            tab.mover(coord.0,coord.1);
        }else {
            println!("Entrada inválida");
        }
    }

}

fn get_coord(input:String) -> ((usize, usize), (usize, usize)) {

    let coord: Vec<&str> = input.split(" -> ").collect();
        
    let coord_peca: Vec<&str> = coord[0].split(',').collect();
    let coord_vai: Vec<&str> = coord[1].split(',').collect();

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

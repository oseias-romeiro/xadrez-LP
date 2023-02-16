# xadrez-LP

Projeto de Linguagens de Programação, desenvolvendo um jogo duo (dois jogadores) de xadrez, utilizando a linguagem Rust

## wiki

Foi escrito uma wiki com as estruturas suportadas da linguagem, prensente em [wiki.md](./wiki.md).

## Análise da linguagem

Além, da wiki. Foi feita também uma análise da linguagem, descrevendo-a e fazendo comparações com a linguagem C. [Análise da linguagem](./analise_linguage_rust_grupo2.pdf).

## Descrição

O jogo, utiliza de uma arquitetura P2P (peer to peer) para dois usuários jogarem simultâneamente, utlizando-se da rede. Sendo um jogador o server e outro o client, utilizando-se de uma conexão TCP e do módulo [xadrez/tabuleiro.rs](./src/xadrez/tabuleiro.rs).

## Compilando e rodando

-   `rustc src/server.rs && ./server`: jogador inicial sobe o servidor na qual outro jogador pode se conectar;
-   `rustc src/client.rs && ./client`: o oponente se conecta ao servidor (seu oponentes) e inicia o jogo.


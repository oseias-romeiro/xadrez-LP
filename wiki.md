# Introdução à Rust

Rust é uma linguagem de programação multiparadigma projetada com foco em memória e desempenho. Ele fornece uma linguagem segura, prática, rápido sem sacrificar a expressividade. Rust é a linguagem para um foco em alto desempenho e de forma confiável.


## Start

É necessário instalar o compilador e as ferramentas Rust. Você pode encontrar instruções no site oficial do Rust [rust-lang.org](https://www.rust-lang.org/).<br>
Após instalar o compilador Rust, podemos começar com um simples código que aprensenta uma menssagem na tela. 

Depois de instalar o compilador Rust, você estará pronto para começar a codificar! Vamos começar escrevendo um simples "Hello, World!" programa:

``` rust
    fn main() {
        println!("Olá, Mundo!");
    }
```

Este código imprime a string "Hello, World!" para o terminal usando o comando println!.


### Compilando e rodando o programa

```sh
    $rusc olá.rs
    $ ./olá
    Olá Mundo!
```


## Variáveis e Tipos

No Rust, as variáveis são declaradas usando a palavra-chave let. Ex.:

```rust
    let meu_num = 5;
```

As variáveis Rust são imutáveis por padrão, o que significa que seu valor não pode ser alterado. Para tornar uma variável mutável, é necessario usar a palavra-chave mut:

```rust
    let mut meu_num = 5;
```

Rust também contem uma grande variedade de tipos de dados. Ex.:


- Inteiro:
    - podem ser sinalizados ou não sinzalizados
    - tamanhos: 8, 16, 32 (padrão), 64, 128 e arch (tamanho da arquitetura usada 32/64) 
    - ex.: i32

- Flutuante:
    - tamanhos: 32 e 64 (padrão)
    - ex.: f64

- Booleano:
    - aceita valores booleanos (verdadeiro ou falso)

- Cáracter:
    - aceita caracteres mais que o ASCII, utilizando codigos unicode



## Funções

As funções são declaradas usando a palavra-chave fn, seguida por um nome de função e parâmetros

```rust
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }
```

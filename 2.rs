/*

2) Dado a sequência de Fibonacci, onde se inicia por 0 e 1 e o próximo valor sempre será a soma dos 2 valores anteriores (exemplo: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34...), escreva um programa na linguagem que desejar onde, informado um número, ele calcule a sequência de Fibonacci e retorne uma mensagem avisando se o número informado pertence ou não a sequência.

IMPORTANTE:
Esse número pode ser informado através de qualquer entrada de sua preferência ou pode ser previamente definido no código;

*/

use std::io;

fn main() {
    println!("Digite um número:");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)
        .expect("Erro ao ler entrada.");
        
    let num: u32 = input.trim().parse().expect("Entrada inválida.");

    let mut aux0 = 0;
    let mut aux1 = 1;
    while aux1 < num {
        let temp = aux1;
        aux1 += aux0;
        aux0 = temp;
    }

    if aux1 == num {
        println!("O número {} está na sequência de Fibonacci.", num);
    } else {
        println!("O número {} não está na sequência de Fibonacci.", num);
    }
}

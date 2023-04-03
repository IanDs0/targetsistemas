/*

4) Dado o valor de faturamento mensal de uma distribuidora, detalhado por estado:

SP – R$67.836,43
RJ – R$36.678,66
MG – R$29.229,88
ES – R$27.165,48
Outros – R$19.849,53

Escreva um programa na linguagem que desejar onde calcule o percentual de representação que cada estado teve dentro do valor total mensal da distribuidora.

*/

use std::io;

fn soma_vetor(vetor: &[f32]) -> f32 {
    let mut soma: f32 = 0.0;
    for valor in vetor {
        soma += valor;
    }
    soma
}


fn main() {

/*
    Tem como eu criar uma struct que armazena a regiao 
    assim eu poderia mandar preencher a regiao e o valor
    e depois exibir a regiao, valor e percential de participasao
*/

    let mut faturamento: Vec<f32> = Vec::new();
    
    loop{
        println!("Digite o Faturamento:");
        let mut input = String::new();
        
        io::stdin().read_line(&mut input)
            .expect("Erro ao ler entrada.");
            
        let num: f32 = input.trim().parse().expect("Entrada inválida.");
        
        faturamento.push(num);
        
        println!("0) Sair\n1) Adcionar outro valor");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Erro ao ler entrada.");
            
        let num: u32 = input.trim().parse().expect("Entrada inválida.");
        if num == 0{
            break;
        }
    }

    let total: f32 = soma_vetor(&faturamento);
    
    for i in 0..faturamento.len(){
        
        let mut percentual: f32 = faturamento[i]/total;
        
        println!("O Faturamento do indice [{}] foi de {}%", i, percentual*100.0)
    }
    


}

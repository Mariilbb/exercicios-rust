//Escreva um programa que calcule a soma dos números ímpares em um intervalo de 0 a 100

fn main() {
    let mut soma = 0;
    
    for i in 0..100 {
        if i % 2 == 1 {
            soma += i;
        }
    }
    println!("A soma dos termos impares é {}", soma);
}
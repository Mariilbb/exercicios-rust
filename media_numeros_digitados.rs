//Escreva um programa em Rust que leia números inteiros do teclado. Quando digitado o valor 0, o programa deverá encerrar e mostrar a média dos números fornecidos de entrada

fn main() {
    let mut total = 0;
    let mut soma = 0;
    
    loop {
        println!("Digite um numero (0 para encerrar): ");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
        let num: i32 = input.trim().parse().expect("Digite um numero valido");
        
        if num == 0 {
        break;
        }
        
        total += num;
        soma += 1;
    }
    
    if soma > 0 {
        println!("A media é {:.2}", total as f64 / soma as f64);
    }
}
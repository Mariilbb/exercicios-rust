//Crie um programa em Rust que recebe como entrada a idade de um usuário, mostre se ele é maior de idade ou não

fn main() {
    println!("Digite sua idade: ");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
    
    let idade: u32 = input.trim().parse().expect("Digite um numero valido");
    
    if idade >= 18 {
        println!("Você é maior de idade")
    } else {
        println!("Você é menor de idade")
    }
}
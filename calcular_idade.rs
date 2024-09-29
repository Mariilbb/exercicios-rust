// ESCREVA UM PROGRAMA QUE CALCULE A IDADE DO USUÁRIO SABENDO SEU ANO DE NASCIMENTO

fn main() {

    println!("Digite o ano de nascimento:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    let ano_nascimento: i32 = input.trim().parse().expect("Digite um ano válido");

    let ano_atual = 2024;
    let idade = ano_atual - ano_nascimento;

    println!("A idade é: {}", idade);
}

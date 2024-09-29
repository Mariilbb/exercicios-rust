// Implemente um código que verifique se um número é par ou ímpar. Solicite do usuário o número e ao final mostre o resultado

fn main() {
    let mut input = String::new();
    
    println!("Digite um numero: ");
    std::io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    let num: i32 = input.trim().parse().expect("Entrada invalida");
    
    if num % 2 == 0 {
        println!("O numero {} é par", num);
    } else {
        println!("O numero {} é impar", num);
    }
}

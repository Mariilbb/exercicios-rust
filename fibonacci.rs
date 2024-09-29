//Escreva um programa em Rust para mostrar a sequência de Fibonacci até n termos

fn main() {
    println!("Digite o numero de termos: ");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
    
    let n: usize = input.trim().parse().expect("Digite um numero valido");
    
    if n == 0 {
        println!("A sequencia não pode ter 0 termos");
        return;
    }
    
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        println!("{}", a);
        let c = a + b;
        a = b;
        b = c;
    }
}

// Crie uma função que calcule o fatorial de um número fornecido como parâmetro

fn fatorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    
    let mut resultado = 1;
    
    for i in 1..=n {
        resultado *= i;
    }
    resultado
}

fn main() {
    let num = 5;
    let resultado = fatorial(num);
    
    println!("O fatorial de {} é {}", num, resultado);
}

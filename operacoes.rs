// Implemente um código que (1) solicite dois números inteiros assinados de 32 bits do usuário e (2) uma operação matemática: soma, subtração, divisão ou multiplicação. Ao final, mostre o resultado de cada operação

fn main() {
    let mut input = String::new();
    
    println!("Digite o numero: ");
    std::io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    let num1: i32 = input.trim().parse().expect("Entrada invalida");
    
    input.clear();
    
    println!("Digite o numero: ");
    std::io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    let num2: i32 = input.trim().parse().expect("Entrada invalida");
    
    input.clear();
    
 println!("Escolha uma operação (+, -, *, /):");
    std::io::stdin().read_line(&mut input).expect("Erro ao ler a entrada");
    let operation = input.trim().chars().next().expect("Entrada inválida");

    let result = match operation {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 != 0 {
                num1 / num2
            } else {
                println!("Erro: Divisão por zero.");
                return;
            }
        }
        _ => {
            println!("Operação inválida.");
            return;
        }
    };

    println!("Resultado: {}", result);
}
// Implemente um código que para verificar se um triângulo é equilátero, isósceles ou escaleno. Solicite do usuário os três lados do triângulo.

fn main() {
    fn ler_num() -> f64 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
        input.trim().parse().expect("Entrada inválida")
    }

    println!("Digite o comprimento do primeiro lado:");
    let a = ler_num();
    println!("Digite o comprimento do segundo lado:");
    let b = ler_num();
    println!("Digite o comprimento do terceiro lado:");
    let c = ler_num();

    if a == b && b == c {
        println!("O triângulo é equilátero.");
    } else if a == b || a == c || b == c {
        println!("O triângulo é isósceles.");
    } else {
        println!("O triângulo é escaleno.");
    }
}
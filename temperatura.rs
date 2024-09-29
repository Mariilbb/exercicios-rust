//Leia a temperatura do teclado em Celsius e imprima o equivalente em Fahrenheit

fn main() {
    println!("Digite a temperatura em Celcius: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
    
    let temp_celsius: f64 = input.trim().parse().expect("Digite um numero valido");
    let temp_fahrenheit = (temp_celsius * 9.0 / 5.0) + 32.0;
    
    println!("A temperatura em fahrenheit é {:.2}", temp_fahrenheit);
}

//Escreva um algoritmo em Rust para calcular o valor, em reais, que deve ser pago para um cliente de uma locadora de carros. 
// 1. O valor da locação para cada carro é 100.00 reais;
   // 2. O cliente pode locar um único carro por vários dias;
   
   fn main() {
    let valor_diaria = 100.00;
    
    println!("Digite o numero de dias de locação: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    
    let dias_locacao: i32 = input.trim().parse().expect("Digite um valor valido");
    let valor_total = valor_diaria * dias_locacao as f64;
    
    println!("O valor a ser pago é: R$ {:.2}", valor_total);
    }
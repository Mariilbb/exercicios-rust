//Implemente um código que converta segundos para horas, minutos e segundos. Solicite do usuário a quantidade de segundos e no final mostre os valores obtidos

fn main() {
    let mut input = String::new();
    
    println!("Digite os segundos: ");
    std::io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    let seg: u32 = input.trim().parse().expect("Entrada invalida");
    
    let horas = seg / 3600;
    let minutos = (seg % 3600) / 60;
    let segundos = seg % 60;
    
    println!("{} segundos correspondem a {} horas, {} minutos e {} segundos",
        seg, horas, minutos, segundos);
}

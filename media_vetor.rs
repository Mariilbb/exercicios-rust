//Crie uma função que receba um vetor como parâmetro e retorne a sua média

fn calcular_media (vetor: &[i32]) -> f64 {
    let mut soma = 0;
    
    for &num in vetor {
        soma += num;
    }
    
    soma as f64 / vetor.len() as f64
}

fn main() {
    let num = vec![10,20,30,40,50];
    
    let media = calcular_media(&num);
    
    println!("A media é: {:.2}", media);
}
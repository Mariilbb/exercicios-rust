//Implemente um código que, através de um laço de repetição (while, loop ou for), encontre o maior número (qualquer tipo numérico) de um vetor. Ao final, mostre o valor encontrado.

fn main() {
    let num = vec![10,20,30,40,50];
    let mut max = num[0];
    
    for &numeros in &num {
        if numeros > max {
            max = numeros
        }
    }
    
    println!("O maior numero é: {}", max);
}
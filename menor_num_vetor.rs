//Implemente um código que, através de um laço de repetição (while, loop ou for), encontre o menor número (qualquer tipo numérico) de um vetor. Ao final, mostre o valor encontrado.

fn main() {
    let num = vec![10,20,30,40,50];
    let mut min = num[0];
    
    for &numeros in &num {
        if numeros < min {
            min = numeros
        }
    }
    
    println!("O menor numero é: {}", min);
}
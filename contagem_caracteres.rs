//Implemente um código que realize a contagem de caracteres em uma variável do tipo String. A String deverá ser uma referência. Aqui você deverá transformar a String em um iterador utilizando o método .chars() da variável. Mostre a quantidade e o valor da variável.

fn main() {
    let mut input = String::new();
    
    println!("Digite uma string: ");
    std::io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    let input = input.trim();
    
    let quantidade = input.chars().count();
    
    println!("A string {} tem {} caracteres", input, quantidade);
}
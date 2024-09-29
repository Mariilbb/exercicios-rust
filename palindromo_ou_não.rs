//Crie uma função que verifique se uma palavra é um palíndromo A função deve receber uma referência à uma string como parâmetro e retornar um booleano indicando se é um palíndromo ou não.

fn is_palindromo(texto: &String) -> bool {
    let texto_reverso: String = texto.chars().rev().collect();
    texto == &texto_reverso
}

fn main() {
    let texto1 = String::from("teste");
    println!("{} é palíndromo? {}", texto1, is_palindromo(&texto1));
    
    let texto2 = String::from("subinoonibus");
    println!("{} é palíndromo? {}", texto2, is_palindromo(&texto2));
}

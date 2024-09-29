// Implemente uma estrutura chamada Pessoa e que esteja de acordo com a seguinte especificação:
//- Possui três campos: nome, do tipo String; idade, do tipo u8 e altura, do tipo f32.
//- Uma função estática new, que retorna uma estrutura Pessoa, com os seguintes parâmetros: nome, idade e altura. Ao final, retorne uma instância de Pessoa inicializada com esses parâmetros.
//- Crie uma função associada a estrutura Pessoa chamada falar, com um parâmetro mensagem do tipo String e sem retorno, que mostre na tela “A pessoa <NOME> disse <MENSAGEM>”, substituindo NOME pelo campo nome e MENSAGEM pelo parâmetro mensagem
//- Crie uma função main para criar uma pessoa através do método new e chamar a função falar fornecendo uma mensagem arbitrária como parâmetro

struct Pessoa {
    nome: String,
    idade: u8,
    altura: f32,
}

impl Pessoa {
    fn new(nome: String, idade: u8, altura: f32) -> Self {
        Pessoa {
            nome,
            idade,
            altura,
        }
    }

    fn falar(&self, mensagem: String) {
        println!("A pessoa {} ({} anos, {:.2} m) disse: {}", self.nome, self.idade, self.altura, mensagem);
    }
}

fn main() {
    let pessoa = Pessoa::new(String::from("Marina"), 19, 1.52);
    pessoa.falar(String::from("Olá"));
}

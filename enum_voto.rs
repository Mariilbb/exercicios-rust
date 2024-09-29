// Crie uma enumeração chamada Voto que represente diferentes tipos de votos, como voto a favor, voto contra e voto em branco. Em seguida, crie uma função que aceite uma lista de votos representados como uma lista de variantes da enumeração. Use a correspondência de padrões (pattern matching) para contar e imprimir o número total de votos a favor, votos contra e votos em branco

enum Voto {
    Favor,
    Contra,
    Branco,
}

fn contar_votos(votos: &[Voto]) -> (u32, u32, u32) {
    let mut votos_a_favor = 0;
    let mut votos_contra = 0;
    let mut votos_branco = 0;

    for voto in votos {
        match voto {
            Voto::Favor => votos_a_favor += 1,
            Voto::Contra => votos_contra += 1,
            Voto::Branco => votos_branco += 1,
        }
    }

    (votos_a_favor, votos_contra, votos_branco)
}

fn main() {
    let mut votos = Vec::new();

    loop {
        let mut buf = String::new();

        println!("Digite uma opção:");
        println!("1. Votar a favor");
        println!("2. Votar contra");
        println!("3. Votar em branco");
        println!("4. Mostrar votos");
        println!("5. Sair");

        std::io::stdin().read_line(&mut buf).expect("Erro ao ler linha");
        let opcao: i32 = buf.trim().parse().expect("Erro ao converter");

        match opcao {
            1 => votos.push(Voto::Favor),
            2 => votos.push(Voto::Contra),
            3 => votos.push(Voto::Branco),
            4 => {
                let (a_favor, contra, branco) = contar_votos(&votos);
                println!("Votos a favor: {}", a_favor);
                println!("Votos contra: {}", contra);
                println!("Votos em branco: {}", branco);
            }
            5 => break,
            _ => println!("Opção '{}' não existe", opcao),
        }
    }
}s
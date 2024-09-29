//Crie um programa para autenticar um usuário. Solicite o nome de usuário e
//senha e, em seguida, verifique se o nome de usuário é “admin” e a senha é
//“@dm1n”. Caso a senha ou o usuário não corresponderem, mostre na tela
//que o usuário ou a senha estão incorretos. Caso contrário, mostre que o
//usuário foi autenticado. 

fn main() {
    println!("Digite o usuário: ");
    
    let mut nome_usuario = String::new();
    std::io::stdin().read_line(&mut nome_usuario).expect("Falha ao ler entrada");
    let nome_usuario = nome_usuario.trim();
    
    println!("Digite a senha: ");
    
    let mut senha = String::new();
    std::io::stdin().read_line(&mut senha).expect("Falha ao ler entrada");
    let senha = senha.trim();
    
    if nome_usuario == "admin" && senha == "@dm1n" {
        println!("Usuario autenticado com sucesso");
    } else {
        println!("Usuario ou senha incorretos");
    }
}
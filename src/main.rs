use std::io;

fn main() {
    println!("========");
    println!("PERFIL");
    println!("========");

    let ler_nome = ler_nome();
    let ler_idade = ler_idade();

    println!("Nome: {ler_nome}");
    println!("Idade: {ler_idade}");
}

fn ler_nome() -> String {
    println!("Digite seu nome:");
    let mut nome: String = String::new();
    io::stdin().read_line(&mut nome).unwrap();

    nome.trim().to_string()
}

fn ler_idade() -> u8 {
    loop {
        println!("Digite sua idade:");
        let mut entrada: String = String::new();
        io::stdin().read_line(&mut entrada).unwrap();

        match entrada.trim().parse::<u8>() {
            Ok(idade) => return idade,
            Err(_) => println!("Idade iválida."),
        }
    }
}

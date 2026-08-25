fn main() {
    let nome: &str = "Richard";
    let idade: i32 = 25;
    let altura: f64 = 1.71;
    let ativo: bool = true;
    let mut pontos: i32 = 100;

    println!("=================");
    println!(" RUST PROFILE");
    println!("=================");

    println!("Nome: {nome}.");
    println!("Idade: {idade}");
    println!("Altura: {altura}");
    println!("Ativo: {ativo}\n");

    let status = if idade >= 18 {
    "Maior de idade."
    }
    else {
    "Menor de idade."
    };

    println!("Status: {status}");

    pontos += 50;
    pontos -= 30;

    println!("Pontos: {pontos}");

    if idade >= 18 && ativo {
        println!("Perfil autorizado.");
    }
    else {
        println!("Perfil não autorizado.");
    }

    println!("=================");


}

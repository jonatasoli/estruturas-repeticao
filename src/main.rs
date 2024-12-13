use std::io;

fn main() {
    println!("Bem vindo, Spike Spiegel! Você está no comando da nave Bebop.");

    println!("Escolha um planeta para explorar: (Tijuana, Ganymede, Callisto)");

        let mut planeta = String::new();

        io::stdin().read_line(&mut planeta).unwrap();

        let planeta = planeta.trim();

    if planeta == "Tijuana" {
        println!("Tijuana: Um planeta desértico cheio de oportunidades ilegais.");

    } else if planeta == "Ganymede" {
        println!("Ganymede: Prepare-se para aventuras subaquáticas e pratos incríveis!");
    } else if planeta == "Callisto" {
        println!("Callisto: Um deserto gelado dominado por ganges perigosas.");
    } else {
        println!("Destino desconhecido. Prepare-se para o inesperado!");
    }

    println!("Você encontrou uma situação tensa! O que deseja fazer? (Investigar, evitar, confrontar)");

    let mut acao = String::new();
    io::stdin().read_line(&mut acao).unwrap();
    let acao = acao.trim();

    match acao {
        "investigar" => {
            println!("Você decide investigar a área...");
            if planeta == "Tijuana" {
                println!("Você encontra informações sobre uma valiosa carga roubada.");
            } else if planeta == "Ganymede" {
                println!("Você descobre uma criatura misteriosa que pode ser a chave para uma recompensa.");
            } else if planeta == "Callisto" {
                println!("Você descobre que uma gangue está planejando uma grande invasão.");
            }

        }
        "evitar" => println!("Você evita o conflito e segue sua jornada com segurança"),
        "confrontar" => {
            let resultado = if planeta == "Callisto" {
                "As gangues te capturaram, mas Jet te salva a tempo."
            } else {
                "Você resolve a sitauação e ganha respeito local."
            };

            println!("{}", resultado);
        }

        _ => println!("Ação inválida! Algo deu errado.")
    }

}

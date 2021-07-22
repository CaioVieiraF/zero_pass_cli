use zero_pass_backend::{ self as zpb, encrypt };
use std::io;
use std::io::prelude::*;

fn main() {

    let unique: String = input("Digite a senha única: ").expect("Falha ao ler a entrada!");
    let variable:String = input("Digite a senha variável: ").expect("Falha ao ler a entrada!");

    let method_args = encrypt::MethodArgs {
        word: unique.as_str(),
        password: variable.as_str()
    };

    let method: encrypt::Methods;

    match input("Usar o método padrão do sistema?[s/n]: ") {
        Err(why) => panic!("falha ao ler a entrada! {}", why),
        Ok(choice) => match choice.as_str() {
            "s" | "S" => {
                use toml::Value;
                use std::fs::File;
                use std::path::Path;

                let file_path = Path::new("/home/v/.config/zero_pass/config.toml");
    
                let mut file = File::open(&file_path)
                    .expect("Não foi possível abrir o arquivo de configuração!");

                let mut s = String::new();
                file.read_to_string(&mut s).expect("Não foi possível ler");

                let arq = s.parse::<Value>().expect("Erro ao ler o arquivo no formato TOML.");

                let def_met = arq["props"]["default_method"].as_str()
                    .expect("Erro: não foi ler a propriedade \"default_method\" 
                        do arquivo de configuração.");
                method = zpb::get_methods().get(def_met)
                    .expect(
                        format!("Erro: \"{}\" não é um método de criptografia conhecido.", def_met)
                        .as_str()
                    ).to_owned()(method_args);
            },

        _ => {
                let methods = zpb::get_methods();
                let method_names: Vec<&String> = methods.keys().collect();

                for i in 0..method_names.len() {
                    println!("[{}] - {}", i, method_names[i]);
                }

                let choice = input("Escolha um método de criptografia: ")
                    .expect("Falha ao ler a entrada!")
                    .parse::<usize>()
                    .expect("Erro: O valor inserido tem que ser um número!");

                method = zpb::get_methods().get(method_names[choice])
                    .expect(
                        format!("Erro: \"{}\" não é um método de criptografia conhecido.", choice)
                        .as_str()
                    )
                    .to_owned()(method_args);
            }
        }
    }

    let result:String = encrypt::gen_pass(&method);

    println!("A senha gerada é \"{}\"", result);
}


fn input(message: &str) -> io::Result<String> {

    print!("{}", message);

    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

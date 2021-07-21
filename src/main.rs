use zero_pass_backend::SymetricMethod;
use std::io;
use std::io::prelude::*;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {

    let unique: String = input("Digite a senha única: ").expect("Falha ao ler a entrada!");
    let variable:String = input("Digite a senha variável: ").expect("Falha ao ler a entrada!");


    let method: SymetricMethod;
    match input("Usar o método padrão do sistema?[s/n]: ") {
        Err(why) => panic!("falha ao ler a entrada! {}", why),
        Ok(choice) => match choice.as_str() {
            "s" | "S" => {
                use toml::Value;
                use std::fs::File;
                use std::path::Path;

                let file_path = Path::new("/home/v/.config/zero_pass/config.toml");
    
                let display = file_path.display();

                let mut file = match File::open(&file_path) {
                    Err(why) => {
                        panic!(
                            "Não foi possível abrir o arquivo de configuração!\n{}: {}", display, why
                            );
                    },
                    Ok(file) => file,
                };
                let mut s = String::new();
                match file.read_to_string(&mut s) {
                    Err(why) => panic!("Não foi possível ler {}: {}", display, why),
                    Ok(_) => {}
                }

                let arq = s.parse::<Value>().unwrap();

                let def_met = arq["props"]["default_method"].as_str();
                method = SymetricMethod::get_methods().get(def_met.unwrap()).unwrap().to_owned();
            },

        _ => {
                let methods = SymetricMethod::get_methods();
                let method_names: Vec<&String> = methods.keys().collect();

                for i in 0..method_names.len() {
                    println!("[{}] - {}", i, method_names[i]);
                }

                let choice = input("Escolha um método de criptografia: ")
                    .expect("Falha ao ler a entrada!")
                    .parse::<usize>()
                    .expect("Erro: O valor inserido tem que ser um número!");

                method = SymetricMethod::get_methods().get(method_names[choice]).unwrap().to_owned();
            }
        }
    }

    let result:String = SymetricMethod::gen_pass(&method, &unique, &variable);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(result.to_owned()).unwrap();
    println!("A senha \"{}\" foi copiada para a área de transferencia.", result);
}


fn input(message: &str) -> io::Result<String> {

    print!("{}", message);

    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

pub mod languages;

use languages::{Languages, Messages};
use std::io;
use std::io::prelude::*;
use toml::Value;
use zero_pass_backend::{self as zpb, encrypt, CipherError};

fn main() {
    let config_file: Value = load_file();
    let lang: Languages = config_file["props"]["lang"]
        .as_str()
        .unwrap()
        .parse::<Languages>()
        .unwrap();
    let mess: Messages = load_lang(lang);

    let unique: String =
        input(format!("{}: ", mess.ask_unique_pass).as_str()).expect(mess.error_input);
    let variable: String =
        input(format!("{}: ", mess.ask_variable_pass).as_str()).expect(mess.error_input);

    let method_args = encrypt::MethodArgs {
        word: unique.as_str(),
        password: variable.as_str(),
    };

    let method: encrypt::Methods;

    match input(mess.ask_get_sys_default_method) {
        Err(why) => {
            println!("{}! {}", mess.error_input, why);
            return;
        }
        Ok(choice) => {
            method = match choice.as_str() {
                "s" | "S" | "y" | "Y" => use_config_file(&mess, method_args, config_file),
                _ => chose_from_menu(&mess, method_args),
            }
        }
    }

    let result: String;

    match encrypt::gen_pass(&method) {
        Ok(s) => {
            result = s;
        }
        Err(e) => match e {
            CipherError::InvalidCharacterError => {
                println!("{:?}: {}.", e, mess.error_invalid_character);
                return;
            }
        },
    }

    println!("{} \"{}\"", mess.final_result, result);
}

fn chose_from_menu<'a>(
    mess: &Messages,
    method_args: encrypt::MethodArgs<'a>,
) -> encrypt::Methods<'a> {
    let methods = zpb::get_methods();
    let method_names: Vec<&String> = methods.keys().collect();

    for (index, i) in method_names.iter().enumerate() {
        println!("[{}] - {}", index, i);
    }

    let choice = input(format!("{}: ", mess.ask_menu_method).as_str())
        .expect(mess.error_input)
        .parse::<usize>()
        .unwrap_or_else(|_| panic!("Error: {}", mess.error_parse));

    zpb::get_methods()
        .get(method_names[choice])
        .unwrap_or_else(|| panic!("Erro: \"{}\" {}", choice, mess.error_unknown_method))
        .to_owned()(method_args)
}

fn load_lang<'a>(lang: Languages) -> Messages<'a> {
    Messages::new(lang)
}

fn load_file() -> Value {
    use std::fs::File;
    use std::path::Path;

    let file_path = Path::new("/home/v/.config/zero_pass/config.toml");

    let mut file =
        File::open(&file_path).expect("Não foi possível abrir o arquivo de configuração!");

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Não foi possível ler");

    s.parse::<Value>()
        .expect("Erro ao ler o arquivo no formato TOML.")
}

fn use_config_file<'a>(
    mess: &Messages,
    method_args: encrypt::MethodArgs<'a>,
    arq: Value,
) -> encrypt::Methods<'a> {
    let def_met = arq["props"]["default_method"]
        .as_str()
        .unwrap_or_else(|| panic!("Error: {}", mess.error_file_prop));

    zpb::get_methods()
        .get(def_met)
        .unwrap_or_else(|| panic!("Erro: \"{}\" {}", def_met, mess.error_unknown_method))
        .to_owned()(method_args)
}

fn input(message: &str) -> io::Result<String> {
    print!("{}", message);

    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

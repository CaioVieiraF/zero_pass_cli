pub mod languages;

use languages::{Languages, Messages};
use std::io;
use std::io::prelude::*;
use toml::Value;
use zero_pass_backend::{self as zpb, encrypt, CipherError};

fn main() {
    let mut config_file: Option<Value> = None;
    let lang: Languages;

    match load_file(Messages::new(Languages::EnUs)) {
        Some(f) => {
            config_file = Some(f);
            lang = config_file.clone().unwrap()["props"]["lang"]
                .as_str()
                .unwrap()
                .parse::<Languages>()
                .unwrap();
        }
        None => {
            println!("Could not load configuration file, fallbacking to en_us...");
            lang = Languages::EnUs;
        }
    }

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

    if config_file != None {
        match input(mess.ask_get_sys_default_method) {
            Err(why) => {
                println!("{}! {}", mess.error_input, why);
                return;
            }
            Ok(choice) => {
                method = match choice.as_str() {
                    "s" | "S" | "y" | "Y" => {
                        use_config_file(&mess, method_args, config_file.unwrap())
                    }
                    _ => chose_from_menu(&mess, method_args),
                }
            }
        }
    } else {
        method = chose_from_menu(&mess, method_args);
    }

    let repeat: u8;

    match input(mess.ask_repeat_method_times) {
        Err(why) => {
            println!("{}! {}", mess.error_input, why);
            return;
        }
        Ok(choice) => {
            repeat = match choice.len() {
                0 => 1,
                _ => choice
                    .parse::<u8>()
                    .unwrap_or_else(|_| panic!("{}", mess.error_number_parse)),
            }
        }
    }

    let result: String;

    match encrypt::gen_pass(&method, Some(repeat)) {
        Ok(s) => {
            result = s;
        }
        Err(e) => match e {
            CipherError::InvalidCharacterError => {
                println!("{:?}: {}.", e, mess.error_invalid_character);
                return;
            }
            _ => return,
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

fn load_file(mess: Messages) -> Option<Value> {
    use std::fs::File;
    use std::path::Path;

    let file_path = Path::new("/home/v/.config/zero_pass/config.toml");

    let file: Option<File> = match File::open(&file_path) {
        Ok(f) => Some(f),
        Err(_) => match input(mess.ask_create_file) {
            Err(why) => {
                panic!("{}! {}", mess.error_input, why);
            }
            Ok(choice) => match choice.as_str() {
                "y" | "Y" | "s" | "S" => {
                    let mut fl = File::create(&file_path).expect("");
                    fl.write(b"[props]\ndefault_method = 'Base64'\nlang = 'EnUs'")
                        .expect("Could not write to file!");

                    drop(fl);
                    Some(File::open(&file_path).expect("Could not open file. "))
                }
                _ => None,
            },
        },
    };

    let mut file: File = match file {
        Some(f) => f,
        None => return None,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Não foi possível ler");

    Some(
        s.parse::<Value>()
            .expect("Erro ao ler o arquivo no formato TOML."),
    )
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

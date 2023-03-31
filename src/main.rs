pub mod languages;

use languages::{Languages, Messages};
use std::io;
use std::io::prelude::*;
use toml::Value;
use zero_pass_backend::{ self as zpb, encrypt::{ PasswordBuilder, Unique, Variable }, CipherError };

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

    let unique: String = input(format!("{}: ", mess.ask_unique_pass)).expect(mess.error_input);
    let variable: String = input(format!("{}: ", mess.ask_variable_pass)).expect(mess.error_input);

    let password_builder = PasswordBuilder::new()
        .unique(unique)
        .variable(variable.as_str());
    
    let repeat: u8;

    match input(mess.ask_repeat_method_times) {
        Err(why) => {
            println!("{}! {why}", mess.error_input);
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

    let password_builder = password_builder.repeat(repeat);

    let password_builder = if config_file != None {
        let choice = match input(mess.ask_get_sys_default_method) {
            Err(why) => {
                println!("{}! {why}", mess.error_input);
                return;
            }
            Ok(c) => c,
        };

        let password_gen = match choice.as_str() {
            "s" | "S" | "y" | "Y" => {
                use_config_file(&mess, password_builder, config_file.unwrap())
            }
            _ => chose_from_menu(&mess, password_builder),
        };

        password_gen.unwrap_or_else(|e| {
            match e {
                CipherError::InvalidCharacterError => {
                    panic!("{e:?}: {}.", mess.error_invalid_character);
                },
                _ => panic!("Error"),
            }
        })

    } else {
        chose_from_menu(&mess, password_builder).unwrap_or_else(|e| {
            match e {
                CipherError::InvalidCharacterError => {
                    panic!("{e:?}: {}.", mess.error_invalid_character);
                },
                _ => panic!("Error"),
            }
        })
    };

    let result: String = password_builder.build();

    println!("{} \"{result}\"", mess.final_result);
}

fn chose_from_menu(
    mess: &Messages,
    password_builder: PasswordBuilder<Unique, Variable>
) -> Result<PasswordBuilder<Unique, Variable>, CipherError> {
    let method_names = zpb::Methods::get_methods();

    for (index, i) in method_names.iter().enumerate() {
        println!("[{index}] - {i}");
    }

    let choice = input(format!("{}: ", mess.ask_menu_method)).expect(mess.error_input);
    let choice = choice.parse::<usize>().unwrap_or_else(|_| panic!("{}", mess.error_number_parse));

    let method = zpb::Methods::get_method(method_names[choice]).unwrap_or_else(|_| panic!("Erro: \"{choice}\" {}", mess.error_unknown_method));

    password_builder.method_ptr(method)
}

fn load_lang(lang: Languages) -> Messages {
    Messages::new(lang)
}

fn load_file(mess: Messages) -> Option<Value> {
    use std::fs::File;

    let mut home = match dirs::home_dir() {
        Some(h) => h,
        None => return None,
    };

    home.push(".config/zero_pass/config.toml");
    let file_path = home;

    let file: Option<File> = match File::open(&file_path) {
        Ok(f) => Some(f),
        Err(_) => match input(mess.ask_create_file) {
            Err(why) => {
                panic!("{}! {why}", mess.error_input);
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

fn use_config_file(
    mess: &Messages,
    password_builder: PasswordBuilder<Unique, Variable>,
    arq: Value,
) -> Result<PasswordBuilder<Unique, Variable>, CipherError> {
    let def_met = arq["props"]["default_method"]
        .as_str()
        .unwrap_or_else(|| panic!("Error: {}", mess.error_file_prop));

    let method = zpb::Methods::get_method(def_met).unwrap_or_else(|_| panic!("Erro: \"{def_met}\" {}", mess.error_unknown_method));
    password_builder.method_ptr(method)
}

fn input(message: impl Into<String>) -> io::Result<String> {
    print!("{}", message.into());

    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

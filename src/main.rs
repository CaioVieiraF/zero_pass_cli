pub mod error;
pub mod languages;
pub mod prelude;

use std::str::FromStr;

use clap::Parser;
use inquire::{Confirm, Password, Select, Text};
use languages::{Languages, Messages};
use toml::Value;
use zero_pass_backend::{self as zpb, encrypt::PasswordBuilder};
use zpb::Methods;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The unique password used on all cases
    #[arg(short, long)]
    unique: Option<String>,
    /// The password that changes for each different service
    #[arg(short, long)]
    variable: Option<String>,
    /// The number of times to repeat a method
    #[arg(short, long, default_value_t = 1)]
    repeat: u8,
    /// Method to use for encryption
    #[arg(short, long)]
    method: Option<Methods>,
}

fn main() {
    // Define the configuration file and the language variables.
    let mut config_file: Option<Value> = None;
    let lang: Languages;

    // Tryies to load the file. If loaded successfully it sets the config_file variable and tryies
    // to get the defined language if any. If the file is not loaded, then we use the default
    // language.
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
            lang = Languages::default()
        }
    }

    // Instantiate the messages helper.
    let mess = Messages::new(lang);

    // Get the command arguments from the environment.
    let cli_args = Args::parse();

    // Get the unique pass either from command line, if specified, or from user input.
    let unique = match cli_args.unique {
        Some(u) => u,
        None => Password::new(mess.ask_unique_pass).prompt().expect(""),
    };

    // Get the variable pass either from command line, if specified, or from user input.
    let variable = match cli_args.variable {
        Some(v) => v,
        None => Text::new(mess.ask_variable_pass).prompt().expect(""),
    };

    // Start building the password with the PasswordBuilder. This must initialize with unique and
    // variable to use the other methods.
    let mut password_builder = PasswordBuilder::new()
        .unique(unique)
        .variable(variable.as_str());

    password_builder = password_builder.repeat(cli_args.repeat);

    // Get method from command line argument or prompt
    let method = match cli_args.method {
        Some(m) => m.to_method(),
        None => match config_file {
            Some(f) => {
                let method_name = f["props"]["default_method"]
                    .as_str()
                    .expect(mess.error_file_parse);
                let method = Methods::try_from(method_name).expect(mess.error_file_prop);
                method.to_method()
            }
            None => {
                let choice = Select::new(mess.ask_menu_method, Methods::get_methods()).prompt();
                let method = Methods::from_str(choice.unwrap()).expect(mess.error_unknown_method);
                method.to_method()
            }
        },
    };
    password_builder = password_builder.method_ptr(method).unwrap();
    // Get the generated password and then show to the user.
    let result: String = password_builder.build();

    println!("{} \"{result}\"", mess.final_result);
}

fn load_file(mess: Messages) -> Option<Value> {
    use std::fs::File;
    use std::io::{Read, Write};

    let mut home = dirs::home_dir()?;

    home.push(".config/zero_pass/config.toml");
    let file_path = home;
    let file = match File::open(&file_path).ok() {
        Some(file) => Some(file),
        None => match Confirm::new(mess.ask_create_file)
            .with_default(false)
            .prompt()
        {
            Ok(true) => {
                let mut file = File::create(&file_path).expect("Could not create file!");
                file.write_all(b"[props]\ndefault_method = 'Base64'\nlang = 'EnUs'")
                    .expect("Could not write to file!");
                Some(file)
            }
            _ => None,
        },
    };
    let mut s = String::new();
    file?.read_to_string(&mut s).expect("Não foi possível ler");

    Some(
        s.parse::<Value>()
            .expect("Erro ao ler o arquivo no formato TOML."),
    )
}

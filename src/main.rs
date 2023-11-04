pub mod error;
pub mod languages;
pub mod prelude;

use clap::Parser;
use languages::{Languages, Messages};
use prelude::*;
use std::io;
use toml::Value;
use zero_pass_backend::{
    self as zpb,
    encrypt::{PasswordBuilder, Unique, Variable},
};

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
    method: String,
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
        None => input(format!("{}: ", mess.ask_unique_pass)).expect(mess.error_input),
    };

    // Get the variable pass either from command line, if specified, or from user input.
    let variable = match cli_args.variable {
        Some(v) => v,
        None => input(format!("{}: ", mess.ask_variable_pass)).expect(mess.error_input),
    };

    // Start building the password with the PasswordBuilder. This must initialize with unique and
    // variable to use the other methods.
    let mut password_builder = PasswordBuilder::new()
        .unique(unique)
        .variable(variable.as_str());

    password_builder = password_builder.repeat(cli_args.repeat);

    // Get method from command line argument
    if let Ok(s) = zpb::Methods::get_method(cli_args.method) {
        password_builder = password_builder.method_ptr(s).unwrap();
    }
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
        None => match input(mess.ask_create_file) {
            Ok(choice) if ["y", "Y", "s", "S"].contains(&choice.as_str()) => {
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

fn use_config_file(
    mess: &Messages,
    password_builder: PasswordBuilder<Unique, Variable>,
    arq: Value,
) -> Result<PasswordBuilder<Unique, Variable>> {
    let def_met = arq["props"]["default_method"]
        .as_str()
        .unwrap_or_else(|| panic!("Error: {}", mess.error_file_prop));

    let method = zpb::Methods::get_method(def_met)
        .unwrap_or_else(|_| panic!("Erro: \"{def_met}\" {}", mess.error_unknown_method));
    Ok(password_builder.method_ptr(method)?)
}

fn input(message: impl Into<String>) -> Result<String> {
    use std::io::Write;
    print!("{}", message.into());

    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

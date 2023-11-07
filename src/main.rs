pub mod error;
pub mod languages;
pub mod prelude;

use std::str::FromStr;

use clap::Parser;
use copypasta::{ClipboardContext, ClipboardProvider};
use inquire::{Password, Select, Text};
use languages::Messages;
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

    /// Choose to show the result password or copy do clipboard
    #[arg(short, long)]
    show_result: bool,
}

fn main() {
    // Instantiate the messages helper that sets the language.
    let mess = Messages::new();

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
        None => {
            let choice = Select::new(mess.ask_menu_method, Methods::get_methods()).prompt();
            let method = Methods::from_str(choice.unwrap()).expect(mess.error_unknown_method);
            method.to_method()
        }
    };
    password_builder = password_builder.method_ptr(method).unwrap();
    // Get the generated password and then show to the user.
    let result: String = password_builder.build();

    if cli_args.show_result {
        println!("{} \"{result}\"", mess.final_result_show);
    } else {
        let mut clip_ctx = ClipboardContext::new().unwrap();
        clip_ctx.set_contents(result).unwrap();
        println!("{}", mess.final_result);
    }
}

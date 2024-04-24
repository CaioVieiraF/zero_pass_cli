pub mod error;
pub mod languages;
pub mod prelude;

use std::str::FromStr;

use clap::Parser;
use copypasta::{ClipboardContext, ClipboardProvider};
use inquire::{Password, Select, Text};
use languages::Messages;
use rand::{seq::SliceRandom, thread_rng};
use zero_pass_backend::{encrypt::PasswordBuilder, Methods};

#[derive(Debug, Parser)]
#[command(author, version, long_about = None)]
/// Cli client for the zero pass project.
///
/// This is the most basic implementation of the
/// library, it is mostly aimed to be a example of
/// what you can do with it.
struct Args {
    /// The unique password used on all cases
    ///
    /// Read the docs to learn more about the core concepts.
    #[arg(short, long, long_help = None)]
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

#[tokio::main]
async fn main() {
    // Instantiate the messages helper that sets the language.
    let mess = Messages::default();

    // Get the command arguments from the environment.
    let cli_args = Args::parse();

    // Get the unique pass either from command line, if specified, or from user input.
    let unique = cli_args
        .unique
        .unwrap_or_else(|| Password::new(mess.ask_unique_pass).prompt().expect(""));
    // Get the variable pass either from command line, if specified, or from user input.
    let variable = cli_args
        .variable
        .unwrap_or_else(|| Text::new(mess.ask_variable_pass).prompt().expect(""));

    // Start building the password with the PasswordBuilder. This must initialize with unique and
    // variable to use the other methods.
    let mut password_builder = PasswordBuilder::new().unique(unique).variable(variable);

    password_builder = password_builder.repeat(cli_args.repeat);

    // Get method from command line argument or prompt
    let method = match cli_args.method {
        Some(m) => m.to_method(),
        None => {
            let mut random_method_list = Methods::get_methods();
            random_method_list.shuffle(&mut thread_rng());
            // An error with the select menu is not expected to fail, so we unwrap it here.
            let choice = Select::new(mess.ask_menu_method, random_method_list)
                .prompt()
                .unwrap();
            let method = Methods::from_str(choice).expect(mess.error_unknown_method);
            method.to_method()
        }
    };
    password_builder = password_builder.method_ptr(method).await.unwrap();
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

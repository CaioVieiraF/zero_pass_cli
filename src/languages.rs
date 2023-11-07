use crate::prelude::*;
use std::env;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Languages {
    PtBr,
    #[default]
    EnUs,
}

impl From<String> for Languages {
    fn from(value: String) -> Self {
        match value.as_str() {
            "pt_BR.UTF-8" => Languages::PtBr,
            "en_US.UTF-8" => Languages::EnUs,
            _ => Languages::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Messages<'a> {
    pub ask_unique_pass: &'a str,
    pub ask_variable_pass: &'a str,
    pub ask_get_sys_default_method: &'a str,
    pub ask_menu_method: &'a str,
    pub ask_repeat_method_times: &'a str,
    pub ask_create_file: &'a str,
    pub error_parse: &'a str,
    pub error_unknown_method: &'a str,
    pub error_number_parse: &'a str,
    pub error_file_open: &'a str,
    pub error_file_parse: &'a str,
    pub error_file_read: &'a str,
    pub error_file_prop: &'a str,
    pub error_input: &'a str,
    pub error_invalid_character: &'a str,
    pub final_result: &'a str,
}

impl<'a> Default for Messages<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Messages<'a> {
    pub fn new() -> Self {
        let lang = match env::var("LANG") {
            Ok(l) => Languages::from(l),
            Err(_) => Languages::default(),
        };
        match lang {
            Languages::PtBr => Messages {
                ask_menu_method: "Escolha um método de criptografia",
                ask_unique_pass: "Digite a senha única",
                ask_variable_pass: "Digite a senha variável",
                ask_get_sys_default_method: "Usar o método padrão do sistema? ",
                ask_repeat_method_times: "Número de repetições: ",
                ask_create_file: "Você não tem um arquivo de configuração, quer criar um? ",
                error_input: "Falha ao ler a entrada!",
                error_invalid_character: "O caractere inserido é inválido",
                error_parse: "Erro: O valor inserido tem que ser um número",
                error_unknown_method: "não é um método de criptografia conhecido",
                error_number_parse: "O número inserido é inválido",
                error_file_open: "Não foi possível abrir o arquivo de configuração",
                error_file_read: "Não foi possível ler",
                error_file_parse: "Erro ao ler o arquivo no formato TOML",
                error_file_prop: "não foi ler a propriedade \"default_method\"
                        do arquivo de configuração",
                final_result: "A senha gerada é ",
            },
            Languages::EnUs => Messages {
                ask_menu_method: "Choose a cryptography method",
                ask_unique_pass: "Type your unique password",
                ask_variable_pass: "Type the variable password",
                ask_get_sys_default_method: "Use the system's default method? ",
                ask_repeat_method_times: "Number of repetitions: ",
                ask_create_file: "You don't have a configuration file, want to create one? ",
                error_input: "Failed to read input",
                error_invalid_character: "The character is invalid",
                error_parse: "Erro: the value must be a number",
                error_unknown_method: "is not a known cryptography method",
                error_number_parse: "This number is invalid",
                error_file_open: "Unable to open the file",
                error_file_read: "Unable to read the file",
                error_file_parse: "Error while parsing TOML file",
                error_file_prop: "Unable to read prop \"default_method\" from config file",
                final_result: "The generated password is",
            },
        }
    }
}

impl std::str::FromStr for Languages {
    type Err = Error;

    fn from_str(lang: &str) -> Result<Self> {
        match lang {
            "pt_br" => Ok(Languages::PtBr),
            "en_us" => Ok(Languages::EnUs),
            _ => Ok(Languages::EnUs),
        }
    }
}

mod en_us;
mod pt_br;

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
    pub final_result_show: &'a str,
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
            Languages::PtBr => pt_br::PtBr::default().0,
            Languages::EnUs => en_us::EnUs::default().0,
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

use super::Messages;

pub struct EnUs<'a>(pub Messages<'a>);

impl<'a> Default for EnUs<'a> {
    fn default() -> Self {
        EnUs(Messages {
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
            final_result: "The password is on the clipboard.",
            final_result_show: "The generated password is ",
        })
    }
}

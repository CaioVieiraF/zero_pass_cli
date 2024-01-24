use super::Messages;

pub struct PtBr<'a>(pub Messages<'a>);

impl<'a> Default for PtBr<'a> {
    fn default() -> Self {
        PtBr(Messages {
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
            final_result: "Senha copiada para a área de transferência.",
            final_result_show: "A senha gerada é ",
        })
    }
}

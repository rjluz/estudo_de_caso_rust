use regex::Regex;
use lazy_static::lazy_static;

/// Limpa e normaliza um texto:
/// - Remove pontuações
/// - Converte para minúsculas
/// - Remove espaços extras
pub fn normalizar(texto: &str) -> String {
    lazy_static! {
        static ref RE_PONTUACAO: Regex = Regex::new(r"[^\w\s]").unwrap();
        static ref RE_ESPACO: Regex = Regex::new(r"\s+").unwrap();
    }

    let sem_pontuacao = RE_PONTUACAO.replace_all(texto, "");
    let sem_espaco_extra = RE_ESPACO.replace_all(&sem_pontuacao, " ");
    sem_espaco_extra.to_lowercase().trim().to_string()
}

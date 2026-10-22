//! Formatação de apresentação; valores canônicos e IDs do histórico não são modificados.
pub fn numero(valor: f64, casas: usize) -> String {
    format!("{valor:.casas$}").replace('.', ",")
}

pub fn sinal(valor: f64, casas: usize) -> String {
    format!("{valor:+.casas$}").replace('.', ",")
}

pub fn estado(valor: &str) -> String {
    match valor {
        "OTIMO_NO_MODELO" => "Ótimo no modelo",
        "VIAVEL_SEM_OTIMO" => "Viável sem prova de ótimo",
        "INVIAVEL_NO_MODELO" => "Inviável no modelo",
        "INCONCLUSIVA" | "INCONCLUSIVO" => "Inconclusivo",
        "ATENDE" => "Atende",
        "NAO_ATENDE" => "Não atende",
        "NAO_AVALIADO" => "Não avaliado",
        "AVALIADO" => "Avaliado",
        "INVALIDO" => "Inválido",
        "BLOQUEADO" => "Bloqueado",
        "FORA_DO_PERFIL" => "Fora do perfil",
        outro => return outro.replace('_', " "),
    }
    .into()
}

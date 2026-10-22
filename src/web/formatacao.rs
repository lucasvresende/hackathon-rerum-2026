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
        "SEM_SOLUCAO" => "Sem combinação validada",
        "RESOLUCAO_PENDENTE" => "Solução contínua aproximada; informar resolução de pesagem",
        "FORNECIMENTO_VALIDADO" => "Fornecimento validado no perfil",
        "FORNECIMENTO_INCOMPATIVEL" => "Fornecimento incompatível; não usar como recomendação",
        "INVALIDO" => "Inválido",
        "BLOQUEADO" => "Bloqueado",
        "FORA_DO_PERFIL" => "Fora do perfil",
        outro => return outro.replace('_', " "),
    }
    .into()
}

/// Mantém diferenças pequenas visíveis, sem confundir arredondamento visual com igualdade.
pub fn medida(valor: f64, casas: usize) -> String {
    if valor != 0.0 && valor.abs() < 0.5 * 10_f64.powi(-(casas as i32)) {
        format!("{valor:.3e}").replace('.', ",")
    } else {
        numero(valor, casas)
    }
}

/// Valores equivalentes na tolerância do motor não recebem sinal negativo por ruído binário.
pub fn balanco(oferta: f64, requisito: f64) -> String {
    if crate::dominio::unidades::proximos(oferta, requisito) {
        numero(0.0, 5)
    } else {
        let delta = oferta - requisito;
        format!("{}{}", if delta > 0.0 { "+" } else { "" }, medida(delta, 5))
    }
}

/// Representação que permite copiar a pesagem sem perder casas decimais validadas.
pub fn quantidade_pratica(valor: f64) -> String {
    valor.to_string().replace('.', ",")
}

use serde::{Deserialize, Serialize};

use super::unidades::preco_f64;
use super::{Alimento, Cenario, Lote};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problema {
    pub campo: String,
    pub mensagem: String,
}

pub fn problema(campo: &str, mensagem: &str) -> Problema {
    Problema {
        campo: campo.into(),
        mensagem: mensagem.into(),
    }
}

pub fn validar_lote(lote: &Lote) -> Vec<Problema> {
    let mut erros = Vec::new();
    if lote.nome.trim().is_empty() || lote.nome.len() > 150 {
        erros.push(problema("nome", "Informe um nome de até 150 caracteres."));
    }
    if lote.animais == 0 {
        erros.push(problema(
            "animais",
            "O número de animais deve ser inteiro positivo.",
        ));
    }
    for (campo, valor, positivo) in [
        ("peso", lote.peso, true),
        ("producao", lote.producao, false),
        ("gordura", lote.gordura, true),
    ] {
        if !valor.is_finite() || valor < 0.0 || (positivo && valor == 0.0) {
            erros.push(problema(
                campo,
                "Informe um número finito na faixa permitida.",
            ));
        }
    }
    for (campo, valor) in [
        ("dispersao_peso", lote.dispersao_peso),
        ("condicao_corporal", lote.condicao_corporal),
    ] {
        if valor.is_some_and(|v| !v.is_finite() || v < 0.0) {
            erros.push(problema(campo, "Informe um valor finito não negativo."));
        }
    }
    erros
}

pub fn validar_alimento(a: &Alimento) -> Vec<Problema> {
    let mut erros = Vec::new();
    if a.nome.trim().is_empty() || a.evidencia.len() > 4000 {
        erros.push(problema(
            "nome",
            "Informe identidade e evidência com tamanho válido.",
        ));
    }
    for (campo, valor) in [
        ("fonte_id", &a.fonte_id),
        ("metodo", &a.metodo),
        ("origem", &a.origem),
    ] {
        if valor.trim().is_empty() {
            erros.push(problema(
                campo,
                "Preenchimento obrigatório para identificar a composição.",
            ));
        }
    }
    if ![
        "ANALISE",
        "MEDIA_TABELADA",
        "SINTETICA",
        "GARANTIA_MINIMA",
        "GARANTIA_MAXIMA",
    ]
    .contains(&a.natureza.as_str())
    {
        erros.push(problema("natureza", "Selecione a natureza da composição."));
    }
    if !a.ms.is_finite() || a.ms <= 0.0 || a.ms > 1.0 {
        erros.push(problema("ms", "MS deve estar entre 0 (exclusive) e 100%."));
    }
    for (i, fracao) in a.nutrientes.iter().enumerate() {
        if fracao.is_some_and(|x| !x.is_finite() || x < 0.0 || (i != 0 && x > 1.0)) {
            erros.push(problema(
                ["ndt", "pb", "ca", "p", "fdn", "amido", "ee"][i],
                "Teor incompatível com a base em fração de MS.",
            ));
        }
    }
    if a.rotulo_max_ms.is_some_and(|v| !v.is_finite() || v < 0.0) {
        erros.push(problema(
            "rotulo_max_ms",
            "O teto do rótulo em kg MS deve ser não negativo.",
        ));
    }
    erros
}

pub fn validar(c: &Cenario) -> Vec<Problema> {
    let mut erros = validar_lote(&c.lote);
    for (campo, data) in [
        ("data_avaliacao", &c.data_avaliacao),
        ("data_aplicacao", &c.data_aplicacao),
    ] {
        if chrono::NaiveDate::parse_from_str(data, "%Y-%m-%d").is_err() || data.len() != 10 {
            erros.push(problema(campo, "Informe uma data civil válida."));
        }
    }
    if let Some(m) = &c.medicao
        && (!m.oferta_ms.is_finite()
            || !m.sobras_ms.is_finite()
            || !m.periodo_dias.is_finite()
            || m.oferta_ms <= 0.0
            || m.sobras_ms < 0.0
            || m.sobras_ms >= m.oferta_ms
            || m.periodo_dias <= 0.0
            || m.animais != c.lote.animais
            || m.animais == 0
            || c.consumo_alvo.is_some())
    {
        erros.push(problema(
            "medicao",
            "Oferta, sobras, período ou animais incompatíveis; \
                não combine consumo-alvo informado e medição observada.",
        ));
    }
    if c.horizonte == 0 {
        erros.push(problema(
            "horizonte",
            "O horizonte deve ser inteiro positivo.",
        ));
    }
    if !c.sobras.is_finite() || !(0.0..1.0).contains(&c.sobras) {
        erros.push(problema(
            "sobras",
            "Sobras devem estar entre 0 e 100% (exclusive).",
        ));
    }
    for (campo, valor) in [
        ("origem_sobras", &c.origem_sobras),
        ("origem_consumo", &c.origem_consumo),
    ] {
        if valor.trim().is_empty() {
            erros.push(problema(campo, "Registre a origem ou hipótese explícita."));
        }
    }
    if c.consumo_alvo.is_some_and(|v| !v.is_finite() || v <= 0.0) {
        erros.push(problema(
            "consumo_alvo",
            "O consumo-alvo deve ser positivo e finito.",
        ));
    }
    if !c.limite_segundos.is_finite() || c.limite_segundos <= 0.0 || c.limite_segundos > 10.0 {
        erros.push(problema(
            "limite",
            "O limite de formulação deve estar entre 0 e 10 segundos.",
        ));
    }
    if c.itens.is_empty() || c.itens.len() > if c.formular { 10 } else { 20 } {
        erros.push(problema(
            "itens",
            "Selecione de 1 a 10 ingredientes para formular ou 20 para verificar.",
        ));
    }
    erros.extend(super::validacao_itens::validar(c));
    if c.preco_leite
        .as_ref()
        .is_some_and(|p| preco_f64(p).is_none_or(|v| v < 0.0))
    {
        erros.push(problema("preco_leite", "Preço do leite inválido em R$/kg."));
    }
    erros
}

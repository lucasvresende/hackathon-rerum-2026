use std::collections::BTreeSet;

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
    if !a.ms.is_finite() || a.ms <= 0.0 || a.ms > 1.0 {
        erros.push(problema("ms", "MS deve estar entre 0 (exclusive) e 100%."));
    }
    for (i, fracao) in a.nutrientes.iter().enumerate() {
        if fracao.is_some_and(|x| !x.is_finite() || x < 0.0 || (i != 0 && x > 1.0)) {
            erros.push(problema(
                "nutrientes",
                "Teor incompatível com a base em fração de MS.",
            ));
        }
    }
    if a.rotulo_max_ms.is_some_and(|v| !v.is_finite() || v < 0.0) {
        erros.push(problema(
            "rotulo",
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
    if c.origem_sobras.trim().is_empty() || c.origem_consumo.trim().is_empty() {
        erros.push(problema(
            "premissas",
            "Registre a origem do consumo e das sobras.",
        ));
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
    let mut ids = BTreeSet::new();
    for (i, item) in c.itens.iter().enumerate() {
        let campo = format!("item_{i}");
        if !ids.insert(&item.alimento.id) {
            erros.push(problema(
                &campo,
                "Selecione somente uma composição de cada ingrediente.",
            ));
        }
        erros.extend(
            validar_alimento(&item.alimento)
                .into_iter()
                .map(|mut erro| {
                    erro.campo = campo.clone();
                    erro
                }),
        );
        if !item.quantidade.is_finite()
            || item.quantidade < 0.0
            || !item.perda.is_finite()
            || !(0.0..1.0).contains(&item.perda)
            || !item.minimo_ms.is_finite()
            || item.minimo_ms < 0.0
            || item
                .maximo_ms
                .is_some_and(|v| !v.is_finite() || v < item.minimo_ms)
            || item.estoque.is_some_and(|v| !v.is_finite() || v < 0.0)
            || !item.resolucao_mn.is_finite()
            || item.resolucao_mn < 0.0
        {
            erros.push(problema(
                &campo,
                "Quantidade, perda, estoque ou limites inválidos.",
            ));
        }
        if preco_f64(&item.preco).is_none_or(|v| v < 0.0) || item.origem_preco.trim().is_empty() {
            erros.push(problema(
                &campo,
                "Informe preço decimal não negativo e sua origem.",
            ));
        }
        if item.sem_limite && item.estoque.is_some() {
            erros.push(problema(
                &campo,
                "Escolha estoque informado ou hipótese sem limite.",
            ));
        }
    }
    if c.preco_leite
        .as_ref()
        .is_some_and(|p| preco_f64(p).is_none_or(|v| v < 0.0))
    {
        erros.push(problema("preco_leite", "Preço do leite inválido em R$/kg."));
    }
    erros
}

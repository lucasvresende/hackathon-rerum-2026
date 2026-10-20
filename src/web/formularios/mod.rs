mod cadastros;
mod cenario;

pub use cadastros::*;
pub use cenario::*;

use crate::dominio::Conhecido;
use crate::dominio::validacao::{Problema, problema};
use std::collections::BTreeMap;

pub type Campos = BTreeMap<String, String>;

pub fn texto<'a>(c: &'a Campos, nome: &str) -> &'a str {
    c.get(nome).map(String::as_str).unwrap_or("").trim()
}

fn numero(c: &Campos, nome: &str, erros: &mut Vec<Problema>) -> Option<f64> {
    let valor = texto(c, nome);
    if valor.is_empty() {
        return None;
    }
    match valor.replace(',', ".").parse::<f64>() {
        Ok(v) if v.is_finite() => Some(v),
        _ => {
            erros.push(problema(nome, "Informe um número finito válido."));
            None
        }
    }
}

fn obrigatorio(c: &Campos, nome: &str, erros: &mut Vec<Problema>) -> f64 {
    numero(c, nome, erros).unwrap_or_else(|| {
        erros.push(problema(nome, "Campo numérico obrigatório."));
        0.0
    })
}

fn inteiro(c: &Campos, nome: &str, erros: &mut Vec<Problema>) -> u32 {
    let valor = obrigatorio(c, nome, erros);
    if valor < 0.0 || valor.fract() != 0.0 || valor > u32::MAX as f64 {
        erros.push(problema(nome, "Informe um inteiro não negativo."));
        return 0;
    }
    valor as u32
}

fn estado(c: &Campos, nome: &str) -> Conhecido {
    match texto(c, nome) {
        "SIM" => Conhecido::Sim,
        "NAO" => Conhecido::Nao,
        _ => Conhecido::Desconhecido,
    }
}

fn campos_serializados<T: serde::Serialize>(valor: &T) -> Campos {
    let mut campos = Campos::new();
    if let Ok(serde_json::Value::Object(mapa)) = serde_json::to_value(valor) {
        for (nome, v) in mapa {
            let texto = match v {
                serde_json::Value::Null => String::new(),
                serde_json::Value::String(s) => s,
                outro => outro.to_string(),
            };
            campos.insert(nome, texto);
        }
    }
    campos
}

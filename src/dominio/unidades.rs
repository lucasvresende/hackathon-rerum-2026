use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};

pub const TOLERANCIA: f64 = 1e-8;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MassaMs(pub f64);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MassaMn(pub f64);

impl MassaMn {
    pub fn materia_seca(self, fracao_ms: f64) -> MassaMs {
        MassaMs(self.0 * fracao_ms)
    }
}

impl MassaMs {
    pub fn materia_natural(self, fracao_ms: f64) -> MassaMn {
        MassaMn(self.0 / fracao_ms)
    }
}

pub fn proximos(a: f64, b: f64) -> bool {
    (a - b).abs() <= TOLERANCIA + TOLERANCIA * a.abs().max(b.abs())
}

pub fn preco_f64(preco: &str) -> Option<f64> {
    preco.parse::<Decimal>().ok()?.to_f64()
}

/// Preço guarda a escala decimal original; centavos são arredondados só na apresentação.
pub fn dinheiro(valor: f64) -> String {
    Decimal::from_f64_retain(valor)
        .map(|v| v.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        .map(|v| format!("R$ {v:.2}").replace('.', ","))
        .unwrap_or_else(|| "Não calculável".into())
}

/// Normaliza preço por tonelada/embalagem com aritmética decimal, preservando a cotação original.
pub fn normalizar_preco(cotacao: &super::CotacaoOriginal) -> Option<String> {
    let valor: Decimal = cotacao.valor.parse().ok()?;
    if valor < Decimal::ZERO {
        return None;
    }
    let divisor = match cotacao.unidade.as_str() {
        "" | "kg" => Decimal::ONE,
        "t" => Decimal::from(1000),
        "embalagem" => {
            let massa = cotacao
                .massa_embalagem
                .filter(|m| m.is_finite() && *m > 0.0)?;
            Decimal::from_f64_retain(massa)?
        }
        _ => return None,
    };
    Some((valor / divisor).normalize().to_string())
}

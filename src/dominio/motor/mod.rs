//! Entrada única para os modos de verificação e formulação do motor especialista.

use serde::{Deserialize, Serialize};

use super::*;

mod avaliacoes;
pub mod calculos;
pub mod contaminantes;
#[path = "formulacao/diagnostico_otimizacao.rs"]
mod diagnostico_otimizacao;
pub mod elegibilidade;
pub mod formulacao;
mod fornecimento;
pub mod inferencia;
#[path = "formulacao/linear.rs"]
mod linear;
pub use formulacao as otimizacao;

/// Entradas congeladas enviadas ao processo que executa a formulação.
#[derive(Serialize, Deserialize)]
pub struct PedidoMotor {
    pub cenario: Cenario,
    pub conhecimento: Conhecimento,
}

/// Avalia o cenário no modo solicitado, sem acesso a HTTP ou persistência.
///
/// O chamador pode executar a formulação em um processo separado para impor o limite de tempo.
pub fn avaliar(cenario: &Cenario, conhecimento: &Conhecimento) -> Parecer {
    if cenario.formular {
        otimizacao::formular(cenario, conhecimento)
    } else {
        inferencia::verificar(cenario, conhecimento)
    }
}

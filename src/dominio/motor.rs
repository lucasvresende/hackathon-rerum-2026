//! Entrada única para os modos de verificação e formulação do motor especialista.

use serde::{Deserialize, Serialize};

use super::{Cenario, Conhecimento, Parecer, inferencia, otimizacao};

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

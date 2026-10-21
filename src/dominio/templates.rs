use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModeloPreenchimento {
    pub id: String,
    pub versao: String,
    pub nome: String,
    pub tipo: String,
    pub contexto: String,
    pub lacunas: String,
    pub fonte_id: String,
    pub fonte_titulo: String,
    pub fonte_url: String,
    pub valores: Vec<ValorModelo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValorModelo {
    pub campo: String,
    pub valor: String,
    pub unidade: String,
    pub valor_original: String,
    pub unidade_original: String,
    pub natureza: String,
    pub fonte_id: String,
}

/// Snapshot da referência e dos valores usados, nunca aceito como verdade do navegador.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcedenciaModelo {
    pub modelo: ModeloPreenchimento,
    pub campos: Vec<CampoPreenchido>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampoPreenchido {
    pub campo: String,
    pub utilizado: String,
    pub alterado: bool,
}

/// Lista fechada de campos; condições assumidas exigem natureza sintética na validação.
pub fn unidade_campo(tipo: &str, campo: &str) -> Option<&'static str> {
    match (tipo, campo) {
        ("lote", "peso") => Some("kg"),
        ("lote", "producao") => Some("kg/animal/dia"),
        ("lote", "gordura") => Some("%"),
        ("lote", "raca")
        | (
            "alimento",
            "nome" | "classe" | "origem" | "processamento" | "natureza" | "metodo" | "fonte_id",
        ) => Some("texto"),
        ("alimento", "ms") => Some("% MN"),
        ("alimento", "ndt" | "pb" | "ca" | "p" | "fdn" | "amido" | "ee") => Some("% MS"),
        (
            "lote",
            "nome" | "animais" | "idade_meses" | "estado_produtivo" | "destinacao"
            | "dias_lactacao" | "adulta" | "gestante" | "crescimento" | "peso_estavel"
            | "confinada" | "correcao_ambiente" | "correcao_atividade" | "agua" | "ordem_parto"
            | "dispersao_peso" | "condicao_corporal" | "observacoes",
        )
        | (
            "alimento",
            "data" | "origem_animal" | "excecao_animal" | "nnp_adicionado" | "medicamento"
            | "autorizado" | "evidencia" | "fornecedor" | "lote_fisico" | "rotulo_max_ms",
        ) => Some("texto"),
        _ => None,
    }
}

pub fn equivalentes(a: &str, b: &str) -> bool {
    match (
        a.replace(',', ".").parse::<f64>(),
        b.replace(',', ".").parse::<f64>(),
    ) {
        (Ok(a), Ok(b)) => (a - b).abs() <= 8.0 * f64::EPSILON * a.abs().max(b.abs()).max(1.0),
        _ => a.trim() == b.trim(),
    }
}

/// Recursos que aceitam referências de preenchimento; não altera o formato persistido.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TipoCadastro {
    Lote,
    Alimento,
}
impl TipoCadastro {
    pub fn nome(self) -> &'static str {
        match self {
            Self::Lote => "lote",
            Self::Alimento => "alimento",
        }
    }
}

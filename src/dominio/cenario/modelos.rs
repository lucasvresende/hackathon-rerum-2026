//! Tipos de domínio para lotes, alimentos, cenários, conhecimento e regras.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Conhecido {
    Sim,
    Nao,
    Desconhecido,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lote {
    #[serde(default)]
    pub procedencia: Option<super::templates::ProcedenciaModelo>,
    pub id: String,
    pub revisao_id: String,
    pub nome: String,
    pub animais: u32,
    pub raca: String,
    pub idade_meses: Option<u32>,
    #[serde(default)]
    pub estado_produtivo: EstadoProdutivo,
    #[serde(default)]
    pub destinacao: String,
    pub peso: f64,
    pub producao: f64,
    pub gordura: f64,
    pub dias_lactacao: u32,
    pub adulta: Conhecido,
    pub gestante: Conhecido,
    pub crescimento: Conhecido,
    pub peso_estavel: Conhecido,
    pub confinada: Conhecido,
    pub correcao_ambiente: Conhecido,
    pub correcao_atividade: Conhecido,
    pub agua: Conhecido,
    pub ordem_parto: Option<u32>,
    pub dispersao_peso: Option<f64>,
    pub condicao_corporal: Option<f64>,
    pub observacoes: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Classe {
    Forragem,
    Concentrado,
    Suplemento,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Alimento {
    #[serde(default)]
    pub procedencia: Option<super::templates::ProcedenciaModelo>,
    pub id: String,
    pub composicao_id: String,
    pub nome: String,
    pub classe: Classe,
    pub ms: f64,
    /// Ordem canônica: NDT, PB, Ca, P, FDN, amido, EE (frações de MS).
    pub nutrientes: [Option<f64>; 7],
    pub fonte_id: String,
    pub natureza: String,
    pub origem: String,
    pub processamento: String,
    pub metodo: String,
    pub data: String,
    pub origem_animal: Conhecido,
    pub excecao_animal: Conhecido,
    pub nnp_adicionado: Conhecido,
    pub medicamento: Conhecido,
    pub autorizado: Conhecido,
    pub evidencia: String,
    pub fornecedor: String,
    pub lote_fisico: String,
    pub rotulo_max_ms: Option<f64>,
    pub pendf: Option<f64>,
    pub metodo_fisico: String,
    pub contaminantes: Vec<Laudo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Laudo {
    pub analito: String,
    pub matriz: String,
    pub valor: f64,
    pub unidade: String,
    pub ms: f64,
    pub qualificador: String,
    pub loq: Option<f64>,
    pub metodo: String,
    pub documento: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BaseQuantidade {
    ConsumoMs,
    ConsumoMn,
    OfertaMn,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub alimento: Alimento,
    pub quantidade: f64,
    pub base: BaseQuantidade,
    pub preco: String,
    #[serde(default)]
    pub cotacao_original: Option<CotacaoOriginal>,
    pub origem_preco: String,
    pub perda: f64,
    pub estoque: Option<f64>,
    pub sem_limite: bool,
    pub minimo_ms: f64,
    pub maximo_ms: Option<f64>,
    pub resolucao_mn: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cenario {
    pub nome: String,
    pub original_id: Option<String>,
    pub lote: Lote,
    pub perfil_id: String,
    pub data_avaliacao: String,
    pub data_aplicacao: String,
    pub horizonte: u32,
    pub sobras: f64,
    pub origem_sobras: String,
    pub consumo_alvo: Option<f64>,
    #[serde(default)]
    pub medicao: Option<MedicaoConsumo>,
    pub origem_consumo: String,
    pub referencia_ue: bool,
    pub preco_leite: Option<String>,
    pub base_comparacao: String,
    pub formular: bool,
    pub limite_segundos: f64,
    pub itens: Vec<Item>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coeficiente {
    pub tabela: String,
    pub eixo: f64,
    pub eixo2: f64,
    pub valores: [f64; 4],
    pub unidade: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fonte {
    pub id: String,
    pub titulo: String,
    pub edicao: String,
    pub localizador: String,
    pub natureza: String,
    pub conteudo: String,
    pub hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Norma {
    pub id: String,
    pub fonte_id: String,
    pub natureza: String,
    pub jurisdicao: String,
    pub publicacao: String,
    pub inicio: String,
    pub fim: Option<String>,
    pub transicao: Option<String>,
    pub condicoes: String,
    pub excecoes: String,
    pub evidencia: String,
    pub acao: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Conhecimento {
    pub perfil_id: String,
    pub versao: String,
    pub fdn_min: f64,
    pub amido_max: f64,
    pub ee_max: f64,
    pub discrepancia: f64,
    pub coeficientes: Vec<Coeficiente>,
    pub fontes: Vec<Fonte>,
    pub normas: Vec<Norma>,
    #[serde(default)]
    pub limites_contaminantes: Vec<LimiteContaminante>,
    pub versao_app: String,
    pub metodo: String,
}

impl Norma {
    pub fn vigente(&self, data: &str) -> bool {
        data >= self.inicio.as_str() && self.fim.as_ref().is_none_or(|fim| data < fim.as_str())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CotacaoOriginal {
    pub valor: String,
    pub unidade: String,
    pub massa_embalagem: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MedicaoConsumo {
    pub oferta_ms: f64,
    pub sobras_ms: f64,
    pub periodo_dias: f64,
    pub animais: u32,
}

impl Cenario {
    pub fn consumo_informado(&self) -> Option<f64> {
        self.consumo_alvo.or_else(|| {
            self.medicao
                .as_ref()
                .map(|m| (m.oferta_ms - m.sobras_ms) / (m.animais as f64 * m.periodo_dias))
        })
    }
}

impl Conhecido {
    pub fn e(self, outro: Self) -> Self {
        match (self, outro) {
            (Self::Nao, _) | (_, Self::Nao) => Self::Nao,
            (Self::Sim, Self::Sim) => Self::Sim,
            _ => Self::Desconhecido,
        }
    }
    pub fn ou(self, outro: Self) -> Self {
        match (self, outro) {
            (Self::Sim, _) | (_, Self::Sim) => Self::Sim,
            (Self::Nao, Self::Nao) => Self::Nao,
            _ => Self::Desconhecido,
        }
    }
    pub fn negar(self) -> Self {
        match self {
            Self::Sim => Self::Nao,
            Self::Nao => Self::Sim,
            Self::Desconhecido => Self::Desconhecido,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EstadoProdutivo {
    Lactacao,
    Seca,
    Transicao,
    Recria,
    #[default]
    Desconhecido,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LimiteContaminante {
    pub norma_id: String,
    pub analito: String,
    pub matriz: String,
    pub limite: f64,
    pub umidade: f64,
    pub unidade: String,
}

impl Item {
    /// Exemplos sintéticos admitem teto explícito do cenário, sem inventar rótulo comercial.
    pub fn teto_suplemento(&self) -> Option<f64> {
        self.alimento.rotulo_max_ms.or_else(|| {
            if self.alimento.natureza == "SINTETICA" {
                self.maximo_ms
            } else {
                None
            }
        })
    }
}

impl Cenario {
    /// Origem determinada pelas entradas estruturadas; o comentário livre é preservado à parte.
    pub fn tipo_consumo(&self) -> &'static str {
        if self.consumo_alvo.is_some() {
            "Informado pelo operador"
        } else if self.medicao.is_some() {
            "Observado por pesagem do lote"
        } else {
            "Estimado pelo perfil"
        }
    }
}

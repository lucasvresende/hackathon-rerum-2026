//! Estados, verificações e estrutura explicável do resultado do motor.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Estado {
    Atende,
    Viola,
    Desconhecido,
    NaoAplicavel,
}

impl Estado {
    pub fn rotulo(self) -> &'static str {
        match self {
            Self::Atende => "Atende",
            Self::Viola => "Viola",
            Self::Desconhecido => "Desconhecido",
            Self::NaoAplicavel => "Não aplicável",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Verificacao {
    pub regra: String,
    pub dimensao: String,
    pub estado: Estado,
    pub valor: Option<f64>,
    pub limite: Option<f64>,
    pub unidade: String,
    pub fonte: String,
    pub explicacao: String,
    pub dependencias: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fornecimento {
    pub ingrediente: String,
    pub consumo_ms: f64,
    pub oferta_mn: f64,
    pub oferta_lote: f64,
    pub necessidade_bruta: f64,
    pub pratico_mn: f64,
    pub custo: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subproblema {
    pub limiar: f64,
    pub estado: String,
    pub motivo: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parecer {
    pub condicao: String,
    pub lcg4: f64,
    pub consumo_referencia: Option<f64>,
    pub consumo_ms: f64,
    pub requisitos: Option<[f64; 4]>,
    pub totais: [Option<f64>; 7],
    pub fdn_forragem: Option<f64>,
    pub custo: Option<f64>,
    pub verificacoes: Vec<Verificacao>,
    pub fornecimento: Vec<Fornecimento>,
    pub busca: Option<String>,
    pub subproblemas: Vec<Subproblema>,
    pub pratico_valido: bool,
    pub duracao_ms: u64,
    pub avisos: Vec<String>,
    #[serde(default)]
    pub caminho: Vec<String>,
}

impl Default for Parecer {
    fn default() -> Self {
        Self {
            condicao: "EM_AVALIACAO".into(),
            lcg4: 0.0,
            consumo_referencia: None,
            consumo_ms: 0.0,
            requisitos: None,
            totais: [None; 7],
            fdn_forragem: None,
            custo: None,
            verificacoes: Vec::new(),
            fornecimento: Vec::new(),
            busca: None,
            subproblemas: Vec::new(),
            pratico_valido: false,
            duracao_ms: 0,
            avisos: Vec::new(),
            caminho: Vec::new(),
        }
    }
}

impl Parecer {
    pub fn registrar(&mut self, regra: &str, dimensao: &str, estado: Estado, texto: String) {
        let fonte = match regra.split('-').next().unwrap_or("") {
            "NUT" | "ESC" => "C01 · perfil v1",
            "LEG" => "N06/N03/N07 · BR_LEGAL · versão preservada",
            "CON" => "N11 · REFERENCIA_ESTRANGEIRA · versão preservada",
            _ => "REQUISITOS · POLITICA_MVP v1",
        };
        self.verificacoes.push(Verificacao {
            regra: regra.into(),
            dimensao: dimensao.into(),
            estado,
            valor: None,
            limite: None,
            unidade: String::new(),
            fonte: fonte.into(),
            explicacao: texto,
            dependencias: Vec::new(),
        });
    }

    pub fn comparar(&mut self, dado: Comparacao<'_>) {
        let estado = match dado.valor {
            Some(v) if dado.maximo && v <= dado.limite => Estado::Atende,
            Some(v) if !dado.maximo && v >= dado.limite => Estado::Atende,
            Some(v) if !dado.sanitario && super::unidades::proximos(v, dado.limite) => {
                Estado::Atende
            }
            Some(_) => Estado::Viola,
            None => Estado::Desconhecido,
        };
        self.registrar(dado.regra, dado.dimensao, estado, dado.texto.to_string());
        let ultima = self
            .verificacoes
            .last_mut()
            .expect("verificação recém-inserida");
        ultima.valor = dado.valor;
        ultima.limite = Some(dado.limite);
        ultima.unidade = dado.unidade.into();
        ultima.dependencias = vec![
            "entradas normalizadas".into(),
            "coeficientes congelados".into(),
        ];
    }

    pub fn dimensao(&self, nome: &str) -> &'static str {
        let dados: Vec<_> = self
            .verificacoes
            .iter()
            .filter(|v| v.dimensao == nome)
            .collect();
        if dados.iter().any(|v| v.estado == Estado::Viola) {
            "NAO_ATENDE"
        } else if dados.iter().any(|v| v.estado == Estado::Desconhecido) {
            "INCONCLUSIVO"
        } else if dados.iter().any(|v| v.estado == Estado::Atende) {
            "ATENDE"
        } else {
            "NAO_AVALIADO"
        }
    }
}

pub struct Comparacao<'a> {
    pub regra: &'a str,
    pub dimensao: &'a str,
    pub valor: Option<f64>,
    pub limite: f64,
    pub maximo: bool,
    pub sanitario: bool,
    pub unidade: &'a str,
    pub texto: &'a str,
}

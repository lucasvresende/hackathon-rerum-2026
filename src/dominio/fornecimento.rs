//! Distingue o certificado contínuo da recomendação de pesagem, inclusive no histórico.
use super::unidades::TOLERANCIA;
use super::{Cenario, Parecer};

impl Parecer {
    /// Resolução zero mantém a aproximação contínua; somente ingredientes usados exigem pesagem.
    /// Históricos antigos sem ID de composição usam o nome apenas quando não há ambiguidade.
    pub fn resolucao_pendente(&self, c: &Cenario) -> bool {
        c.formular
            && self.fornecimento.iter().any(|f| {
                if f.consumo_ms <= TOLERANCIA {
                    return false;
                }
                let itens: Vec<_> = c
                    .itens
                    .iter()
                    .filter(|i| {
                        if f.composicao_id.is_empty() {
                            i.alimento.nome == f.ingrediente
                        } else {
                            i.alimento.composicao_id == f.composicao_id
                        }
                    })
                    .collect();
                itens.len() != 1 || itens[0].resolucao_mn <= 0.0
            })
    }

    pub fn estado_fornecimento(&self, c: &Cenario) -> &'static str {
        if self.busca.is_some() && self.custo_resultado().is_none() {
            "SEM_SOLUCAO"
        } else if !self.pratico_valido {
            "FORNECIMENTO_INCOMPATIVEL"
        } else if self.resolucao_pendente(c) {
            "RESOLUCAO_PENDENTE"
        } else {
            "FORNECIMENTO_VALIDADO"
        }
    }

    pub fn fornecimento_recomendado(&self, c: &Cenario) -> bool {
        self.estado_fornecimento(c) == "FORNECIMENTO_VALIDADO"
    }
}

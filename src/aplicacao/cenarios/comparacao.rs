//! Comparação de cenários salvos e diferenças econômicas entre execuções.
use crate::{
    Resultado,
    dominio::unidades::proximos,
    persistencia::historico::{self, Execucao},
};
use sqlx::SqlitePool;

pub struct Economia {
    pub absoluta: f64,
    pub percentual: Option<f64>,
    pub alternativa_validada: bool,
}

pub struct Comparacao {
    pub referencia: Execucao,
    pub alternativa: Execucao,
    pub mesma_revisao: bool,
    pub equivalentes: bool,
    pub motivos: Vec<String>,
    pub economia: Option<Economia>,
}
impl Comparacao {
    pub fn entre(a: Execucao, b: Execucao) -> Self {
        let mesma_revisao = !a.cenario.lote.id.is_empty()
            && !a.cenario.lote.revisao_id.is_empty()
            && a.cenario.lote.id == b.cenario.lote.id
            && a.cenario.lote.revisao_id == b.cenario.lote.revisao_id;
        let mut motivos = Vec::new();
        for (igual, motivo) in [
            (mesma_revisao, "Lote ou revisão diferentes"),
            (
                a.cenario.perfil_id == b.cenario.perfil_id,
                "Perfil diferente",
            ),
            (
                proximos(a.cenario.lote.producao, b.cenario.lote.producao),
                "Produção diferente",
            ),
            (
                proximos(a.parecer.consumo_ms, b.parecer.consumo_ms),
                "Consumo diferente",
            ),
            (
                a.cenario.horizonte == b.cenario.horizonte,
                "Horizonte diferente",
            ),
            (
                a.cenario.referencia_ue == b.cenario.referencia_ue,
                "Referência UE diferente",
            ),
            (
                a.cenario.data_avaliacao == b.cenario.data_avaliacao,
                "Data de avaliação diferente",
            ),
            (
                a.cenario.data_aplicacao == b.cenario.data_aplicacao,
                "Data de aplicação das regras diferente",
            ),
        ] {
            if !igual {
                motivos.push(motivo.into());
            }
        }
        let diferencas = diferencas_conhecimento(&a, &b);
        motivos.extend(
            diferencas
                .iter()
                .map(|d| format!("{} diferentes", d.rotulo)),
        );
        let equivalentes = motivos.is_empty();
        let economia = a
            .parecer
            .custo_resultado()
            .zip(b.parecer.custo_resultado())
            .filter(|_| mesma_revisao)
            .map(|(ca, cb)| Economia {
                absoluta: ca - cb,
                percentual: (ca > 0.0).then(|| 100.0 * (ca - cb) / ca),
                alternativa_validada: b.parecer.fornecimento_recomendado(&b.cenario),
            });
        Self {
            referencia: a,
            alternativa: b,
            mesma_revisao,
            equivalentes,
            motivos,
            economia,
        }
    }
}

pub async fn comparar(pool: &SqlitePool, a: &str, b: &str) -> Resultado<Comparacao> {
    let a = historico::obter(pool, a).await?;
    let b = historico::obter(pool, b).await?;
    Ok(Comparacao::entre(a, b))
}

#[path = "comparacao_fornecimento.rs"]
mod fornecimento;
pub use fornecimento::{fornecimento, tem_resultado};

/// Compara toda a estrutura preservada, incluindo conteúdo e hashes das fontes.
/// A ordem das listas é preservada; diferenças nunca são descartadas por um hash resumido.
pub struct DiferencaConhecimento {
    pub rotulo: String,
    pub referencia: serde_json::Value,
    pub alternativa: serde_json::Value,
}

pub fn diferencas_conhecimento(a: &Execucao, b: &Execucao) -> Vec<DiferencaConhecimento> {
    let valores = (
        serde_json::to_value(&a.conhecimento),
        serde_json::to_value(&b.conhecimento),
    );
    let (Ok(serde_json::Value::Object(a)), Ok(serde_json::Value::Object(b))) = valores else {
        return vec![DiferencaConhecimento {
            rotulo: "Conhecimentos não comparáveis".into(),
            referencia: serde_json::Value::Null,
            alternativa: serde_json::Value::Null,
        }];
    };
    a.keys()
        .chain(b.keys())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .filter(|k| a.get(*k) != b.get(*k))
        .map(|k| {
            let rotulo = match k.as_str() {
                "perfil_id" => "Identificadores do perfil",
                "versao" => "Versões do perfil",
                "metodo" => "Métodos",
                "versao_app" => "Versões da aplicação",
                "coeficientes" => "Coeficientes preservados",
                "fontes" => "Fontes preservadas (conteúdo e hashes)",
                "normas" => "Normas preservadas",
                "limites_contaminantes" => "Limites de contaminantes",
                "fdn_min" => "Pisos de FDN",
                "amido_max" => "Tetos de amido",
                "ee_max" => "Tetos de extrato etéreo",
                "discrepancia" => "Tolerâncias de discrepância de consumo",
                _ => k,
            };
            DiferencaConhecimento {
                rotulo: rotulo.into(),
                referencia: a.get(k).cloned().unwrap_or_default(),
                alternativa: b.get(k).cloned().unwrap_or_default(),
            }
        })
        .collect()
}

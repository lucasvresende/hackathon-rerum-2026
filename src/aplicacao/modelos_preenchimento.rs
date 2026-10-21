//! Referências de preenchimento e procedência, independentes de HTTP e apresentação.
use sqlx::SqlitePool;

use crate::Resultado;
use crate::dominio::templates::{
    CampoPreenchido, ModeloPreenchimento, ProcedenciaModelo, TipoCadastro, equivalentes,
};
use crate::dominio::validacao::{Problema, problema};
use crate::dominio::{Alimento, Lote};

pub async fn listar(pool: &SqlitePool) -> Resultado<Vec<ModeloPreenchimento>> {
    crate::persistencia::templates::ler(&mut *pool.acquire().await?).await
}

pub enum AcaoModelo {
    Previa,
    Aplicar { confirmado: bool },
}

/// Substituições autorizadas pelo catálogo; nunca inclui valores enviados pelo navegador.
pub struct CargaModelo {
    pub previa: bool,
    pub substituicoes: Vec<(String, String)>,
    pub aplicado: Option<String>,
    pub erros: Vec<Problema>,
}

pub async fn preparar(
    pool: &SqlitePool,
    tipo: TipoCadastro,
    id: &str,
    acao: AcaoModelo,
) -> Resultado<CargaModelo> {
    let modelos = listar(pool).await?;
    let modelo = modelos.iter().find(|m| m.id == id && m.tipo == tipo.nome());
    let mut carga = CargaModelo {
        previa: modelo.is_some(),
        substituicoes: vec![],
        aplicado: None,
        erros: vec![],
    };
    let Some(modelo) = modelo else {
        carga.erros.push(problema(
            "modelo",
            "Escolha um template para consultar a prévia.",
        ));
        return Ok(carga);
    };
    match acao {
        AcaoModelo::Previa => {}
        AcaoModelo::Aplicar { confirmado: false } => {
            carga.erros.push(problema(
                "confirmar_modelo",
                "Confirme a substituição antes de carregar.",
            ));
        }
        AcaoModelo::Aplicar { confirmado: true } => {
            // Uma lacuna da referência nunca herda composição ou confirmação do cadastro anterior.
            let (limpar, estados): (&[&str], &[&str]) = match tipo {
                TipoCadastro::Alimento => (
                    &["ms", "ndt", "pb", "ca", "p", "fdn", "amido", "ee"],
                    &[
                        "origem_animal",
                        "excecao_animal",
                        "nnp_adicionado",
                        "medicamento",
                        "autorizado",
                    ],
                ),
                TipoCadastro::Lote => (
                    &["peso", "producao", "gordura"],
                    &[
                        "adulta",
                        "gestante",
                        "crescimento",
                        "peso_estavel",
                        "confinada",
                        "correcao_ambiente",
                        "correcao_atividade",
                        "agua",
                    ],
                ),
            };
            carga
                .substituicoes
                .extend(estados.iter().map(|c| ((*c).into(), "DESCONHECIDO".into())));
            carga
                .substituicoes
                .extend(limpar.iter().map(|c| ((*c).into(), String::new())));
            carga.substituicoes.extend(
                modelo
                    .valores
                    .iter()
                    .map(|v| (v.campo.clone(), v.valor.clone())),
            );
            carga.aplicado = Some(modelo.id.clone());
            carga.previa = false;
        }
    }
    Ok(carga)
}

/// Preserva a representação serializada dos lotes usada nos snapshots anteriores.
pub async fn procedencia_lote(
    pool: &SqlitePool,
    id: &str,
    lote: &Lote,
) -> Result<Option<ProcedenciaModelo>, Problema> {
    let valores = serde_json::to_value(lote).unwrap_or_default();
    procedencia(pool, id, TipoCadastro::Lote, |campo| {
        valor_serializado(&valores, campo)
    })
    .await
}

/// Compara a composição normalizada com a referência em percentuais de MN/MS.
pub async fn procedencia_alimento(
    pool: &SqlitePool,
    id: &str,
    alimento: &Alimento,
) -> Result<Option<ProcedenciaModelo>, Problema> {
    let valores = serde_json::to_value(alimento).unwrap_or_default();
    procedencia(pool, id, TipoCadastro::Alimento, |campo| {
        if campo == "ms" {
            return (alimento.ms * 100.0).to_string();
        }
        if let Some(i) = ["ndt", "pb", "ca", "p", "fdn", "amido", "ee"]
            .iter()
            .position(|nome| *nome == campo)
        {
            return alimento.nutrientes[i]
                .map(|v| (v * 100.0).to_string())
                .unwrap_or_default();
        }
        valor_serializado(&valores, campo)
    })
    .await
}

fn valor_serializado(valores: &serde_json::Value, campo: &str) -> String {
    match &valores[campo] {
        serde_json::Value::Null => String::new(),
        serde_json::Value::String(s) => s.clone(),
        valor => valor.to_string(),
    }
}

async fn procedencia(
    pool: &SqlitePool,
    id: &str,
    tipo: TipoCadastro,
    valor: impl Fn(&str) -> String,
) -> Result<Option<ProcedenciaModelo>, Problema> {
    if id.is_empty() {
        return Ok(None);
    }
    let modelo = listar(pool)
        .await
        .map_err(|_| problema("modelo", "Não foi possível consultar os templates."))?
        .into_iter()
        .find(|m| m.id == id && m.tipo == tipo.nome())
        .ok_or_else(|| problema("modelo", "Template inexistente ou incompatível."))?;
    let mut campos: Vec<_> = modelo
        .valores
        .iter()
        .map(|v| {
            let utilizado = valor(&v.campo).trim().to_owned();
            CampoPreenchido {
                campo: v.campo.clone(),
                alterado: !equivalentes(&utilizado, &v.valor),
                utilizado,
            }
        })
        .collect();
    let adicionais: &[&str] = match tipo {
        TipoCadastro::Alimento => &["ms", "ndt", "pb", "ca", "p", "fdn", "amido", "ee"],
        TipoCadastro::Lote => &["raca", "peso", "producao", "gordura"],
    };
    for campo in adicionais {
        let utilizado = valor(campo).trim().to_owned();
        if !utilizado.is_empty() && !modelo.valores.iter().any(|v| v.campo == *campo) {
            campos.push(CampoPreenchido {
                campo: (*campo).into(),
                utilizado,
                alterado: true,
            });
        }
    }
    Ok(Some(ProcedenciaModelo { modelo, campos }))
}

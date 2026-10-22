//! Casos de uso de consulta e gravação de revisões de lotes e alimentos.
use super::modelos_preenchimento;
use crate::persistencia::cadastros;
use crate::{
    Resultado,
    dominio::{Alimento, Lote, validacao::Problema},
};
use sqlx::SqlitePool;

#[derive(Debug)]
pub enum FalhaCadastro {
    Referencia(Problema),
    Persistencia(crate::Erro),
}

pub async fn obter_lote(pool: &SqlitePool, id: &str) -> Resultado<Lote> {
    cadastros::lotes(pool)
        .await?
        .into_iter()
        .find(|l| l.id == id)
        .ok_or_else(|| "Lote não encontrado".into())
}

pub async fn obter_alimento(pool: &SqlitePool, id: &str) -> Resultado<Alimento> {
    cadastros::alimentos(pool)
        .await?
        .into_iter()
        .find(|a| a.composicao_id == id)
        .ok_or_else(|| "Composição não encontrada".into())
}

pub async fn pesquisar_alimentos(pool: &SqlitePool, busca: &str) -> Resultado<Vec<Alimento>> {
    let mut alimentos: Vec<_> = cadastros::alimentos(pool)
        .await?
        .into_iter()
        .filter(|a| a.nome.to_lowercase().contains(&busca.to_lowercase()))
        .collect();
    alimentos.sort_by_key(|a| {
        let destaque = ["ex-a01", "ex-a06", "ex-a07", "ex-a12"].contains(&a.id.as_str());
        (!destaque, format!("{:?}", a.classe), a.nome.clone())
    });
    Ok(alimentos)
}

pub async fn salvar_lote(
    pool: &SqlitePool,
    lote: &mut Lote,
    novo: bool,
    modelo_id: &str,
    origem_copia: &str,
) -> Result<(), FalhaCadastro> {
    let anterior = if !origem_copia.is_empty() {
        Some(
            obter_lote(pool, origem_copia)
                .await
                .map_err(FalhaCadastro::Persistencia)?,
        )
    } else if !novo {
        Some(
            obter_lote(pool, &lote.id)
                .await
                .map_err(FalhaCadastro::Persistencia)?,
        )
    } else {
        None
    };
    let referencia = anterior.as_ref().and_then(|l| l.procedencia.as_ref());
    let modelo_id = if modelo_id.is_empty() {
        referencia.map(|p| p.modelo.id.as_str()).unwrap_or("")
    } else {
        modelo_id
    };
    lote.procedencia = modelos_preenchimento::procedencia_lote(pool, modelo_id, lote)
        .await
        .map_err(FalhaCadastro::Referencia)?;
    cadastros::salvar_lote(pool, lote, novo)
        .await
        .map_err(FalhaCadastro::Persistencia)
}

pub async fn salvar_alimento(
    pool: &SqlitePool,
    alimento: &mut Alimento,
    modelo_id: &str,
    origem_copia: &str,
) -> Result<(), FalhaCadastro> {
    let anterior = if !origem_copia.is_empty() {
        Some(
            obter_alimento(pool, origem_copia)
                .await
                .map_err(FalhaCadastro::Persistencia)?,
        )
    } else {
        cadastros::alimentos(pool)
            .await
            .map_err(FalhaCadastro::Persistencia)?
            .into_iter()
            .find(|a| a.id == alimento.id)
    };
    let referencia = anterior.as_ref().and_then(|a| a.procedencia.as_ref());
    let modelo_id = if modelo_id.is_empty() {
        referencia.map(|p| p.modelo.id.as_str()).unwrap_or("")
    } else {
        modelo_id
    };
    if modelo_id.starts_with("ex-") {
        alimento.natureza = "SINTETICA".into();
    }
    alimento.procedencia = modelos_preenchimento::procedencia_alimento(pool, modelo_id, alimento)
        .await
        .map_err(FalhaCadastro::Referencia)?;
    cadastros::salvar_alimento(pool, alimento)
        .await
        .map_err(FalhaCadastro::Persistencia)
}

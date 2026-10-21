//! Revisões de cadastros e associação à referência usada pelo operador.
use super::modelos_preenchimento;
use crate::persistencia::cadastros;
use crate::{
    Resultado,
    dominio::{Alimento, Lote, validacao::Problema},
};
use sqlx::SqlitePool;

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
    Ok(cadastros::alimentos(pool)
        .await?
        .into_iter()
        .filter(|a| a.nome.to_lowercase().contains(&busca.to_lowercase()))
        .collect())
}

pub async fn salvar_lote(
    pool: &SqlitePool,
    lote: &mut Lote,
    novo: bool,
    modelo_id: &str,
) -> Result<(), FalhaCadastro> {
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
) -> Result<(), FalhaCadastro> {
    alimento.procedencia = modelos_preenchimento::procedencia_alimento(pool, modelo_id, alimento)
        .await
        .map_err(FalhaCadastro::Referencia)?;
    cadastros::salvar_alimento(pool, alimento)
        .await
        .map_err(FalhaCadastro::Persistencia)
}

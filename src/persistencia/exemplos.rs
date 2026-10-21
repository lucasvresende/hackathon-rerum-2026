//! Originais imutáveis e transações de ativação; exclusões usam os cadastros existentes.
use crate::Resultado;
use sqlx::{SqliteConnection, SqlitePool};

#[derive(Clone)]
pub struct Exemplo {
    pub id: String,
    pub tipo: String,
    pub conteudo: String,
}

pub async fn listar(pool: &SqlitePool) -> Resultado<Vec<Exemplo>> {
    Ok(sqlx::query_file!("queries/exemplos/listar.sql")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| Exemplo {
            id: r.id,
            tipo: r.tipo,
            conteudo: r.conteudo,
        })
        .collect())
}

pub async fn inicial(pool: &SqlitePool) -> Resultado<Option<crate::dominio::Cenario>> {
    sqlx::query_file!("queries/exemplos/inicial.sql")
        .fetch_optional(pool)
        .await?
        .map(|r| Ok(serde_json::from_str(&r.cenario)?))
        .transpose()
}

pub async fn existe(c: &mut SqliteConnection, tipo: &str, id: &str) -> Resultado<bool> {
    Ok(if tipo == "lote" {
        sqlx::query_file!("queries/exemplos/lote.sql", id)
            .fetch_optional(&mut *c)
            .await?
            .is_some()
    } else {
        sqlx::query_file!("queries/exemplos/alimento.sql", id)
            .fetch_optional(&mut *c)
            .await?
            .is_some()
    })
}

pub async fn reativar(c: &mut SqliteConnection, tipo: &str, id: &str) -> Resultado<()> {
    if tipo == "lote" {
        sqlx::query_file!("queries/exemplos/reativar_lote.sql", id)
            .execute(&mut *c)
            .await?;
    } else {
        sqlx::query_file!("queries/exemplos/reativar_alimento.sql", id)
            .execute(&mut *c)
            .await?;
    }
    Ok(())
}

pub async fn desativar(pool: &SqlitePool, tipo: &str, id: &str) -> Resultado<()> {
    if tipo == "lote" {
        sqlx::query_file!("queries/lotes/desativar.sql", id)
            .execute(pool)
            .await?;
    } else if tipo == "alimento" {
        sqlx::query_file!("queries/alimentos/desativar.sql", id)
            .execute(pool)
            .await?;
    } else {
        return Err("Tipo de cadastro inválido".into());
    }
    Ok(())
}

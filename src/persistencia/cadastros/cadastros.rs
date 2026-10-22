//! Leitura e gravação transacional dos cadastros e suas revisões.
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::Resultado;
use crate::dominio::{Alimento, Lote};

pub async fn lotes(pool: &SqlitePool) -> Resultado<Vec<Lote>> {
    sqlx::query_file!("queries/lotes/listar.sql")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| Ok(serde_json::from_str(&r.conteudo)?))
        .collect()
}

pub async fn salvar_lote(pool: &SqlitePool, lote: &mut Lote, novo: bool) -> Resultado<()> {
    let mut tx = pool.begin().await?;
    salvar_lote_conexao(&mut tx, lote, novo).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn salvar_lote_conexao(
    conexao: &mut sqlx::SqliteConnection,
    lote: &mut Lote,
    novo: bool,
) -> Resultado<()> {
    let erros = crate::dominio::validacao::validar_lote(lote);
    if !erros.is_empty() {
        return Err(format!("Lote inválido: {erros:?}").into());
    }
    lote.revisao_id = Uuid::new_v4().to_string();
    let conteudo = serde_json::to_string(lote)?;
    let animais = lote.animais as i64;
    let dias = lote.dias_lactacao as i64;
    if novo {
        sqlx::query_file!("queries/lotes/inserir.sql", lote.id, lote.nome)
            .execute(&mut *conexao)
            .await?;
    } else {
        sqlx::query_file!("queries/lotes/atualizar.sql", lote.nome, lote.id)
            .execute(&mut *conexao)
            .await?;
    }
    sqlx::query_file!(
        "queries/lotes/inserir_revisao.sql",
        lote.revisao_id,
        lote.id,
        lote.id,
        lote.peso,
        animais,
        lote.producao,
        lote.gordura,
        dias,
        conteudo
    )
    .execute(&mut *conexao)
    .await?;
    Ok(())
}

pub async fn alimentos(pool: &SqlitePool) -> Resultado<Vec<Alimento>> {
    sqlx::query_file!("queries/alimentos/listar_composicoes.sql")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| Ok(serde_json::from_str(&r.conteudo)?))
        .collect()
}

pub async fn salvar_alimento(pool: &SqlitePool, a: &mut Alimento) -> Resultado<()> {
    let mut tx = pool.begin().await?;
    salvar_alimento_conexao(&mut tx, a).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn salvar_alimento_conexao(
    conexao: &mut sqlx::SqliteConnection,
    a: &mut Alimento,
) -> Resultado<()> {
    let erros = crate::dominio::validacao::validar_alimento(a);
    if !erros.is_empty() {
        return Err(format!("Composição inválida: {erros:?}").into());
    }
    a.composicao_id = Uuid::new_v4().to_string();
    let conteudo = serde_json::to_string(a)?;
    let classe = format!("{:?}", a.classe);
    let [ndt, pb, ca, p, fdn, amido, ee] = a.nutrientes;
    sqlx::query_file!("queries/alimentos/inserir.sql", a.id, a.nome, classe)
        .execute(&mut *conexao)
        .await?;
    sqlx::query_file!(
        "queries/alimentos/inserir_composicao.sql",
        a.composicao_id,
        a.id,
        a.fonte_id,
        a.id,
        a.ms,
        ndt,
        pb,
        ca,
        p,
        fdn,
        amido,
        ee,
        a.natureza,
        conteudo
    )
    .execute(&mut *conexao)
    .await?;
    let produto_id = Uuid::new_v4().to_string();
    sqlx::query_file!(
        "queries/alimentos/inserir_produto.sql",
        produto_id,
        a.composicao_id,
        a.fornecedor,
        a.lote_fisico,
        a.evidencia,
        conteudo
    )
    .execute(&mut *conexao)
    .await?;
    Ok(())
}

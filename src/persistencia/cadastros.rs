use sqlx::SqlitePool;
use uuid::Uuid;

use crate::Resultado;
use crate::dominio::{Alimento, Lote};

pub async fn lotes(pool: &SqlitePool) -> Resultado<Vec<Lote>> {
    sqlx::query_file!("queries/listar_lotes.sql")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| Ok(serde_json::from_str(&r.conteudo)?))
        .collect()
}

pub async fn salvar_lote(pool: &SqlitePool, lote: &mut Lote, novo: bool) -> Resultado<()> {
    let erros = crate::dominio::validacao::validar_lote(lote);
    if !erros.is_empty() {
        return Err(format!("Lote inválido: {erros:?}").into());
    }
    lote.revisao_id = Uuid::new_v4().to_string();
    let conteudo = serde_json::to_string(lote)?;
    let animais = lote.animais as i64;
    let dias = lote.dias_lactacao as i64;
    let mut tx = pool.begin().await?;
    if novo {
        sqlx::query_file!("queries/inserir_lote.sql", lote.id, lote.nome)
            .execute(&mut *tx)
            .await?;
    } else {
        sqlx::query_file!("queries/atualizar_lote.sql", lote.nome, lote.id)
            .execute(&mut *tx)
            .await?;
    }
    sqlx::query_file!(
        "queries/inserir_revisao.sql",
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
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn alimentos(pool: &SqlitePool) -> Resultado<Vec<Alimento>> {
    sqlx::query_file!("queries/listar_composicoes.sql")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| Ok(serde_json::from_str(&r.conteudo)?))
        .collect()
}

pub async fn salvar_alimento(pool: &SqlitePool, a: &mut Alimento) -> Resultado<()> {
    let erros = crate::dominio::validacao::validar_alimento(a);
    if !erros.is_empty() {
        return Err(format!("Composição inválida: {erros:?}").into());
    }
    a.composicao_id = Uuid::new_v4().to_string();
    let conteudo = serde_json::to_string(a)?;
    let classe = format!("{:?}", a.classe);
    let [ndt, pb, ca, p, fdn, amido, ee] = a.nutrientes;
    let mut tx = pool.begin().await?;
    sqlx::query_file!("queries/inserir_alimento.sql", a.id, a.nome, classe)
        .execute(&mut *tx)
        .await?;
    sqlx::query_file!(
        "queries/inserir_composicao.sql",
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
    .execute(&mut *tx)
    .await?;
    let produto_id = Uuid::new_v4().to_string();
    sqlx::query_file!(
        "queries/inserir_produto.sql",
        produto_id,
        a.composicao_id,
        a.fornecedor,
        a.lote_fisico,
        a.evidencia,
        conteudo
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

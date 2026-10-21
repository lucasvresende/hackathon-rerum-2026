//! Ativação, restauração e remoção lógica dos exemplos da aplicação.
use super::modelos_preenchimento;
use crate::persistencia::{cadastros, exemplos};
use crate::{
    Resultado,
    dominio::{Alimento, Cenario, Lote, templates::ProcedenciaModelo},
};
use sqlx::SqlitePool;
use uuid::Uuid;

/// Insere registros e cenário no mesmo commit do marcador; repetir não repovoa exclusões.
pub async fn inicializar(pool: &SqlitePool) -> Resultado<()> {
    let originais = exemplos::listar(pool).await?;
    let modelos = modelos_preenchimento::listar(pool).await?;
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
    if sqlx::query_file!("queries/exemplos/inicial.sql")
        .fetch_optional(&mut *tx)
        .await?
        .is_some()
    {
        return Ok(());
    }
    let mut lotes = vec![];
    let mut alimentos = vec![];
    for original in originais {
        // Um banco preexistente nunca tem seus cadastros substituídos por esta inicialização.
        if exemplos::existe(&mut tx, &original.tipo, &original.id).await? {
            return Err(
                "Identificador reservado de exemplo já utilizado; faça backup e revise.".into(),
            );
        }
        let modelo = modelos
            .iter()
            .find(|m| m.id == original.id)
            .ok_or("Exemplo sem referência de preenchimento")?
            .clone();
        let procedencia = Some(ProcedenciaModelo {
            campos: modelo
                .valores
                .iter()
                .map(|v| crate::dominio::templates::CampoPreenchido {
                    campo: v.campo.clone(),
                    utilizado: v.valor.clone(),
                    alterado: false,
                })
                .collect(),
            modelo,
        });
        if original.tipo == "lote" {
            let mut l: Lote = serde_json::from_str(&original.conteudo)?;
            l.procedencia = procedencia;
            cadastros::salvar_lote_conexao(&mut tx, &mut l, true).await?;
            lotes.push(l);
        } else {
            let mut a: Alimento = serde_json::from_str(&original.conteudo)?;
            a.procedencia = procedencia;
            cadastros::salvar_alimento_conexao(&mut tx, &mut a).await?;
            alimentos.push(a);
        }
    }
    let lote = lotes
        .into_iter()
        .find(|l| l.id == "ex-l03")
        .ok_or("Lote inicial ausente")?;
    let mut cenario = super::demonstracao::cenario(lote, vec![], false);
    cenario.nome = "Primeira simulação — exemplo sintético".into();
    for (id, quantidade, preco, maximo) in [
        ("ex-a01", 11.0, "0.25", None),
        ("ex-a06", 0.8, "1.30", None),
        ("ex-a07", 3.0, "2.20", None),
        ("ex-a12", 0.3, "4.00", Some(0.3)),
        ("ex-a10", 2.0, "1.10", None),
    ] {
        let alimento = alimentos
            .iter()
            .find(|a| a.id == id)
            .ok_or("Alimento inicial ausente")?;
        let mut item =
            super::demonstracao::cenario(cenario.lote.clone(), vec![alimento.clone()], false)
                .itens
                .remove(0);
        item.quantidade = quantidade;
        item.preco = preco.into();
        item.maximo_ms = maximo;
        // Limite demonstrativo expresso, sem rótulo comercial inventado.

        item.origem_preco = "Hipótese sintética de preço e disponibilidade — 21/10/2026".into();
        cenario.itens.push(item);
    }
    let conteudo = serde_json::to_string(&cenario)?;
    sqlx::query_file!("queries/exemplos/inicializar.sql", conteudo)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}

/// Só uma ação explícita restaura; conflitos exigem criar cópia ou uma nova revisão.
pub async fn restaurar(pool: &SqlitePool, id: &str, modo: &str) -> Resultado<()> {
    if !["copiar", "revisar"].contains(&modo) {
        return Err("Escolha cópia ou revisão.".into());
    }
    let original = exemplos::listar(pool)
        .await?
        .into_iter()
        .find(|e| e.id == id)
        .ok_or("Exemplo inexistente")?;
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
    let existe = exemplos::existe(&mut tx, &original.tipo, id).await?;
    let destino = if modo == "copiar" {
        Uuid::new_v4().to_string()
    } else {
        id.into()
    };
    if original.tipo == "lote" {
        let mut lote: Lote = serde_json::from_str(&original.conteudo)?;
        lote.procedencia = modelos_preenchimento::procedencia_lote(pool, id, &lote)
            .await
            .map_err(|e| e.mensagem)?;
        lote.id = destino;
        cadastros::salvar_lote_conexao(&mut tx, &mut lote, modo == "copiar" || !existe).await?;
    } else {
        let mut a: Alimento = serde_json::from_str(&original.conteudo)?;
        a.procedencia = modelos_preenchimento::procedencia_alimento(pool, id, &a)
            .await
            .map_err(|e| e.mensagem)?;
        a.id = destino;
        cadastros::salvar_alimento_conexao(&mut tx, &mut a).await?;
    }
    if modo == "revisar" {
        exemplos::reativar(&mut tx, &original.tipo, id).await?;
    }
    tx.commit().await?;
    Ok(())
}

pub async fn cenario_inicial(pool: &SqlitePool) -> Resultado<Cenario> {
    let mut c = exemplos::inicial(pool)
        .await?
        .ok_or("Cenário inicial indisponível")?;
    let hoje = chrono::Utc::now()
        .with_timezone(&chrono_tz::America::Sao_Paulo)
        .date_naive()
        .to_string();
    c.data_aplicacao = hoje.clone();
    c.data_avaliacao = hoje;
    Ok(c)
}

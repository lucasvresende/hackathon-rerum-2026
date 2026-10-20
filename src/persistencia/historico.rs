use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::Resultado;
use crate::dominio::{Cenario, Conhecimento, Parecer};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Execucao {
    pub id: String,
    pub cenario_id: String,
    pub criada_em: String,
    pub cenario: Cenario,
    pub parecer: Parecer,
    pub conhecimento: Conhecimento,
}

pub async fn salvar(
    pool: &SqlitePool,
    c: &Cenario,
    p: &Parecer,
    base: &Conhecimento,
) -> Resultado<String> {
    let id = Uuid::new_v4().to_string();
    let cenario_id = Uuid::new_v4().to_string();
    let criada_em = Utc::now().to_rfc3339();
    let entradas = serde_json::to_string(c)?;
    let resultado = serde_json::to_string(p)?;
    let artefatos = serde_json::to_string(base)?;
    let horizonte = c.horizonte as i64;
    let duracao = p.duracao_ms as i64;
    let mut tx = pool.begin().await?;
    sqlx::query_file!(
        "queries/inserir_cenario.sql",
        cenario_id,
        c.nome,
        c.lote.revisao_id,
        c.perfil_id,
        c.original_id,
        horizonte,
        c.data_aplicacao,
        entradas
    )
    .execute(&mut *tx)
    .await?;
    for item in &c.itens {
        sqlx::query_file!(
            "queries/inserir_item.sql",
            cenario_id,
            item.alimento.composicao_id,
            item.quantidade,
            item.preco,
            item.estoque,
            item.perda,
            item.minimo_ms,
            item.maximo_ms
        )
        .execute(&mut *tx)
        .await?;
    }
    sqlx::query_file!(
        "queries/inserir_execucao.sql",
        id,
        cenario_id,
        criada_em,
        base.metodo,
        base.versao_app,
        p.condicao,
        p.custo,
        duracao,
        resultado,
        artefatos
    )
    .execute(&mut *tx)
    .await?;
    for (ordem, v) in p.verificacoes.iter().enumerate() {
        let ordem = ordem as i64;
        let estado = format!("{:?}", v.estado);
        let dependencias = serde_json::to_string(&v.dependencias)?;
        sqlx::query_file!(
            "queries/inserir_verificacao.sql",
            id,
            ordem,
            v.regra,
            v.dimensao,
            estado,
            v.valor,
            v.limite,
            v.unidade,
            v.fonte,
            v.explicacao,
            dependencias
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(id)
}

pub async fn listar(pool: &SqlitePool) -> Resultado<Vec<Execucao>> {
    sqlx::query_file!("queries/historico.sql")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| {
            Ok(Execucao {
                id: r.id,
                cenario_id: String::new(),
                criada_em: r.criada_em,
                cenario: serde_json::from_str(&r.entradas)?,
                parecer: serde_json::from_str(&r.resultado)?,
                conhecimento: serde_json::from_str(&r.artefatos)?,
            })
        })
        .collect()
}

pub async fn obter(pool: &SqlitePool, id: &str) -> Resultado<Execucao> {
    let r = sqlx::query_file!("queries/execucao.sql", id)
        .fetch_one(pool)
        .await?;
    Ok(Execucao {
        id: r.id,
        cenario_id: r.cenario_id,
        criada_em: r.criada_em,
        cenario: serde_json::from_str(&r.entradas)?,
        parecer: serde_json::from_str(&r.resultado)?,
        conhecimento: serde_json::from_str(&r.artefatos)?,
    })
}

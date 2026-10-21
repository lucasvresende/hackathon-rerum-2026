use std::collections::BTreeMap;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use maud::{Markup, Render};

use super::rotas::{ErroWeb, EstadoApp};
use crate::persistencia::{cadastros, conhecimento as base, historico as banco};

pub async fn inicio(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    let lotes = cadastros::lotes(&e.pool).await?;
    let alimentos = cadastros::alimentos(&e.pool).await?;
    let execucoes = banco::listar(&e.pool).await?;
    let conhecimento = base::ler(&e.pool).await?;
    Ok(super::templates::paginas::inicio::Inicio {
        lotes: &lotes,
        alimentos: &alimentos,
        execucoes: &execucoes,
        conhecimento: &conhecimento,
    }
    .render())
}

pub async fn lotes(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    let lotes = cadastros::lotes(&e.pool).await?;
    Ok(super::templates::paginas::lotes::Lotes { lotes: &lotes }.render())
}

pub async fn alimentos(
    State(e): State<EstadoApp>,
    Query(q): Query<BTreeMap<String, String>>,
) -> Result<Markup, ErroWeb> {
    let alimentos = cadastros::alimentos(&e.pool).await?;
    let busca = q.get("busca").cloned().unwrap_or_default();
    Ok(super::templates::paginas::alimentos::Alimentos {
        alimentos: &alimentos,
        busca: &busca,
    }
    .render())
}

pub async fn historico(
    State(e): State<EstadoApp>,
    Query(q): Query<BTreeMap<String, String>>,
) -> Result<Markup, ErroWeb> {
    let pagina = q
        .get("pagina")
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(1)
        .clamp(1, 1000000);
    let busca = q.get("lote").cloned().unwrap_or_default();
    let data = q.get("data").cloned().unwrap_or_default();
    let filtro = (!busca.is_empty()).then(|| format!("%{busca}%"));
    let dia = chrono::NaiveDate::parse_from_str(&data, "%Y-%m-%d").ok();
    let instante = |dia: chrono::NaiveDate| {
        dia.and_hms_opt(0, 0, 0)
            .and_then(|n| {
                n.and_local_timezone(chrono_tz::America::Sao_Paulo)
                    .earliest()
            })
            .map(|v| v.with_timezone(&chrono::Utc).to_rfc3339())
    };
    let inicio = dia.and_then(instante);
    let fim = dia.and_then(|d| d.succ_opt()).and_then(instante);
    let execucoes = banco::filtrar(
        &e.pool,
        filtro.as_deref(),
        inicio.as_deref(),
        fim.as_deref(),
        (pagina - 1) * 100,
    )
    .await?;
    Ok(super::templates::paginas::historico::Historico {
        execucoes: &execucoes,
        pagina,
        busca: &busca,
        data: &data,
    }
    .render())
}

pub async fn conhecimento(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    let c = base::ler(&e.pool).await?;
    Ok(super::templates::paginas::conhecimento::Conhecimento { conhecimento: &c }.render())
}

pub async fn nao_encontrado() -> (StatusCode, Markup) {
    (
        StatusCode::NOT_FOUND,
        super::templates::paginas::erro::Erro::NaoEncontrado.render(),
    )
}

/// Ajuda estática disponível sem consultar cadastros ou execuções.
pub async fn ajuda() -> Markup {
    super::templates::paginas::ajuda::Ajuda.render()
}

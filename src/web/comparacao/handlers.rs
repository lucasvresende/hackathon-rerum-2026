//! Handler HTTP para comparacao, mantendo a renderização desta responsabilidade isolada.
use crate::web::{erro::ErroWeb, estado::EstadoApp};
use axum::extract::{Query, State};
use axum::{Router, routing::get};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::Render;
use std::collections::BTreeMap;

/// Rotas HTTP deste recurso para composição no roteador principal.
pub fn rotas() -> Router<EstadoApp> {
    Router::new().route("/comparar", get(comparar))
}

pub async fn comparar(
    State(e): State<EstadoApp>,
    Query(q): Query<BTreeMap<String, String>>,
) -> Result<Response, ErroWeb> {
    let comparacao = crate::aplicacao::comparacao::comparar(
        &e.pool,
        q.get("a").map(String::as_str).unwrap_or(""),
        q.get("b").map(String::as_str).unwrap_or(""),
    )
    .await?;
    let status = if comparacao.mesma_revisao {
        StatusCode::OK
    } else {
        StatusCode::UNPROCESSABLE_ENTITY
    };
    Ok((
        status,
        crate::web::templates::paginas::comparacao::Comparacao {
            comparacao: &comparacao,
        }
        .render(),
    )
        .into_response())
}

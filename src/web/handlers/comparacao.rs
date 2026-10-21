use crate::web::{erro::ErroWeb, estado::EstadoApp};
use axum::extract::{Query, State};
use maud::{Markup, Render};
use std::collections::BTreeMap;

pub async fn comparar(
    State(e): State<EstadoApp>,
    Query(q): Query<BTreeMap<String, String>>,
) -> Result<Markup, ErroWeb> {
    let comparacao = crate::aplicacao::comparacao::comparar(
        &e.pool,
        q.get("a").map(String::as_str).unwrap_or(""),
        q.get("b").map(String::as_str).unwrap_or(""),
    )
    .await?;
    Ok(crate::web::templates::paginas::comparacao::Comparacao {
        comparacao: &comparacao,
    }
    .render())
}

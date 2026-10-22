//! Handler HTTP para conhecimento, mantendo a renderização desta responsabilidade isolada.
use crate::persistencia::conhecimento as base;
use crate::web::{erro::ErroWeb, estado::EstadoApp};
use axum::extract::State;
use axum::{Router, routing::get};
use maud::{Markup, Render};

/// Rotas HTTP deste recurso para composição no roteador principal.
pub fn rotas() -> Router<EstadoApp> {
    Router::new().route("/conhecimento", get(conhecimento))
}

pub async fn conhecimento(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    let c = base::ler(&e.pool).await?;
    Ok(crate::web::templates::paginas::conhecimento::Conhecimento { conhecimento: &c }.render())
}

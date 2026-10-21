use crate::persistencia::conhecimento as base;
use crate::web::{erro::ErroWeb, estado::EstadoApp};
use axum::extract::State;
use maud::{Markup, Render};

pub async fn conhecimento(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    let c = base::ler(&e.pool).await?;
    Ok(crate::web::templates::paginas::conhecimento::Conhecimento { conhecimento: &c }.render())
}

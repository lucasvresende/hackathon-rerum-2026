//! Handler HTTP para execucoes, mantendo a renderização desta responsabilidade isolada.
use crate::aplicacao::cenarios as servico;
use crate::persistencia::historico;
use crate::web::{apresentacao, entradas as formularios};
use crate::web::{erro::ErroWeb, estado::EstadoApp};
use axum::{
    Router,
    extract::{Path, State},
    routing::get,
};
use maud::{Markup, Render};

/// Rotas HTTP deste recurso para composição no roteador principal.
pub fn rotas() -> Router<EstadoApp> {
    Router::new()
        .route("/execucoes/{id}", get(resultado))
        .route("/execucoes/{id}/duplicar", get(duplicar))
}

pub async fn resultado(
    State(e): State<EstadoApp>,
    Path(id): Path<String>,
) -> Result<Markup, ErroWeb> {
    let execucao = historico::obter(&e.pool, &id).await?;
    Ok(crate::web::templates::paginas::execucao::Execucao {
        execucao: &execucao,
        token: &e.csrf,
    }
    .render())
}

pub async fn duplicar(
    State(e): State<EstadoApp>,
    Path(id): Path<String>,
) -> Result<Markup, ErroWeb> {
    let c = servico::duplicar(&e.pool, &id).await?;
    apresentacao::cenario(&e, &formularios::campos_cenario(&c), &[], false).await
}

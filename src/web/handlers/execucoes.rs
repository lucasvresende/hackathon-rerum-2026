//! Handler HTTP para execucoes, mantendo a renderização desta responsabilidade isolada.
use crate::aplicacao::cenarios as servico;
use crate::persistencia::historico;
use crate::web::entradas::Campos;
use crate::web::{apresentacao, entradas as formularios};
use crate::web::{erro::ErroWeb, estado::EstadoApp, resposta::redirecionar, seguranca::autorizado};
use axum::{
    Form,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use axum::{
    Router,
    routing::{get, post},
};
use maud::{Markup, Render};

/// Rotas HTTP deste recurso para composição no roteador principal.
pub fn rotas() -> Router<EstadoApp> {
    Router::new()
        .route("/execucoes/{id}", get(resultado))
        .route("/execucoes/{id}/duplicar", get(duplicar))
        .route("/execucoes/{id}/reproduzir", post(reproduzir))
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

pub async fn reproduzir(
    State(e): State<EstadoApp>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Form(c): Form<Campos>,
) -> Result<Response, ErroWeb> {
    if !autorizado(&e, &headers, &c) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }
    let novo = servico::reproduzir(&e.pool, &id).await?;
    Ok(redirecionar(&format!("/execucoes/{novo}"), &headers))
}

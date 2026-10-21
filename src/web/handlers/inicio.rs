use crate::persistencia::{cadastros, conhecimento as base, historico};
use crate::web::{erro::ErroWeb, estado::EstadoApp};
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Router, routing::get};
use maud::{Markup, Render};

/// Rotas HTTP deste recurso para composição no roteador principal.
pub fn rotas() -> Router<EstadoApp> {
    Router::new()
        .route("/", get(inicio))
        .route("/saude", get(saude))
}

pub async fn inicio(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    let lotes = cadastros::lotes(&e.pool).await?;
    let alimentos = cadastros::alimentos(&e.pool).await?;
    let execucoes = historico::listar(&e.pool).await?;
    let conhecimento = base::ler(&e.pool).await?;
    Ok(crate::web::templates::paginas::inicio::Inicio {
        lotes: &lotes,
        alimentos: &alimentos,
        execucoes: &execucoes,
        conhecimento: &conhecimento,
    }
    .render())
}

pub async fn nao_encontrado() -> (StatusCode, Markup) {
    (
        StatusCode::NOT_FOUND,
        crate::web::templates::paginas::erro::Erro(
            crate::web::templates::paginas::erro::TipoErro::NaoEncontrado,
        )
        .render(),
    )
}

async fn saude() -> &'static str {
    "FeedYourCow: disponível"
}

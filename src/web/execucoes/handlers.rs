//! Handler HTTP para execucoes, mantendo a renderização desta responsabilidade isolada.
use crate::aplicacao::reutilizacao::{self, OrigemDieta};
use crate::persistencia::historico;
use crate::web::{apresentacao, entradas as formularios};
use crate::web::{erro::ErroWeb, estado::EstadoApp};
use axum::{
    Router,
    extract::{Path, Query, State},
    routing::get,
};
use maud::{Markup, Render};
use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Copia {
    #[serde(default)]
    origem: OrigemDieta,
}

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
    Query(copia): Query<Copia>,
) -> Result<axum::response::Response, ErroWeb> {
    use axum::{http::StatusCode, response::IntoResponse};
    let anterior = historico::obter(&e.pool, &id).await?;
    let c = match reutilizacao::copiar(&anterior, copia.origem) {
        Ok(c) => c,
        Err(mensagem) => {
            let pagina = crate::web::templates::paginas::erro::AvisoOperacao {
                mensagem,
                retorno: &format!("/execucoes/{id}"),
            }
            .render();
            return Ok((StatusCode::UNPROCESSABLE_ENTITY, pagina).into_response());
        }
    };
    Ok(
        apresentacao::cenario(&e, &formularios::campos_cenario(&c), &[], false)
            .await?
            .into_response(),
    )
}

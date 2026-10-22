//! Handler HTTP para historico, mantendo a renderização desta responsabilidade isolada.
use crate::aplicacao::historico::{self, FiltroHistorico};
use crate::web::{erro::ErroWeb, estado::EstadoApp};
use axum::extract::{Query, State};
use axum::{Router, routing::get};
use maud::{Markup, Render};
use std::collections::BTreeMap;

/// Rotas HTTP deste recurso para composição no roteador principal.
pub fn rotas() -> Router<EstadoApp> {
    Router::new().route("/historico", get(historico))
}

pub async fn historico(
    State(e): State<EstadoApp>,
    Query(q): Query<BTreeMap<String, String>>,
) -> Result<Markup, ErroWeb> {
    let filtro = FiltroHistorico::novo(
        q.get("pagina").and_then(|v| v.parse().ok()),
        q.get("lote").cloned().unwrap_or_default(),
        q.get("data").cloned().unwrap_or_default(),
    );
    let execucoes = historico::listar(&e.pool, &filtro).await?;
    let referencia_id = q.get("referencia").map(String::as_str).unwrap_or("");
    let referencia = historico::referencia(&e.pool, referencia_id).await?;
    Ok(crate::web::templates::paginas::historico::Historico {
        execucoes: &execucoes,
        referencia: referencia.as_ref(),
        referencia_id,
        pagina: filtro.pagina,
        busca: &filtro.busca,
        data: &filtro.data,
    }
    .render())
}

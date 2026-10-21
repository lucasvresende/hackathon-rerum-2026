use crate::aplicacao::historico::{self, FiltroHistorico};
use crate::web::{erro::ErroWeb, estado::EstadoApp};
use axum::extract::{Query, State};
use maud::{Markup, Render};
use std::collections::BTreeMap;

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
    Ok(crate::web::templates::paginas::historico::Historico {
        execucoes: &execucoes,
        pagina: filtro.pagina,
        busca: &filtro.busca,
        data: &filtro.data,
    }
    .render())
}

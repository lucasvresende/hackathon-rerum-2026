use super::rotas::{ErroWeb, EstadoApp};
use crate::dominio::unidades::proximos;
use crate::persistencia::historico as banco;
use axum::extract::{Query, State};
use maud::{Markup, Render};
use std::collections::BTreeMap;

pub async fn comparar(
    State(e): State<EstadoApp>,
    Query(q): Query<BTreeMap<String, String>>,
) -> Result<Markup, ErroWeb> {
    let a = banco::obter(&e.pool, q.get("a").map(String::as_str).unwrap_or("")).await?;
    let b = banco::obter(&e.pool, q.get("b").map(String::as_str).unwrap_or("")).await?;
    let equivalentes = a.cenario.perfil_id == b.cenario.perfil_id
        && proximos(a.cenario.lote.producao, b.cenario.lote.producao)
        && proximos(a.parecer.consumo_ms, b.parecer.consumo_ms)
        && a.cenario.horizonte == b.cenario.horizonte
        && a.cenario.referencia_ue == b.cenario.referencia_ue;
    Ok(super::templates::paginas::comparacao::Comparacao {
        a: &a,
        b: &b,
        equivalentes,
    }
    .render())
}

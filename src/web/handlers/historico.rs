use crate::persistencia::historico;
use crate::web::{erro::ErroWeb, estado::EstadoApp};
use axum::extract::{Query, State};
use maud::{Markup, Render};
use std::collections::BTreeMap;

pub async fn historico(
    State(e): State<EstadoApp>,
    Query(q): Query<BTreeMap<String, String>>,
) -> Result<Markup, ErroWeb> {
    let pagina = q
        .get("pagina")
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(1)
        .clamp(1, 1000000);
    let busca = q.get("lote").cloned().unwrap_or_default();
    let data = q.get("data").cloned().unwrap_or_default();
    let filtro = (!busca.is_empty()).then(|| format!("%{busca}%"));
    let dia = chrono::NaiveDate::parse_from_str(&data, "%Y-%m-%d").ok();
    let instante = |dia: chrono::NaiveDate| {
        dia.and_hms_opt(0, 0, 0)
            .and_then(|n| {
                n.and_local_timezone(chrono_tz::America::Sao_Paulo)
                    .earliest()
            })
            .map(|v| v.with_timezone(&chrono::Utc).to_rfc3339())
    };
    let inicio = dia.and_then(instante);
    let fim = dia.and_then(|d| d.succ_opt()).and_then(instante);
    let execucoes = historico::filtrar(
        &e.pool,
        filtro.as_deref(),
        inicio.as_deref(),
        fim.as_deref(),
        (pagina - 1) * 100,
    )
    .await?;
    Ok(crate::web::templates::paginas::historico::Historico {
        execucoes: &execucoes,
        pagina,
        busca: &busca,
        data: &data,
    }
    .render())
}

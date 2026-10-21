use crate::{
    Resultado,
    persistencia::historico::{self, Execucao},
};
use sqlx::SqlitePool;

pub struct FiltroHistorico {
    pub pagina: i64,
    pub busca: String,
    pub data: String,
}
impl FiltroHistorico {
    pub fn novo(pagina: Option<i64>, busca: String, data: String) -> Self {
        Self {
            pagina: pagina.unwrap_or(1).clamp(1, 1000000),
            busca,
            data,
        }
    }
}

/// A data civil de São Paulo corresponde a um intervalo UTC semiaberto.
pub async fn listar(pool: &SqlitePool, filtro: &FiltroHistorico) -> Resultado<Vec<Execucao>> {
    let busca = (!filtro.busca.is_empty()).then(|| format!("%{}%", filtro.busca));
    let dia = chrono::NaiveDate::parse_from_str(&filtro.data, "%Y-%m-%d").ok();
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
    historico::filtrar(
        pool,
        busca.as_deref(),
        inicio.as_deref(),
        fim.as_deref(),
        (filtro.pagina - 1) * 100,
    )
    .await
}

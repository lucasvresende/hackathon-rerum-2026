//! Consulta e filtragem de execuções preservadas no histórico.
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

impl FiltroHistorico {
    /// Codifica valores como componentes de query, inclusive UTF-8 e separadores literais.
    pub fn url(&self, pagina: i64, referencia: &str) -> String {
        fn codificar(valor: &str) -> String {
            use std::fmt::Write;
            let mut saida = String::new();
            for b in valor.bytes() {
                if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
                    saida.push(b as char);
                } else {
                    write!(saida, "%{b:02X}").expect("escrita em String");
                }
            }
            saida
        }
        format!(
            "/historico?pagina={}&lote={}&data={}&referencia={}",
            pagina,
            codificar(&self.busca),
            codificar(&self.data),
            codificar(referencia)
        )
    }
}

/// Só a ausência da referência é recuperável; falhas de banco continuam sendo erros.
pub async fn referencia(pool: &SqlitePool, id: &str) -> Resultado<Option<Execucao>> {
    if id.is_empty() {
        return Ok(None);
    }
    match historico::obter(pool, id).await {
        Ok(e) => Ok(Some(e)),
        Err(e)
            if matches!(
                e.downcast_ref::<sqlx::Error>(),
                Some(sqlx::Error::RowNotFound)
            ) =>
        {
            Ok(None)
        }
        Err(e) => Err(e),
    }
}

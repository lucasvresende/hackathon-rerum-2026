use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

use crate::Resultado;

pub static MIGRACOES: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn abrir(caminho: &Path, criar: bool) -> Resultado<SqlitePool> {
    if criar {
        if let Some(pai) = caminho.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(pai)?;
        }
    } else if !caminho.is_file() {
        return Err("Base ausente. Execute `feed-your-cow preparar` antes de servir.".into());
    }
    let opcoes = SqliteConnectOptions::from_str(&format!("sqlite://{}", caminho.display()))?
        .create_if_missing(criar)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(5));
    Ok(SqlitePoolOptions::new()
        .max_connections(4)
        .connect_with(opcoes)
        .await?)
}

pub async fn preparar(caminho: &Path) -> Resultado<SqlitePool> {
    let pool = abrir(caminho, true).await?;
    MIGRACOES.run(&pool).await?;
    super::conhecimento::carregar(
        &pool,
        include_str!("../../dados_referencia/001_conhecimento.sql"),
    )
    .await?;
    Ok(pool)
}

use std::path::Path;

use sqlx::SqlitePool;

use crate::Resultado;

/// VACUUM INTO gera um snapshot SQLite consistente, inclusive com escritores concorrentes.
pub async fn backup(pool: &SqlitePool, destino: &Path) -> Resultado<()> {
    std::fs::create_dir(destino)?;
    let caminho = destino.join("aplicacao.sqlite3");
    let nome = caminho.to_str().ok_or("Caminho inválido")?;
    sqlx::query_file!("queries/backup.sql", nome)
        .execute(pool)
        .await?;
    std::fs::write(
        destino.join("anexo.md"),
        include_str!("../../hackathon/TEMA_2_DIETAS_LITERATURA_E_MOTOR.md"),
    )?;
    std::fs::write(
        destino.join("requisitos.md"),
        include_str!("../../REQUISITOS.md"),
    )?;
    std::fs::write(
        destino.join("conhecimento.sql"),
        include_str!("../../dados_referencia/001_conhecimento.sql"),
    )?;
    std::fs::write(
        destino.join("modelos.md"),
        include_str!("../../fontes/MODELOS.md"),
    )?;
    std::fs::write(
        destino.join("modelos.sql"),
        include_str!("../../dados_referencia/004_modelos.sql"),
    )?;
    let mut manifesto = std::collections::BTreeMap::new();
    for nome in [
        "aplicacao.sqlite3",
        "anexo.md",
        "requisitos.md",
        "conhecimento.sql",
        "modelos.md",
        "modelos.sql",
    ] {
        let bytes = std::fs::read(destino.join(nome))?;
        use sha2::{Digest, Sha256};
        manifesto.insert(nome, format!("{:x}", Sha256::digest(bytes)));
    }
    std::fs::write(
        destino.join("manifesto.json"),
        serde_json::to_vec_pretty(&manifesto)?,
    )?;
    Ok(())
}

pub async fn restaurar(origem: &Path, destino: &Path) -> Resultado<()> {
    if destino.exists() {
        return Err("Destino já existe; restauração não sobrescreve dados.".into());
    }
    let manifesto: std::collections::BTreeMap<String, String> =
        serde_json::from_slice(&std::fs::read(origem.join("manifesto.json"))?)?;
    for nome in [
        "aplicacao.sqlite3",
        "anexo.md",
        "requisitos.md",
        "conhecimento.sql",
    ] {
        use sha2::{Digest, Sha256};
        let resumo = format!("{:x}", Sha256::digest(std::fs::read(origem.join(nome))?));
        if manifesto.get(nome) != Some(&resumo) {
            return Err(format!("Backup corrompido: {nome}").into());
        }
    }
    for nome in ["modelos.md", "modelos.sql"] {
        if let Some(esperado) = manifesto.get(nome) {
            use sha2::{Digest, Sha256};
            let hash = format!("{:x}", Sha256::digest(std::fs::read(origem.join(nome))?));
            if esperado != &hash {
                return Err(format!("Backup corrompido: {nome}").into());
            }
        }
    }
    std::fs::create_dir(destino)?;
    let banco = destino.join("aplicacao.sqlite3");
    std::fs::copy(origem.join("aplicacao.sqlite3"), &banco)?;
    let pool = super::banco::abrir(&banco, false).await?;
    let integridade = sqlx::query_file!("queries/integridade.sql")
        .fetch_all(&pool)
        .await?;
    if integridade.len() != 1 || integridade[0].integrity_check.as_deref() != Some("ok") {
        return Err("Integridade SQLite inválida.".into());
    }
    if !sqlx::query_file!("queries/vinculos.sql")
        .fetch_all(&pool)
        .await?
        .is_empty()
    {
        return Err("Backup com referências inválidas.".into());
    }
    pool.close().await;
    for nome in [
        "anexo.md",
        "requisitos.md",
        "conhecimento.sql",
        "manifesto.json",
    ] {
        std::fs::copy(origem.join(nome), destino.join(nome))?;
    }
    for nome in ["modelos.md", "modelos.sql"] {
        if manifesto.contains_key(nome) {
            std::fs::copy(origem.join(nome), destino.join(nome))?;
        }
    }
    Ok(())
}

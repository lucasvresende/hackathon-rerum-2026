use std::collections::BTreeMap;

use axum::http::HeaderMap;

pub fn validar(
    headers: &HeaderMap,
    campos: &BTreeMap<String, String>,
    token: &str,
    origem: &str,
) -> bool {
    let csrf = campos.get("csrf").is_some_and(|v| v == token);
    let mesma_origem = headers
        .get("origin")
        .is_none_or(|v| v.to_str().ok() == Some(origem));
    let contexto = headers
        .get("sec-fetch-site")
        .is_none_or(|v| v != "cross-site");
    csrf && mesma_origem && contexto
}

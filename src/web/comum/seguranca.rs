//! Proteções HTTP, validação CSRF e política de cabeçalhos da aplicação.
use super::estado::EstadoApp;
use std::collections::BTreeMap;

use axum::http::{HeaderMap, HeaderValue};
use axum::response::Response;

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

pub async fn cabecalhos(req: axum::extract::Request, proximo: axum::middleware::Next) -> Response {
    let mut r = proximo.run(req).await;
    r.headers_mut().insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );
    r.headers_mut()
        .insert("referrer-policy", HeaderValue::from_static("same-origin"));
    r.headers_mut()
        .insert("x-frame-options", HeaderValue::from_static("DENY"));
    r
}

pub fn autorizado(
    estado: &EstadoApp,
    headers: &HeaderMap,
    campos: &super::entradas::Campos,
) -> bool {
    validar(headers, campos, &estado.csrf, &estado.origem)
}

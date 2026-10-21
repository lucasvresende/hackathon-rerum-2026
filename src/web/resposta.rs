//! Respostas HTTP auxiliares, incluindo redirecionamento compatível com htmx.
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};

pub fn redirecionar(url: &str, headers: &HeaderMap) -> Response {
    if headers.contains_key("hx-request") {
        ([("hx-redirect", url)], "").into_response()
    } else {
        axum::response::Redirect::to(url).into_response()
    }
}

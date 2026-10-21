use super::estado::EstadoApp;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};

pub fn autorizado(
    estado: &EstadoApp,
    headers: &HeaderMap,
    campos: &super::entradas::Campos,
) -> bool {
    super::seguranca::validar(headers, campos, &estado.csrf, &estado.origem)
}

pub fn redirecionar(url: &str, headers: &HeaderMap) -> Response {
    if headers.contains_key("hx-request") {
        ([("hx-redirect", url)], "").into_response()
    } else {
        axum::response::Redirect::to(url).into_response()
    }
}

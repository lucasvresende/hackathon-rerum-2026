use crate::web::estado::EstadoApp;
use crate::web::handlers::conhecimento;
use axum::{Router, routing::get};

pub(super) fn rotas() -> Router<EstadoApp> {
    Router::new().route("/conhecimento", get(conhecimento::conhecimento))
}

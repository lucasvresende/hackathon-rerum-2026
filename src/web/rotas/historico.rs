use crate::web::estado::EstadoApp;
use crate::web::handlers::{comparacao, historico};
use axum::{Router, routing::get};

pub(super) fn rotas() -> Router<EstadoApp> {
    Router::new()
        .route("/historico", get(historico::historico))
        .route("/comparar", get(comparacao::comparar))
}

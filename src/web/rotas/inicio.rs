use crate::web::estado::EstadoApp;
use crate::web::handlers::{ajuda, inicio};
use axum::{Router, routing::get};

pub(super) fn rotas() -> Router<EstadoApp> {
    Router::new()
        .route("/", get(inicio::inicio))
        .route("/ajuda", get(ajuda::ajuda))
        .route("/saude", get(|| async { "FeedYourCow: disponível" }))
}

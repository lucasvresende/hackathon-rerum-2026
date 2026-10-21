use crate::web::estado::EstadoApp;
use crate::web::handlers::cenarios;
use axum::{
    Router,
    routing::{get, post},
};

pub(super) fn rotas() -> Router<EstadoApp> {
    Router::new()
        .route("/cenarios/atualizar", post(cenarios::atualizar))
        .route("/cenarios/novo", get(cenarios::novo))
        .route("/cenarios", post(cenarios::executar))
}

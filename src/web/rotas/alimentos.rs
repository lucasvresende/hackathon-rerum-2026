use crate::web::estado::EstadoApp;
use crate::web::handlers::alimentos;
use axum::{Router, routing::get};

pub(super) fn rotas() -> Router<EstadoApp> {
    Router::new()
        .route("/alimentos", get(alimentos::listar).post(alimentos::salvar))
        .route("/alimentos/novo", get(alimentos::novo))
        .route("/alimentos/{id}/revisar", get(alimentos::revisar))
}

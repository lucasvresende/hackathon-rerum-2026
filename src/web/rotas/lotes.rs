use crate::web::estado::EstadoApp;
use crate::web::handlers::lotes;
use axum::{Router, routing::get};

pub(super) fn rotas() -> Router<EstadoApp> {
    Router::new()
        .route("/lotes", get(lotes::listar).post(lotes::salvar))
        .route("/lotes/novo", get(lotes::novo))
        .route("/lotes/{id}/editar", get(lotes::editar))
}

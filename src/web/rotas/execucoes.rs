use crate::web::estado::EstadoApp;
use crate::web::handlers::execucoes;
use axum::{
    Router,
    routing::{get, post},
};

pub(super) fn rotas() -> Router<EstadoApp> {
    Router::new()
        .route("/execucoes/{id}", get(execucoes::resultado))
        .route("/execucoes/{id}/duplicar", get(execucoes::duplicar))
        .route("/execucoes/{id}/reproduzir", post(execucoes::reproduzir))
}

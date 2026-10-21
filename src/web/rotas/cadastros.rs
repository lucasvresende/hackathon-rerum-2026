use crate::web::estado::EstadoApp;
use crate::web::handlers::{cadastros, modelos_preenchimento};
use axum::{
    Router,
    routing::{get, post},
};

pub(super) fn rotas() -> Router<EstadoApp> {
    Router::new()
        .route(
            "/cadastros/{tipo}/modelo",
            post(modelos_preenchimento::carregar),
        )
        .route("/cadastros/salvo", get(cadastros::pagina))
}

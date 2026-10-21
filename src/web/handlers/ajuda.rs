use crate::web::estado::EstadoApp;
use axum::{Router, routing::get};
use maud::{Markup, Render};

/// Rotas HTTP deste recurso para composição no roteador principal.
pub fn rotas() -> Router<EstadoApp> {
    Router::new().route("/ajuda", get(ajuda))
}

pub async fn ajuda() -> Markup {
    crate::web::templates::paginas::ajuda::Ajuda.render()
}

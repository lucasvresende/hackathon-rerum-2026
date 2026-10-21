//! Composição das rotas por recurso; middlewares são aplicados uma única vez.
pub use super::estado::EstadoApp;
use axum::{Router, extract::DefaultBodyLimit};
use tower_http::{services::ServeDir, trace::TraceLayer};
// Compatibilidade com consumidores anteriores à separação dos adaptadores HTTP.
pub use super::erro::ErroWeb;
pub use super::resposta::{autorizado, redirecionar};
mod alimentos;
mod cadastros;
mod cenarios;
mod conhecimento;
mod execucoes;
mod historico;
mod inicio;
mod lotes;

pub fn app(estado: EstadoApp) -> Router {
    let app = Router::new()
        .merge(inicio::rotas())
        .merge(lotes::rotas())
        .merge(alimentos::rotas())
        .merge(cadastros::rotas())
        .merge(cenarios::rotas())
        .merge(execucoes::rotas())
        .merge(historico::rotas())
        .merge(conhecimento::rotas())
        .nest_service("/static", ServeDir::new("static"))
        .fallback(super::handlers::inicio::nao_encontrado)
        .layer(DefaultBodyLimit::max(1024 * 1024))
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(super::seguranca::cabecalhos))
        .with_state(estado);
    #[cfg(feature = "desenvolvimento")]
    let app = app.layer(tower_livereload::LiveReloadLayer::new());
    app
}

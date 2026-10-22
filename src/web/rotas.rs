//! Composição das rotas por recurso e aplicação dos middlewares globais.
pub use super::estado::EstadoApp;
use axum::{Router, extract::DefaultBodyLimit};
use tower_http::{services::ServeDir, trace::TraceLayer};
// Compatibilidade com consumidores anteriores à separação dos adaptadores HTTP.
pub use super::erro::ErroWeb;
pub use super::resposta::redirecionar;
pub use super::seguranca::autorizado;
use super::{
    ajuda, alimentos, cadastros, cenarios, comparacao, conhecimento, execucoes, exemplos,
    historico, inicio, lotes, modelos_preenchimento,
};

pub fn app(estado: EstadoApp) -> Router {
    let app = Router::new()
        .merge(inicio::rotas::rotas())
        .merge(exemplos::rotas::rotas())
        .merge(ajuda::rotas::rotas())
        .merge(lotes::rotas::rotas())
        .merge(alimentos::rotas::rotas())
        .merge(cadastros::rotas::rotas())
        .merge(modelos_preenchimento::rotas::rotas())
        .merge(cenarios::rotas::rotas())
        .merge(execucoes::rotas::rotas())
        .merge(historico::rotas::rotas())
        .merge(comparacao::rotas::rotas())
        .merge(conhecimento::rotas::rotas())
        .nest_service("/static", ServeDir::new("static"))
        .fallback(inicio::handlers::nao_encontrado)
        .layer(DefaultBodyLimit::max(1024 * 1024))
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(super::seguranca::cabecalhos))
        .with_state(estado);
    #[cfg(feature = "desenvolvimento")]
    let app = app.layer(tower_livereload::LiveReloadLayer::new());
    app
}

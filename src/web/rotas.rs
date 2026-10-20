use std::sync::Arc;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use maud::Render;
use sqlx::SqlitePool;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use super::{cadastros, cenarios, paginas};

#[derive(Clone)]
pub struct EstadoApp {
    pub pool: SqlitePool,
    pub csrf: Arc<String>,
    pub origem: Arc<String>,
}

impl EstadoApp {
    pub fn novo(pool: SqlitePool, origem: String) -> Self {
        Self {
            pool,
            csrf: Arc::new(uuid::Uuid::new_v4().to_string()),
            origem: Arc::new(origem),
        }
    }
}

pub fn app(estado: EstadoApp) -> Router {
    let app = Router::new()
        .route("/", get(paginas::inicio))
        .route("/ajuda", get(paginas::ajuda))
        .route("/saude", get(|| async { "FeedYourCow: disponível" }))
        .route("/lotes", get(paginas::lotes).post(cadastros::salvar_lote))
        .route("/lotes/novo", get(cadastros::novo_lote))
        .route("/lotes/{id}/editar", get(cadastros::editar_lote))
        .route(
            "/alimentos",
            get(paginas::alimentos).post(cadastros::salvar_alimento),
        )
        .route("/alimentos/novo", get(cadastros::novo_alimento))
        .route("/alimentos/{id}/revisar", get(cadastros::revisar_alimento))
        .route("/cenarios/novo", get(cenarios::novo))
        .route("/cenarios", post(cenarios::executar))
        .route("/historico", get(paginas::historico))
        .route("/comparar", get(super::comparacao::comparar))
        .route("/conhecimento", get(paginas::conhecimento))
        .route("/execucoes/{id}", get(cenarios::resultado))
        .route("/execucoes/{id}/duplicar", get(cenarios::duplicar))
        .route("/execucoes/{id}/reproduzir", post(cenarios::reproduzir))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(paginas::nao_encontrado)
        .layer(DefaultBodyLimit::max(1024 * 1024))
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(cabecalhos))
        .with_state(estado);
    #[cfg(feature = "desenvolvimento")]
    let app = app.layer(tower_livereload::LiveReloadLayer::new());
    app
}

async fn cabecalhos(req: axum::extract::Request, proximo: axum::middleware::Next) -> Response {
    let mut r = proximo.run(req).await;
    r.headers_mut().insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );
    r.headers_mut()
        .insert("referrer-policy", HeaderValue::from_static("same-origin"));
    r.headers_mut()
        .insert("x-frame-options", HeaderValue::from_static("DENY"));
    r
}

pub struct ErroWeb(pub crate::Erro);
impl From<crate::Erro> for ErroWeb {
    fn from(e: crate::Erro) -> Self {
        Self(e)
    }
}
impl IntoResponse for ErroWeb {
    fn into_response(self) -> Response {
        tracing::error!(erro = %self.0, "Falha operacional");
        let corpo = super::componentes::estrutura::Pagina {
            titulo: "Não foi possível concluir",
            ativo: "",
            ajuda: "inicio",
            conteudo: maud::html! {
                h1 { "Não foi possível concluir esta operação" }
                p { "Houve uma falha operacional. Seus resultados anteriores estão preservados. \
                    Volte à tela anterior para tentar novamente." }
                a.button href="/" { "Voltar à visão geral" }
            },
        }
        .render();
        (StatusCode::INTERNAL_SERVER_ERROR, corpo).into_response()
    }
}

pub fn autorizado(
    estado: &EstadoApp,
    headers: &HeaderMap,
    campos: &super::formularios::Campos,
) -> bool {
    super::seguranca::validar(headers, campos, &estado.csrf, &estado.origem)
}

pub fn redirecionar(url: &str, headers: &HeaderMap) -> Response {
    if headers.contains_key("hx-request") {
        ([("hx-redirect", url)], "").into_response()
    } else {
        axum::response::Redirect::to(url).into_response()
    }
}

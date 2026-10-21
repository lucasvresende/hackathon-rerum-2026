use super::templates::paginas::erro::{Erro, TipoErro};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use maud::Render;

pub struct ErroWeb(pub crate::Erro);
impl From<crate::Erro> for ErroWeb {
    fn from(e: crate::Erro) -> Self {
        Self(e)
    }
}
impl IntoResponse for ErroWeb {
    fn into_response(self) -> Response {
        tracing::error!(erro = %self.0, "Falha operacional");
        let corpo = Erro(TipoErro::Operacional).render();
        (StatusCode::INTERNAL_SERVER_ERROR, corpo).into_response()
    }
}

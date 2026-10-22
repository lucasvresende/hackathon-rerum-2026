//! Handler HTTP para cadastros, mantendo a renderização desta responsabilidade isolada.
use crate::web::entradas::{Campos, texto};
use crate::web::estado::EstadoApp;
use crate::web::templates::paginas::cadastro_salvo::CadastroSalvo;
use axum::extract::Query;
use axum::{Router, routing::get};
use maud::{Markup, Render};

/// Rotas HTTP deste recurso para composição no roteador principal.
pub fn rotas() -> Router<EstadoApp> {
    Router::new().route("/cadastros/salvo", get(pagina))
}

pub async fn pagina(Query(campos): Query<Campos>) -> Markup {
    CadastroSalvo {
        cancelado: texto(&campos, "tipo") == "cancelado",
        lote: texto(&campos, "tipo") == "lote",
    }
    .render()
}

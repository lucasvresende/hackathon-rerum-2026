use crate::web::entradas::{Campos, texto};
use crate::web::templates::paginas::cadastro_salvo::CadastroSalvo;
use axum::extract::Query;
use maud::{Markup, Render};

pub async fn pagina(Query(campos): Query<Campos>) -> Markup {
    CadastroSalvo {
        cancelado: texto(&campos, "tipo") == "cancelado",
        lote: texto(&campos, "tipo") == "lote",
    }
    .render()
}

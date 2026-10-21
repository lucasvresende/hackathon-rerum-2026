use maud::{Markup, Render};

pub async fn ajuda() -> Markup {
    crate::web::templates::paginas::ajuda::Ajuda.render()
}

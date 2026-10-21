use maud::{Markup, Render, html};

pub struct Cabecalho<'a> {
    pub titulo: &'a str,
    pub texto: &'a str,
    pub ajuda: &'a str,
}
impl Render for Cabecalho<'_> {
    fn render(&self) -> Markup {
        let Self {
            titulo,
            texto,
            ajuda,
        } = self;
        html! {
            div.page-heading {
                div.page-title-row {
                    h1 { (titulo) }
                    a.button.secondary.page-help.no-print href=(format!("/ajuda#{ajuda}")) {
                        "Ajuda nesta página"
                    }
                }
                @if !texto.is_empty() {
                    p.subtitle { (texto) }
                }
            }
        }
    }
}

//! Página de apresentação para ajuda, mantendo a renderização desta responsabilidade isolada.
use super::super::{componentes, layouts::Principal};
use maud::{Markup, Render};

pub struct Ajuda;
impl Render for Ajuda {
    fn render(&self) -> Markup {
        Principal {
            titulo: "Ajuda",
            ativo: "/ajuda",
            conteudo: componentes::ajuda::Ajuda.render(),
        }
        .render()
    }
}

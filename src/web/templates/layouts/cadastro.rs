//! Layout HTML para cadastro, mantendo a renderização desta responsabilidade isolada.
use super::documento::Documento;
use maud::{Markup, Render, html};

/// Página de cadastro em guia própria, sem navegação lateral.
pub struct Cadastro<'a> {
    pub titulo: &'a str,
    pub conteudo: Markup,
}
impl Render for Cadastro<'_> {
    fn render(&self) -> Markup {
        Documento {
            titulo: self.titulo,
            isolada: true,
            menu: html! {},
            conteudo: &self.conteudo,
        }
        .render()
    }
}

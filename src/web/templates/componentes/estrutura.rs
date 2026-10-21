pub use super::cabecalho::Cabecalho;
pub use super::campo::Campo;
use maud::{Markup, Render};

pub struct Pagina<'a> {
    pub titulo: &'a str,
    pub ativo: &'a str,
    pub conteudo: Markup,
}
impl Render for Pagina<'_> {
    fn render(&self) -> Markup {
        if self.ativo == "cadastro" {
            super::super::layouts::Cadastro {
                titulo: self.titulo,
                conteudo: self.conteudo.clone(),
            }
            .render()
        } else {
            super::super::layouts::Principal {
                titulo: self.titulo,
                ativo: self.ativo,
                conteudo: self.conteudo.clone(),
            }
            .render()
        }
    }
}

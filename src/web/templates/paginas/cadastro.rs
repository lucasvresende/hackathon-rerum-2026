use super::super::componentes::cabecalho::Cabecalho;
use super::super::layouts::Cadastro;
use maud::{Markup, Render, html};

pub struct PaginaCadastro<'a> {
    pub tipo: &'a str,
    pub formulario: Markup,
}
impl Render for PaginaCadastro<'_> {
    fn render(&self) -> Markup {
        Cadastro {
            titulo: "Cadastro",
            conteudo: html! {
                (Cabecalho { titulo: if self.tipo == "lote" { "Cadastro de lote" }
                else { "Composição do alimento" },
                    texto: "Cada alteração cria uma revisão. \
                        Execuções anteriores permanecem iguais.",
                    ajuda: if self.tipo == "lote" { "lotes" } else { "alimentos" } })
                (self.formulario)
            },
        }
        .render()
    }
}

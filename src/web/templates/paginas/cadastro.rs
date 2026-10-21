use super::super::componentes::cabecalho::Cabecalho;
use super::super::layouts::Cadastro;
use crate::dominio::templates::TipoCadastro;
use maud::{Markup, Render, html};

pub struct PaginaCadastro {
    pub tipo: TipoCadastro,
    pub formulario: Markup,
}
impl Render for PaginaCadastro {
    fn render(&self) -> Markup {
        Cadastro {
            titulo: "Cadastro",
            conteudo: html! {
                (Cabecalho { titulo: if self.tipo == TipoCadastro::Lote { "Cadastro de lote" }
                else { "Composição do alimento" },
                    texto: "Cada alteração cria uma revisão. \
                        Execuções anteriores permanecem iguais.",
                    ajuda: if self.tipo == TipoCadastro::Lote { "lotes" } else { "alimentos" } })
                (self.formulario)
            },
        }
        .render()
    }
}

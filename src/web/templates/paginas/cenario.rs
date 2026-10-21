use super::super::componentes::cabecalho::Cabecalho;
use super::super::layouts::Cadastro;
use maud::{Markup, Render, html};

pub struct Cenario {
    pub sem_lotes: bool,
    pub formulario: Markup,
}
impl Render for Cenario {
    fn render(&self) -> Markup {
        Cadastro {
            titulo: "Novo cenário",
            conteudo: html! {
                (Cabecalho {
                    titulo: "Novo cenário",
                    texto: "Selecione o lote, o tipo de cálculo e os ingredientes.",
                    ajuda: "cenarios"
                })
                @if self.sem_lotes { div.alert { "Cadastre um lote antes de calcular." } }
                (self.formulario)
            },
        }
        .render()
    }
}

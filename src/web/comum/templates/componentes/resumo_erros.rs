//! Exibição resumida dos problemas de validação.
use crate::dominio::validacao::Problema;
use maud::{Markup, Render, html};

pub struct ResumoErros<'a>(pub &'a [Problema]);
impl Render for ResumoErros<'_> {
    fn render(&self) -> Markup {
        let problemas = self.0;
        html! {
            @if !problemas.is_empty() {
                div #erros-formulario.alert.error tabindex="-1" role="alert" {
                    strong { "Revise os campos para continuar" }
                    ul { @for erro in problemas { li {
                            a href=(format!("#{}", if erro.campo == "itens" {
                                    "selecao-ingredientes" } else { &erro.campo })) {
                                    (&erro.mensagem) } } } }
                }
            }
        }
    }
}

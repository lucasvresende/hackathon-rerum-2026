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
                    ul { @for erro in problemas { li { a href=(format!("#{}", erro.campo)) {
                                    (&erro.campo) ": " (&erro.mensagem) } } } }
                }
            }
        }
    }
}

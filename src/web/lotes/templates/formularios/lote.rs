//! Formulário de apresentação para lote, mantendo a renderização desta responsabilidade isolada.
use super::DadosCadastro;
use crate::dominio::templates::TipoCadastro;
use maud::{Markup, Render};

pub struct FormularioLote<'a>(pub DadosCadastro<'a>);
impl Render for FormularioLote<'_> {
    fn render(&self) -> Markup {
        self.0.formulario(TipoCadastro::Lote).render()
    }
}

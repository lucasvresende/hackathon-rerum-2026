use super::DadosCadastro;
use crate::dominio::templates::TipoCadastro;
use maud::{Markup, Render};

pub struct FormularioAlimento<'a>(pub DadosCadastro<'a>);
impl Render for FormularioAlimento<'_> {
    fn render(&self) -> Markup {
        self.0.formulario(TipoCadastro::Alimento).render()
    }
}

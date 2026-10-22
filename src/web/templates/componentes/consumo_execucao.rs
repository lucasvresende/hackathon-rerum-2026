//! Alvo efetivo e origem estruturada do consumo utilizado na execução.
use crate::dominio::{Cenario, Parecer};
use crate::web::formatacao::numero;
use maud::{Markup, Render, html};

pub struct ConsumoExecucao<'a> {
    pub cenario: &'a Cenario,
    pub parecer: &'a Parecer,
}
impl Render for ConsumoExecucao<'_> {
    fn render(&self) -> Markup {
        let c = self.cenario;
        let alvo = c.consumo_informado().or(self.parecer.consumo_referencia);
        html! {
            dt { "Consumo-alvo utilizado" }
            dd { (alvo.map(|v| format!("{} kg MS/animal/dia", numero(v, 4)))
                .unwrap_or("Não calculável".into())) }
            dt { "Origem efetiva do consumo" } dd { (c.tipo_consumo()) }
            dt { "Consumo-alvo informado" }
            dd { (c.consumo_alvo.map(|v| numero(v, 4)).unwrap_or("Não informado".into())) }
            dt { "Registro da origem do consumo" } dd { (&c.origem_consumo) }
        }
    }
}

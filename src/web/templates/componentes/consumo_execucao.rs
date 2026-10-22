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
            @if c.formular {
                dt { "Consumo-alvo utilizado na formulação" }
                dd { (alvo.map(|v| format!("{} kg MS/animal/dia", numero(v, 4)))
                    .unwrap_or("Não calculável".into())) }
                dt { "Origem do consumo-alvo" } dd { (c.tipo_consumo()) }
            } @else {
                dt { "Consumo calculado das quantidades informadas" }
                dd { (numero(self.parecer.consumo_ms, 4)) " kg MS/animal/dia" }
                dt { "Critério da verificação" }
                dd { "Avalia a dieta informada; o consumo-alvo não impõe igualdade. "
                    "O aviso de discrepância compara a dieta com a referência do perfil." }
            }
            dt { "Consumo-alvo informado" }
            dd { (c.consumo_alvo.map(|v| numero(v, 4)).unwrap_or("Não informado".into())) }
            dt { "Registro da origem do consumo-alvo" } dd { (&c.origem_consumo) }
        }
    }
}

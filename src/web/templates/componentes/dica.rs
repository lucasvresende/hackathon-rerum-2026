use maud::{Markup, Render, html};

/// Explicação curta acionada por botão nativo; o popover funciona por clique sem JavaScript.
pub struct Dica {
    pub id: &'static str,
    pub conceito: &'static str,
    pub texto: &'static str,
}

impl Render for Dica {
    fn render(&self) -> Markup {
        html! {
            button.concept-trigger type="button" popovertarget=(self.id)
                aria-label=(format!("Entender: {}", self.conceito)) aria-describedby=(self.id) {
                "?"
            }
            div.concept-tip id=(self.id) popover="auto" role="tooltip" {
                b { (self.conceito) }
                p { (self.texto) }
            }
        }
    }
}

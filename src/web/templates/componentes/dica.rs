//! Componente de apresentação para dica, mantendo a renderização desta responsabilidade isolada.
use maud::{Markup, Render, html};

/// Explicação curta acionada por botão nativo; o popover funciona por clique sem JavaScript.
pub struct Dica {
    pub id: &'static str,
    pub conceito: &'static str,
    pub texto: &'static str,
}

impl Dica {
    /// Sufixos por campo ou composição evitam IDs duplicados em listas e formulários.
    pub fn com_id(&self, id: &str) -> Markup {
        html! {
            button.concept-trigger type="button" popovertarget=(id)
                aria-label=(format!("Entender: {}", self.conceito)) aria-describedby=(id) {
                "?"
            }
            div.concept-tip id=(id) popover="auto" role="tooltip" {
                b { (self.conceito) }
                p { (self.texto) }
            }
        }
    }
}

impl Render for Dica {
    fn render(&self) -> Markup {
        self.com_id(self.id)
    }
}

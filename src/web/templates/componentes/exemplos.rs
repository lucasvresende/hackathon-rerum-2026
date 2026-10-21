use maud::{Markup, Render, html};

pub struct Remover<'a> {
    pub tipo: &'a str,
    pub id: &'a str,
    pub token: &'a str,
}

impl Render for Remover<'_> {
    fn render(&self) -> Markup {
        html! {
            details {
                summary { "Remover cadastro" }
                form method="post"
                action=(format!("/cadastros/{}/{}/remover", self.tipo, self.id)) {
                    input type="hidden" name="csrf" value=(self.token);
                    label.check {
                        input type="checkbox" name="confirmar" required;
                        "Retirar das opções disponíveis; o histórico será preservado"
                    }
                    button.button.secondary { "Confirmar remoção" }
                }
            }
        }
    }
}

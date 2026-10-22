//! Substituição de rascunho em duas etapas, com confirmação processada no servidor.
use super::cenario::FormularioCenario;
use maud::{Markup, Render, html};

pub struct ExemploCenario<'a, 'b>(pub &'a FormularioCenario<'b>);
impl Render for ExemploCenario<'_, '_> {
    fn render(&self) -> Markup {
        let f = self.0;
        let pendente = f.valor("exemplo_pendente");
        html! {
            @for dieta in ["A", "B"] {
                @let rota = format!("/cenarios/exemplo?dieta={dieta}");
                button.button.secondary type="submit" formnovalidate data-sem-validacao
                    formaction=(&rota) hx-post=(&rota) hx-target="#formulario"
                    hx-swap="outerHTML settle:0" { "Demonstração " (dieta) }
            }
            @if ["A", "B"].contains(&pendente) {
                section.card #exemplo_pendente {
                    h2 { "Substituir pelo exemplo " (pendente) "?" }
                    p { "O preenchimento substituirá nome, lote, ingredientes e premissas. "
                        "O tipo de cálculo será mantido. Seu rascunho permanece até confirmar." }
                    input type="hidden" name="exemplo_pendente" value=(pendente);
                    (super::super::componentes::campo::MensagemErro {
                        nome: "confirmar_exemplo", erro: f.erro("confirmar_exemplo") })
                    label.check for="confirmar_exemplo" {
                        input #confirmar_exemplo type="checkbox" name="confirmar_exemplo";
                        "Confirmo substituir os dados do rascunho."
                    }
                    @for (acao, rotulo) in [("aplicar", "Preencher com exemplo"),
                        ("cancelar", "Manter rascunho")] {
                        @let rota = format!("/cenarios/exemplo?dieta={pendente}&acao={acao}");
                        button.button.secondary type="submit" formnovalidate data-sem-validacao
                            formaction=(&rota) hx-post=(&rota) hx-target="#formulario"
                            hx-swap="outerHTML settle:0" { (rotulo) }
                    }
                }
            }
        }
    }
}

//! Inspeção opcional das diferenças completas do conhecimento congelado.
use crate::aplicacao::comparacao::diferencas_conhecimento;
use crate::persistencia::historico::Execucao;
use maud::{Markup, Render, html};

pub struct Conhecimento<'a>(pub &'a Execucao, pub &'a Execucao);
impl Render for Conhecimento<'_> {
    fn render(&self) -> Markup {
        let diferencas = diferencas_conhecimento(self.0, self.1);
        html! {
            @if !diferencas.is_empty() {
                h2 { "Diferenças no conhecimento preservado" }
                @for d in diferencas {
                    details {
                        summary { (&d.rotulo) }
                        table {
                            thead { tr { th { "Referência" } th { "Alternativa" } } }
                            tbody { tr { @for v in [&d.referencia, &d.alternativa] {
                                td { pre.code { (serde_json::to_string_pretty(v)
                                    .unwrap_or_else(|_| "Indisponível".into())) } }
                            } } }
                        }
                    }
                }
            }
        }
    }
}

//! Premissas completas do ingrediente, compartilhadas pelos dois lados da comparação.
use crate::dominio::Item;
use crate::dominio::unidades::preco_f64;
use crate::web::formatacao::{numero, quantidade_pratica};
use maud::{Markup, Render, html};

pub struct PremissasItem<'a>(pub Option<&'a Item>);
impl Render for PremissasItem<'_> {
    fn render(&self) -> Markup {
        html! {
            @if let Some(i) = self.0 {
                dl.details-grid {
                    dt { "MS" } dd { (numero(i.alimento.ms * 100.0, 4)) "% MN" }
                    dt { "Preço normalizado" }
                    dd { (preco_f64(&i.preco).map(|v| numero(v, 4))
                        .unwrap_or("Desconhecido".into())) " R$/kg MN" }
                    dt { "Perda" } dd { (numero(i.perda * 100.0, 2)) "%" }
                    dt { "Estoque bruto" } dd {
                        (i.estoque.map(|v| format!("{} kg MN", numero(v, 4)))
                            .unwrap_or_else(|| if i.sem_limite { "Sem limite (hipótese)".into() }
                                else { "Desconhecido".into() }))
                    }
                    dt { "Inclusão mínima" }
                    dd { (numero(i.minimo_ms, 4)) " kg MS/animal/dia" }
                    dt { "Inclusão máxima" }
                    dd { (i.maximo_ms.map(|v| format!("{} kg MS/animal/dia", numero(v, 4)))
                        .unwrap_or("Sem restrição adicional".into())) }
                    dt { "Teto de suplemento" }
                    dd { (i.teto_suplemento().map(|v| format!("{} kg MS/animal/dia", numero(v, 4)))
                        .unwrap_or("Não informado / não aplicável".into())) }
                    dt { "Resolução de oferta" }
                    dd { (quantidade_pratica(i.resolucao_mn)) " kg MN/animal/dia" }
                }
            } @else { "Ausente" }
        }
    }
}

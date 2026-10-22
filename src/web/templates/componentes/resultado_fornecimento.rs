//! Quantidades aproximadas e pesagens revalidadas com unidades e escopos explícitos.
use crate::dominio::{Cenario, Parecer};
use crate::web::formatacao::{estado, numero, quantidade_pratica};
use maud::{Markup, Render, html};

pub struct Fornecimento<'a> {
    pub cenario: &'a Cenario,
    pub parecer: &'a Parecer,
}
impl Render for Fornecimento<'_> {
    fn render(&self) -> Markup {
        let p = self.parecer;
        let c = self.cenario;
        let pendente = p.resolucao_pendente(c);
        html! {
            section.card.table-wrap #fornecimento {
                h2 { "Quantidades de fornecimento" }
                p { (estado(p.estado_fornecimento(c))) }
                @if c.formular {
                    p.muted {
                        "Consumo e oferta contínuos são aproximações de apresentação. "
                        "Não copie esses valores como pesagem validada. "
                        "A resolução zero exige definir a precisão de pesagem e recalcular."
                    }
                }
                p.muted.no-print { "Arraste a tabela para consultar todas as colunas." }
                table {
                    thead { tr {
                        th { "Ingrediente" }
                        th { "Oferta prática kg MN / animal / dia" }
                        th { "Consumo contínuo kg MS / animal / dia ≈" }
                        th { "Oferta contínua kg MN / animal / dia ≈" }
                        th { "Oferta contínua kg MN / lote / dia ≈" }
                        th { "Bruto contínuo kg MN / lote / horizonte ≈" }
                    } }
                    tbody { @for f in &p.fornecimento {
                        tr class=(if f.consumo_ms == 0.0 { "muted" } else { "" }) {
                            td { (&f.ingrediente) }
                            td {
                                @if !pendente {
                                    (quantidade_pratica(f.pratico_mn))
                                } @else { "Não definida" }
                            }
                            @for v in [f.consumo_ms, f.oferta_mn, f.oferta_lote,
                                f.necessidade_bruta] { td { (numero(v, 4)) } }
                        }
                    } }
                }
                p.muted {
                    "MN: matéria natural; MS: matéria seca. A quantidade bruta inclui perdas "
                    "de estoque. Consumo estimado e oferta são grandezas diferentes."
                }
            }
        }
    }
}

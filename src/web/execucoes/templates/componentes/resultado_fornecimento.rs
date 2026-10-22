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
                div.concept-label.section-concept {
                    h2 { "Quantidades de fornecimento" }
                    (super::conceitos_resultado::FORNECIMENTO)
                }
                p { (estado(p.estado_fornecimento(c))) }
                @if pendente {
                    p.muted {
                        "Consumo e oferta sem resolução são aproximações de apresentação. "
                        "Não copie esses valores como pesagem validada. "
                        "A resolução zero exige definir a precisão de pesagem e recalcular."
                    }
                }
                p.muted.no-print { "Arraste a tabela para consultar todas as colunas." }
                table {
                    thead { tr {
                        th { "Ingrediente" }
                        th { "Oferta prática kg MN / animal / dia" }
                        th { (if c.formular { "Consumo contínuo" }
                            else { "Consumo informado" })
                            " kg MS / animal / dia ≈" }
                        th { "Oferta calculada kg MN / animal / dia ≈" }
                        th { "Oferta calculada kg MN / lote / dia ≈" }
                        th { "Bruto calculado kg MN / lote / horizonte ≈" }
                    } }
                    tbody { @for f in &p.fornecimento {
                        tr class=(if f.consumo_ms == 0.0 { "muted" } else { "" }) {
                            td { (&f.ingrediente) }
                            td data-rotulo="Oferta prática kg MN/animal/dia" {
                                @if c.itens.iter().any(|i|
                                    i.alimento.composicao_id == f.composicao_id
                                    && i.resolucao_mn > 0.0) || f.consumo_ms == 0.0 {
                                    (quantidade_pratica(f.pratico_mn))
                                } @else { "Não definida" }
                            }
                            @for (rotulo, v) in [
                                (if c.formular { "Consumo contínuo kg MS/animal/dia ≈" }
                                    else { "Consumo informado kg MS/animal/dia ≈" }, f.consumo_ms),
                                ("Oferta calculada kg MN/animal/dia ≈", f.oferta_mn),
                                ("Oferta calculada kg MN/lote/dia ≈", f.oferta_lote),
                                ("Bruto calculado kg MN/lote/horizonte ≈", f.necessidade_bruta),
                            ] { td data-rotulo=(rotulo) { (numero(v, 4)) } }
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

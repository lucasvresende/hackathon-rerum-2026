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
                    dt { "Revisão da composição" } dd { (&i.alimento.composicao_id) }
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

/// Consumo e oferta vêm do parecer; a entrada pode conter apenas um chute de formulação.
pub struct QuantidadesItem<'a>(
    pub &'a crate::persistencia::historico::Execucao,
    pub &'a Item,
);
impl Render for QuantidadesItem<'_> {
    fn render(&self) -> Markup {
        use crate::aplicacao::comparacao::fornecimento;
        let e = self.0;
        html! {
            h3 { "Quantidades e custo / animal / dia" }
            @match fornecimento(e, self.1, false) {
                Ok(f) => {
                    dl.details-grid {
                        dt { "Consumo calculado (kg MS)" } dd { (numero(f.consumo_ms, 4)) }
                        dt { "Oferta calculada (kg MN)" } dd { (numero(f.oferta_mn, 4)) }
                        dt { "Custo do ingrediente" } dd {
                            @if e.parecer.custo.is_some() {
                                (crate::dominio::unidades::dinheiro(f.custo))
                            } @else { "Indisponível" }
                        }
                    }
                },
                Err(motivo) => { p { (motivo) } }
            }
            h3 { "Quantidades práticas / animal / dia" }
            p { (crate::web::formatacao::estado(e.parecer.estado_fornecimento(&e.cenario))) }
            @match fornecimento(e, self.1, true) {
                Ok(f) => {
                    dl.details-grid {
                        dt { "Consumo prático (kg MS)" } dd { (numero(f.consumo_ms, 4)) }
                        dt { "Oferta prática (kg MN)" } dd { (quantidade_pratica(f.oferta_mn)) }
                        dt { "Custo prático do ingrediente" } dd {
                            @if e.parecer.pratico.as_deref().is_some_and(|p| p.custo.is_some()) {
                                (crate::dominio::unidades::dinheiro(f.custo))
                            } @else { "Indisponível" }
                        }
                    }
                },
                Err(motivo) => { p { (motivo) } }
            }
        }
    }
}

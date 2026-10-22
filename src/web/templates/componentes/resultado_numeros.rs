//! Valores da dieta contínua; a revalidação prática tem apresentação própria.
use super::conceitos_resultado as conceitos;
use crate::dominio::calculos::NUTRIENTES;
use crate::dominio::unidades::{dinheiro, preco_f64};
use crate::dominio::{Cenario, Parecer};
use crate::web::formatacao::{balanco, diferenca, medida, numero};
use maud::{Markup, Render, html};

pub struct Resumo<'a> {
    pub cenario: &'a Cenario,
    pub parecer: &'a Parecer,
}
impl Render for Resumo<'_> {
    fn render(&self) -> Markup {
        let p = self.parecer;
        let c = self.cenario;
        html! {
        section.stats {
            @for (nome, valor, dica) in [
            ("Custo / animal / dia", p.custo.map(dinheiro).unwrap_or("—".into()),
            "Oferta, perdas e sobras incluídas"),
            ("Consumo da dieta", format!("{} kg MS", numero(p.consumo_ms, 3)),
            "Por animal e por dia"),
            ("Custo do lote / dia", p.custo.map(|v| dinheiro(v * c.lote.animais
            as f64))
            .unwrap_or("—".into()),
            "Custo alimentar; outros custos não incluídos"),
            ("Horizonte", format!("{} dias", c.horizonte),
            "Estoque não é baixado pelo cálculo"),
            ] { article.stat { small { (nome) } strong { (valor) } span { (dica) } } }
        }

        }
    }
}

pub struct Balanco<'a>(pub &'a Parecer);
impl Render for Balanco<'_> {
    fn render(&self) -> Markup {
        let p = self.0;
        html! {
        section.card.table-wrap {
            p.muted.no-print { "Arraste a tabela para consultar todas as colunas." }
            div.section-title {
                div.concept-label { h2 { "Balanço nutricional" } (conceitos::BALANCO) }
                span.pill {
                    "BASE: MATÉRIA SECA" } }
            table {
                thead { tr { th { "Nutriente" } th { "Oferta (kg/dia)" } th {
                            "Requisito" }
                        th { "Balanço" } th { "Déficit" } th { "Excesso" } th {
                            "Cobertura" } } }
                tbody {
                    @for (i, nome) in NUTRIENTES.iter().enumerate().take(4) {
                        @let requisito = p.requisitos.map(|r| r[i]);
                        @let valor = p.totais[i];
                        tr {
                            td { strong { (nome) } }
                            td { (valor.map(|v|
                                numero(v, 5)).unwrap_or("Desconhecido".into())) }
                            td { (requisito.map(|v|
                                numero(v, 5)).unwrap_or("—".into())) }
                            td { (valor.zip(requisito).map(|(v, r)| balanco(v, r))
                                .unwrap_or("—".into())) }
                            td { (valor.zip(requisito)
                                .map(|(v,r)|medida((-diferenca(v,r)).max(0.0), 5))
                                .unwrap_or("—".into())) }
                            td { (valor.zip(requisito)
                                .map(|(v,r)|medida(diferenca(v,r).max(0.0), 5))
                                .unwrap_or("—".into())) }
                            td { (valor.zip(requisito).filter(|(_, r)| *r > 0.0)
                                .map(|(v, r)| format!("{}%", numero(100.0*v/r, 1)))
                                .unwrap_or("—".into())) }
                        }
                    }
                }
            }
            p.muted { "Balanço negativo = déficit; positivo = excesso sobre o mínimo. \
                    Excesso não é diagnóstico de intoxicação." }
        }

        }
    }
}

pub struct Economia<'a> {
    pub cenario: &'a Cenario,
    pub parecer: &'a Parecer,
}
impl Render for Economia<'_> {
    fn render(&self) -> Markup {
        let p = self.parecer;
        let c = self.cenario;
        html! {
        section.card {
            h2 { "Premissas e economia" }
            dl.details-grid {
                dt { "Consumo de referência" }
                dd { (p.consumo_referencia.map(|v| format!("{} kg MS/animal/dia", numero(v, 4)))
                    .unwrap_or("Fora da Tabela B; informar consumo".into())) }
                (super::consumo_execucao::ConsumoExecucao { cenario: c, parecer: p })
                dt { "Sobras planejadas" } dd { (format!("{}% · {}", numero(c.sobras*100.0, 2),
                    c.origem_sobras)) }
                dt { "Base da comparação" } dd { (&c.base_comparacao) }
                dt { "Custo no horizonte" } dd { (p.custo.map(|v|
                    dinheiro(v*c.lote.animais as f64
                    * c.horizonte as f64)).unwrap_or("—".into())) }
                @if let Some(custo) = p.custo {
                    @if c.lote.producao > 0.0 {
                        dt { "Custo por kg de leite" } dd { (dinheiro(custo /
                            c.lote.producao)) }
                    }
                    @if let Some(preco) = c.preco_leite.as_deref().and_then(preco_f64) {
                        dt { "Margem sobre alimentação / animal / dia" }
                        dd { (dinheiro(preco*c.lote.producao-custo))
                            " · não é lucro total" }
                    }
                }
            }
        }

        }
    }
}

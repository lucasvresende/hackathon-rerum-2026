//! Balanço lado a lado exclusivamente com valores preservados nas duas execuções.
use crate::aplicacao::comparacao::tem_resultado;
use crate::persistencia::historico::Execucao;
use crate::web::formatacao::{balanco, estado, numero};
use maud::{Markup, Render, html};

pub struct Balanco<'a>(pub &'a Execucao, pub &'a Execucao);
impl Render for Balanco<'_> {
    fn render(&self) -> Markup {
        let execucoes = [self.0, self.1];
        let dimensoes: std::collections::BTreeSet<_> = execucoes
            .iter()
            .flat_map(|e| e.parecer.verificacoes.iter().map(|v| &v.dimensao))
            .collect();
        html! {
            h2 { "Resultados por dimensão" }
            table {
                thead { tr { th { "Dimensão" } th { "Referência" } th { "Alternativa" } } }
                tbody { @for d in dimensoes {
                    tr { th { (d) } @for e in execucoes {
                        td { (estado(e.parecer.dimensao(d))) }
                    } }
                } }
            }
            h2 { "Balanço nutricional" }
            p { "Valores em kg/animal/dia da dieta informada ou solução contínua." }
            table {
                thead { tr { th { "Nutriente" } th { "Referência" } th { "Alternativa" } } }
                tbody {
                    @for (i, n) in crate::dominio::calculos::NUTRIENTES.iter().enumerate() {
                        tr { th { (n) } @for e in execucoes {
                            td {
                                @if tem_resultado(e) {
                                    dl.details-grid {
                                        dt { "Consumo" } dd { (e.parecer.totais[i]
                                            .map(|v| numero(v, 5))
                                            .unwrap_or("Desconhecido".into())) }
                                        @if i < 4 {
                                            @let r = e.parecer.requisitos.map(|r| r[i]);
                                            dt { "Requisito" } dd { (r.map(|v| numero(v, 5))
                                                .unwrap_or("Indisponível".into())) }
                                            dt { "Balanço" } dd { (e.parecer.totais[i].zip(r)
                                                .map(|(v,r)| balanco(v,r))
                                                .unwrap_or("Indisponível".into())) }
                                        }
                                    }
                                } @else { "Indisponível: busca sem candidata" }
                            }
                        } }
                    }
                }
            }
        }
    }
}

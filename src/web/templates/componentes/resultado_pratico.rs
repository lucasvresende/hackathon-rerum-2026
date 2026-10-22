//! Diagnóstico independente do fornecimento arredondado.
use crate::dominio::calculos::NUTRIENTES;
use crate::dominio::unidades::dinheiro;
use crate::dominio::{Estado, Parecer};
use crate::web::formatacao::{balanco, numero, sinal};
use maud::{Markup, Render, html};

pub struct ResultadoPratico<'a>(pub &'a Parecer, pub &'a crate::dominio::Cenario);
impl Render for ResultadoPratico<'_> {
    fn render(&self) -> Markup {
        let p = self.0;
        let pendente = p.resolucao_pendente(self.1);
        html! {
            section.card #resultado-pratico {
                h2 { "Revalidação das quantidades práticas" }
                p class=(if p.pratico_valido && !pendente { "alert" } else { "alert error" }) {
                    @if p.pratico_valido && !pendente { "Fornecimento prático validado no perfil." }
                    @else if pendente && p.pratico_valido {
                        "Resolução de pesagem incompleta. Não usar como recomendação."
                    } @else { "Fornecimento prático incompatível. Não usar como recomendação." }
                }
                dl.details-grid {
                    dt { "Consumo prático por animal/dia" }
                    dd { (numero(p.consumo_ms, 4)) " kg MS" }
                    dt { "Custo prático por animal/dia" }
                    dd { (p.custo.map(dinheiro).unwrap_or("Não calculável".into())) }
                }
                div.table-wrap { table {
                    caption { "Balanço das quantidades práticas por animal/dia" }
                    thead { tr { th { "Nutriente" } th { "Oferta (kg)" }
                        th { "Requisito (kg)" } th { "Balanço (kg)" } } }
                    tbody { @for (i, nome) in NUTRIENTES.iter().enumerate().take(4) {
                        @let requisito = p.requisitos.map(|r| r[i]);
                        tr { td { (nome) }
                            td { (p.totais[i].map(|v| numero(v, 5))
                                .unwrap_or("Desconhecido".into())) }
                            td { (requisito.map(|v| numero(v, 5)).unwrap_or("—".into())) }
                            td { (p.totais[i].zip(requisito).map(|(v,r)| balanco(v, r))
                                .unwrap_or("—".into())) }
                        }
                    } }
                } }
                h3 { "Restrições e pendências práticas" }
                ul { @for v in p.verificacoes.iter().filter(|v| v.estado != Estado::Atende
                    && v.estado != Estado::NaoAplicavel) {
                    li {
                        strong { (&v.dimensao) " · " (v.estado.rotulo()) }
                        span.muted { " · referência " (&v.regra) }
                        " · " (&v.explicacao)
                        @if let (Some(valor), Some(limite)) = (v.valor, v.limite) {
                            " Calculado: " (numero(valor, 8)) " · limite: "
                            (numero(limite, 8)) " · diferença: " (sinal(valor-limite, 8))
                            " " (&v.unidade)
                        }
                        " · Fonte: " (&v.fonte)
                    }
                } }
                p.muted { "O ótimo contínuo e seu custo permanecem separados deste diagnóstico." }
            }
        }
    }
}

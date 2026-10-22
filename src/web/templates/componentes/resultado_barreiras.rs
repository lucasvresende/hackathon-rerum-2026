//! Valores, limites e linha efetiva das barreiras, sem exigir abertura da auditoria.
use crate::dominio::{Conhecimento, Parecer};
use crate::web::formatacao::{medida, numero, sinal};
use maud::{Markup, Render, html};

pub struct Barreiras<'a>(pub &'a Parecer, pub &'a Conhecimento, pub bool);
impl Render for Barreiras<'_> {
    fn render(&self) -> Markup {
        let p = self.0;
        let fdnf = p
            .fdn_forragem
            .filter(|_| p.consumo_ms > 0.0)
            .map(|v| 100.0 * v / p.consumo_ms);
        let linha = fdnf.and_then(|v| {
            self.1
                .coeficientes
                .iter()
                .filter(|r| {
                    r.tabela == "fibra"
                        && (r.eixo <= v || crate::dominio::unidades::proximos(r.eixo, v))
                })
                .max_by(|a, b| a.eixo.total_cmp(&b.eixo))
        });
        html! {
            section.card.table-wrap #barreiras-ruminais {
                h2 { "Fibra, amido e gordura"
                    @if self.2 { " da solução contínua" } @else { " da dieta informada" } }
                p {
                    @if let Some(linha) = linha {
                        "Linha efetiva: FDN de forragem ≥ " (numero(linha.eixo, 0)) "% da MS; "
                            "governa os limites de FDN total e amido."
                    } @else { "Sem linha efetiva de fibra admissível." }
                }
                p.muted { "FDN: fibra em detergente neutro; EE: extrato etéreo. "
                    "Limites técnicos e políticas do perfil; não são limites legais brasileiros." }
                p.muted.no-print { "Arraste a tabela para consultar todas as colunas." }
                table {
                    thead { tr { th { "Barreira" } th { "Calculado % MS" }
                        th { "Limite % MS" } th { "Diferença (p.p.)" } th { "Resultado" } } }
                    tbody {
                        @for v in p.verificacoes.iter()
                            .filter(|v| v.dimensao == "Barreiras ruminais") {
                            tr {
                                th { @if v.regra == "RUM-01" {
                                    "FDN de forragem (piso global do perfil)" }
                                    @else { (v.explicacao.split(':').next().unwrap_or("Barreira"))
                                        @if v.explicacao.starts_with("FDN") { " (mínimo)" }
                                        @else { " (máximo)" } }
                                }
                                td { (v.valor.map(|x| medida(x, 4)).unwrap_or("—".into())) }
                                td { (v.limite.map(|x| numero(x, 4)).unwrap_or("—".into())) }
                                td { (v.valor.zip(v.limite).map(|(x,l)| sinal(x-l, 4))
                                    .unwrap_or("—".into())) }
                                td { (v.estado.rotulo()) }
                            }
                        }
                    }
                }
            }
        }
    }
}

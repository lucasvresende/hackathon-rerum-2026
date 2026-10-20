use maud::{Markup, Render, html};

use super::estrutura::cabecalho;
use crate::dominio::unidades::dinheiro;
use crate::dominio::{Alimento, Conhecimento, Lote};
use crate::persistencia::historico::Execucao;

pub struct Inicio<'a> {
    pub lotes: &'a [Lote],
    pub alimentos: &'a [Alimento],
    pub execucoes: &'a [Execucao],
    pub conhecimento: &'a Conhecimento,
}

impl Render for Inicio<'_> {
    fn render(&self) -> Markup {
        let animais: u32 = self.lotes.iter().map(|l| l.animais).sum();
        html! {
            (cabecalho("DO CAMPO À DECISÃO", "Uma boa dieta começa com clareza.",
                "Conheça os custos, confira o equilíbrio e entenda cada resultado."))
            section.hero {
                div.hero-copy {
                    span.pill.light { "PLANEJAMENTO NUTRICIONAL" }
                    h2 { "Mais equilíbrio no cocho." br; "Mais confiança nas escolhas." }
                    p { "Verifique a dieta do seu lote ou encontre uma combinação de menor custo, \
                        com cada cálculo ao seu alcance." }
                    div.actions {
                        a.button.white href="/cenarios/novo" { "+ Criar cenário" }
                        a.text-link href="/conhecimento" { "Conhecer o perfil ↗" }
                    }
                }
                div.hero-art aria-hidden="true" {
                    div.orbit.one {} div.orbit.two {} div.orbit.three {}
                    div.grain.g1 {} div.grain.g2 {} div.grain.g3 {}
                    div.art-caption { "DADOS + CONHECIMENTO" strong { "Uma dieta.\nToda a explicação." } }
                }
            }
            section.stats aria-label="Resumo da aplicação" {
                @for (numero, nome, dica) in [
                    (self.lotes.len().to_string(), "Lotes cadastrados", "Grupos com revisões preservadas"),
                    (animais.to_string(), "Animais no planejamento", "Distribuídos nos seus lotes"),
                    (self.alimentos.len().to_string(), "Composições disponíveis", "Fontes e versões identificadas"),
                    (self.execucoes.len().to_string(), "Execuções recentes", "Entradas e pareceres preservados"),
                ] {
                    article.stat { small { (nome) } strong { (numero) } span { (dica) } }
                }
            }
            div.columns {
                section.card {
                    div.section-title { h2 { "Por onde começar?" } span.muted { "SEU PRÓXIMO PASSO" } }
                    @for (n, titulo, texto, url) in [
                        ("01", "Conheça seu lote", "Registre os animais e o perfil de produção.", "/lotes"),
                        ("02", "Monte um cenário", "Selecione ingredientes, quantidades e premissas.",
                            "/cenarios/novo"),
                        ("03", "Entenda o resultado", "Consulte balanços, custos e o caminho das regras.",
                            "/historico"),
                    ] {
                        a.step-card href=(url) { span.step-number { (n) }
                            div { h3 { (titulo) } p { (texto) } } span.arrow { "↗" }
                        }
                    }
                }
                section.card.knowledge-card {
                    span.pill { "CONHECIMENTO VERSIONADO" }
                    h2 { "Cada conclusão tem uma fonte." }
                    p { "NDT, proteína, cálcio, fósforo e barreiras ruminais, com hipóteses explícitas." }
                    dl {
                        dt { "Perfil ativo" } dd.code { (&self.conhecimento.perfil_id) }
                        dt { "Cobertura" } dd { "Vacas adultas em lactação e confinamento" }
                        dt { "Importante" } dd { "Atendimento ao núcleo não é adequação integral." }
                    }
                    a.text-link href="/conhecimento" { "Ver fontes e limitações ↗" }
                }
            }
            section.card {
                div.section-title { h2 { "Últimos cenários" } a href="/historico" { "Ver histórico →" } }
                @if self.execucoes.is_empty() {
                    div.empty { span { "◷" } h3 { "Seu histórico começa no primeiro cenário" }
                        p { "Os resultados calculados aparecerão aqui, com suas fontes e premissas." }
                        a.button href="/cenarios/novo?dieta=B" { "Explorar dieta B sintética" }
                    }
                } @else {
                    table {
                        thead { tr { th { "Cenário" } th { "Lote" } th { "Custo / animal / dia" }
                            th { "Núcleo químico" } th {} } }
                        tbody {
                            @for e in self.execucoes.iter().take(5) {
                                tr { td { strong { (&e.cenario.nome) } } td { (&e.cenario.lote.nome) }
                                    td { (e.parecer.custo.map(dinheiro).unwrap_or("—".into())) }
                                    td { span.pill { (e.parecer.dimensao("Núcleo químico")) } }
                                    td { a href=(format!("/execucoes/{}", e.id)) { "Abrir ↗" } }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

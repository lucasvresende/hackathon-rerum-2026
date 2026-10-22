//! Componente de apresentação para inicio, mantendo a renderização desta responsabilidade isolada.
use maud::{Markup, Render, html};

use super::{cabecalho::Cabecalho, conceitos_inicio as conceitos};
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
            (Cabecalho { titulo: "Visão geral", texto: "", ajuda: "inicio" })
            div.actions {
                a.button href="/cenarios/novo" target="_blank"
                rel="noopener" title="Abre em nova guia" { "Verificar dieta" }
                a.button.secondary href="/cenarios/novo?modo=formular"
                target="_blank" rel="noopener" title="Abre em nova guia" {
                    "Formular dieta" }
            }
            section.card {
                h2 { "Primeira simulação" }
                p { "Lotes e alimentos de exemplo já estão cadastrados. "
                    "Execute o cenário completo e altere valores para comparar resultados." }
                a.button.secondary href="/cenarios/novo?exemplo=inicial" target="_blank"
                rel="noopener" { "Abrir cenário de exemplo" }
            }
            section.stats.overview-stats aria-label="Resumo da aplicação" {
                @for (numero, dica) in [
                (self.lotes.len().to_string(), &conceitos::LOTES),
                (animais.to_string(), &conceitos::ANIMAIS),
                (self.alimentos.len().to_string(), &conceitos::COMPOSICOES),
                ] {
                    article.stat {
                        div.concept-label { small { (dica.conceito) } (dica) }
                        strong { (numero) }
                    }
                }
            }
            section.card.recent-scenarios {
                div.section-title {
                    div.concept-label { h2 { "Últimos cenários" } (conceitos::CENARIOS) }
                    a href="/historico" { "Ver histórico →" }
                }
                @if self.execucoes.is_empty() {
                    p.empty { "Nenhum cenário calculado." }
                } @else {
                    div.table-wrap { table {
                        thead { tr { th { "Cenário" } th { "Lote" } th {
                                    "Custo / animal / dia" }
                                th { "Núcleo químico" } th {} } }
                        tbody {
                            @for e in self.execucoes.iter().take(5) {
                                tr { td { strong { (&e.cenario.nome) } } td {
                                        (&e.cenario.lote.nome) }
                                    td {
                                        (e.parecer.custo_resultado().map(dinheiro)
                                            .unwrap_or("—".into())) }
                                    td { span.pill {
                                            (e.parecer.dimensao("Núcleo químico")) } }
                                    td { a href=(format!("/execucoes/{}", e.id)) {
                                            "Abrir ↗" } }
                                }
                            }
                        }
                    } }
                }
            }
            section.card {
                div.section-title {
                    h2 { "Cadastros" }
                }
                div.actions {
                    a.button.secondary href="/lotes" { "Lotes de animais" }
                    a.button.secondary href="/alimentos" { "Alimentos" }
                }
                div.concept-label.profile-title { h2 { "Perfil ativo" } (conceitos::PERFIL) }
                dl.details-grid {
                    dt { "Identificador" } dd.code { (&self.conhecimento.perfil_id) }
                    dt { "População" } dd { "Vacas adultas em lactação e confinamento" }
                    dt { "Limitação" } dd {
                        "Atendimento ao núcleo não é adequação integral." }
                }
                a.text-link href="/conhecimento" { "Consultar perfil nutricional" }
            }
        }
    }
}

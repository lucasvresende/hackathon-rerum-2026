use maud::{Markup, Render, html};

use super::cabecalho::Cabecalho;
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
            section.stats aria-label="Resumo da aplicação" {
                @for (numero, nome) in [
                (self.lotes.len().to_string(), "Lotes cadastrados"),
                (animais.to_string(), "Animais cadastrados"),
                (self.alimentos.len().to_string(), "Composições disponíveis"),
                (self.execucoes.len().to_string(), "Execuções recentes"),
                ] {
                    article.stat { small { (nome) } strong { (numero) } }
                }
            }
            section.card.table-wrap {
                div.section-title { h2 { "Últimos cenários" } a href="/historico" {
                        "Ver histórico →" } }
                @if self.execucoes.is_empty() {
                    p.empty { "Nenhum cenário calculado." }
                } @else {
                    table {
                        thead { tr { th { "Cenário" } th { "Lote" } th {
                                    "Custo / animal / dia" }
                                th { "Núcleo químico" } th {} } }
                        tbody {
                            @for e in self.execucoes.iter().take(5) {
                                tr { td { strong { (&e.cenario.nome) } } td {
                                        (&e.cenario.lote.nome) }
                                    td {
                                        (e.parecer.custo.map(dinheiro).unwrap_or("—".into())) }
                                    td { span.pill {
                                            (e.parecer.dimensao("Núcleo químico")) } }
                                    td { a href=(format!("/execucoes/{}", e.id)) {
                                            "Abrir ↗" } }
                                }
                            }
                        }
                    }
                }
            }
            section.card {
                div.section-title {
                    h2 { "Cadastros" }
                }
                div.actions {
                    a.button.secondary href="/lotes" { "Lotes" }
                    a.button.secondary href="/alimentos" { "Alimentos" }
                }
                h2 { "Perfil ativo" }
                dl.details-grid {
                    dt { "Identificador" } dd.code { (&self.conhecimento.perfil_id) }
                    dt { "População" } dd { "Vacas adultas em lactação e confinamento" }
                    dt { "Limitação" } dd {
                        "Atendimento ao núcleo não é adequação integral." }
                }
                a.text-link href="/conhecimento" { "Consultar perfil e fontes" }
            }
        }
    }
}

//! Página de apresentação para historico, mantendo a renderização desta responsabilidade isolada.
use super::super::componentes::cabecalho::Cabecalho;
use super::super::layouts::Principal;
use crate::dominio::unidades::dinheiro;
use maud::{Markup, Render, html};

pub struct Historico<'a> {
    pub execucoes: &'a [crate::persistencia::historico::Execucao],
    pub pagina: i64,
    pub busca: &'a str,
    pub data: &'a str,
}
impl Render for Historico<'_> {
    fn render(&self) -> Markup {
        let execucoes = self.execucoes;
        let pagina = self.pagina;
        let busca = self.busca;
        let data = self.data;
        Principal {
            titulo: "Histórico e análise",
            ativo: "/historico",
            conteudo: html! {
                (Cabecalho { titulo: "Histórico e análise", texto: "", ajuda: "historico" })
                form.card method="get" action="/historico" {
                    div.form-grid {
                        div.field { label for="filtro-lote" { "Nome do lote" }
                            input #filtro-lote name="lote" value=(&busca);
                        }
                        div.field { label for="filtro-data" { "Data da execução (São Paulo)" }
                            input #filtro-data name="data" type="date" value=(&data);
                        }
                    }
                    button.button.secondary { "Filtrar histórico" }
                }
                div.actions {
                    @if pagina>1 { a.button.secondary
                        href=(format!("/historico?pagina={}",pagina-1)) {
                            "← Página anterior" } }
                    span.muted { "Página " (pagina) " · até 100 resultados por página" }
                    @if execucoes.len()==100 { a.button.secondary
                        href=(format!("/historico?pagina={}",pagina+1)) { "Próxima página →" } }
                }
                form.card method="get" action="/comparar" {
                    h2 { "Comparar duas execuções" }
                    div.form-grid {
                        @for (nome, rotulo) in [("a", "Cenário de referência"), ("b",
                        "Cenário alternativo")] {
                            div.field { label for=(nome) { (rotulo) }
                                select id=(nome) name=(nome) required {
                                    @for execucao in execucoes {
                                        option value=(&execucao.id) { (&execucao.cenario.nome)
                                            " · " (&execucao.id[..8]) }
                                    }
                                }
                            }
                        }
                    }
                    button.button.secondary { "Comparar premissas e custos" }
                }
                section.card.table-wrap {
                    table { thead { tr { th { "Cenário" } th { "Lote" } th { "Estado" }
                                th { "Custo / animal / dia" } th {} } }
                        tbody { @for x in execucoes {
                                tr { td { (&x.cenario.nome) } td { (&x.cenario.lote.nome) }
                                    td { span.pill { (&x.parecer.condicao) } }
                                    td { (x.parecer.custo.map(dinheiro).unwrap_or("—".into())) }
                                    td { a href=(format!("/execucoes/{}", x.id)) {
                                            "Ver parecer ↗" } }
                                }
                            } }
                    }
                    @if execucoes.is_empty() { p.empty { "Nenhum cenário calculado ainda." } }
                }
            },
        }
        .render()
    }
}

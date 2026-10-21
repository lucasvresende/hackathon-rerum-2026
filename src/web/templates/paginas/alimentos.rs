use super::super::componentes::cabecalho::Cabecalho;
use super::super::componentes::expansivel::Expansivel;
use super::super::layouts::Principal;
use maud::{Markup, Render, html};

pub struct Alimentos<'a> {
    pub alimentos: &'a [crate::dominio::Alimento],
    pub busca: &'a str,
}
impl Render for Alimentos<'_> {
    fn render(&self) -> Markup {
        let alimentos = self.alimentos;
        let busca = self.busca;
        Principal {
            titulo: "Alimentos",
            ativo: "/alimentos",
            conteudo: html! {
                (Cabecalho { titulo: "Catálogo de alimentos", texto: "", ajuda: "alimentos" })
                div.actions { a.button href="/alimentos/novo" target="_blank"
                    rel="noopener" title="Abre em nova guia" {
                        "+ Cadastrar composição" }
                    form.search method="get" action="/alimentos" {
                        label.sr-only for="busca" { "Pesquisar alimento" }
                        input #busca name="busca" value=(&busca)
                        placeholder="Pesquisar alimento…";
                        button.button.secondary { "Pesquisar" }
                    }
                }
                section.food-grid {
                    @for a in alimentos {
                        article.card.food-card {
                            span.pill { (&a.natureza) }
                            h2 { (&a.nome) }
                            p.muted { (format!("{:?}", a.classe)) " · fonte " (&a.fonte_id) }
                            dl { dt { "Matéria seca" } dd { (format!("{:.1}% MN", a.ms *
                                    100.0)) }
                                @for (i, nome) in ["NDT", "PB", "Ca", "P", "FDN", "Amido", "EE"]
                                .iter().enumerate() {
                                    dt { (nome) } dd { (a.nutrientes[i].map(|v|
                                        format!("{:.2}% MS",
                                        v*100.0))
                                        .unwrap_or("Desconhecido".into())) }
                                }
                            }
                            (Expansivel { id:
                                &format!("alimento-{}",
                                a.composicao_id), aberto: false,
                                titulo: html! { "Procedência e condições" },
                                conteudo: html! {
                                    p { (&a.origem) " · " (&a.metodo) " · " (&a.data) }
                                    p { (&a.evidencia) }
                                    p.code { "Versão " (&a.composicao_id) }
                                },
                            })
                            a.text-link href=(format!("/alimentos/{}/revisar", a.composicao_id))
                            target="_blank" rel="noopener" title="Abre em nova guia" {
                                "Criar revisão ou nova análise ↗"
                            }
                        }
                    }
                }
            },
        }
        .render()
    }
}

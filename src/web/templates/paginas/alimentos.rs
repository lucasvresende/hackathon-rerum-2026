use super::super::componentes::expansivel::Expansivel;
use super::super::componentes::{cabecalho::Cabecalho, conceitos_alimentos as conceitos};
use super::super::layouts::Principal;
use maud::{Markup, Render, html};

pub struct Alimentos<'a> {
    pub token: &'a str,
    pub alimentos: &'a [crate::dominio::Alimento],
    pub busca: &'a str,
    pub classe: &'a str,
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
                div.actions {
                a.button.secondary href="/exemplos" { "Restaurar exemplos" }
                    div.concept-label {
                        a.button href="/alimentos/novo" target="_blank"
                        rel="noopener" title="Abre em nova guia" { "+ Cadastrar composição" }
                        (conceitos::COMPOSICAO)
                    }
                    form.search method="get" action="/alimentos" {
                        label.sr-only for="busca" { "Pesquisar alimento" }
                        input #busca name="busca" value=(&busca)
                        placeholder="Pesquisar alimento…";
                        label.sr-only for="classe-busca" { "Categoria" }
                        select #classe-busca name="classe" {
                            @for (valor, nome) in [("", "Todas as categorias"),
                                ("Forragem", "Forragens"), ("Concentrado", "Concentrados"),
                                ("Suplemento", "Suplementos")] {
                                option value=(valor) selected[self.classe == valor] { (nome) }
                            }
                        }
                        button.button.secondary { "Pesquisar" }
                    }
                }
                section.food-grid {
                    @for a in alimentos {
                        article.card.food-card {
                            div.concept-label {
                                span.pill { (&a.natureza) }
                                (conceitos::NATUREZA.com_id(
                                    &format!("natureza-{}", a.composicao_id)))
                            }
                            h2 { (&a.nome) }
                            @if let Some(p) = &a.procedencia {
                                @if p.modelo.id.starts_with("ex-") {
                                    span.pill {
                                        @if a.id == p.modelo.id { "Incluído no sistema" }
                                        @else { "Baseado em exemplo" }
                                    }
                                    p.muted { "Exemplo sintético · hipóteses assumidas" }
                                }
                            }
                            p.muted { (format!("{:?}", a.classe)) " · fonte " (&a.fonte_id) }
                            dl {
                                dt { div.concept-label { "Matéria seca"
                                    (conceitos::MS.com_id(&format!("ms-{}", a.composicao_id)))
                                } } dd { (format!("{:.1}% MN", a.ms *
                                    100.0)) }
                                @for (i, nome) in ["NDT", "PB", "Ca", "P", "FDN", "Amido", "EE"]
                                .iter().enumerate() {
                                    dt { div.concept-label { (nome)
                                        (conceitos::NUTRIENTES[i].com_id(
                                            &format!("nutriente-{i}-{}", a.composicao_id)))
                                    } } dd { (a.nutrientes[i].map(|v|
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
                                    @if let Some(p) = &a.procedencia {
                                        (super::super::componentes::modelos_preenchimento::
                                            Procedencia(p))
                                    }
                                    p.code { "Versão " (&a.composicao_id) }
                                },
                            }.com_dica(&conceitos::PROCEDENCIA,
                                &format!("procedencia-{}", a.composicao_id)))
                            div.cadastro-actions {
                                div.concept-label {
                                a.text-link href=(format!("/alimentos/{}/revisar", a.composicao_id))
                                target="_blank" rel="noopener" title="Abre em nova guia" {
                                    "Criar revisão ou nova análise ↗"
                                }
                                (conceitos::REVISAO.com_id(
                                    &format!("revisao-{}", a.composicao_id))) }
                                a href=(format!("/alimentos/{}/copiar", a.composicao_id))
                                target="_blank" rel="noopener" { "Criar cópia" }
                                (super::super::componentes::exemplos::Remover {
                                    tipo: "alimento", id: &a.id, token: self.token,
                                })
                            }
                        }
                    }
                }
            },
        }
        .render()
    }
}

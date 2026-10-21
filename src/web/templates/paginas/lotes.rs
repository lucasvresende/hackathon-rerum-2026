use super::super::componentes::cabecalho::Cabecalho;
use super::super::layouts::Principal;
use maud::{Markup, Render, html};

pub struct Lotes<'a> {
    pub lotes: &'a [crate::dominio::Lote],
}
impl Render for Lotes<'_> {
    fn render(&self) -> Markup {
        let lotes = self.lotes;
        Principal { titulo: "Lotes", ativo: "/lotes", conteudo: html! {
            (Cabecalho { titulo: "Lotes", texto: "", ajuda: "lotes" })
            div.actions { a.button href="/lotes/novo" target="_blank"
                rel="noopener" title="Abre em nova guia" { "+ Cadastrar lote" } }
            section.card.table-wrap {
                @if lotes.is_empty() { p.empty {
                        "Nenhum lote cadastrado." } }
                @else {
                    table { thead { tr { th { "Lote" } th { "Animais" } th { "Peso (kg)" }
                                th { "Leite (kg/animal/dia)" } th { "Dias em lactação" } th {} } }
                        tbody { @for l in lotes { tr { td { strong { (&l.nome) } }
                                    td { (l.animais) } td { (l.peso) } td { (l.producao) } td {
                                        (l.dias_lactacao) }
                                    td { a href=(format!("/lotes/{}/editar", l.id))
                                        target="_blank" rel="noopener" title="Abre em nova guia" {
                                            "Revisar ↗" } }
                                } } }
                    }
                }
            }
        }}.render()
    }
}

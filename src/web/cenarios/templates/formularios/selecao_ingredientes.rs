//! Pesquisa sem alterar seleção, agrupamento por classe e limite visível.
use super::super::componentes::{campo::MensagemErro, expansivel::Expansivel};
use super::cenario::FormularioCenario;
use crate::dominio::Classe;
use crate::web::formatacao::numero;
use maud::{Markup, Render, html};

pub struct SelecaoIngredientes<'a, 'b>(pub &'a FormularioCenario<'b>);
impl Render for SelecaoIngredientes<'_, '_> {
    fn render(&self) -> Markup {
        let f = self.0;
        let limite = if f.valor("modo") == "formular" {
            10
        } else {
            20
        };
        let quantidade = f
            .alimentos
            .iter()
            .filter(|a| f.valor(&format!("usar-{}", a.composicao_id)) == "on")
            .count();
        Expansivel {
            id: "selecao-ingredientes",
            aberto: f.valor("expansao-selecao-ingredientes") == "true" || f.erro("itens").is_some(),
            titulo: html! { "Selecionar ingredientes" },
            conteudo: html! {
                (MensagemErro { nome: "itens", erro: f.erro("itens") })
                div.form-grid {
                    div.field { label for="busca-ingredientes" { "Pesquisar ingrediente" }
                        input #busca-ingredientes name="busca-ingredientes" type="search"
                            value=(f.valor("busca-ingredientes")); }
                    div.field { label for="classe-ingredientes" { "Classe do alimento" }
                        select #classe-ingredientes name="classe-ingredientes" {
                            @for (v, r) in [("", "Todas"), ("Forragem", "Forragens"),
                                ("Concentrado", "Concentrados"), ("Suplemento", "Suplementos")] {
                                option value=(v) selected[f.valor("classe-ingredientes") == v] {
                                    (r)
                                }
                            }
                        }
                    }
                }
                p #contador-ingredientes role="status" aria-live="polite" {
                    (quantidade) " selecionados de " (limite) " permitidos."
                }
                div hx-post="/cenarios/atualizar"
                    hx-trigger="change target:'input[type=checkbox]'"
                    hx-target="#formulario" hx-swap="outerHTML settle:0" hx-include="#formulario"
                    hx-validate="false" {
                    @for (classe, rotulo) in [(Classe::Forragem, "Forragens"),
                        (Classe::Concentrado, "Concentrados"),
                        (Classe::Suplemento, "Suplementos")] {
                        @let classe_id = format!("{classe:?}");
                        fieldset.selecao-classe data-classe=(&classe_id) {
                            legend { (rotulo) }
                            @for a in f.alimentos.iter().filter(|a| a.classe == classe) {
                                @let id = &a.composicao_id;
                                @let visivel = a.nome.to_lowercase()
                                    .contains(&f.valor("busca-ingredientes").to_lowercase())
                                    && (f.valor("classe-ingredientes").is_empty()
                                        || f.valor("classe-ingredientes") == classe_id);
                                label.check data-alimento=(&a.nome) hidden[!visivel] {
                                    input type="checkbox" name=(format!("usar-{id}"))
                                        id=(format!("usar-{id}"))
                                        checked[f.valor(&format!("usar-{id}")) == "on"];
                                    span.composicao-opcao {
                                        span { (&a.nome) " · "
                                            (if a.natureza == "SINTETICA" { "Exemplo sintético" }
                                                else { &a.natureza }) }
                                        small {
                                            "MS " (numero(a.ms * 100.0, 2)) "% MN · PB "
                                            (a.nutrientes[1].map(|v| {
                                                format!("{}% MS", numero(v * 100.0, 2))
                                            }).unwrap_or("Desconhecida".into()))
                                            " · Data da análise: "
                                            (if a.data.is_empty() { "Não informada" }
                                                else { &a.data })
                                        }
                                        small { "Composição: " (id) }
                                    }
                                }
                            }
                        }
                    }
                }
                button.button.secondary type="submit" formnovalidate data-sem-validacao
                    formaction="/cenarios/atualizar" hx-post="/cenarios/atualizar"
                    hx-target="#formulario" hx-swap="outerHTML settle:0" {
                        "Atualizar ingredientes selecionados"
                    }
                p.muted { "A pesquisa não desmarca ingredientes. Sem JavaScript, use o botão "
                    "para aplicar a pesquisa e atualizar a seleção." }
            },
        }
        .render()
    }
}

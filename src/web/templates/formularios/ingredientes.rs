//! Seletor e quantidades dos ingredientes de um cenário.
use super::super::componentes::campo::Campo;
use super::super::componentes::expansivel::Expansivel;
use super::cenario::FormularioCenario;
use maud::{Markup, Render, html};

pub struct Ingredientes<'a, 'b>(pub &'a FormularioCenario<'b>);
impl Render for Ingredientes<'_, '_> {
    fn render(&self) -> Markup {
        html! { section.card {
            div.section-title { h2 {
                    "2. Ingredientes e disponibilidade" }
                a href="/alimentos/novo" target="_blank" rel="noopener"
                title="Abre em nova guia" { "Cadastrar alimento ↗" }
            }
            p.muted {
                "Selecione uma composição por alimento. Formulação: até 10; verificação: até 20. "
                "Preços em R$/kg MN; limites em kg MS/animal/dia."
            }
            (super::selecao_ingredientes::SelecaoIngredientes(self.0))
            @for a in self.0.alimentos {
                @let id = &a.composicao_id;
                @if self.0.valor(&format!("usar-{id}")) != "on" {
                    // Mantém o rascunho ao desmarcar e marcar novamente, sem exibir controles.
                    @for prefixo in ["q", "preco", "unidade-preco", "embalagem", "ms",
                        "perda", "estoque", "min", "max", "resolucao", "origem", "livre"] {
                        @let nome = format!("{prefixo}-{id}");
                        @if self.0.campos.contains_key(&nome) {
                            input type="hidden" name=(&nome) value=(self.0.valor(&nome));
                        }
                    }
                } @else {
                    article.ingredient data-composicao=(id) {
                        h3 { (&a.nome) }
                        div.form-grid {
                            @for (prefixo, rotulo) in [("q", "Quantidade na base escolhida"),
                                ("preco", "Preço na unidade comercial"),
                                ("estoque", "Estoque bruto (kg MN)")] {
                                @let nome = format!("{prefixo}-{id}");
                                @let formular = self.0.valor("modo") == "formular";
                                div data-quantidade=[(prefixo == "q").then_some("")]
                                    hidden[prefixo == "q" && formular] {
                                (Campo { erro: self.0.erro(&nome), nome: &nome, rotulo,
                                    valor: self.0.valor(&nome),
                                    tipo: if prefixo == "q" && formular { "hidden" }
                                        else { "number" },
                                    dica: if prefixo == "estoque" { "Vazio significa desconhecido" }
                                        else if prefixo == "q" { "Ignorada ao formular" }
                                        else { "Padrão: R$/kg MN; informe sua cotação" },
                                    obrigatorio: prefixo == "preco" || (prefixo == "q"
                                        && !formular) })
                                }
                            }
                        }
                        p.muted { "Informe o estoque, deixe vazio para desconhecido ou marque "
                            "a hipótese de disponibilidade sem limite abaixo." }
                                label.check { input type="checkbox"
                                    name=(format!("livre-{id}"))
                                    checked[self.0.valor(&format!("livre-{id}")) ==
                                    "on"];
                                    "Hipótese explícita: disponibilidade sem limite neste cenário"
                                }
                        (Expansivel {
                            id: &format!("ingrediente-{id}"),
                            aberto: self.0.valor(&format!("expansao-ingrediente-{id}"))
                                == "true",
                            titulo: html! { "Configurações avançadas do ingrediente" },
                            conteudo: html! {
                                div.form-grid {
                                    div.field {
                                        label for=(format!("unidade-preco-{id}")) {
                                            "Unidade comercial do preço" }
                                        select id=(format!("unidade-preco-{id}"))
                                        name=(format!("unidade-preco-{id}")) {
                                            @for (v, rotulo) in [("kg", "R$/kg MN"),
                                            ("t", "R$/tonelada MN"),
                                            ("embalagem",
                                            "R$/embalagem")] {
                                                option value=(v)
                                                selected[
                                                self.0.valor(
                                                &format!("unidade-preco-{id}")
                                                ) == v
                                                ] {
                                                    (rotulo)
                                                }
                                            }
                                        }
                                    }
                                    @for (prefixo, rotulo, dica) in [
                                    ("embalagem", "Massa da embalagem (kg MN)",
                                    "Obrigatória para preço por embalagem"),
                                    ("ms", "MS neste cenário (%)",
                                    "Simula alteração sem editar o catálogo"),
                                    ("perda",
                                    "Perda anterior ao fornecimento (%)",
                                    "Entre 0 e 100% exclusive"),
                                    ("min", "Inclusão mínima (kg MS)",
                                    "Positivo torna a inclusão obrigatória"),
                                    ("max", "Inclusão máxima (kg MS)",
                                    "Vazio: sem restrição adicional"),
                                    ("resolucao",
                                    "Resolução de oferta (kg MN/animal)",
                                    "Zero mantém a quantidade contínua"),
                                    ] {
                                        @let nome = format!("{prefixo}-{id}");
                                        (Campo { erro: self.0.erro(&nome), nome:
                                            &nome, rotulo,
                                            valor: self.0.valor(&nome),
                                            tipo: "number", dica, obrigatorio:
                                            self.0.valor(&format!("usar-{id}")) == "on"
                                            && (["ms", "perda", "preco"].contains(&prefixo)
                                            || (prefixo == "q" && self.0.valor("modo")
                                            != "formular")
                                            || (prefixo == "embalagem" && self.0.valor(
                                            &format!("unidade-preco-{id}")) ==
                                            "embalagem"))
                                        })
                                    }
                                    @let nome = format!("origem-{id}");
                                    (Campo { erro: self.0.erro(&nome), nome: &nome,
                                        rotulo:
                                        "Origem e data da cotação",
                                        valor: self.0.valor(&nome), tipo: "text",
                                        dica: "Justifique também preços zero",
                                        obrigatorio: self.0.valor(&format!("usar-{id}")) ==
                                        "on" })
                                }
                            },
                        })
                    }
                }
            }
        } }
    }
}

use super::super::componentes::campo::Campo;
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
            @for a in self.0.alimentos {
                @let id = &a.composicao_id;
                article.ingredient {
                    div.ingredient-selection {
                        label.check {
                            input type="checkbox"
                            name=(format!("usar-{id}")) id=(format!("usar-{id}"))
                            aria-invalid=[self.0.erro(&format!("usar-{id}")).map(|_| "true")]
                            checked[self.0.valor(&format!("usar-{id}")) == "on"];
                            strong { (&a.nome) }
                            span.pill { (&a.natureza) }
                        }
                        span.muted { "MS " (format!("{:.1}%", a.ms *
                            100.0))
                            " · " (&id[..id.len().min(12)]) }
                    }
                    (super::super::componentes::expansivel::Expansivel {
                        id: &format!("ingrediente-{id}"),
                        aberto: match self.0.valor(&format!("expansao-ingrediente-{id}")) {
                            "true" => true, "false" => false,
                            _ => self.0.valor(&format!("usar-{id}")) == "on",
                        },
                        titulo: html! { "Quantidades, preço e disponibilidade" },
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
                                ("q", "Quantidade na base escolhida",
                                "Ignorada ao formular"),
                                ("preco", "Preço na unidade comercial",
                                "Cotação decimal; zero precisa de justificativa"),
                                ("embalagem", "Massa da embalagem (kg MN)",
                                "Obrigatória para preço por embalagem"),
                                ("ms", "MS neste cenário (%)",
                                "Simula alteração sem editar o catálogo"),
                                ("perda",
                                "Perda anterior ao fornecimento (%)",
                                "Entre 0 e 100% exclusive"),
                                ("estoque", "Estoque bruto (kg MN)",
                                "Vazio significa desconhecido"),
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
                            label.check { input type="checkbox"
                                name=(format!("livre-{id}"))
                                checked[self.0.valor(&format!("livre-{id}")) ==
                                "on"];
                                "Hipótese explícita: disponibilidade sem limite neste cenário"
                            }
                        },
                    })
                }
            }
        } }
    }
}

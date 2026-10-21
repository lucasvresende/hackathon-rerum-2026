//! Premissas e opções avançadas do cenário.
use super::super::componentes::campo::{Campo, Rotulo};
use super::super::componentes::expansivel::Expansivel;
use super::cenario::FormularioCenario;
use maud::{Markup, Render, html};

pub struct ConfiguracoesCenario<'a, 'b>(pub &'a FormularioCenario<'b>);
impl Render for ConfiguracoesCenario<'_, '_> {
    fn render(&self) -> Markup {
        html! {
            (Expansivel { id: "configuracoes-avancadas",
                aberto: self.0.valor("expansao-configuracoes-avancadas") == "true",
                titulo: html! { "Configurações avançadas" },
                conteudo: html! {
                    div.form-grid {
                        @for (nome, rotulo, tipo, dica, obrigatorio) in [
                        ("data_avaliacao", "Data de avaliação", "date",
                        "Calendário Brasil", true),
                        ("data_aplicacao",
                        "Data de aplicação das regras", "date",
                        "Define a vigência normativa", true),
                        ("consumo_alvo",
                        "Consumo-alvo (kg MS/animal/dia)", "number",
                        "Opcional. Vazio: estimativa tabelada/extrapolada",
                        false),
                        ("sobras",
                        "Sobras planejadas (% da MS ofertada)", "number",
                        "Zero também deve ser uma hipótese declarada", true),
                        ("preco_leite", "Preço do leite (R$/kg)", "text",
                        "Opcional; permite margem sobre alimentação",
                        false),
                        ] {
                            div {
                                (Campo { erro: self.0.erro(nome), nome, rotulo,
                                    valor: self.0.valor(nome), tipo, dica, obrigatorio })
                            }
                        }
                        div.field {
                            (Rotulo { nome: "base", texto: "Quantidade informada por ingrediente",
                                obrigatorio: false })
                            select #base name="base" {
                                @for (v, texto) in [("CONSUMO_MS",
                                "Consumo em kg MS/animal/dia"),
                                ("CONSUMO_MN",
                                "Consumo em kg MN/animal/dia"),
                                ("OFERTA_MN",
                                "Oferta em kg MN/animal/dia")] {
                                    option value=(v)
                                    selected[self.0.valor("base") == v] {
                                        (texto) }
                                }
                            }
                        }
                    }
                    div.form-grid {
                        @for (nome, rotulo, dica) in [
                        ("origem_consumo", "Origem do consumo",
                        "Estimado, informado ou observado"),
                        ("origem_sobras", "Origem das sobras e perdas",
                        "Medição ou hipótese explícita"),
                        ] {
                            (Campo { erro: self.0.erro(nome), nome, rotulo, valor:
                                self.0.valor(nome), tipo: "text", dica,
                                obrigatorio: true })
                        }
                    }
                    (Expansivel { id: "medicao", aberto:
                        false, titulo:
                        html! { "Consumo observado por pesagem do lote (opcional)" },
                        conteudo:
                        html! {
                            p.muted {
                                "Mesmos animais e período; informe MS de oferta e sobras. \
                            Isso não determina a composição ingerida quando há seleção de
                            partículas." }
                            div.form-grid {
                                @for (nome, rotulo) in [("oferta_observada",
                                "MS ofertada ao lote (kg)"),
                                ("sobras_observadas",
                                "MS das sobras do lote (kg)"),
                                ("periodo_observado",
                                "Período da medição (dias)")] {
                                    (Campo { erro: self.0.erro(nome), nome, rotulo,
                                        valor:
                                        self.0.valor(nome), tipo: "number",
                                        dica:
                                        "Deixe consumo-alvo vazio ao usar medição",
                                        obrigatorio: false })
                                }
                            }
                        }, })
                    label.check { input type="checkbox" name="ue"
                        checked[self.0.valor("ue")
                        == "on"];
                        "Ativar referência UE adicional para contaminantes (não é legislação \
                                        brasileira)"
                    }
                    div.field {
                        (Rotulo { nome: "base_comparacao", texto: "Base da simulação de MS",
                            obrigatorio: false })
                        select #base_comparacao name="base_comparacao" {
                            @for v in ["Consumo de MS fixo",
                            "Quantidade de MN fixa"] {
                                option selected[self.0.valor("base_comparacao")
                                == v] { (v) }
                            }
                        }
                        small {
                            "Ao alterar MS, esta escolha governa a conversão." }
                    }
                },
            })
        }
    }
}

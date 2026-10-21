use super::super::componentes::expansivel::Expansivel;
use std::collections::BTreeMap;

use super::super::componentes::campo::{MensagemErro, Rotulo};
use maud::{Markup, Render, html};

use super::super::componentes::campo::Campo;
use super::super::componentes::resumo_erros::ResumoErros;
use crate::dominio::validacao::Problema;
use crate::dominio::{Alimento, Lote};

pub struct FormularioCenario<'a> {
    pub campos: &'a BTreeMap<String, String>,
    pub lotes: &'a [Lote],
    pub alimentos: &'a [Alimento],
    pub erros: &'a [Problema],
    pub token: &'a str,
}

impl FormularioCenario<'_> {
    pub(super) fn erro(&self, nome: &str) -> Option<&str> {
        self.erros
            .iter()
            .find(|e| e.campo == nome)
            .map(|e| e.mensagem.as_str())
    }

    pub(super) fn valor(&self, nome: &str) -> &str {
        self.campos.get(nome).map(String::as_str).unwrap_or("")
    }
}

impl Render for FormularioCenario<'_> {
    fn render(&self) -> Markup {
        html! {
            form #formulario method="post" action="/cenarios"
            hx-post="/cenarios"
            hx-target="#formulario" hx-swap="outerHTML"
            {
                input type="hidden" name="csrf" value=(self.token);
                input type="hidden" name="original_id"
                value=(self.valor("original_id"));
                (ResumoErros(self.erros))
                div.actions {
                    button #atualizar-cadastros.button.secondary type="submit"
                    formnovalidate
                    formaction="/cenarios/atualizar"
                    hx-post="/cenarios/atualizar"
                    hx-target="#formulario" hx-swap="outerHTML"
                    data-sem-validacao { "Atualizar cadastros" }
                    a href="/cenarios/novo?dieta=A" { "Demonstração A" }
                    a href="/cenarios/novo?dieta=B" { "Demonstração B" }
                }
                p.muted { "Campos obrigatórios: *" }
                section.card {
                    h2 { "1. Dados do cenário" }
                    div.form-grid {
                        (Campo { erro: self.erro("nome"), nome: "nome", rotulo:
                            "Nome do cenário", valor: self.valor("nome"),
                            tipo: "text", dica:
                            "Uma identificação para comparar depois",
                            obrigatorio: true })
                        div.field { (Rotulo {
                            nome: "lote",
                            texto: "Lote e revisão atual",
                            obrigatorio: true
                        })
                            (MensagemErro { nome: "lote", erro: self.erro("lote") })
                            select #lote name="lote" required {
                                option value=""
                                selected[self.valor("lote").is_empty()] {
                                    "Selecione o lote"
                                }
                                @for lote in self.lotes {
                                    option value=(&lote.id)
                                    data-peso=(lote.peso)
                                    selected[self.valor("lote") ==
                                    lote.id] {
                                        (&lote.nome) " · " (lote.animais)
                                        " animais · "
                                        (lote.peso) " kg"
                                    }
                                }
                            }
                            small { a href="/lotes/novo" target="_blank"
                                rel="noopener" title="Abre em nova guia" {
                                    "+ Cadastrar outro lote" } }
                        }
                        div.field { label for="modo" { "Tipo de cálculo" }
                            select #modo name="modo" {
                                option value="verificar"
                                selected[self.valor("modo") !=
                                "formular"] {
                                    "Verificar as quantidades informadas"
                                }
                                option value="formular"
                                selected[self.valor("modo") ==
                                "formular"] {
                                    "Formular combinação de menor custo"
                                }
                            }
                        }
                        @for (nome, rotulo, tipo, dica, obrigatorio) in [
                        ("horizonte", "Horizonte (dias)", "number",
                        "Inteiro positivo", true),
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
                                (Campo { erro: self.erro(nome), nome, rotulo,
                                    valor: self.valor(nome), tipo, dica, obrigatorio })
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
                                    selected[self.valor("base") == v] {
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
                            (Campo { erro: self.erro(nome), nome, rotulo, valor:
                                self.valor(nome), tipo: "text", dica,
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
                                    (Campo { erro: self.erro(nome), nome, rotulo,
                                        valor:
                                        self.valor(nome), tipo: "number",
                                        dica:
                                        "Deixe consumo-alvo vazio ao usar medição",
                                        obrigatorio: false })
                                }
                            }
                        }, })
                    label.check { input type="checkbox" name="ue"
                        checked[self.valor("ue")
                        == "on"];
                        "Ativar referência UE adicional para contaminantes (não é legislação \
                                        brasileira)"
                    }
                }
                (super::ingredientes::Ingredientes(self))
                section.card {
                    h2 { "3. Base da comparação" }
                    div.field {
                        (Rotulo { nome: "base_comparacao", texto: "Base da simulação de MS",
                            obrigatorio: false })
                        select #base_comparacao name="base_comparacao" {
                            @for v in ["Consumo de MS fixo",
                            "Quantidade de MN fixa"] {
                                option selected[self.valor("base_comparacao")
                                == v] { (v) }
                            }
                        }
                        small {
                            "Ao alterar MS, esta escolha governa a conversão." }
                    }
                    div.alert {
                        "O resultado avalia o perfil implementado. Fibra física, vitaminas e \
                                demais dimensões sem dados continuam identificadas como não
                        avaliadas." }
                    div.form-actions {
                        a.button.secondary
                        href="/cadastros/salvo?tipo=cancelado" {
                            "Cancelar" }
                        button.button type="submit" {
                            "Calcular e salvar cenário →"
                        }
                    }
                }
            }
        }
    }
}

//! Formulário de apresentação para cenario, mantendo a renderização desta responsabilidade isolada.
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
            hx-target="#formulario" hx-swap="outerHTML settle:0"
            hx-sync:inherited="this:drop"
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
                    hx-target="#formulario" hx-swap="outerHTML settle:0"
                    data-sem-validacao { "Atualizar cadastros" }
                    (super::exemplo_cenario::ExemploCenario(self))
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
                        (Campo { erro: self.erro("horizonte"), nome: "horizonte",
                            rotulo: "Horizonte (dias)", valor: self.valor("horizonte"),
                            tipo: "number", dica: "Inteiro positivo", obrigatorio: true })
                    }
                    p.muted {
                        "Padrão: consumo estimado, quantidades em kg MS/animal/dia e "
                        "hipótese de sobras e perdas zero. Revise conforme sua realidade."
                    }
                    (super::configuracoes_cenario::ConfiguracoesCenario(self))
                }
                (super::ingredientes::Ingredientes(self))
                section.card {
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

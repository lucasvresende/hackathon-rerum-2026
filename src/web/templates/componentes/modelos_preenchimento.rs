//! Interface para selecionar modelos e revisar procedência.
use maud::{Markup, Render, html};

use crate::dominio::templates::{ModeloPreenchimento, ProcedenciaModelo};
use crate::web::entradas::{Campos, texto};

pub struct SeletorModelo<'a> {
    pub tipo: &'a str,
    pub modelos: &'a [ModeloPreenchimento],
    pub campos: &'a Campos,
}
impl Render for SeletorModelo<'_> {
    fn render(&self) -> Markup {
        let rota = format!("/cadastros/{}/modelo", self.tipo);
        let aplicado = texto(self.campos, "modelo_aplicado");
        html! {
            section.templates {
                h2 { "Preencher com exemplo" }
                p.muted { "Carregue uma referência e edite os valores antes de salvar. "
                    "O original permanece igual. "
                    "Condições sintéticas são hipóteses para simulação." }
                input type="hidden" name="modelo_aplicado" value=(aplicado);
                div.field {
                    label for="modelo" { "Exemplo disponível" }
                    select #modelo name="modelo" {
                        option value="" { "Selecione para consultar a prévia" }
                        @for m in self.modelos.iter()
                        .filter(|m| m.tipo == self.tipo && m.id.starts_with("ex-")) {
                            option value=(&m.id) selected[texto(self.campos,
                            "modelo") == m.id] {
                                (&m.nome)
                            }
                        }
                    }
                }
                button.button.secondary type="submit" formaction=(&rota) hx-post=(&rota)
                hx-target="#formulario" hx-swap="outerHTML"
                formnovalidate data-sem-validacao name="acao_modelo"
                value="previa" {
                    "Ver prévia do exemplo"
                }
                @if !aplicado.is_empty() {
                    p.alert role="status" {
                        "Exemplo aplicado: " (aplicado)
                        ". Edite os valores abaixo; alterações ficam registradas por campo."
                    }
                }
                @if texto(self.campos, "previa_modelo") == "on" {
                    @if let Some(m) = self.modelos.iter()
                    .find(|m| m.id == texto(self.campos, "modelo")) {
                        div.template-preview tabindex="-1" {
                            h3 { (&m.nome) " · versão " (&m.versao) }
                            p { (&m.contexto) }
                            p.alert { (&m.lacunas) }
                            p { "Fonte: " a href=(&m.fonte_url) target="_blank"
                                rel="noopener" {
                                    (&m.fonte_titulo) } }
                            div.table-wrap { table {
                                    thead { tr { th { "Campo" } th { "Valor sugerido" }
                                            th { "Original / método" } } }
                                    tbody { @for v in &m.valores { tr {
                                                td { (&v.campo) } td { (&v.valor) " " (&v.unidade) }
                                                td { (&v.valor_original) " " (&v.unidade_original)
                                                    " · " (&v.natureza) }
                                            } } }
                                } }
                            label.check for="confirmar_modelo" {
                                input #confirmar_modelo type="checkbox"
                                name="confirmar_modelo";
                                "Confirmo substituir os campos do exemplo e revisar os valores."
                            }
                            button.button type="submit" formaction=(format!("{rota}?acao=aplicar"))
                            hx-post=(format!("{rota}?acao=aplicar"))
                            hx-target="#formulario" hx-swap="outerHTML"
                            formnovalidate data-sem-validacao name="acao_modelo"
                            value="aplicar" {
                                "Preencher com exemplo"
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct Procedencia<'a>(pub &'a ProcedenciaModelo);
impl Render for Procedencia<'_> {
    fn render(&self) -> Markup {
        html! {
            div.template-history {
                h3 { "Exemplo de origem: " (&self.0.modelo.nome) }
                p { (&self.0.modelo.contexto) " " (&self.0.modelo.lacunas) }
                p { "Fonte: " (&self.0.modelo.fonte_titulo)
                    " · versão " (&self.0.modelo.versao) }
                ul {
                    @for campo in &self.0.campos {
                        @if let Some(v) = self.0.modelo.valores.iter()
                            .find(|v| v.campo == campo.campo) {
                            li {
                                (&campo.campo) ": referência " (&v.valor) " " (&v.unidade)
                                " → utilizado " (&campo.utilizado)
                                @if campo.alterado { strong { " · alterado pelo operador" } }
                            }
                        } @else {
                            li {
                                (&campo.campo) ": referência desconhecida → utilizado "
                                (&campo.utilizado) strong { " · informado pelo operador" }
                            }
                        }
                    }
                }
            }
        }
    }
}

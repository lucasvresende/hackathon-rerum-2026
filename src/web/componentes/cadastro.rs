use std::collections::BTreeMap;

use maud::{Markup, Render, html};

use super::estrutura::Campo;
use crate::dominio::validacao::Problema;

pub struct Cadastro<'a> {
    pub tipo: &'a str,
    pub campos: &'a BTreeMap<String, String>,
    pub erros: &'a [Problema],
    pub modelos: &'a [crate::dominio::templates::ModeloPreenchimento],
    pub token: &'a str,
}

impl Cadastro<'_> {
    fn erro(&self, nome: &str) -> Option<&str> {
        self.erros
            .iter()
            .find(|e| e.campo == nome)
            .map(|e| e.mensagem.as_str())
    }

    fn valor(&self, nome: &str) -> &str {
        self.campos.get(nome).map(String::as_str).unwrap_or("")
    }
}

pub fn erros(problemas: &[Problema]) -> Markup {
    html! {
        @if !problemas.is_empty() {
            div #erros-formulario.alert.error tabindex="-1" role="alert" {
                strong { "Revise os campos para continuar" }
                ul { @for erro in problemas { li { a href=(format!("#{}", erro.campo)) {
                                (&erro.campo) ": " (&erro.mensagem) } } } }
            }
        }
    }
}

impl Render for Cadastro<'_> {
    fn render(&self) -> Markup {
        let lote = self.tipo == "lote";
        let rota = if lote { "/lotes" } else { "/alimentos" };
        html! {
            form #formulario.card method="post" action=(rota)
            hx-post=(rota) hx-target="#formulario" hx-swap="outerHTML"
            data-simples=(self.valor("visualizacao") != "completo")
            hx-live:class="{ 'simplificado': data.simples }" {
                input type="hidden" name="csrf" value=(self.token);
                input type="hidden" name="id" value=(self.valor("id"));
                input type="hidden" name="anterior"
                value=(self.valor("anterior"));
                (erros(self.erros))
                p.muted { "Campos obrigatórios: *" }
                (super::expansivel::Visualizacao)
                (super::templates::Templates {
                    tipo: self.tipo, modelos: self.modelos, campos: self.campos,
                })
                h2 { "Identificação e dados" }
                div.form-grid {
                    (Campo { erro: self.erro("nome"), nome: "nome", rotulo:
                        "Nome", valor:
                        self.valor("nome"),
                        tipo: "text", dica:
                        "Identificação para consulta e histórico",
                        obrigatorio: true })
                    @if lote {
                        div.field {
                            (super::campo::rotulo("estado_produtivo",
                            "Estado produtivo",
                            false))
                            (super::campo::mensagem("estado_produtivo",
                            self.erro("estado_produtivo")))
                            select #estado_produtivo name="estado_produtivo" {
                                @for (v, t) in [("DESCONHECIDO",
                                "Desconhecido"),
                                ("LACTACAO", "Lactação"), ("SECA", "Seca"),
                                ("TRANSICAO", "Transição"), ("RECRIA",
                                "Recria")] {
                                    option value=(v)
                                    selected[self.valor("estado_produtivo") == v] {
                                        (t)
                                    }
                                }
                            }
                        }
                        (Campo { erro: self.erro("destinacao"), nome:
                            "destinacao", rotulo:
                            "Destinação econômica",
                            valor: self.valor("destinacao"), tipo: "text",
                            dica: "Por exemplo: produção de leite",
                            obrigatorio: false })
                        (Campo { erro: self.erro("raca"), nome: "raca", rotulo:
                            "Raça / grupo genético", valor:
                            self.valor("raca"),
                            tipo: "text", dica: "Pode ser desconhecida",
                            obrigatorio:
                            false })
                    } @else {
                        div.field { (super::campo::rotulo("classe",
                            "Classificação", true))
                            (super::campo::mensagem("classe",
                            self.erro("classe")))
                            select #classe name="classe" required {
                                @for (v, texto) in [("FORRAGEM", "Forragem"),
                                ("CONCENTRADO", "Concentrado"),
                                ("SUPLEMENTO",
                                "Suplemento")] {
                                    option value=(v)
                                    selected[self.valor("classe") == v] {
                                        (texto) }
                                }
                            }
                        }
                    }
                    @for (nome, rotulo, dica, obrigatorio)
                        in super::campos_cadastro::numericos(lote) {
                        div class=[(lote && ["idade_meses", "ordem_parto",
                        "dispersao_peso",
                        "condicao_corporal"].contains(&nome)).then_some("complementar")] {
                            (Campo { erro: self.erro(nome), nome, rotulo, valor:
                                self.valor(nome), tipo: "number",
                                dica, obrigatorio })
                        }
                    }
                }
                h2 { "Condições declaradas" }
                p.muted { "Desconhecido é diferente de não. Confirme somente o que foi avaliado." }
                div.form-grid {
                    @for (nome, rotulo) in super::campos_cadastro::estados(lote) {
                        div.field { label for=(nome) { (rotulo) }
                            select id=(nome) name=(nome) {
                                @for (v, texto) in [("DESCONHECIDO",
                                "Desconhecido"),
                                ("SIM", "Sim"), ("NAO", "Não")] {
                                    option value=(v)
                                    selected[self.valor(nome) == v] {
                                        (texto) }
                                }
                            }
                        }
                    }
                }
                @if !lote {
                    h2 { "Procedência da composição" }
                    div.form-grid {
                        div.field { (super::campo::rotulo("natureza",
                            "Natureza dos valores", true))
                            (super::campo::mensagem("natureza",
                            self.erro("natureza")))
                            select #natureza name="natureza" required {
                                @for v in ["ANALISE", "MEDIA_TABELADA",
                                "SINTETICA",
                                "GARANTIA_MINIMA", "GARANTIA_MAXIMA"] {
                                    option selected[self.valor("natureza") ==
                                    v] { (v) }
                                }
                            }
                        }
                        @for (nome, rotulo, dica) in [
                        ("fonte_id", "Identificador da fonte",
                        "Fonte existente em Conhecimento"),
                        ("origem", "Origem",
                        "Procedência e local da coleta"),
                        ("processamento", "Processamento",
                        "Forma de preparo do alimento"),
                        ("metodo", "Método, edição e página",
                        "Como a composição foi determinada"),
                        ("data", "Data da análise",
                        "Data civil da composição"),
                        ("fornecedor", "Fornecedor / produto",
                        "Identificação comercial"),
                        ("lote_fisico", "Lote físico",
                        "Rastreabilidade do produto"),
                        ("evidencia", "Evidência de autorização",
                        "Documento e condições de uso"),
                        ] {
                            div class=[(["fornecedor", "lote_fisico", "data"]
                            .contains(&nome)).then_some("complementar")] {
                                (Campo { erro: self.erro(nome), nome, rotulo, valor:
                                    self.valor(nome), tipo: "text", dica,
                                    obrigatorio: ["fonte_id", "metodo",
                                    "origem"].contains(&nome) })
                            }
                        }
                    }
                    (super::expansivel::Expansivel { id: "laudos", aberto:
                        false,
                        titulo: html! { "Laudos de contaminantes (opcional)" },
                        conteudo: html! {
                            p.muted {
                                "Sem laudo não significa zero. Registre unidade e matriz." }
                            @for i in 0..3 {
                                div.form-grid {
                                    @for (sufixo, rotulo, tipo) in [
                                    ("analito",
                                    "Analito: AFB1, GOSSIPOL_LIVRE ou HCN",
                                    "text"),
                                    ("valor", "Concentração", "number"),
                                    ("unidade",
                                    "Unidade: µg/kg MN, µg/kg MS ou mg/kg MN",
                                    "text"),
                                    ("ms", "MS da amostra (%)", "number"),
                                    ("qualificador",
                                    "Resultado: EXATO ou <LOQ", "text"),
                                    ("loq", "LOQ na unidade informada",
                                    "number"),
                                    ("metodo", "Método laboratorial", "text"),
                                    ("documento", "Identificação do laudo",
                                    "text"),
                                    ] {
                                        @let nome = format!("laudo_{i}_{sufixo}");
                                        (Campo { erro: self.erro(&nome), nome:
                                            &nome, rotulo,
                                            valor: self.valor(&nome), tipo,
                                            dica: "Matriz: matéria-prima",
                                            obrigatorio: false })
                                    }
                                }
                            }
                        },
                    })
                } @else {
                    div.field { label for="observacoes" {
                            "Observações e hipóteses complementares" }
                        textarea #observacoes name="observacoes" {
                            (self.valor("observacoes")) }
                    }
                }
                div.form-actions { a.button.secondary
                    href="/cadastros/salvo?tipo=cancelado" { "Cancelar" }
                    button.button type="submit" { "Salvar revisão" }
                }
            }
        }
    }
}

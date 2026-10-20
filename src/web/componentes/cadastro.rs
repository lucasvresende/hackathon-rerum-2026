use std::collections::BTreeMap;

use maud::{Markup, Render, html};

use super::estrutura::Campo;
use crate::dominio::validacao::Problema;

pub struct Cadastro<'a> {
    pub tipo: &'a str,
    pub campos: &'a BTreeMap<String, String>,
    pub erros: &'a [Problema],
    pub token: &'a str,
}

impl Cadastro<'_> {
    fn valor(&self, nome: &str) -> &str {
        self.campos.get(nome).map(String::as_str).unwrap_or("")
    }
}

pub fn erros(problemas: &[Problema]) -> Markup {
    html! {
        @if !problemas.is_empty() {
            div #erros-formulario.alert.error tabindex="-1" role="alert" {
                strong { "Revise os campos para continuar" }
                ul { @for erro in problemas { li { (&erro.campo) ": " (&erro.mensagem) } } }
            }
        }
    }
}

impl Render for Cadastro<'_> {
    fn render(&self) -> Markup {
        let lote = self.tipo == "lote";
        let rota = if lote { "/lotes" } else { "/alimentos" };
        let numericos = if lote {
            vec![
                ("animais", "Número de animais", "Inteiro positivo", true),
                ("peso", "Peso médio (kg)", "Cobertura: 400 a 700 kg", true),
                (
                    "producao",
                    "Leite (kg/animal/dia)",
                    "Produção real, não litros",
                    true,
                ),
                (
                    "gordura",
                    "Gordura do leite (%)",
                    "Pontos percentuais: por exemplo, 4",
                    true,
                ),
                (
                    "dias_lactacao",
                    "Dias em lactação",
                    "Cobertura: 60 a 250",
                    true,
                ),
                (
                    "idade_meses",
                    "Idade (meses)",
                    "Vazio significa desconhecida",
                    false,
                ),
                ("ordem_parto", "Ordem de parto", "Opcional", false),
                (
                    "dispersao_peso",
                    "Dispersão de peso (kg)",
                    "Opcional",
                    false,
                ),
                (
                    "condicao_corporal",
                    "Condição corporal",
                    "Registre a escala em observações",
                    false,
                ),
            ]
        } else {
            vec![
                ("ms", "Matéria seca (% MN)", "Maior que 0 e até 100%", true),
                (
                    "ndt",
                    "NDT (% MS)",
                    "Não negativo; definição do perfil",
                    false,
                ),
                (
                    "pb",
                    "Proteína bruta (% MS)",
                    "Vazio significa desconhecido",
                    false,
                ),
                ("ca", "Cálcio (% MS)", "Vazio significa desconhecido", false),
                ("p", "Fósforo (% MS)", "Vazio significa desconhecido", false),
                ("fdn", "FDN (% MS)", "Vazio significa desconhecido", false),
                (
                    "amido",
                    "Amido (% MS)",
                    "Vazio significa desconhecido",
                    false,
                ),
                (
                    "ee",
                    "Extrato etéreo (% MS)",
                    "Vazio significa desconhecido",
                    false,
                ),
                (
                    "rotulo_max_ms",
                    "Máximo do rótulo (kg MS/animal/dia)",
                    "Obrigatório para recomendar suplemento",
                    false,
                ),
            ]
        };
        let estados = if lote {
            vec![
                ("adulta", "Classe adulta"),
                ("gestante", "Gestante"),
                ("crescimento", "Em crescimento"),
                ("peso_estavel", "Peso estável"),
                ("confinada", "Confinada com dieta total quantificável"),
                ("correcao_ambiente", "Precisa de correção por ambiente"),
                ("correcao_atividade", "Precisa de correção por atividade"),
                ("agua", "Acesso à água"),
            ]
        } else {
            vec![
                ("origem_animal", "Origem animal"),
                ("excecao_animal", "Exceção legal documentada"),
                ("nnp_adicionado", "NNP intencionalmente adicionado"),
                ("medicamento", "Contém medicamento"),
                ("autorizado", "Autorização comprovada"),
            ]
        };
        html! {
            form #formulario.card method="post" action=(rota)
                hx-post=(rota) hx-target="#formulario" hx-swap="outerHTML" {
                input type="hidden" name="csrf" value=(self.token);
                input type="hidden" name="id" value=(self.valor("id"));
                input type="hidden" name="anterior" value=(self.valor("anterior"));
                (erros(self.erros))
                h2 { "Identificação e dados" }
                div.form-grid {
                    (Campo { nome: "nome", rotulo: "Nome", valor: self.valor("nome"),
                        tipo: "text", dica: "Identificação para consulta e histórico", obrigatorio: true })
                    @if lote {
                        (Campo { nome: "raca", rotulo: "Raça / grupo genético", valor: self.valor("raca"),
                            tipo: "text", dica: "Pode ser desconhecida", obrigatorio: false })
                    } @else {
                        div.field { label for="classe" { "Classificação" }
                            select #classe name="classe" {
                                @for (v, texto) in [("FORRAGEM", "Forragem"),
                                    ("CONCENTRADO", "Concentrado"), ("SUPLEMENTO", "Suplemento")] {
                                    option value=(v) selected[self.valor("classe") == v] { (texto) }
                                }
                            }
                        }
                    }
                    @for (nome, rotulo, dica, obrigatorio) in numericos {
                        (Campo { nome, rotulo, valor: self.valor(nome), tipo: "number",
                            dica, obrigatorio })
                    }
                }
                h2 { "Condições declaradas" }
                p.muted { "Desconhecido é diferente de não. Confirme somente o que foi avaliado." }
                div.form-grid {
                    @for (nome, rotulo) in estados {
                        div.field { label for=(nome) { (rotulo) }
                            select id=(nome) name=(nome) {
                                @for (v, texto) in [("DESCONHECIDO", "Desconhecido"),
                                    ("SIM", "Sim"), ("NAO", "Não")] {
                                    option value=(v) selected[self.valor(nome) == v] { (texto) }
                                }
                            }
                        }
                    }
                }
                @if !lote {
                    h2 { "Procedência da composição" }
                    div.form-grid {
                        div.field { label for="natureza" { "Natureza dos valores" }
                            select #natureza name="natureza" {
                                @for v in ["ANALISE", "MEDIA_TABELADA", "SINTETICA",
                                    "GARANTIA_MINIMA", "GARANTIA_MAXIMA"] {
                                    option selected[self.valor("natureza") == v] { (v) }
                                }
                            }
                        }
                        @for (nome, rotulo, dica) in [
                            ("fonte_id", "Identificador da fonte", "Fonte existente em Conhecimento"),
                            ("origem", "Origem", "Procedência e local da coleta"),
                            ("processamento", "Processamento", "Forma de preparo do alimento"),
                            ("metodo", "Método, edição e página", "Como a composição foi determinada"),
                            ("data", "Data da análise", "Data civil da composição"),
                            ("fornecedor", "Fornecedor / produto", "Identificação comercial"),
                            ("lote_fisico", "Lote físico", "Rastreabilidade do produto"),
                            ("evidencia", "Evidência de autorização", "Documento e condições de uso"),
                        ] {
                            (Campo { nome, rotulo, valor: self.valor(nome), tipo: "text", dica,
                                obrigatorio: nome == "fonte_id" || nome == "metodo" })
                        }
                    }
                    details {
                        summary { "Laudos de contaminantes (opcional)" }
                        p.muted { "Sem laudo não significa concentração zero. Registre unidade e matriz." }
                        @for i in 0..3 {
                            div.form-grid {
                                @for (sufixo, rotulo, tipo) in [
                                    ("analito", "Analito: AFB1, GOSSIPOL_LIVRE ou HCN", "text"),
                                    ("valor", "Concentração", "number"),
                                    ("unidade", "Unidade: µg/kg MN, µg/kg MS ou mg/kg MN", "text"),
                                    ("ms", "MS da amostra (%)", "number"),
                                    ("qualificador", "Resultado: EXATO ou <LOQ", "text"),
                                    ("loq", "LOQ na unidade informada", "number"),
                                    ("metodo", "Método laboratorial", "text"),
                                    ("documento", "Identificação do laudo", "text"),
                                ] {
                                    @let nome = format!("laudo_{i}_{sufixo}");
                                    (Campo { nome: &nome, rotulo, valor: self.valor(&nome), tipo,
                                        dica: "Matriz: matéria-prima", obrigatorio: false })
                                }
                            }
                        }
                    }
                } @else {
                    div.field { label for="observacoes" { "Observações e hipóteses complementares" }
                        textarea #observacoes name="observacoes" { (self.valor("observacoes")) }
                    }
                }
                div.form-actions { a.button.secondary href=(rota) { "Voltar" }
                    button.button type="submit" { "Salvar revisão" }
                }
            }
        }
    }
}

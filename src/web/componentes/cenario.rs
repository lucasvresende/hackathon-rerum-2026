use std::collections::BTreeMap;

use maud::{Markup, Render, html};

use super::cadastro::erros;
use super::estrutura::{Campo, cabecalho};
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

impl Render for FormularioCenario<'_> {
    fn render(&self) -> Markup {
        html! {
            form #formulario method="post" action="/cenarios" hx-post="/cenarios"
                hx-target="#formulario" hx-swap="outerHTML" {
                input type="hidden" name="csrf" value=(self.token);
                input type="hidden" name="original_id" value=(self.valor("original_id"));
                (erros(self.erros))
                p.muted { "Campos obrigatórios: *" }
                section.card {
                    h2 { "1. Dados do cenário" }
                    div.form-grid {
                        (Campo { erro: self.erro("nome"), nome: "nome", rotulo:
                        "Nome do cenário", valor: self.valor("nome"),
                            tipo: "text", dica: "Uma identificação para comparar depois",
                            obrigatorio: true })
                        div.field { (super::campo::rotulo("lote", "Lote e revisão atual", true))
                            (super::campo::mensagem("lote", self.erro("lote")))
                            select #lote name="lote" required {
                                @for lote in self.lotes {
                                    option value=(&lote.id) selected[self.valor("lote") ==
                                    lote.id] {
                                        (&lote.nome) " · " (lote.animais) " animais · "
                                        (lote.peso) " kg"
                                    }
                                }
                            }
                            small { a href="/lotes/novo" { "+ Cadastrar outro lote" } }
                        }
                        div.field { label for="modo" { "Tipo de cálculo" }
                            select #modo name="modo" {
                                option value="verificar" selected[self.valor("modo") !=
                                "formular"] {
                                    "Verificar as quantidades informadas"
                                }
                                option value="formular" selected[self.valor("modo") == "formular"] {
                                    "Formular combinação de menor custo"
                                }
                            }
                        }
                        @for (nome, rotulo, tipo, dica, obrigatorio) in [
                            ("horizonte", "Horizonte (dias)", "number", "Inteiro positivo", true),
                            ("data_avaliacao", "Data de avaliação", "date",
                            "Calendário Brasil", true),
                            ("data_aplicacao", "Data de aplicação das regras", "date",
                                "Define a vigência normativa", true),
                            ("consumo_alvo", "Consumo-alvo (kg MS/animal/dia)", "number",
                                "Vazio: Tabela B; obrigatório para PV acima de 600 kg", false),
                            ("sobras", "Sobras planejadas (% da MS ofertada)", "number",
                                "Zero também deve ser uma hipótese declarada", true),
                            ("preco_leite", "Preço do leite (R$/kg)", "text",
                                "Opcional; permite margem sobre alimentação", false),
                        ] {
                            (Campo { erro: self.erro(nome), nome, rotulo, valor:
                            self.valor(nome), tipo, dica,
                            obrigatorio })
                        }
                        div.field { label for="base" { "Quantidade informada por ingrediente" }
                            select #base name="base" {
                                @for (v, texto) in [("CONSUMO_MS", "Consumo em kg MS/animal/dia"),
                                    ("CONSUMO_MN", "Consumo em kg MN/animal/dia"),
                                    ("OFERTA_MN", "Oferta em kg MN/animal/dia")] {
                                    option value=(v) selected[self.valor("base") == v] { (texto) }
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
                    details {
                        summary { "Consumo observado por pesagem do lote (opcional)" }
                        p.muted { "Mesmos animais e período; informe MS de oferta e sobras. \
                            Isso não determina a composição ingerida quando há seleção de
                            partículas." }
                        div.form-grid {
                            @for (nome, rotulo) in [("oferta_observada",
                            "MS ofertada ao lote (kg)"),
                                ("sobras_observadas", "MS das sobras do lote (kg)"),
                                ("periodo_observado", "Período da medição (dias)")] {
                                (Campo { erro: self.erro(nome), nome, rotulo, valor:
                                self.valor(nome), tipo: "number",
                                    dica: "Deixe consumo-alvo vazio ao usar medição",
                                    obrigatorio: false })
                            }
                        }
                    }
                    label.check { input type="checkbox" name="ue" checked[self.valor("ue") == "on"];
                        "Ativar referência UE adicional para contaminantes (não é legislação \
                        brasileira)"
                    }
                }
                section.card {
                    div.section-title { h2 { "2. Ingredientes e disponibilidade" }
                        a href="/alimentos" { "Consultar catálogo ↗" }
                    }
                    p.muted {
                    "Selecione uma composição por alimento. Formulação: até 10; verificação: \
                        até 20. Preços em R$/kg MN; limites em kg MS/animal/dia." }
                    @for a in self.alimentos {
                        @let id = &a.composicao_id;
                        details.ingredient open[self.valor(&format!("usar-{id}")) == "on"] {
                            summary {
                                label.check {
                                    input type="checkbox" name=(format!("usar-{id}"))
                                        checked[self.valor(&format!("usar-{id}")) == "on"];
                                    strong { (&a.nome) }
                                    span.pill { (&a.natureza) }
                                }
                                span.muted { "MS " (format!("{:.1}%", a.ms * 100.0))
                                    " · " (&id[..id.len().min(12)]) }
                            }
                            div.form-grid {
                                div.field {
                                    label for=(format!("unidade-preco-{id}")) {
                                    "Unidade comercial do preço" }
                                    select id=(format!("unidade-preco-{id}"))
                                        name=(format!("unidade-preco-{id}")) {
                                        @for (v, rotulo) in [("kg", "R$/kg MN"),
                                            ("t", "R$/tonelada MN"), ("embalagem",
                                            "R$/embalagem")] {
                                            option value=(v)
                                                selected[
                                                    self.valor(
                                                        &format!("unidade-preco-{id}")
                                                    ) == v
                                                ] {
                                                (rotulo)
                                            }
                                        }
                                    }
                                }
                                @for (prefixo, rotulo, dica) in [
                                    ("q", "Quantidade na base escolhida", "Ignorada ao formular"),
                                    ("preco", "Preço na unidade comercial",
                                    "Cotação decimal; zero precisa de justificativa"),
                                    ("embalagem", "Massa da embalagem (kg MN)",
                                        "Obrigatória para preço por embalagem"),
                                    ("ms", "MS neste cenário (%)",
                                    "Simula alteração sem editar o catálogo"),
                                    ("perda", "Perda anterior ao fornecimento (%)",
                                    "Entre 0 e 100% exclusive"),
                                    ("estoque", "Estoque bruto (kg MN)",
                                    "Vazio significa desconhecido"),
                                    ("min", "Inclusão mínima (kg MS)",
                                    "Positivo torna a inclusão obrigatória"),
                                    ("max", "Inclusão máxima (kg MS)",
                                    "Vazio: sem restrição adicional"),
                                    ("resolucao", "Resolução de oferta (kg MN/animal)",
                                        "Zero mantém a quantidade contínua"),
                                ] {
                                    @let nome = format!("{prefixo}-{id}");
                                    (Campo { erro: self.erro(&nome), nome: &nome, rotulo,
                                    valor: self.valor(&nome),
                                        tipo: "number", dica, obrigatorio: false })
                                }
                                @let nome = format!("origem-{id}");
                                (Campo { erro: self.erro(&nome), nome: &nome, rotulo:
                                "Origem e data da cotação",
                                    valor: self.valor(&nome), tipo: "text",
                                    dica: "Justifique também preços zero", obrigatorio: false })
                            }
                            label.check { input type="checkbox" name=(format!("livre-{id}"))
                                checked[self.valor(&format!("livre-{id}")) == "on"];
                                "Hipótese explícita: disponibilidade sem limite neste cenário"
                            }
                        }
                    }
                }
                section.card {
                    h2 { "3. Base da comparação" }
                    div.field { label for="base_comparacao" { "Base da simulação de MS" }
                        select #base_comparacao name="base_comparacao" {
                            @for v in ["Consumo de MS fixo", "Quantidade de MN fixa"] {
                                option selected[self.valor("base_comparacao") == v] { (v) }
                            }
                        }
                        small {
                        "Ao alterar MS em cenário duplicado, essa escolha governa a conversão." }
                    }
                    div.alert {
                    "O resultado avalia o perfil implementado. Fibra física, vitaminas e \
                        demais dimensões sem dados continuam identificadas como não avaliadas." }
                    div.form-actions {
                        a.button.secondary href="/" { "Voltar" }
                        button.button type="submit" {
                            "Calcular e salvar cenário →"
                        }
                    }
                }
            }
        }
    }
}

pub fn titulo() -> Markup {
    cabecalho(
        "Novo cenário",
        "Selecione o lote, o tipo de cálculo e os ingredientes.",
        "cenarios",
    )
}

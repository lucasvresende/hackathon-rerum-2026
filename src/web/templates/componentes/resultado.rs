//! Renderização do parecer e dos resultados de uma execução.
use maud::{Markup, Render, html};

use super::resultado_numeros as numeros;
use super::{cabecalho::Cabecalho, conceitos_resultado as conceitos};
use crate::dominio::Estado;
use crate::persistencia::historico::Execucao;
use crate::web::formatacao::{estado, numero, sinal};

pub struct ResultadoExecucao<'a> {
    pub execucao: &'a Execucao,
    pub token: &'a str,
}

impl Render for ResultadoExecucao<'_> {
    fn render(&self) -> Markup {
        let e = self.execucao;
        let p = &e.parecer;
        let c = &e.cenario;
        let sem_candidata = c.formular
            && !matches!(
                p.busca.as_deref(),
                Some("OTIMO_NO_MODELO" | "VIAVEL_SEM_OTIMO")
            );
        let data = chrono::DateTime::parse_from_rfc3339(&e.criada_em)
            .ok()
            .map(|d| {
                d.with_timezone(&chrono_tz::America::Sao_Paulo)
                    .format("%d/%m/%Y às %H:%M")
                    .to_string()
            })
            .unwrap_or(e.criada_em.clone());
        html! {
            (Cabecalho { titulo: &c.nome, texto: &format!("{} · {} · Execução {}",
            c.lote.nome, data, e.id), ajuda: "resultados" })
            div.actions.no-print {
                a.button href=(format!("/execucoes/{}/duplicar", e.id))
                target="_blank" rel="noopener" title="Abre em nova guia" {
                    "Duplicar e simular" }
                button.button.secondary type="button" data-imprimir { "Imprimir parecer" }
                form method="post" action=(format!("/execucoes/{}/reproduzir", e.id)) {
                    input type="hidden" name="csrf" value=(self.token);
                    button.button.secondary { "Reproduzir versões originais" }
                }
            }
            div class=(if sem_candidata { "alert error" } else { "alert" }) {
                @if sem_candidata {
                    strong { "Nenhuma combinação validada foi encontrada." }
                    " Custos e quantidades da entrada não representam uma solução de menor custo."
                } @else {
                    strong { (estado(&p.condicao)) }
                    " · Avaliação do perfil implementado; não é avaliação nutricional integral, "
                    "previsão de produção ou diagnóstico clínico."
                }
            }
            @if let Some(busca) = &p.busca {
                section.card {
                    div.concept-label.section-concept {
                        h2 data-busca=(busca) { "Formulação: " (estado(busca)) } (conceitos::OTIMO)
                    }
                    p { (&e.conhecimento.metodo) }
                    p.muted {
                        "Ótimo se refere ao modelo contínuo. Não prova mínimo global de \
                            ingredientes ou ótimo das quantidades arredondadas." }
                    @for s in &p.subproblemas {
                        p { strong { "FDN de forragem ≥ " (numero(s.limiar, 0)) "%: "
                            (estado(&s.estado)) }
                            " · " (s.motivo.replace("ProvenOptimal", "Ótimo comprovado")) }
                    }
                }
            }
            @let dados = html! {
            (numeros::Resumo { cenario: c, parecer: p })
            section.card {
                div.concept-label.section-concept {
                    h2 { "Resultados por dimensão" } (conceitos::ESTADOS)
                }
                div.dimension-grid {
                    @for nome in ["Dados", "Elegibilidade", "Núcleo químico",
                    "Barreiras ruminais",
                    "Ingredientes", "Contaminantes", "Inclusão e estoque",
                    "Fibra física e micronutrientes"] {
                        @let estado = p.dimensao(nome);
                        div class=(format!("dimension {}", if estado == "ATENDE" { "ok" }
                        else if estado == "NAO_ATENDE" { "bad" } else { "neutral" })) {
                            div.concept-label {
                                small { (nome) }
                                @if nome == "Dados" { (conceitos::DADOS) }
                                @else if nome == "Elegibilidade" { (conceitos::ELEGIBILIDADE) }
                                @else if nome == "Núcleo químico" { (conceitos::NUCLEO) }
                                @else if nome == "Barreiras ruminais" { (conceitos::BARREIRAS) }
                                @else if nome == "Ingredientes" { (conceitos::INGREDIENTES) }
                                @else if nome == "Contaminantes" { (conceitos::CONTAMINANTES) }
                                @else if nome == "Inclusão e estoque" {
                                    (conceitos::INCLUSAO_ESTOQUE)
                                }
                                @else if nome == "Fibra física e micronutrientes" {
                                    (conceitos::FIBRA_FISICA)
                                }
                            }
                            strong { (crate::web::formatacao::estado(estado)) }
                        }
                    }
                }
                @for aviso in &p.avisos { p.alert { (aviso) } }
            }
            (numeros::Balanco(p))
            (numeros::Fornecimento(p))
            (numeros::Economia { cenario: c, parecer: p })
            };
            @if sem_candidata {
                (super::expansivel::Expansivel { id: "diagnostico-entrada", aberto: false,
                    titulo: html! { "Diagnóstico das quantidades de entrada (não é solução)" },
                    conteudo: dados })
            } @else {
                @if c.formular {
                    h2 { "Resultados da solução contínua" }
                }
                (dados)
                @if let Some(pratico) = &p.pratico {
                    (super::resultado_pratico::ResultadoPratico(pratico))
                }
            }
            (super::expansivel::Expansivel { id: "calculos-regras",
                aberto: false, titulo: html! { "Cálculos e regras aplicadas" }, conteudo: html! {
                p.muted { "Entradas → normalização → cálculo → comparação → conclusão. \
                        Todas as verificações independentes são preservadas." }
                ol { @for etapa in &p.caminho { li { (etapa) } } }
                @for (indice, v) in p.verificacoes.iter().enumerate() {
                    (super::expansivel::Expansivel { id: &format!("regra-{indice}-{}", v.regra),
                        aberto:
                        false, titulo: html! { span class=(if v.estado == Estado::Viola {
                                "pill danger" }
                            else { "pill" }) { (v.estado.rotulo()) }
                            " " strong { (&v.regra) } " · " (&v.dimensao)
                        }, conteudo: html! {
                            p { (&v.explicacao) }
                            @if let (Some(valor), Some(limite)) = (v.valor, v.limite) {
                                p.code { "Calculado: " (numero(valor, 8)) " · limite: "
                                    (numero(limite, 8)) " · diferença: "
                                    (sinal(valor-limite, 8)) " " (&v.unidade) }
                            }
                            p.muted { "Fonte: " (&v.fonte) " · perfil "
                                (&e.conhecimento.perfil_id) }
                            p.muted { "Dependências: " (v.dependencias.join(" → ")) }
                        }, })
                }
            }, })
            (super::expansivel::Expansivel { id: "entradas-preservadas",
                aberto: false, titulo: html! { "Entradas preservadas nesta execução" },
                conteudo: html! { (super::entradas::Entradas(e)) }, })
            (super::expansivel::Expansivel { id: "fontes-versoes",
                aberto: false, titulo: html! { "Fontes e versões preservadas" }, conteudo: html! {
                p.code { (&e.conhecimento.perfil_id) " · versão " (&e.conhecimento.versao)
                    " · aplicação " (&e.conhecimento.versao_app) }
                @for f in &e.conhecimento.fontes {
                    (super::expansivel::Expansivel { id: &format!("fonte-{}", f.id), aberto:
                        false, titulo: html! { (&f.id) " · " (&f.titulo) }, conteudo:
                        html! {
                            p { (&f.edicao) " · " (&f.localizador) }
                            p { (&f.conteudo) } p.code { "SHA-256 " (&f.hash) }
                        }, })
                }
            }, })
        }
    }
}

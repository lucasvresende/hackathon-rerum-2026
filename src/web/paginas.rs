use std::collections::BTreeMap;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use maud::{Markup, Render, html};

use super::componentes::estrutura::{Pagina, cabecalho};
use super::rotas::{ErroWeb, EstadoApp};
use crate::dominio::unidades::dinheiro;
use crate::persistencia::{cadastros, conhecimento as base, historico as banco};

pub async fn inicio(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    let lotes = cadastros::lotes(&e.pool).await?;
    let alimentos = cadastros::alimentos(&e.pool).await?;
    let execucoes = banco::listar(&e.pool).await?;
    let conhecimento = base::ler(&e.pool).await?;
    Ok(Pagina {
        titulo: "Visão geral",
        ativo: "/",
        ajuda: "inicio",
        conteudo: super::componentes::inicio::Inicio {
            lotes: &lotes,
            alimentos: &alimentos,
            execucoes: &execucoes,
            conhecimento: &conhecimento,
        }
        .render(),
    }
    .render())
}

pub async fn lotes(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    let lotes = cadastros::lotes(&e.pool).await?;
    Ok(Pagina {
        titulo: "Lotes",
        ativo: "/lotes",
        ajuda: "lotes",
        conteudo: html! {
            (cabecalho("Lotes", ""))
            div.actions { a.button href="/lotes/novo" { "+ Cadastrar lote" } }
            section.card.table-wrap {
                @if lotes.is_empty() { p.empty {
                "Nenhum lote cadastrado." } }
                @else {
                    table { thead { tr { th { "Lote" } th { "Animais" } th { "Peso (kg)" }
                        th { "Leite (kg/animal/dia)" } th { "Dias em lactação" } th {} } }
                        tbody { @for l in lotes { tr { td { strong { (&l.nome) } }
                            td { (l.animais) } td { (l.peso) } td { (l.producao) } td {
                            (l.dias_lactacao) }
                            td { a href=(format!("/lotes/{}/editar", l.id)) { "Revisar ↗" } }
                        } } }
                    }
                }
            }
        },
    }
    .render())
}

pub async fn alimentos(
    State(e): State<EstadoApp>,
    Query(q): Query<BTreeMap<String, String>>,
) -> Result<Markup, ErroWeb> {
    let alimentos = cadastros::alimentos(&e.pool).await?;
    let busca = q.get("busca").cloned().unwrap_or_default();
    Ok(Pagina {
        titulo: "Alimentos",
        ativo: "/alimentos",
        ajuda: "alimentos",
        conteudo: html! {
            (cabecalho("Catálogo de alimentos", ""))
            div.actions { a.button href="/alimentos/novo" { "+ Cadastrar composição" }
                form.search method="get" action="/alimentos" {
                    label.sr-only for="busca" { "Pesquisar alimento" }
                    input #busca name="busca" value=(&busca) placeholder="Pesquisar alimento…";
                    button.button.secondary { "Pesquisar" }
                }
            }
            section.food-grid {
                @for a in alimentos.iter().filter(|a|
                a.nome.to_lowercase().contains(&busca.to_lowercase())) {
                    article.card.food-card {
                        span.pill { (&a.natureza) }
                        h2 { (&a.nome) }
                        p.muted { (format!("{:?}", a.classe)) " · fonte " (&a.fonte_id) }
                        dl { dt { "Matéria seca" } dd { (format!("{:.1}% MN", a.ms * 100.0)) }
                            @for (i, nome) in ["NDT", "PB", "Ca", "P", "FDN", "Amido", "EE"]
                                .iter().enumerate() {
                                dt { (nome) } dd { (a.nutrientes[i].map(|v| format!("{:.2}% MS",
                                v*100.0))
                                    .unwrap_or("Desconhecido".into())) }
                            }
                        }
                        details { summary { "Procedência e condições" }
                            p { (&a.origem) " · " (&a.metodo) " · " (&a.data) }
                            p { (&a.evidencia) }
                            p.code { "Versão " (&a.composicao_id) }
                        }
                        a.text-link href=(format!("/alimentos/{}/revisar", a.composicao_id)) {
                            "Criar revisão ou nova análise ↗"
                        }
                    }
                }
            }
        },
    }
    .render())
}

pub async fn historico(
    State(e): State<EstadoApp>,
    Query(q): Query<BTreeMap<String, String>>,
) -> Result<Markup, ErroWeb> {
    let pagina = q
        .get("pagina")
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(1)
        .clamp(1, 1000000);
    let busca = q.get("lote").cloned().unwrap_or_default();
    let data = q.get("data").cloned().unwrap_or_default();
    let filtro = (!busca.is_empty()).then(|| format!("%{busca}%"));
    let dia = chrono::NaiveDate::parse_from_str(&data, "%Y-%m-%d").ok();
    let instante = |dia: chrono::NaiveDate| {
        dia.and_hms_opt(0, 0, 0)
            .and_then(|n| {
                n.and_local_timezone(chrono_tz::America::Sao_Paulo)
                    .earliest()
            })
            .map(|v| v.with_timezone(&chrono::Utc).to_rfc3339())
    };
    let inicio = dia.and_then(instante);
    let fim = dia.and_then(|d| d.succ_opt()).and_then(instante);
    let execucoes = banco::filtrar(
        &e.pool,
        filtro.as_deref(),
        inicio.as_deref(),
        fim.as_deref(),
        (pagina - 1) * 100,
    )
    .await?;
    Ok(Pagina {
        titulo: "Histórico",
        ativo: "/historico",
        ajuda: "historico",
        conteudo: html! {
            (cabecalho("Histórico de cenários", ""))
            form.card method="get" action="/historico" {
                div.form-grid {
                    div.field { label for="filtro-lote" { "Nome do lote" }
                        input #filtro-lote name="lote" value=(&busca);
                    }
                    div.field { label for="filtro-data" { "Data da execução (São Paulo)" }
                        input #filtro-data name="data" type="date" value=(&data);
                    }
                }
                button.button.secondary { "Filtrar histórico" }
            }
            div.actions {
                @if pagina>1 { a.button.secondary href=(format!("/historico?pagina={}",pagina-1)) {
                    "← Página anterior" } }
                span.muted { "Página " (pagina) " · até 100 resultados por página" }
                @if execucoes.len()==100 { a.button.secondary
                    href=(format!("/historico?pagina={}",pagina+1)) { "Próxima página →" } }
            }
            form.card method="get" action="/comparar" {
                h2 { "Comparar duas execuções" }
                div.form-grid {
                    @for (nome, rotulo) in [("a", "Cenário de referência"), ("b",
                    "Cenário alternativo")] {
                        div.field { label for=(nome) { (rotulo) }
                            select id=(nome) name=(nome) required {
                                @for execucao in &execucoes {
                                    option value=(&execucao.id) { (&execucao.cenario.nome)
                                        " · " (&execucao.id[..8]) }
                                }
                            }
                        }
                    }
                }
                button.button.secondary { "Comparar premissas e custos" }
            }
            section.card.table-wrap {
                table { thead { tr { th { "Cenário" } th { "Lote" } th { "Estado" }
                    th { "Custo / animal / dia" } th {} } }
                    tbody { @for x in &execucoes {
                        tr { td { (&x.cenario.nome) } td { (&x.cenario.lote.nome) }
                            td { span.pill { (&x.parecer.condicao) } }
                            td { (x.parecer.custo.map(dinheiro).unwrap_or("—".into())) }
                            td { a href=(format!("/execucoes/{}", x.id)) { "Ver parecer ↗" } }
                        }
                    } }
                }
                @if execucoes.is_empty() { p.empty { "Nenhum cenário calculado ainda." } }
            }
        },
    }
    .render())
}

pub async fn conhecimento(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    let c = base::ler(&e.pool).await?;
    Ok(Pagina {
        titulo: "Perfil nutricional e fontes",
        ativo: "/conhecimento",
        ajuda: "conhecimento",
        conteudo: html! {
            (cabecalho("Perfil nutricional e fontes", ""))
            section.card { h2 { (&c.perfil_id) }
                p { "Adultas em lactação, confinadas, estáveis e sem gestação/crescimento. \
                O perfil histórico NRC 1989 não implementa integralmente NASEM 2021 ou
                BR-LEITE 2024." }
                p { "PV 400–700 kg; referência de consumo apenas até 600 kg; gordura 3–5%; \
                    DEL 60–250; LCG4 5–30 kg/dia." }
            }
            section.card.table-wrap { h2 { "Coeficientes do perfil" }
                table { thead { tr { @for t in ["Tabela", "Eixo", "Eixo 2", "A", "B", "C", "D",
                "Unidade"] {
                    th { (t) } } } }
                    tbody { @for r in &c.coeficientes { tr {
                        td { (&r.tabela) } td { (r.eixo) } td { (r.eixo2) }
                        @for v in r.valores { td { (v) } } td { (&r.unidade) }
                    } } }
                }
            }
            section.card { h2 { "Fontes preservadas" }
                @for f in &c.fontes { details { summary { span.pill { (&f.natureza) } " "
                    (&f.id) " · " (&f.titulo) } p { (&f.edicao) " · " (&f.localizador) }
                    p { (&f.conteudo) } p.code { (&f.hash) }
                } }
            }
            section.card { h2 { "Normas, vigência e condições" }
                @for n in &c.normas { details { summary { (&n.id) " · " (&n.natureza) }
                    p { "Início: " (&n.inicio) " · fim exclusivo: "
                        (n.fim.as_deref().unwrap_or("Sem fim registrado")) }
                    p { "Transição: " (n.transicao.as_deref().unwrap_or("Não informada")) }
                    p { (&n.condicoes) } p { "Exceções: " (&n.excecoes) }
                    p { "Evidência: " (&n.evidencia) } p { "Ação: " (&n.acao) }
                } }
            }
        },
    }
    .render())
}

pub async fn nao_encontrado() -> (StatusCode, Markup) {
    (
        StatusCode::NOT_FOUND,
        Pagina {
            titulo: "Página não encontrada",
            ativo: "",
            ajuda: "inicio",
            conteudo: html! { h1 { "Página não encontrada" } a href="/" { "Voltar ao início" } },
        }
        .render(),
    )
}

/// Ajuda estática disponível sem consultar cadastros ou execuções.
pub async fn ajuda() -> Markup {
    Pagina {
        titulo: "Ajuda",
        ativo: "/ajuda",
        ajuda: "inicio",
        conteudo: super::componentes::ajuda::Ajuda.render(),
    }
    .render()
}

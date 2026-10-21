use std::collections::BTreeMap;

use axum::Form;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use maud::{Markup, Render, html};

use super::componentes::cenario::{FormularioCenario, titulo};
use super::componentes::estrutura::Pagina;
use super::formularios::{self, Campos};
use super::rotas::{ErroWeb, EstadoApp, autorizado, redirecionar};
use crate::aplicacao::{demonstracao, execucao};
use crate::persistencia::{cadastros, conhecimento, historico};

async fn formulario(
    e: &EstadoApp,
    campos: &Campos,
    erros: &[crate::dominio::validacao::Problema],
    parcial: bool,
) -> Result<Markup, ErroWeb> {
    let lotes = cadastros::lotes(&e.pool).await?;
    let alimentos = cadastros::alimentos(&e.pool).await?;
    let formulario = FormularioCenario {
        campos,
        lotes: &lotes,
        alimentos: &alimentos,
        erros,
        token: &e.csrf,
    }
    .render();
    if parcial {
        return Ok(formulario);
    }
    Ok(Pagina {
        titulo: "Novo cenário",
        ativo: "cadastro",
        conteudo: html! {
            (titulo())
            @if lotes.is_empty() { div.alert { "Cadastre um lote antes de calcular." } }
            (formulario)
        },
    }
    .render())
}

pub async fn novo(
    State(e): State<EstadoApp>,
    Query(q): Query<BTreeMap<String, String>>,
) -> Result<Markup, ErroWeb> {
    let lotes = cadastros::lotes(&e.pool).await?;
    let alimentos = cadastros::alimentos(&e.pool).await?;
    let mut campos = if let Some(lote) = lotes
        .iter()
        .find(|l| l.nome == "Lote demonstração")
        .filter(|_| q.get("dieta").is_some_and(|v| v == "A" || v == "B"))
    {
        let mut selecionados = std::collections::BTreeSet::new();
        let alimentos = alimentos
            .iter()
            .filter(|a| {
                ["silagem", "milho", "soja", "mineral"].contains(&a.id.as_str())
                    && a.natureza == "SINTETICA"
                    && selecionados.insert(a.id.clone())
            })
            .cloned()
            .collect();
        formularios::campos_cenario(&demonstracao::cenario(
            lote.clone(),
            alimentos,
            q.get("dieta").is_some_and(|d| d == "A"),
        ))
    } else {
        Campos::new()
    };
    if q.get("modo").is_some_and(|v| v == "formular") {
        campos.insert("modo".into(), "formular".into());
    }
    completar_campos(&mut campos, &alimentos);
    formulario(&e, &campos, &[], false).await
}

pub async fn executar(
    State(e): State<EstadoApp>,
    headers: HeaderMap,
    Form(c): Form<Campos>,
) -> Result<Response, ErroWeb> {
    if !autorizado(&e, &headers, &c) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }
    let lotes = cadastros::lotes(&e.pool).await?;
    let alimentos = cadastros::alimentos(&e.pool).await?;
    let erros = match formularios::ler_cenario(&c, &lotes, &alimentos) {
        Ok(mut cenario) => {
            let base = conhecimento::ler(&e.pool).await?;
            cenario.perfil_id = base.perfil_id.clone();
            match execucao::executar(&e.pool, &cenario, &base).await {
                Ok(id) => return Ok(redirecionar(&format!("/execucoes/{id}"), &headers)),
                Err(erro) => {
                    tracing::error!(%erro, "Falha ao executar cenário");
                    vec![crate::dominio::validacao::problema(
                        "execução",
                        "Falha operacional no cálculo ou persistência. Tente novamente.",
                    )]
                }
            }
        }
        Err(erros) => erros,
    };
    Ok(
        formulario(&e, &c, &erros, headers.contains_key("hx-request"))
            .await?
            .into_response(),
    )
}

pub async fn resultado(
    State(e): State<EstadoApp>,
    Path(id): Path<String>,
) -> Result<Markup, ErroWeb> {
    let execucao = historico::obter(&e.pool, &id).await?;
    Ok(Pagina {
        titulo: &execucao.cenario.nome,
        ativo: "/historico",
        conteudo: super::componentes::resultado::ResultadoExecucao {
            execucao: &execucao,
            token: &e.csrf,
        }
        .render(),
    }
    .render())
}

pub async fn duplicar(
    State(e): State<EstadoApp>,
    Path(id): Path<String>,
) -> Result<Markup, ErroWeb> {
    let anterior = historico::obter(&e.pool, &id).await?;
    let mut c = anterior.cenario;
    c.original_id = Some(anterior.cenario_id);
    c.nome = format!("{} · simulação", c.nome);
    formulario(&e, &formularios::campos_cenario(&c), &[], false).await
}

pub async fn reproduzir(
    State(e): State<EstadoApp>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Form(c): Form<Campos>,
) -> Result<Response, ErroWeb> {
    if !autorizado(&e, &headers, &c) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }
    let anterior = historico::obter(&e.pool, &id).await?;
    let mut cenario = anterior.cenario;
    cenario.original_id = Some(anterior.cenario_id);
    let novo = execucao::executar(&e.pool, &cenario, &anterior.conhecimento).await?;
    Ok(redirecionar(&format!("/execucoes/{novo}"), &headers))
}

/// Acrescenta opções novas sem sobrescrever o preenchimento de um cenário em elaboração.
fn completar_campos(campos: &mut Campos, alimentos: &[crate::dominio::Alimento]) {
    for a in alimentos {
        campos
            .entry(format!("ms-{}", a.composicao_id))
            .or_insert_with(|| (a.ms * 100.0).to_string());
        campos
            .entry(format!("unidade-preco-{}", a.composicao_id))
            .or_insert("kg".into());
    }
}

pub async fn atualizar(
    State(e): State<EstadoApp>,
    headers: HeaderMap,
    Form(mut c): Form<Campos>,
) -> Result<Response, ErroWeb> {
    if !autorizado(&e, &headers, &c) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }
    completar_campos(&mut c, &cadastros::alimentos(&e.pool).await?);
    Ok(formulario(&e, &c, &[], headers.contains_key("hx-request"))
        .await?
        .into_response())
}

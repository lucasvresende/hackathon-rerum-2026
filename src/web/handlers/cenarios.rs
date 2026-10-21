use crate::aplicacao::{demonstracao, execucao};
use crate::persistencia::{cadastros, conhecimento};
use crate::web::entradas::Campos;
use crate::web::{apresentacao, entradas as formularios};
use crate::web::{
    erro::ErroWeb,
    estado::EstadoApp,
    resposta::{autorizado, redirecionar},
};
use axum::{
    Form,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use maud::Markup;
use std::collections::BTreeMap;

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
    apresentacao::cenario(&e, &campos, &[], false).await
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
        apresentacao::cenario(&e, &c, &erros, headers.contains_key("hx-request"))
            .await?
            .into_response(),
    )
}

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
    Ok(
        apresentacao::cenario(&e, &c, &[], headers.contains_key("hx-request"))
            .await?
            .into_response(),
    )
}

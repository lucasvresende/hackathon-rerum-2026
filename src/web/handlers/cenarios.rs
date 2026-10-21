use crate::aplicacao::cenarios::{self as servico, DietaDemonstracao, FalhaCenario};
use crate::persistencia::cadastros;
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
    let opcoes = servico::opcoes(&e.pool).await?;
    let dieta = match q.get("dieta").map(String::as_str) {
        Some("A") => Some(DietaDemonstracao::A),
        Some("B") => Some(DietaDemonstracao::B),
        _ => None,
    };
    let mut campos = dieta
        .and_then(|dieta| servico::exemplo(&opcoes, dieta))
        .map(|c| formularios::campos_cenario(&c))
        .unwrap_or_default();
    if q.get("modo").is_some_and(|v| v == "formular") {
        campos.insert("modo".into(), "formular".into());
    }
    formularios::completar_cenario(&mut campos, &opcoes.alimentos);
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
        Ok(cenario) => match servico::executar_novo(&e.pool, cenario).await {
            Ok(id) => return Ok(redirecionar(&format!("/execucoes/{id}"), &headers)),
            Err(FalhaCenario::Conhecimento(erro)) => return Err(ErroWeb(erro)),
            Err(FalhaCenario::Execucao(erro)) => {
                tracing::error!(%erro, "Falha ao executar cenário");
                vec![crate::dominio::validacao::problema(
                    "execução",
                    "Falha operacional no cálculo ou persistência. Tente novamente.",
                )]
            }
        },
        Err(erros) => erros,
    };
    Ok(
        apresentacao::cenario(&e, &c, &erros, headers.contains_key("hx-request"))
            .await?
            .into_response(),
    )
}

pub async fn atualizar(
    State(e): State<EstadoApp>,
    headers: HeaderMap,
    Form(mut c): Form<Campos>,
) -> Result<Response, ErroWeb> {
    if !autorizado(&e, &headers, &c) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }
    formularios::completar_cenario(&mut c, &cadastros::alimentos(&e.pool).await?);
    Ok(
        apresentacao::cenario(&e, &c, &[], headers.contains_key("hx-request"))
            .await?
            .into_response(),
    )
}

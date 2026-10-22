//! Handler HTTP para cenarios, mantendo a renderização desta responsabilidade isolada.
use crate::aplicacao::cenarios::{self as servico, DietaDemonstracao, FalhaCenario};
use crate::persistencia::cadastros;
use crate::web::entradas::Campos;
use crate::web::{apresentacao, entradas as formularios};
use crate::web::{erro::ErroWeb, estado::EstadoApp, resposta::redirecionar, seguranca::autorizado};
use axum::{
    Form,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use axum::{
    Router,
    routing::{get, post},
};
use maud::Markup;
use std::collections::BTreeMap;

/// Rotas HTTP deste recurso para composição no roteador principal.
pub fn rotas() -> Router<EstadoApp> {
    Router::new()
        .route("/cenarios/atualizar", post(atualizar))
        .route("/cenarios/exemplo", post(carregar_exemplo))
        .route("/cenarios/novo", get(novo))
        .route("/cenarios", post(executar))
}

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
        .unwrap_or_else(formularios::padroes_cenario);
    if q.get("exemplo").is_some_and(|v| v == "inicial") {
        let c = crate::aplicacao::exemplos::cenario_inicial(&e.pool).await?;
        campos = formularios::campos_cenario(&c);
        // Limite demonstrativo do suplemento precisa acompanhar o formulário.
        for i in &c.itens {
            if let Some(maximo) = i.maximo_ms {
                campos.insert(
                    format!("max-{}", i.alimento.composicao_id),
                    maximo.to_string(),
                );
            }
        }
    }
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

/// A primeira etapa preserva o rascunho e exige confirmação explícita, inclusive sem JS.
pub async fn carregar_exemplo(
    State(e): State<EstadoApp>,
    Query(q): Query<BTreeMap<String, String>>,
    headers: HeaderMap,
    Form(mut campos): Form<Campos>,
) -> Result<Response, ErroWeb> {
    if !autorizado(&e, &headers, &campos) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }
    let dieta = match q.get("dieta").map(String::as_str) {
        Some("A") => DietaDemonstracao::A,
        Some("B") => DietaDemonstracao::B,
        _ => return Ok(StatusCode::BAD_REQUEST.into_response()),
    };
    let mut erros = vec![];
    if q.get("acao").is_some_and(|a| a == "cancelar") {
        campos.remove("exemplo_pendente");
        campos.remove("confirmar_exemplo");
    } else if q.get("acao").is_some_and(|a| a == "aplicar")
        && campos.get("confirmar_exemplo").is_some_and(|v| v == "on")
    {
        let opcoes = servico::opcoes(&e.pool).await?;
        if let Some(mut exemplo) = servico::exemplo(&opcoes, dieta) {
            exemplo.formular = campos.get("modo").is_some_and(|v| v == "formular");
            campos = formularios::campos_cenario(&exemplo);
            formularios::completar_cenario(&mut campos, &opcoes.alimentos);
        } else {
            erros.push(crate::dominio::validacao::problema(
                "exemplo_pendente",
                "Cadastros da demonstração indisponíveis; seu rascunho foi preservado.",
            ));
        }
    } else {
        campos.insert("exemplo_pendente".into(), q["dieta"].clone());
        if q.get("acao").is_some_and(|a| a == "aplicar") {
            erros.push(crate::dominio::validacao::problema(
                "confirmar_exemplo",
                "Confirme a substituição do rascunho antes de preencher com o exemplo.",
            ));
        }
    }
    Ok(
        apresentacao::cenario(&e, &campos, &erros, headers.contains_key("hx-request"))
            .await?
            .into_response(),
    )
}

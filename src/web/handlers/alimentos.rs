use crate::persistencia::{cadastros, cadastros as banco};
use crate::web::entradas::Campos;
use crate::web::{apresentacao, entradas as formularios};
use crate::web::{
    erro::ErroWeb,
    estado::EstadoApp,
    resposta::{autorizado, redirecionar},
};
use axum::{
    Form,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use maud::{Markup, Render};
use std::collections::BTreeMap;

pub async fn listar(
    State(e): State<EstadoApp>,
    Query(q): Query<BTreeMap<String, String>>,
) -> Result<Markup, ErroWeb> {
    let alimentos = cadastros::alimentos(&e.pool).await?;
    let busca = q.get("busca").cloned().unwrap_or_default();
    Ok(crate::web::templates::paginas::alimentos::Alimentos {
        alimentos: &alimentos,
        busca: &busca,
    }
    .render())
}

pub async fn novo(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    apresentacao::cadastro("alimento", &Campos::new(), &[], &e, false).await
}

pub async fn revisar(
    State(e): State<EstadoApp>,
    Path(id): Path<String>,
) -> Result<Markup, ErroWeb> {
    let a = banco::alimentos(&e.pool)
        .await?
        .into_iter()
        .find(|a| a.composicao_id == id)
        .ok_or_else(|| ErroWeb("Composição não encontrada".into()))?;
    apresentacao::cadastro(
        "alimento",
        &formularios::campos_alimento(&a),
        &[],
        &e,
        false,
    )
    .await
}

pub async fn salvar(
    State(e): State<EstadoApp>,
    headers: HeaderMap,
    Form(c): Form<Campos>,
) -> Result<Response, ErroWeb> {
    if !autorizado(&e, &headers, &c) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }
    let erros = match formularios::ler_alimento(&c) {
        Ok(mut a) => {
            match super::modelos_preenchimento::procedencia(
                &e,
                &c,
                &formularios::campos_alimento(&a),
                "alimento",
            )
            .await
            {
                Ok(p) => a.procedencia = p,
                Err(erro) => {
                    return Ok(apresentacao::cadastro(
                        "alimento",
                        &c,
                        &[erro],
                        &e,
                        headers.contains_key("hx-request"),
                    )
                    .await?
                    .into_response());
                }
            }
            match banco::salvar_alimento(&e.pool, &mut a).await {
                Ok(()) => return Ok(redirecionar("/cadastros/salvo?tipo=alimento", &headers)),
                Err(erro) => {
                    tracing::error!(%erro, "Erro ao salvar composição");
                    vec![crate::dominio::validacao::problema(
                        "fonte_id",
                        "Não foi possível salvar. Confira a fonte e os dados.",
                    )]
                }
            }
        }
        Err(erros) => erros,
    };
    Ok(apresentacao::cadastro(
        "alimento",
        &c,
        &erros,
        &e,
        headers.contains_key("hx-request"),
    )
    .await?
    .into_response())
}

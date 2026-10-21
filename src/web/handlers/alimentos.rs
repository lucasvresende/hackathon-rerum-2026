use crate::aplicacao::cadastros::{self as servico, FalhaCadastro};
use crate::dominio::templates::TipoCadastro;
use crate::web::entradas::Campos;
use crate::web::{apresentacao, entradas as formularios};
use crate::web::{erro::ErroWeb, estado::EstadoApp, resposta::redirecionar, seguranca::autorizado};
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
    let busca = q.get("busca").cloned().unwrap_or_default();
    let alimentos = servico::pesquisar_alimentos(&e.pool, &busca).await?;
    Ok(crate::web::templates::paginas::alimentos::Alimentos {
        alimentos: &alimentos,
        busca: &busca,
    }
    .render())
}

pub async fn novo(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    apresentacao::cadastro(TipoCadastro::Alimento, &Campos::new(), &[], &e, false).await
}

pub async fn revisar(
    State(e): State<EstadoApp>,
    Path(id): Path<String>,
) -> Result<Markup, ErroWeb> {
    let a = servico::obter_alimento(&e.pool, &id).await?;
    apresentacao::cadastro(
        TipoCadastro::Alimento,
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
            match servico::salvar_alimento(
                &e.pool,
                &mut a,
                formularios::texto(&c, "modelo_aplicado"),
            )
            .await
            {
                Ok(()) => return Ok(redirecionar("/cadastros/salvo?tipo=alimento", &headers)),
                Err(FalhaCadastro::Referencia(erro)) => vec![erro],
                Err(FalhaCadastro::Persistencia(erro)) => {
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
        TipoCadastro::Alimento,
        &c,
        &erros,
        &e,
        headers.contains_key("hx-request"),
    )
    .await?
    .into_response())
}

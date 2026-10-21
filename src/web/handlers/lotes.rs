use crate::aplicacao::cadastros::{self as servico, FalhaCadastro};
use crate::dominio::templates::TipoCadastro;
use crate::persistencia::cadastros;
use crate::web::entradas::Campos;
use crate::web::{apresentacao, entradas as formularios};
use crate::web::{erro::ErroWeb, estado::EstadoApp, resposta::redirecionar, seguranca::autorizado};
use axum::{
    Form,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use axum::{Router, routing::get};
use maud::{Markup, Render};

/// Rotas HTTP deste recurso para composição no roteador principal.
pub fn rotas() -> Router<EstadoApp> {
    Router::new()
        .route("/lotes", get(listar).post(salvar))
        .route("/lotes/novo", get(novo))
        .route("/lotes/{id}/editar", get(editar))
}

pub async fn listar(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    let lotes = cadastros::lotes(&e.pool).await?;
    Ok(crate::web::templates::paginas::lotes::Lotes { lotes: &lotes }.render())
}

pub async fn novo(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    apresentacao::cadastro(TipoCadastro::Lote, &Campos::new(), &[], &e, false).await
}

pub async fn editar(State(e): State<EstadoApp>, Path(id): Path<String>) -> Result<Markup, ErroWeb> {
    let lote = servico::obter_lote(&e.pool, &id).await?;
    apresentacao::cadastro(
        TipoCadastro::Lote,
        &formularios::campos_lote(&lote),
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
    let erros = match formularios::ler_lote(&c) {
        Ok(mut lote) => {
            match servico::salvar_lote(
                &e.pool,
                &mut lote,
                formularios::texto(&c, "id").is_empty(),
                formularios::texto(&c, "modelo_aplicado"),
            )
            .await
            {
                Ok(()) => return Ok(redirecionar("/cadastros/salvo?tipo=lote", &headers)),
                Err(FalhaCadastro::Referencia(erro)) => vec![erro],
                Err(FalhaCadastro::Persistencia(erro)) => {
                    tracing::error!(%erro, "Erro ao salvar lote");
                    vec![crate::dominio::validacao::problema(
                        "cadastro",
                        "Não foi possível salvar.",
                    )]
                }
            }
        }
        Err(erros) => erros,
    };
    Ok(apresentacao::cadastro(
        TipoCadastro::Lote,
        &c,
        &erros,
        &e,
        headers.contains_key("hx-request"),
    )
    .await?
    .into_response())
}

use axum::Form;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use maud::{Markup, Render};

use super::entradas::{self as formularios, Campos};
use super::rotas::{ErroWeb, EstadoApp, autorizado, redirecionar};
use super::templates::formularios::cadastro::Cadastro;
use crate::persistencia::cadastros as banco;

pub(super) async fn formulario(
    tipo: &str,
    campos: &Campos,
    erros: &[crate::dominio::validacao::Problema],
    estado: &EstadoApp,
    parcial: bool,
) -> Result<Markup, ErroWeb> {
    let modelos = crate::persistencia::templates::ler(
        &mut *estado.pool.acquire().await.map_err(|e| ErroWeb(e.into()))?,
    )
    .await?;
    let cadastro = Cadastro {
        modelos: &modelos,
        tipo,
        campos,
        erros,
        token: &estado.csrf,
    }
    .render();
    if parcial {
        return Ok(cadastro);
    }
    Ok(super::templates::paginas::cadastro::PaginaCadastro {
        tipo,
        formulario: cadastro,
    }
    .render())
}

pub async fn novo_lote(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    formulario("lote", &Campos::new(), &[], &e, false).await
}
pub async fn novo_alimento(State(e): State<EstadoApp>) -> Result<Markup, ErroWeb> {
    formulario("alimento", &Campos::new(), &[], &e, false).await
}

pub async fn editar_lote(
    State(e): State<EstadoApp>,
    Path(id): Path<String>,
) -> Result<Markup, ErroWeb> {
    let lote = banco::lotes(&e.pool)
        .await?
        .into_iter()
        .find(|l| l.id == id)
        .ok_or_else(|| ErroWeb("Lote não encontrado".into()))?;
    formulario("lote", &formularios::campos_lote(&lote), &[], &e, false).await
}

pub async fn revisar_alimento(
    State(e): State<EstadoApp>,
    Path(id): Path<String>,
) -> Result<Markup, ErroWeb> {
    let a = banco::alimentos(&e.pool)
        .await?
        .into_iter()
        .find(|a| a.composicao_id == id)
        .ok_or_else(|| ErroWeb("Composição não encontrada".into()))?;
    formulario(
        "alimento",
        &formularios::campos_alimento(&a),
        &[],
        &e,
        false,
    )
    .await
}

pub async fn salvar_lote(
    State(e): State<EstadoApp>,
    headers: HeaderMap,
    Form(c): Form<Campos>,
) -> Result<Response, ErroWeb> {
    if !autorizado(&e, &headers, &c) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }
    let erros = match formularios::ler_lote(&c) {
        Ok(mut lote) => {
            match super::modelos_preenchimento::procedencia(
                &e,
                &c,
                &formularios::campos_lote(&lote),
                "lote",
            )
            .await
            {
                Ok(p) => lote.procedencia = p,
                Err(erro) => {
                    return Ok(formulario(
                        "lote",
                        &c,
                        &[erro],
                        &e,
                        headers.contains_key("hx-request"),
                    )
                    .await?
                    .into_response());
                }
            }
            let novo = formularios::texto(&c, "id").is_empty();
            match banco::salvar_lote(&e.pool, &mut lote, novo).await {
                Ok(()) => return Ok(redirecionar("/cadastros/salvo?tipo=lote", &headers)),
                Err(erro) => {
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
    Ok(
        formulario("lote", &c, &erros, &e, headers.contains_key("hx-request"))
            .await?
            .into_response(),
    )
}

pub async fn salvar_alimento(
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
                    return Ok(formulario(
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
    Ok(formulario(
        "alimento",
        &c,
        &erros,
        &e,
        headers.contains_key("hx-request"),
    )
    .await?
    .into_response())
}

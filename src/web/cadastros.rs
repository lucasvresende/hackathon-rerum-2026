use axum::Form;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use maud::{Markup, Render, html};

use super::componentes::cadastro::Cadastro;
use super::componentes::estrutura::{Pagina, cabecalho};
use super::formularios::{self, Campos};
use super::rotas::{ErroWeb, EstadoApp, autorizado, redirecionar};
use crate::persistencia::cadastros as banco;

fn formulario(
    tipo: &str,
    campos: &Campos,
    erros: &[crate::dominio::validacao::Problema],
    estado: &EstadoApp,
    parcial: bool,
) -> Markup {
    let cadastro = Cadastro {
        tipo,
        campos,
        erros,
        token: &estado.csrf,
    }
    .render();
    if parcial {
        return cadastro;
    }
    Pagina {
        titulo: "Cadastro",
        ajuda: if tipo == "lote" { "lotes" } else { "alimentos" },
        ativo: if tipo == "lote" {
            "/lotes"
        } else {
            "/alimentos"
        },
        conteudo: html! {
            (cabecalho(if tipo == "lote" { "Cadastro de lote" }
                else { "Composição do alimento" },
                "Cada alteração cria uma revisão. Execuções anteriores permanecem iguais."))
            (cadastro)
        },
    }
    .render()
}

pub async fn novo_lote(State(e): State<EstadoApp>) -> Markup {
    formulario("lote", &Campos::new(), &[], &e, false)
}
pub async fn novo_alimento(State(e): State<EstadoApp>) -> Markup {
    formulario(
        "alimento",
        &Campos::from([("fonte_id".into(), "C01".into())]),
        &[],
        &e,
        false,
    )
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
    Ok(formulario(
        "lote",
        &formularios::campos_lote(&lote),
        &[],
        &e,
        false,
    ))
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
    Ok(formulario(
        "alimento",
        &formularios::campos_alimento(&a),
        &[],
        &e,
        false,
    ))
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
            let novo = formularios::texto(&c, "id").is_empty();
            match banco::salvar_lote(&e.pool, &mut lote, novo).await {
                Ok(()) => return Ok(redirecionar("/lotes", &headers)),
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
    Ok(formulario("lote", &c, &erros, &e, headers.contains_key("hx-request")).into_response())
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
        Ok(mut a) => match banco::salvar_alimento(&e.pool, &mut a).await {
            Ok(()) => return Ok(redirecionar("/alimentos", &headers)),
            Err(erro) => {
                tracing::error!(%erro, "Erro ao salvar composição");
                vec![crate::dominio::validacao::problema(
                    "fonte_id",
                    "Não foi possível salvar. Confira se a fonte existe e os dados são válidos.",
                )]
            }
        },
        Err(erros) => erros,
    };
    Ok(formulario(
        "alimento",
        &c,
        &erros,
        &e,
        headers.contains_key("hx-request"),
    )
    .into_response())
}

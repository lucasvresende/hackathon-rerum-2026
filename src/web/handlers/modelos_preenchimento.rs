//! Handler HTTP para modelos preenchimento, mantendo a renderização desta responsabilidade isolada.
use axum::{
    Form,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use axum::{Router, routing::post};

use crate::aplicacao::modelos_preenchimento::{self, AcaoModelo};
use crate::dominio::templates::TipoCadastro;
use crate::web::{
    apresentacao,
    entradas::{Campos, texto},
    erro::ErroWeb,
    estado::EstadoApp,
    seguranca::autorizado,
};

/// Rotas HTTP deste recurso para composição no roteador principal.
pub fn rotas() -> Router<EstadoApp> {
    Router::new().route("/cadastros/{tipo}/modelo", post(carregar))
}

/// Traduz a ação HTTP e aplica ao rascunho as substituições preparadas pela aplicação.
pub async fn carregar(
    State(e): State<EstadoApp>,
    Path(tipo): Path<String>,
    Query(q): Query<Campos>,
    headers: HeaderMap,
    Form(mut campos): Form<Campos>,
) -> Result<Response, ErroWeb> {
    if !autorizado(&e, &headers, &campos) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }
    let tipo = match tipo.as_str() {
        "lote" => TipoCadastro::Lote,
        "alimento" => TipoCadastro::Alimento,
        _ => return Ok(StatusCode::NOT_FOUND.into_response()),
    };
    let acao = if texto(&campos, "acao_modelo") == "aplicar" || texto(&q, "acao") == "aplicar" {
        AcaoModelo::Aplicar {
            confirmado: texto(&campos, "confirmar_modelo") == "on",
        }
    } else {
        AcaoModelo::Previa
    };
    let carga =
        modelos_preenchimento::preparar(&e.pool, tipo, texto(&campos, "modelo"), acao).await?;
    if carga.previa {
        campos.insert("previa_modelo".into(), "on".into());
    }
    if carga.aplicado.is_some() {
        // Remove inclusive metadados e laudos anteriores; preserva só identidade e proteção.
        campos.retain(|k, _| ["csrf", "id", "anterior", "modelo"].contains(&k.as_str()));
    }
    campos.extend(carga.substituicoes);
    if let Some(id) = carga.aplicado {
        campos.insert("modelo_aplicado".into(), id);
        campos.remove("previa_modelo");
        campos.remove("confirmar_modelo");
    }
    Ok(apresentacao::cadastro(
        tipo,
        &campos,
        &carga.erros,
        &e,
        headers.contains_key("hx-request"),
    )
    .await?
    .into_response())
}

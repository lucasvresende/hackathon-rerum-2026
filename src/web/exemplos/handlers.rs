//! Handler HTTP para exemplos, mantendo a renderização desta responsabilidade isolada.
use crate::{
    aplicacao::exemplos,
    web::{
        entradas::{Campos, texto},
        erro::ErroWeb,
        estado::EstadoApp,
        resposta::redirecionar,
        seguranca::autorizado,
        templates::layouts::Principal,
    },
};
use axum::{
    Form, Router,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use maud::{Markup, Render, html};

pub fn rotas() -> Router<EstadoApp> {
    Router::new()
        .route("/exemplos", get(listar).post(restaurar))
        .route("/cadastros/{tipo}/{id}/remover", post(remover))
}

pub async fn listar(
    State(e): State<EstadoApp>,
    Query(q): Query<Campos>,
) -> Result<Markup, ErroWeb> {
    pagina(&e, texto(&q, "salvo") == "sim", "").await
}

async fn pagina(e: &EstadoApp, salvo: bool, erro: &str) -> Result<Markup, ErroWeb> {
    let modelos = crate::aplicacao::modelos_preenchimento::listar(&e.pool).await?;
    Ok(Principal { titulo: "Restaurar exemplos", ativo: "/lotes", conteudo: html! {
        h1 { "Restaurar exemplos" }
        a href="/ajuda#alimentos" target="_blank" rel="noopener" { "Ajuda nesta página" }
        p { "Escolha um original. Cópia cria outro cadastro; revisão restaura o registro inicial, "
            "reativando-o e preservando as revisões anteriores." }
        @if salvo { p.alert role="status" { "Exemplo restaurado." } }
        @if !erro.is_empty() { p.alert role="alert" { (erro) } }
        section.food-grid {
            @for m in modelos.iter().filter(|m| m.id.starts_with("ex-")) {
                article.card {
                    h2 { (&m.nome) }
                    p { (&m.contexto) }
                    p.muted { (&m.lacunas) }
                    form method="post" action="/exemplos" {
                        input type="hidden" name="csrf" value=(&e.csrf);
                        input type="hidden" name="exemplo" value=(&m.id);
                        label for=(format!("modo-{}", m.id)) { "Como restaurar" }
                        select id=(format!("modo-{}", m.id)) name="modo" {
                            option value="copiar" { "Criar cópia independente" }
                            option value="revisar" {
                                "Restaurar o registro inicial com nova revisão" }
                        }
                        label.check {
                            input type="checkbox" name="confirmar" required;
                            "Confirmo a restauração selecionada"
                        }
                        button.button { "Restaurar exemplo" }
                    }
                }
            }
        }
    }}.render())
}

async fn restaurar(
    State(e): State<EstadoApp>,
    headers: HeaderMap,
    Form(c): Form<Campos>,
) -> Result<Response, ErroWeb> {
    if !autorizado(&e, &headers, &c) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }
    let disponivel = crate::persistencia::exemplos::listar(&e.pool)
        .await?
        .iter()
        .any(|m| m.id == texto(&c, "exemplo"));
    let erro = if texto(&c, "confirmar") != "on" {
        Some("Confirme a restauração antes de continuar.")
    } else if !disponivel || !["copiar", "revisar"].contains(&texto(&c, "modo")) {
        Some("Escolha um exemplo disponível e como restaurá-lo.")
    } else {
        None
    };
    if let Some(erro) = erro {
        return Ok((StatusCode::BAD_REQUEST, pagina(&e, false, erro).await?).into_response());
    }
    exemplos::restaurar(&e.pool, texto(&c, "exemplo"), texto(&c, "modo")).await?;
    Ok(redirecionar("/exemplos?salvo=sim", &headers))
}

async fn remover(
    State(e): State<EstadoApp>,
    Path((tipo, id)): Path<(String, String)>,
    headers: HeaderMap,
    Form(c): Form<Campos>,
) -> Result<Response, ErroWeb> {
    if !autorizado(&e, &headers, &c) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }
    if !["lote", "alimento"].contains(&tipo.as_str()) {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }
    if texto(&c, "confirmar") != "on" {
        return Ok((StatusCode::BAD_REQUEST, "Confirme a remoção.").into_response());
    }
    crate::persistencia::exemplos::desativar(&e.pool, &tipo, &id).await?;
    Ok(redirecionar(
        if tipo == "lote" {
            "/lotes"
        } else {
            "/alimentos"
        },
        &headers,
    ))
}

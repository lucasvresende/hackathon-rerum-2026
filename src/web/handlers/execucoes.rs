use crate::aplicacao::execucao;
use crate::persistencia::historico;
use crate::web::entradas::Campos;
use crate::web::{apresentacao, entradas as formularios};
use crate::web::{
    erro::ErroWeb,
    estado::EstadoApp,
    resposta::{autorizado, redirecionar},
};
use axum::{
    Form,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use maud::{Markup, Render};

pub async fn resultado(
    State(e): State<EstadoApp>,
    Path(id): Path<String>,
) -> Result<Markup, ErroWeb> {
    let execucao = historico::obter(&e.pool, &id).await?;
    Ok(crate::web::templates::paginas::execucao::Execucao {
        execucao: &execucao,
        token: &e.csrf,
    }
    .render())
}

pub async fn duplicar(
    State(e): State<EstadoApp>,
    Path(id): Path<String>,
) -> Result<Markup, ErroWeb> {
    let anterior = historico::obter(&e.pool, &id).await?;
    let mut c = anterior.cenario;
    c.original_id = Some(anterior.cenario_id);
    c.nome = format!("{} · simulação", c.nome);
    apresentacao::cenario(&e, &formularios::campos_cenario(&c), &[], false).await
}

pub async fn reproduzir(
    State(e): State<EstadoApp>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Form(c): Form<Campos>,
) -> Result<Response, ErroWeb> {
    if !autorizado(&e, &headers, &c) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }
    let anterior = historico::obter(&e.pool, &id).await?;
    let mut cenario = anterior.cenario;
    cenario.original_id = Some(anterior.cenario_id);
    let novo = execucao::executar(&e.pool, &cenario, &anterior.conhecimento).await?;
    Ok(redirecionar(&format!("/execucoes/{novo}"), &headers))
}

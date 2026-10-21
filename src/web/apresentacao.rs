//! Montagem das respostas de formulário, preservando rascunhos e fragmentos htmx.
use super::entradas::Campos;
use super::templates::formularios::{cadastro::Cadastro, cenario::FormularioCenario};
use super::{erro::ErroWeb, estado::EstadoApp};
use crate::persistencia::cadastros;
use maud::{Markup, Render};
pub async fn cadastro(
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

pub async fn cenario(
    e: &EstadoApp,
    campos: &Campos,
    erros: &[crate::dominio::validacao::Problema],
    parcial: bool,
) -> Result<Markup, ErroWeb> {
    let lotes = cadastros::lotes(&e.pool).await?;
    let alimentos = cadastros::alimentos(&e.pool).await?;
    let formulario = FormularioCenario {
        campos,
        lotes: &lotes,
        alimentos: &alimentos,
        erros,
        token: &e.csrf,
    }
    .render();
    if parcial {
        return Ok(formulario);
    }
    Ok(super::templates::paginas::cenario::Cenario {
        sem_lotes: lotes.is_empty(),
        formulario,
    }
    .render())
}

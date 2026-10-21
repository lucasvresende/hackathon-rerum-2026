//! Montagem das respostas de formulário, preservando rascunhos e fragmentos htmx.
use super::entradas::Campos;
use super::templates::formularios::{
    DadosCadastro, alimento::FormularioAlimento, cenario::FormularioCenario, lote::FormularioLote,
};
use super::{erro::ErroWeb, estado::EstadoApp};
use crate::aplicacao::cenarios;
use crate::dominio::templates::TipoCadastro;
use maud::{Markup, Render};
pub async fn cadastro(
    tipo: TipoCadastro,
    campos: &Campos,
    erros: &[crate::dominio::validacao::Problema],
    estado: &EstadoApp,
    parcial: bool,
) -> Result<Markup, ErroWeb> {
    let modelos = crate::aplicacao::modelos_preenchimento::listar(&estado.pool).await?;
    let dados = DadosCadastro {
        modelos: &modelos,
        campos,
        erros,
        token: &estado.csrf,
    };
    let cadastro = match tipo {
        TipoCadastro::Lote => FormularioLote(dados).render(),
        TipoCadastro::Alimento => FormularioAlimento(dados).render(),
    };
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
    let cenarios::OpcoesCenario { lotes, alimentos } = cenarios::opcoes(&e.pool).await?;
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

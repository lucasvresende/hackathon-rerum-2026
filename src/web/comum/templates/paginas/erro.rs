//! Página de apresentação para erro, mantendo a renderização desta responsabilidade isolada.
use super::super::{componentes::cabecalho::Cabecalho, layouts::Principal};
use maud::{Markup, Render, html};

pub struct Erro(pub TipoErro);

pub enum TipoErro {
    NaoEncontrado,
    Operacional,
}
impl Render for Erro {
    fn render(&self) -> Markup {
        let (titulo, cabecalho) = match self.0 {
            TipoErro::NaoEncontrado => ("Página não encontrada", "Página não encontrada"),
            TipoErro::Operacional => (
                "Não foi possível concluir",
                "Não foi possível concluir esta operação",
            ),
        };
        Principal { titulo, ativo: "", conteudo: html! {
            (Cabecalho { titulo: cabecalho, texto: "", ajuda: "inicio" })
            @match self.0 {
                TipoErro::NaoEncontrado => { a href="/" { "Voltar ao início" } }
                TipoErro::Operacional => {
                p { "Houve uma falha operacional. Seus resultados anteriores estão preservados. \
                            Volte à tela anterior para tentar novamente." }
                    a.button href="/" { "Voltar à visão geral" }
                }
            }
        }}.render()
    }
}

/// Recusa de uma operação indisponível, com orientação e retorno ao resultado preservado.
pub struct AvisoOperacao<'a> {
    pub mensagem: &'a str,
    pub retorno: &'a str,
}
impl Render for AvisoOperacao<'_> {
    fn render(&self) -> Markup {
        Principal {
            titulo: "Revise a operação",
            ativo: "/historico",
            conteudo: html! {
                (Cabecalho { titulo: "Revise a operação", texto: "", ajuda: "resultados" })
                section.card {
                    p.alert role="alert" { (self.mensagem) }
                    a.button href=(self.retorno) { "Voltar ao resultado" }
                }
            },
        }
        .render()
    }
}

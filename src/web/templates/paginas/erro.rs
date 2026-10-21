use super::super::{componentes::cabecalho::Cabecalho, layouts::Principal};
use maud::{Markup, Render, html};

pub enum Erro {
    NaoEncontrado,
    Operacional,
}
impl Render for Erro {
    fn render(&self) -> Markup {
        let (titulo, cabecalho) = match self {
            Self::NaoEncontrado => ("Página não encontrada", "Página não encontrada"),
            Self::Operacional => (
                "Não foi possível concluir",
                "Não foi possível concluir esta operação",
            ),
        };
        Principal { titulo, ativo: "", conteudo: html! {
            (Cabecalho { titulo: cabecalho, texto: "", ajuda: "inicio" })
            @match self {
                Self::NaoEncontrado => { a href="/" { "Voltar ao início" } }
                Self::Operacional => {
                p { "Houve uma falha operacional. Seus resultados anteriores estão preservados. \
                            Volte à tela anterior para tentar novamente." }
                    a.button href="/" { "Voltar à visão geral" }
                }
            }
        }}.render()
    }
}

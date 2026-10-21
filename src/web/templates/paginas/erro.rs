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

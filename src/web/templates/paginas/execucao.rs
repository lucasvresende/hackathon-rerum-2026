use super::super::{componentes::resultado::ResultadoExecucao, layouts::Principal};
use crate::persistencia::historico::Execucao as DadosExecucao;
use maud::{Markup, Render};

pub struct Execucao<'a> {
    pub execucao: &'a DadosExecucao,
    pub token: &'a str,
}
impl Render for Execucao<'_> {
    fn render(&self) -> Markup {
        Principal {
            titulo: &self.execucao.cenario.nome,
            ativo: "/historico",
            conteudo: ResultadoExecucao {
                execucao: self.execucao,
                token: self.token,
            }
            .render(),
        }
        .render()
    }
}

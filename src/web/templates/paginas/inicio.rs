use super::super::{componentes, layouts::Principal};
use crate::dominio::{Alimento, Conhecimento, Lote};
use crate::persistencia::historico::Execucao;
use maud::{Markup, Render};

pub struct Inicio<'a> {
    pub lotes: &'a [Lote],
    pub alimentos: &'a [Alimento],
    pub execucoes: &'a [Execucao],
    pub conhecimento: &'a Conhecimento,
}
impl Render for Inicio<'_> {
    fn render(&self) -> Markup {
        Principal {
            titulo: "Visão geral",
            ativo: "/",
            conteudo: componentes::inicio::Inicio {
                lotes: self.lotes,
                alimentos: self.alimentos,
                execucoes: self.execucoes,
                conhecimento: self.conhecimento,
            }
            .render(),
        }
        .render()
    }
}

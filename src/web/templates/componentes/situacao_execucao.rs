//! Estado operacional separado do certificado da busca e dos diagnósticos parciais.
use crate::dominio::{Cenario, Parecer};
use crate::web::formatacao::estado;
use maud::{Markup, Render, html};

pub struct Situacao<'a>(pub &'a Cenario, pub &'a Parecer);
impl Render for Situacao<'_> {
    fn render(&self) -> Markup {
        let (c, p) = (self.0, self.1);
        let situacao = p.estado_fornecimento(c);
        html! {
            div #situacao-fornecimento data-fornecimento=(situacao)
                class=(if situacao == "FORNECIMENTO_VALIDADO" { "alert" }
                    else if situacao == "RESOLUCAO_PENDENTE" { "alert notice" }
                    else { "alert error" }) {
                strong { (estado(situacao)) }
                @if situacao == "SEM_SOLUCAO" {
                    p { "Nenhuma combinação validada foi encontrada. "
                        "Custos e quantidades da entrada não representam "
                        "uma solução de menor custo." }
                } @else if situacao == "RESOLUCAO_PENDENTE" {
                    p { "Informe a resolução de oferta dos ingredientes utilizados e recalcule. "
                        "As quantidades contínuas exibidas são aproximadas." }
                }
                p { "Avaliação do perfil implementado; não é avaliação nutricional integral, "
                    "previsão de produção ou diagnóstico clínico." }
            }
        }
    }
}

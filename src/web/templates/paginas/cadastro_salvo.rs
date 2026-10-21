use super::super::componentes::cabecalho::Cabecalho;
use super::super::layouts::Cadastro;
use maud::{Markup, Render, html};

pub struct CadastroSalvo {
    pub cancelado: bool,
    pub lote: bool,
}
impl Render for CadastroSalvo {
    fn render(&self) -> Markup {
        let cancelado = self.cancelado;
        let lote = self.lote;
        Cadastro {
            titulo: if cancelado {
                "Cadastro cancelado"
            } else {
                "Cadastro salvo"
            },
            conteudo: html! {
                (Cabecalho {
                    titulo: if cancelado { "Cadastro cancelado" } else { "Cadastro salvo" },
                    texto: "",
                    ajuda: if
                lote { "lotes" } else { "alimentos" } })
                section.card data-cadastro-salvo[!cancelado] {
                    p role="status" {
                        @if cancelado { "Nenhuma revisão foi salva. " }
                        @else { "A revisão foi salva. " }
                        "Você pode retornar à guia de origem. "
                        "O cenário preservará os valores digitados e atualizará os cadastros." }
                    p { "Você pode fechar esta guia pelo navegador." }
                    a.button.secondary href=(if lote { "/lotes/novo" } else {
                        "/alimentos/novo" }) {
                        "Cadastrar outro registro nesta guia"
                    }
                }
            },
        }
        .render()
    }
}

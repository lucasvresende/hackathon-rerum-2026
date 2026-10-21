use axum::extract::Query;
use maud::{Markup, Render, html};

use super::componentes::estrutura::{Pagina, cabecalho};
use super::formularios::{Campos, texto};

pub async fn pagina(Query(campos): Query<Campos>) -> Markup {
    let cancelado = texto(&campos, "tipo") == "cancelado";
    let lote = texto(&campos, "tipo") == "lote";
    Pagina {
        titulo: if cancelado { "Cadastro cancelado" } else { "Cadastro salvo" },
        ativo: "cadastro",
        conteudo: html! {
            (cabecalho(if cancelado { "Cadastro cancelado" } else { "Cadastro salvo" }, "", if lote { "lotes" } else { "alimentos" }))
            section.card data-cadastro-salvo[!cancelado] {
                p role="status" {
                    @if cancelado { "Nenhuma revisão foi salva. " }
                    @else { "A revisão foi salva. " }
                    "Você pode retornar à guia de origem. "
                    "O cenário preservará os valores digitados e atualizará os cadastros." }
                p { "Você pode fechar esta guia pelo navegador." }
                a.button.secondary href=(if lote { "/lotes/novo" } else { "/alimentos/novo" }) {
                    "Cadastrar outro registro nesta guia"
                }
            }
        },
    }.render()
}

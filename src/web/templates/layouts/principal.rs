//! Layout HTML para principal, mantendo a renderização desta responsabilidade isolada.
use super::documento::Documento;
use maud::{Markup, Render, html};

pub struct Principal<'a> {
    pub titulo: &'a str,
    pub ativo: &'a str,
    pub conteudo: Markup,
}
impl Render for Principal<'_> {
    fn render(&self) -> Markup {
        Documento {
            titulo: self.titulo,
            isolada: false,
            menu: Menu { ativo: self.ativo }.render(),
            conteudo: &self.conteudo,
        }
        .render()
    }
}

struct Menu<'a> {
    ativo: &'a str,
}
impl Render for Menu<'_> {
    fn render(&self) -> Markup {
        html! {
            aside.sidebar aria-label="Menu principal" {
                a.brand href="/" {
                    img src="/static/cow.svg" alt="" width="36" height="36";
                    "FeedYourCow!"
                }
                nav {
                    @for (url, texto, icone) in [
                    ("/", "Visão geral", "/static/icones/inicio.svg"),
                    ("/lotes", "Lotes de animais", "/static/icones/lotes-de-animais.svg"),
                    ("/alimentos", "Alimentos", "/static/icones/alimentos.svg"),
                    ("/cenarios/novo", "Nova dieta", "/static/icones/nova-dieta.svg"),
                    ("/historico", "Histórico e análise", "/static/icones/historico.svg"),
                    ("/conhecimento", "Perfil nutricional",
                        "/static/icones/perfil-nutricional.svg"),
                    ("/ajuda", "Ajuda", "/static/icones/ajuda.svg"),
                    ] {
                        a class=(if self.ativo == url { "nav-link active" }
                        else { "nav-link" }) href=(url)
                        aria-current=[(self.ativo == url).then_some("page")]
                        target=[(url == "/cenarios/novo").then_some("_blank")]
                        rel=[(url == "/cenarios/novo").then_some("noopener")]
                        title=[(url == "/cenarios/novo")
                        .then_some("Abre em nova guia")] {
                            span aria-hidden="true" {
                                @if icone.starts_with("/") {
                                    img.nav-icon src=(icone) alt="" width="21" height="21";
                                } @else { (icone) }
                            } (texto)
                        }
                    }
                }
                div.sidebar-bottom {
                    span.local-dot {} "Uso local"
                    small { "Versão " (env!("CARGO_PKG_VERSION")) }
                }
            }
        }
    }
}

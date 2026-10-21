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
                                "FeedYourCow"
                            }
                            nav {
                                @for (url, texto, icone) in [
                                ("/", "Visão geral", "◫"),
                                ("/lotes", "Lotes", "♧"),
                                ("/alimentos", "Alimentos", "❋"),
                                ("/cenarios/novo", "Novo cenário", "+"),
                                ("/historico", "Histórico", "◷"),
                                ("/conhecimento", "Perfil e fontes", "▤"),
                                ("/ajuda", "Ajuda", "?"),
                                ] {
                                    a class=(if self.ativo == url { "nav-link active" }
                                    else { "nav-link" }) href=(url)
                                    aria-current=[(self.ativo == url).then_some("page")]
                                    target=[(url == "/cenarios/novo").then_some("_blank")]
                                    rel=[(url == "/cenarios/novo").then_some("noopener")]
                                    title=[(url == "/cenarios/novo")
                                    .then_some("Abre em nova guia")] {
                                        span aria-hidden="true" { (icone) } (texto)
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

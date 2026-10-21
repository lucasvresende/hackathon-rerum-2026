use maud::{DOCTYPE, Markup, Render, html};

pub struct Pagina<'a> {
    pub titulo: &'a str,
    pub ativo: &'a str,
    pub conteudo: Markup,
}

impl Render for Pagina<'_> {
    fn render(&self) -> Markup {
        let isolada = self.ativo == "cadastro";
        html! {
            (DOCTYPE)
            html lang="pt-BR" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    title { (self.titulo) " · FeedYourCow" }
                    link rel="stylesheet" href="/static/app.css";
                    link rel="icon" href="/static/favicon.svg" type="image/svg+xml";
                    script src="/static/vendor/htmx.min.js" {}
                    script src="/static/vendor/hx-live.min.js" {}
                    script src="/static/app.js" defer {}
                    script src="/static/formularios.js" defer {}
                }
                body class=[isolada.then_some("pagina-cadastro")] {
                    a.skip href="#principal" { "Pular para o conteúdo" }
                    @if !isolada {
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
                    div.workspace {
                        main #principal tabindex="-1" {
                            (self.conteudo)
                        }
                    }
                }
            }
        }
    }
}

pub use super::campo::Campo;

pub fn cabecalho(titulo: &str, texto: &str, ajuda: &str) -> Markup {
    html! {
        div.page-heading {
            div.page-title-row {
                h1 { (titulo) }
                a.button.secondary.page-help.no-print href=(format!("/ajuda#{ajuda}")) {
                    "Ajuda nesta página"
                }
            }
            @if !texto.is_empty() {
                p.subtitle { (texto) }
            }
        }
    }
}

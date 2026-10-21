use maud::{DOCTYPE, Markup, Render, html};

pub struct Pagina<'a> {
    pub titulo: &'a str,
    pub ativo: &'a str,
    pub conteudo: Markup,
}

impl Render for Pagina<'_> {
    fn render(&self) -> Markup {
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
                }
                body {
                    a.skip href="#principal" { "Pular para o conteúdo" }
                    aside.sidebar aria-label="Menu principal" {
                        a.brand href="/" { "FeedYourCow" }
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
                                    aria-current=[(self.ativo == url).then_some("page")] {
                                    span aria-hidden="true" { (icone) } (texto)
                                }
                            }
                        }
                        div.sidebar-bottom {
                            span.local-dot {} "Uso local"
                            small { "Versão " (env!("CARGO_PKG_VERSION")) }
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

pub struct Campo<'a> {
    pub nome: &'a str,
    pub rotulo: &'a str,
    pub valor: &'a str,
    pub tipo: &'a str,
    pub dica: &'a str,
    pub obrigatorio: bool,
    pub erro: Option<&'a str>,
}

impl Render for Campo<'_> {
    fn render(&self) -> Markup {
        html! {
            div.field {
                label for=(self.nome) { (self.rotulo) }
                input id=(self.nome) name=(self.nome) type=(self.tipo) value=(self.valor)
                    step=[(self.tipo == "number").then_some("any")]
                    required[self.obrigatorio] aria-invalid=[self.erro.map(|_| "true")]
                    aria-describedby=(format!("{}-dica", self.nome));
                small id=(format!("{}-dica", self.nome)) { (self.dica) }
                @if let Some(erro) = self.erro {
                    span.field-error { (erro) }
                }
            }
        }
    }
}

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

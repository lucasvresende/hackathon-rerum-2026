use maud::{DOCTYPE, Markup, Render, html};

pub(super) struct Documento<'a> {
    pub titulo: &'a str,
    pub isolada: bool,
    pub menu: Markup,
    pub conteudo: &'a Markup,
}

impl Render for Documento<'_> {
    fn render(&self) -> Markup {
        let isolada = self.isolada;
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
                    script src="/static/dicas.js" defer {}
                    script src="/static/formularios.js" defer {}
                }
                body class=[isolada.then_some("pagina-cadastro")] {
                    a.skip href="#principal" { "Pular para o conteúdo" }
                    (self.menu)
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

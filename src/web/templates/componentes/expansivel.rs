use maud::{Markup, Render, html};

/// Conteúdo fica aberto sem JavaScript. Com hx-live, o estado e a acessibilidade são
/// locais.
pub struct Expansivel<'a> {
    pub id: &'a str,
    pub titulo: Markup,
    pub conteudo: Markup,
    pub aberto: bool,
}
impl Render for Expansivel<'_> {
    fn render(&self) -> Markup {
        let conteudo_id = format!("{}-conteudo", self.id);
        html! {
            section.expansivel.expand-open id=(self.id) data-aberta=(self.aberto)
            data-pronta="false"
            hx-live:class="{ 'expand-open': data.aberta, 'expand-ready': data.pronta }"
            hx-on:load="await nextFrame(); await nextFrame(); data.pronta = true" {
                input type="hidden" name=(format!("expansao-{}", self.id))
                    value=(self.aberto) hx-live:value="data.aberta";
                div.expand-heading {
                    button.expand-toggle type="button" hidden hx-live:hidden="false"
                    aria-expanded="true" hx-live:aria-expanded="data.aberta"
                    aria-controls=(&conteudo_id)
                    hx-on:click="data.aberta = !data.aberta" {
                        span { (&self.titulo) }
                        span aria-hidden="true" hx-live:text="data.aberta ? '−' : '+'" {
                            "−" }
                    }
                    span hx-live:hidden="true" { (&self.titulo) }
                }
                div.expand-content id=(&conteudo_id) hx-live:inert="!data.aberta" {
                    div.expand-inner { div.expand-body { (&self.conteudo) } }
                }
            }
        }
    }
}

pub struct Visualizacao;
impl Render for Visualizacao {
    fn render(&self) -> Markup {
        html! {
            div.actions.no-print {
                button.button.secondary type="button" hidden hx-live:hidden="false"
                aria-pressed="false" hx-live:aria-pressed="!data.simples"
                hx-on:click="data.simples = !data.simples"
                hx-live:text="data.simples ? 'Mostrar campos complementares' :
                        'Usar preenchimento simplificado'" { "Mostrar campos
                        complementares" }
                input type="hidden" name="visualizacao" value="completo"
                hx-live:value="data.simples ? 'simples' : 'completo'";
            }
        }
    }
}

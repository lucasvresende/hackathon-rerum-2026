use maud::{Markup, Render, html};

pub struct Campo<'a> {
    pub nome: &'a str,
    pub rotulo: &'a str,
    pub valor: &'a str,
    pub tipo: &'a str,
    pub dica: &'a str,
    pub obrigatorio: bool,
    pub erro: Option<&'a str>,
}

pub struct Rotulo<'a> {
    pub nome: &'a str,
    pub texto: &'a str,
    pub obrigatorio: bool,
}
impl Render for Rotulo<'_> {
    fn render(&self) -> Markup {
        let nome = self.nome;
        let texto = self.texto;
        let obrigatorio = self.obrigatorio;
        let rotulo = html! {
            label for=(nome) {
                (texto)
                @if obrigatorio { span.required aria-hidden="true" { " *" } }
            }
        };
        let dica = super::conceitos_cenario::campo(nome)
            .or_else(|| super::conceitos_alimentos::campo(nome));
        html! {
            @if let Some(dica) = dica {
                div.concept-label {
                    (rotulo)
                    (dica.com_id(&format!("conceito-{nome}")))
                }
            } @else { (rotulo) }
        }
    }
}
pub struct MensagemErro<'a> {
    pub nome: &'a str,
    pub erro: Option<&'a str>,
}
impl Render for MensagemErro<'_> {
    fn render(&self) -> Markup {
        let nome = self.nome;
        let erro = self.erro;
        html! {
            span.field-error id=(format!("{nome}-erro")) hidden[erro.is_none()] {
                @if let Some(erro) = erro { (erro) }
            }
        }
    }
}

/// Restrições de preenchimento, sem confundir faixa do perfil com validade do cadastro.
fn limites(nome: &str) -> (Option<&'static str>, Option<&'static str>, &'static str) {
    let prefixo = nome.split('-').next().unwrap_or(nome);
    match prefixo {
        "animais" | "horizonte" => (Some("1"), Some("4294967295"), "1"),
        "dias_lactacao" | "idade_meses" | "ordem_parto" => (Some("0"), Some("4294967295"), "1"),
        "peso" | "gordura" | "consumo_alvo" | "periodo_observado" | "embalagem" => {
            (Some("0"), None, "any")
        }
        "ms" => (Some("0"), Some("100"), "any"),
        "pb" | "ca" | "p" | "fdn" | "amido" | "ee" | "sobras" | "perda" => {
            (Some("0"), Some("100"), "any")
        }
        _ => (Some("0"), None, "any"),
    }
}

impl Render for Campo<'_> {
    fn render(&self) -> Markup {
        let (minimo, maximo, passo) = limites(self.nome);
        let numero = self.tipo == "number";
        html! {
            div.field {
                (Rotulo { nome: self.nome, texto: self.rotulo, obrigatorio: self.obrigatorio })
                (MensagemErro { nome: self.nome, erro: self.erro })
                input id=(self.nome) name=(self.nome) type=(self.tipo) value=(self.valor)
                    step=[numero.then_some(passo)] min=[numero.then_some(minimo).flatten()]
                    max=[numero.then_some(maximo).flatten()]
                    maxlength=[(self.nome == "nome").then_some("150")]
                    required[self.obrigatorio] aria-invalid=[self.erro.map(|_| "true")]
                    aria-describedby=(format!("{}-erro {}-dica", self.nome, self.nome));
                small id=(format!("{}-dica", self.nome)) { (self.dica) }
            }
        }
    }
}

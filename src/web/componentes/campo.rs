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

pub fn rotulo(nome: &str, texto: &str, obrigatorio: bool) -> Markup {
    html! {
        label for=(nome) {
            (texto)
            @if obrigatorio { span.required aria-hidden="true" { " *" } }
        }
    }
}

pub fn mensagem(nome: &str, erro: Option<&str>) -> Markup {
    html! {
        span.field-error id=(format!("{nome}-erro")) hidden[erro.is_none()] {
            @if let Some(erro) = erro { (erro) }
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
                (rotulo(self.nome, self.rotulo, self.obrigatorio))
                (mensagem(self.nome, self.erro))
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

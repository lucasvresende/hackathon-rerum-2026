use super::super::componentes::cabecalho::Cabecalho;
use super::super::componentes::expansivel::Expansivel;
use super::super::layouts::Principal;
use maud::{Markup, Render, html};

pub struct Conhecimento<'a> {
    pub conhecimento: &'a crate::dominio::Conhecimento,
}
impl Render for Conhecimento<'_> {
    fn render(&self) -> Markup {
        let c = self.conhecimento;
        Principal { titulo: "Perfil nutricional e fontes", ativo: "/conhecimento", conteudo: html! {
            (Cabecalho { titulo: "Perfil nutricional e fontes", texto: "", ajuda: "conhecimento" })
            section.card { h2 { (&c.perfil_id) }
                p { "Adultas em lactação, confinadas, estáveis e sem gestação/crescimento. \
                O perfil histórico NRC 1989 não implementa integralmente NASEM 2021 ou
                BR-LEITE 2024." }
                p { "PV 300–800 kg (tabelas 400–700 kg; consumo 400–600 kg); gordura 3–5%; \
                            DEL 60–250; LCG4 5–30 kg/dia." }
            }
            section.card.table-wrap { h2 { "Coeficientes do perfil" }
                table { thead { tr { @for t in ["Tabela", "Eixo", "Eixo 2", "A", "B",
                            "C", "D",
                            "Unidade"] {
                                th { (t) } } } }
                    tbody { @for r in &c.coeficientes { tr {
                                td { (&r.tabela) } td { (r.eixo) } td { (r.eixo2) }
                                @for v in r.valores { td { (v) } } td { (&r.unidade) }
                            } } }
                }
            }
            section.card { h2 { "Fontes preservadas" }
                @for f in &c.fontes { (Expansivel { id:
                        &format!("conhecimento-{}", f.id), aberto: false,
                        titulo: html! { span.pill { (&f.natureza) } " "
                            (&f.id) " · " (&f.titulo) },
                        conteudo: html! { p { (&f.edicao) " · " (&f.localizador) }
                            p { (&f.conteudo) } p.code { (&f.hash) }
                        },
                    }) }
            }
            section.card { h2 { "Normas, vigência e condições" }
                p.muted {
                    "Ícone de vaca: EmojiOne / JoyPixels, via "
                    a href="https://www.svgrepo.com/svg/401350/cow" { "SVG Repo" }
                    ". Arte " a href="https://creativecommons.org/licenses/by/4.0/" { "CC BY 4.0" }
                    "; dimensões e formatação ajustadas."
                }
                @for n in &c.normas { (Expansivel { id:
                        &format!("norma-{}", n.id), aberto: false,
                        titulo: html! { (&n.id) " · " (&n.natureza) },
                        conteudo: html! {
                            p { "Início: " (&n.inicio) " · fim exclusivo: "
                                (n.fim.as_deref().unwrap_or("Sem fim registrado")) }
                            p { "Transição: " (n.transicao.as_deref().unwrap_or("Não informada")) }
                            p { (&n.condicoes) } p { "Exceções: " (&n.excecoes) }
                            p { "Evidência: " (&n.evidencia) } p { "Ação: " (&n.acao) }
                        },
                    }) }
            }
        }}.render()
    }
}

//! Página que apresenta perfil, fontes e regras carregadas.
use super::super::componentes::cabecalho::Cabecalho;
use super::super::componentes::{dica::Dica, expansivel::Expansivel};
use super::super::layouts::Principal;
use maud::{Markup, Render, html};

pub struct Conhecimento<'a> {
    pub conhecimento: &'a crate::dominio::Conhecimento,
}
impl Render for Conhecimento<'_> {
    fn render(&self) -> Markup {
        let c = self.conhecimento;
        Principal { titulo: "Perfil nutricional", ativo: "/conhecimento", conteudo: html! {
            (Cabecalho { titulo: "Perfil nutricional", texto: "", ajuda: "conhecimento" })
            section.card { h2 { (&c.perfil_id) }
                p { "Um perfil nutricional é o conjunto versionado de referências, "
                    "coeficientes e regras que define para quais animais e condições os "
                    "cálculos são válidos. Ele informa quais nutrientes e limites são avaliados "
                    "e como interpretar os resultados. Neste perfil, os coeficientes históricos "
                    "NRC 1989 são combinados com barreiras técnicas e políticas do MVP claramente "
                    "identificadas; isso não equivale à implementação integral de NASEM 2021 ou "
                    "BR-LEITE 2024. Consulte a natureza e a fonte de cada regra antes de aplicar "
                    "um resultado." }
                p { "População: vacas adultas em lactação, confinadas, estáveis e sem \
                gestação ou crescimento." }
                p { "PV 300–800 kg (tabelas 400–700 kg; consumo 400–600 kg); gordura 3–5%; \
                            DEL 60–250; LCG4 5–30 kg/dia." }
            }
            section.card.table-wrap { h2 { "Coeficientes do perfil" }
                p.muted { "Os coeficientes são valores de referência usados nas equações do "
                    "perfil. Consulte os indicadores ? nos títulos para entender cada coluna; "
                    "os eixos "
                    "identificam a condição ou faixa a que os valores se aplicam." }
                table { thead { tr {
                    @for (id, nome, texto) in [
                        ("coef-tabela", "Tabela", "Grupo: manutenção, leite, consumo ou fibra."),
                        ("coef-eixo", "Eixo", "Primeiro valor que identifica a linha da tabela."),
                        ("coef-eixo2", "Eixo 2", concat!("Segundo valor de referência; zero ",
                            "indica ",
                            "que não há um segundo eixo.")),
                        ("coef-a", "A", concat!("Primeiro coeficiente da linha, conforme a ",
                            "equação e a fonte associadas.")),
                        ("coef-b", "B", concat!("Segundo coeficiente da linha, conforme a ",
                            "equação e a fonte associadas.")),
                        ("coef-c", "C", concat!("Terceiro coeficiente da linha, conforme a ",
                            "equação e a fonte associadas.")),
                        ("coef-d", "D", concat!("Quarto coeficiente da linha, conforme a ",
                            "equação e a fonte associadas.")),
                        ("coef-unidade", "Unidade", concat!("Unidade dos coeficientes; a base ",
                            "faz parte da interpretação.")),
                    ] {
                        th { (nome) (Dica { id, conceito: nome, texto }.render()) }
                    }
                } }
                    tbody { @for r in &c.coeficientes { tr {
                                td { (&r.tabela) } td { (r.eixo) } td { (r.eixo2) }
                                @for v in r.valores { td { (v) } } td { (&r.unidade) }
                            } } }
                }
            }
            section.card { h2 { "Fontes de referência" }
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

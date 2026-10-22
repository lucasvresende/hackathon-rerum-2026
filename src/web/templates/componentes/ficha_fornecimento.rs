//! Ficha operacional de impressão, com estado de uso e escopo visíveis antes das quantidades.
use crate::dominio::{Cenario, Parecer};
use crate::web::formatacao::{estado, numero, quantidade_pratica};
use maud::{Markup, Render, html};

pub struct Ficha<'a>(pub &'a Cenario, pub &'a Parecer, pub &'a str);
impl Render for Ficha<'_> {
    fn render(&self) -> Markup {
        let (c, p) = (self.0, self.1);
        let pendente = p.resolucao_pendente(c);
        html! {
            section.ficha-operacional {
                h2 { "Ficha de fornecimento" }
                h3 { (&c.nome) }
                p { (&c.lote.nome) " · " (c.lote.animais)
                    (if c.lote.animais == 1 { " animal · " } else { " animais · " })
                    (c.horizonte) " dias · avaliação " (&c.data_avaliacao) }
                p.muted { "Execução: " (self.2) }
                p class=(if p.fornecimento_recomendado(c) { "alert" }
                    else if p.resolucao_pendente(c) && p.pratico_valido { "alert notice" }
                    else { "alert error" }) {
                    strong { (estado(p.estado_fornecimento(c))) }
                }
                @if p.fornecimento_recomendado(c) {
                    p { "Quantidades práticas revalidadas no perfil; "
                        "conferir as limitações abaixo." }
                } @else {
                    p { strong { "Não usar esta ficha como recomendação de pesagem." }
                        " Informe as resoluções de oferta, recalcule e resolva as restrições." }
                }
                @if p.custo_resultado().is_some() {
                    table {
                        thead { tr { th { "Ingrediente" }
                            th { "Prática kg MN/animal/dia" }
                            th { "Oferta contínua aproximada kg MN/animal/dia" } } }
                        tbody { @for f in p.fornecimento.iter().filter(|f| f.consumo_ms > 0.0) {
                            tr { th { (&f.ingrediente) }
                                td { @if !pendente { (quantidade_pratica(f.pratico_mn)) }
                                    @else { "Não definida" } }
                                td { "≈ " (numero(f.oferta_mn, 4)) }
                            }
                        } }
                    }
                }
                h2 { "Limitações e condições" }
                p { "Consumo e oferta são diferentes. MN: matéria natural. "
                    "A oferta diária não inclui perdas anteriores ao fornecimento. "
                    "O cálculo não baixa estoque." }
                p { "Contaminantes: " (estado(p.dimensao("Contaminantes")))
                    ". Fibra física e micronutrientes: "
                    (estado(p.dimensao("Fibra física e micronutrientes"))) "." }
                p { "Atendimento ao perfil não é avaliação nutricional integral nem diagnóstico. "
                    "Consulte o parecer completo para balanços, premissas e fontes." }
                p { "Perfil: " (&c.perfil_id) }
            }
        }
    }
}

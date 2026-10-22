//! Página de apresentação para comparacao, mantendo a renderização desta responsabilidade isolada.
use super::super::componentes::cabecalho::Cabecalho;
use super::super::componentes::comparacao_item::{PremissasItem, QuantidadesItem};
use super::super::layouts::Principal;
use crate::dominio::unidades::dinheiro;
use crate::web::formatacao::{estado, numero};
use maud::{Markup, Render, html};

pub struct Comparacao<'a> {
    pub comparacao: &'a crate::aplicacao::comparacao::Comparacao,
}
impl Render for Comparacao<'_> {
    fn render(&self) -> Markup {
        let a = &self.comparacao.referencia;
        let b = &self.comparacao.alternativa;
        let equivalentes = self.comparacao.equivalentes;
        if !self.comparacao.mesma_revisao {
            return Principal {
                titulo: "Comparação indisponível",
                ativo: "/historico",
                conteudo: html! {
                    (Cabecalho { titulo: "Comparação indisponível", texto: "",
                        ajuda: "comparacao" })
                    section.card {
                        p.alert role="alert" { "Selecione cenários da mesma revisão do lote. "
                            "Lotes diferentes ou revisões diferentes não podem ser comparados." }
                        p { "Referência: " (&a.cenario.nome) " · " (&a.cenario.lote.nome) }
                        p { "Alternativa: " (&b.cenario.nome) " · " (&b.cenario.lote.nome) }
                        a.button href="/historico" { "Escolher outras execuções" }
                    }
                },
            }
            .render();
        }

        Principal { titulo: "Comparar cenários",
        ativo: "/historico", conteudo: html! {
        (Cabecalho { titulo: "Comparação de cenários", texto: "", ajuda: "comparacao" })
        section.card.table-wrap {
            table { thead { tr { th { "Premissa" } th { (&a.cenario.nome) } th {
            (&b.cenario.nome) } } }
                tbody {
                    @for (nome, va, vb) in [
                        ("Perfil", a.cenario.perfil_id.clone(), b.cenario.perfil_id.clone()),
                        ("Produção kg/dia", numero(a.cenario.lote.producao, 3),
                            numero(b.cenario.lote.producao, 3)),
                        ("Consumo kg MS", numero(a.parecer.consumo_ms, 4),
                        numero(b.parecer.consumo_ms, 4)),
                        ("Sobras %", numero(100.0*a.cenario.sobras, 2),
                            numero(100.0*b.cenario.sobras, 2)),
                        ("Horizonte dias", a.cenario.horizonte.to_string(),
                        b.cenario.horizonte.to_string()),
                        ("Fornecimento", estado(a.parecer.estado_fornecimento(&a.cenario)),
                            estado(b.parecer.estado_fornecimento(&b.cenario))),
                        ("Base", a.cenario.base_comparacao.clone(),
                        b.cenario.base_comparacao.clone()),
                    ] { tr { th { (nome) } td { (va) } td { (vb) } } }
                    tr { th { "Custo da solução contínua / animal / dia" }
                        td { (a.parecer.custo_resultado().map(dinheiro).unwrap_or("—".into())) }
                        td { (b.parecer.custo_resultado().map(dinheiro).unwrap_or("—".into())) }
                    }
                }
            }
            h2 { "Diagnóstico do fornecimento prático" }
            p.muted { "Valores práticos revalidados, inclusive quando incompatíveis. "
                "Ausência de resolução não representa pesagem validada." }
            table {
                thead { tr { th { "Grandeza" } th { "Referência" } th { "Alternativa" } } }
                tbody {
                    tr { th { "Consumo prático kg MS/animal/dia" }
                        @for e in [a, b] { td { (e.parecer.pratico.as_deref()
                            .map(|p| numero(p.consumo_ms, 4)).unwrap_or("Não definido".into())) } }
                    }
                    tr { th { "Custo prático R$/animal/dia" }
                        @for e in [a, b] { td { (e.parecer.pratico.as_deref()
                            .and_then(|p| p.custo).map(dinheiro)
                            .unwrap_or("Não definido".into())) } }
                    }
                }
            }
            (super::super::componentes::comparacao_balanco::Balanco(a, b))
            h2 { "Ingredientes e premissas de custo" }
            table {
                thead { tr { th { "Ingrediente" } th { "Referência" } th { "Alternativa" } } }
                tbody {
                    @for item in &a.cenario.itens {
                        @let outro=b.cenario.itens.iter()
                            .find(|i|i.alimento.id==item.alimento.id);
                        tr { th { (&item.alimento.nome)
                            @if outro.is_some_and(|i| i.alimento.composicao_id
                                != item.alimento.composicao_id) {
                                p { "Revisões de composição diferentes" }
                            }
                        }
                            @for (e, i) in [(a, Some(item)), (b, outro)] {
                                td { (PremissasItem(i))
                                    @if let Some(i) = i { (QuantidadesItem(e, i)) } }
                            }
                        }
                    }
                    @for i in b.cenario.itens.iter().filter(|i|!a.cenario.itens.iter()
                        .any(|original|original.alimento.id==i.alimento.id)) {
                        tr { th { (&i.alimento.nome) } td { "Ausente" }
                            td { (PremissasItem(Some(i))) (QuantidadesItem(b, i)) } }

                    }
                }
            }
            @if !equivalentes { p.alert { "Há mudanças em produção, consumo, horizonte, perfil ou \
                critérios. Os cenários não são diretamente equivalentes." } }
            @if let Some(economia) = &self.comparacao.economia {
                p { "Diferença de custo contínuo (referência − alternativa): "
                    strong { (dinheiro(economia.absoluta)) }
                    @if let Some(percentual) = economia.percentual {
                        " · " (numero(percentual, 2)) "%" }
                }
                @if !economia.alternativa_validada { p.alert {
                    "Alternativa sem fornecimento validado. \
                    Menor custo não a torna uma substituição equivalente." } }
            }
            div.actions { a.button.secondary href=(format!("/execucoes/{}", a.id)) {
            "Abrir referência" }
                a.button.secondary href=(format!("/execucoes/{}", b.id)) { "Abrir alternativa" } }
        }
    }}.render()
    }
}

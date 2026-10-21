use super::componentes::estrutura::{Pagina, cabecalho};
use super::rotas::{ErroWeb, EstadoApp};
use crate::dominio::unidades::{dinheiro, proximos};
use crate::persistencia::historico as banco;
use axum::extract::{Query, State};
use maud::{Markup, Render, html};
use std::collections::BTreeMap;

pub async fn comparar(
    State(e): State<EstadoApp>,
    Query(q): Query<BTreeMap<String, String>>,
) -> Result<Markup, ErroWeb> {
    let a = banco::obter(&e.pool, q.get("a").map(String::as_str).unwrap_or("")).await?;
    let b = banco::obter(&e.pool, q.get("b").map(String::as_str).unwrap_or("")).await?;
    let equivalentes = a.cenario.perfil_id == b.cenario.perfil_id
        && proximos(a.cenario.lote.producao, b.cenario.lote.producao)
        && proximos(a.parecer.consumo_ms, b.parecer.consumo_ms)
        && a.cenario.horizonte == b.cenario.horizonte
        && a.cenario.referencia_ue == b.cenario.referencia_ue;
    Ok(Pagina { titulo: "Comparar cenários",
        ativo: "/historico", conteudo: html! {
        (cabecalho("Comparação de cenários", "", "comparacao"))
        section.card.table-wrap {
            table { thead { tr { th { "Premissa" } th { (&a.cenario.nome) } th {
            (&b.cenario.nome) } } }
                tbody {
                    @for (nome, va, vb) in [
                        ("Perfil", a.cenario.perfil_id.clone(), b.cenario.perfil_id.clone()),
                        ("Produção kg/dia", a.cenario.lote.producao.to_string(),
                            b.cenario.lote.producao.to_string()),
                        ("Consumo kg MS", a.parecer.consumo_ms.to_string(),
                        b.parecer.consumo_ms.to_string()),
                        ("Sobras %", (100.0*a.cenario.sobras).to_string(),
                            (100.0*b.cenario.sobras).to_string()),
                        ("Horizonte dias", a.cenario.horizonte.to_string(),
                        b.cenario.horizonte.to_string()),
                        ("Base", a.cenario.base_comparacao.clone(),
                        b.cenario.base_comparacao.clone()),
                    ] { tr { th { (nome) } td { (va) } td { (vb) } } }
                    tr { th { "Custo / animal / dia" }
                        td { (a.parecer.custo.map(dinheiro).unwrap_or("—".into())) }
                        td { (b.parecer.custo.map(dinheiro).unwrap_or("—".into())) }
                    }
                }
            }
            h2 { "Ingredientes e premissas de custo" }
            table {
                thead { tr { th { "Ingrediente" } th { "Referência" } th { "Alternativa" } } }
                tbody {
                    @for item in &a.cenario.itens {
                        @let outro=b.cenario.itens.iter()
                            .find(|i|i.alimento.id==item.alimento.id);
                        tr { th { (&item.alimento.nome) }
                            @for i in [Some(item),outro] {
                                td { @if let Some(i)=i {
                                    "MS " (i.alimento.ms*100.0) "% · preço " (&i.preco)
                                    " R$/kg MN · perda " (i.perda*100.0) "% · estoque "
                                    (i.estoque.map(|v|v.to_string()).unwrap_or_else(|| {
                                        if i.sem_limite { "Sem limite (hipótese)".into() }
                                        else { "Desconhecido".into() }
                                    }))
                                } @else { "Ausente" } }
                            }
                        }
                    }
                    @for i in b.cenario.itens.iter().filter(|i|!a.cenario.itens.iter()
                        .any(|original|original.alimento.id==i.alimento.id)) {
                        tr { th { (&i.alimento.nome) } td { "Ausente" }
                            td { "Ingrediente adicionado · MS " (i.alimento.ms*100.0)
                                "% · preço " (&i.preco) " R$/kg MN" } }
                    }
                }
            }
            @if !equivalentes { p.alert { "Há mudanças em produção, consumo, horizonte, perfil ou \
                critérios. Os cenários não são diretamente equivalentes." } }
            @if let (Some(ca), Some(cb)) = (a.parecer.custo, b.parecer.custo) {
                p { "Diferença de custo (referência − alternativa): " strong { (dinheiro(ca-cb)) }
                    @if ca > 0.0 { " · " (format!("{:.2}%", 100.0*(ca-cb)/ca)) }
                }
                @if !b.parecer.pratico_valido { p.alert { "Alternativa sem fornecimento validado. \
                    Menor custo não a torna uma substituição equivalente." } }
            }
            div.actions { a.button.secondary href=(format!("/execucoes/{}", a.id)) {
            "Abrir referência" }
                a.button.secondary href=(format!("/execucoes/{}", b.id)) { "Abrir alternativa" } }
        }
    }}.render())
}

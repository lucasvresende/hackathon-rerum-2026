//! Página de apresentação para historico, mantendo a renderização desta responsabilidade isolada.
use super::super::componentes::cabecalho::Cabecalho;
use super::super::layouts::Principal;
use crate::dominio::unidades::dinheiro;
use maud::{Markup, Render, html};

pub struct Historico<'a> {
    pub execucoes: &'a [crate::persistencia::historico::Execucao],
    pub pagina: i64,
    pub referencia: Option<&'a crate::persistencia::historico::Execucao>,
    pub referencia_id: &'a str,
    pub busca: &'a str,
    pub data: &'a str,
}
impl Render for Historico<'_> {
    fn render(&self) -> Markup {
        let execucoes = self.execucoes;
        let pagina = self.pagina;
        let busca = self.busca;
        let data = self.data;
        let filtro = crate::aplicacao::historico::FiltroHistorico::novo(
            Some(pagina),
            busca.into(),
            data.into(),
        );
        let referencia = self.referencia;
        let sem_alternativa = referencia.is_some_and(|r| {
            !execucoes.iter().any(|e| {
                !r.cenario.lote.revisao_id.is_empty()
                    && r.cenario.lote.id == e.cenario.lote.id
                    && r.cenario.lote.revisao_id == e.cenario.lote.revisao_id
            })
        });
        Principal {
            titulo: "Histórico e análise",
            ativo: "/historico",
            conteudo: html! {
                (Cabecalho { titulo: "Histórico e análise", texto: "", ajuda: "historico" })
                @if let Some(r) = referencia {
                    section.card aria-label="Referência fixada" {
                        h2 { "Referência fixada" }
                        p { (&r.cenario.nome) " · " (&r.cenario.lote.nome)
                            " · revisão " (&r.cenario.lote.revisao_id) }
                        a.button.secondary href=(filtro.url(pagina, "")) {
                            "Remover referência" }
                    }
                } @else if !self.referencia_id.is_empty() {
                    p.alert role="alert" { "Referência não encontrada. "
                        a href=(filtro.url(pagina, "")) { "Limpar seleção" } }
                }
                form.card method="get" action="/historico" {
                    input type="hidden" name="referencia" value=(self.referencia_id);
                    div.form-grid {
                        div.field { label for="filtro-lote" { "Nome do lote" }
                            input #filtro-lote name="lote" value=(&busca);
                        }
                        div.field { label for="filtro-data" { "Data da execução (São Paulo)" }
                            input #filtro-data name="data" type="date" value=(&data);
                        }
                    }
                    button.button.secondary { "Filtrar histórico" }
                }
                div.actions {
                    @if pagina>1 { a.button.secondary
                        href=(filtro.url(pagina-1, self.referencia_id)) {
                            "← Página anterior" } }
                    span.muted { "Página " (pagina) " · até 100 resultados por página" }
                    @if execucoes.len()==100 { a.button.secondary
                        href=(filtro.url(pagina+1, self.referencia_id)) { "Próxima página →" } }
                }
                form.card method="get" action="/comparar" {
                    h2 { "Comparar duas execuções" }
                    p.muted { "A comparação exige a mesma revisão do lote. "
                        "Após editar um lote, execute ambos os cenários com a revisão atual." }
                    div.form-grid {
                        @for (nome, rotulo) in [("a", "Cenário de referência"), ("b",
                        "Cenário alternativo")] {
                            div.field { label for=(nome) { (rotulo) }
                                select id=(nome) name=(nome) required {
                                    @if nome == "a" {
                                        @if let Some(r) = referencia {
                                            option value=(&r.id) { (&r.cenario.nome)
                                                " · referência fixada" }
                                        }
                                    }
                                    @for execucao in execucoes.iter()
                                        .filter(|_| nome != "a" || referencia.is_none()) {
                                        @let compativel = referencia.is_none_or(|r|
                                            r.cenario.lote.id == execucao.cenario.lote.id
                                            && !r.cenario.lote.revisao_id.is_empty()
                                            && r.cenario.lote.revisao_id
                                                == execucao.cenario.lote.revisao_id);
                                        option value=(&execucao.id) disabled[!compativel] {
                                            (&execucao.cenario.nome)
                                            " · " (&execucao.cenario.lote.nome)
                                            " · revisão "
                                            (execucao.cenario.lote.revisao_id.chars()
                                                .take(8).collect::<String>())
                                            " · " (&execucao.id[..8])
                                            @if !compativel { " · lote ou revisão incompatível" } }
                                    }
                                }
                            }
                        }
                    }
                    @if sem_alternativa {
                        p role="status" { "Nenhuma alternativa compatível nesta página. "
                            "Ajuste os filtros ou navegue para outra página." }
                    }
                    button.button.secondary disabled[sem_alternativa] {
                        "Comparar premissas e custos" }
                }
                section.card.table-wrap {
                    table { thead { tr { th { "Cenário" } th { "Lote" }
                                th { "Busca / fornecimento" }
                                th { "Custo / animal / dia" } th {} } }
                        tbody { @for x in execucoes {
                                tr { td { (&x.cenario.nome) } td { (&x.cenario.lote.nome) }
                                    td {
                                        @if let Some(busca) = &x.parecer.busca {
                                            span.pill { (crate::web::formatacao::estado(busca)) }
                                        }
                                        p { (crate::web::formatacao::estado(
                                            x.parecer.estado_fornecimento(&x.cenario))) }
                                    }
                                    td { (x.parecer.custo_resultado().map(dinheiro)
                                            .unwrap_or("—".into()))
                                        @if x.cenario.formular
                                            && x.parecer.custo_resultado().is_some() {
                                            small { " · solução contínua" }
                                        } }
                                    td { a href=(format!("/execucoes/{}", x.id)) {
                                            "Ver parecer ↗" }
                                        " "
                                        a href=(filtro.url(pagina, &x.id)) {
                                            "Fixar como referência" }
                                    }
                                }
                            } }
                    }
                    @if execucoes.is_empty() { p.empty { "Nenhum cenário calculado ainda." } }
                }
            },
        }
        .render()
    }
}

//! Renderização do parecer e dos resultados de uma execução.
use maud::{Markup, Render, html};

use super::{cabecalho::Cabecalho, conceitos_resultado as conceitos};
use crate::dominio::Estado;
use crate::dominio::calculos::NUTRIENTES;
use crate::dominio::unidades::{dinheiro, preco_f64};
use crate::persistencia::historico::Execucao;

pub struct ResultadoExecucao<'a> {
    pub execucao: &'a Execucao,
    pub token: &'a str,
}

impl Render for ResultadoExecucao<'_> {
    fn render(&self) -> Markup {
        let e = self.execucao;
        let p = &e.parecer;
        let c = &e.cenario;
        let data = chrono::DateTime::parse_from_rfc3339(&e.criada_em)
            .ok()
            .map(|d| {
                d.with_timezone(&chrono_tz::America::Sao_Paulo)
                    .format("%d/%m/%Y às %H:%M")
                    .to_string()
            })
            .unwrap_or(e.criada_em.clone());
        html! {
            (Cabecalho { titulo: &c.nome, texto: &format!("{} · {} · Execução {}",
            c.lote.nome, data, e.id), ajuda: "resultados" })
            div.actions.no-print {
                a.button href=(format!("/execucoes/{}/duplicar", e.id))
                target="_blank" rel="noopener" title="Abre em nova guia" {
                    "Duplicar e simular" }
                button.button.secondary type="button" data-imprimir { "Imprimir parecer" }
                form method="post" action=(format!("/execucoes/{}/reproduzir", e.id)) {
                    input type="hidden" name="csrf" value=(self.token);
                    button.button.secondary { "Reproduzir versões originais" }
                }
            }
            div.alert {
                strong { (&p.condicao) }
                " · Atendimento ao perfil implementado. Não constitui \
                    avaliação nutricional integral nem previsão de produção ou
                        diagnóstico clínico."
            }
            section.stats {
                @for (nome, valor, dica) in [
                ("Custo / animal / dia", p.custo.map(dinheiro).unwrap_or("—".into()),
                "Oferta, perdas e sobras incluídas"),
                ("Consumo da dieta", format!("{:.3} kg MS", p.consumo_ms),
                "Por animal e por dia"),
                ("Custo do lote / dia", p.custo.map(|v| dinheiro(v * c.lote.animais
                as f64))
                .unwrap_or("—".into()),
                "Custo alimentar; outros custos não incluídos"),
                ("Horizonte", format!("{} dias", c.horizonte),
                "Estoque não é baixado pelo cálculo"),
                ] { article.stat { small { (nome) } strong { (valor) } span { (dica) } } }
            }
            section.card {
                div.concept-label.section-concept {
                    h2 { "Resultados por dimensão" } (conceitos::ESTADOS)
                }
                div.dimension-grid {
                    @for nome in ["Dados", "Elegibilidade", "Núcleo químico",
                    "Barreiras ruminais",
                    "Ingredientes", "Contaminantes", "Inclusão e estoque",
                    "Fibra física e micronutrientes"] {
                        @let estado = p.dimensao(nome);
                        div class=(format!("dimension {}", if estado == "ATENDE" { "ok" }
                        else if estado == "NAO_ATENDE" { "bad" } else { "neutral" })) {
                            div.concept-label {
                                small { (nome) }
                                @if nome == "Dados" { (conceitos::DADOS) }
                                @else if nome == "Elegibilidade" { (conceitos::ELEGIBILIDADE) }
                                @else if nome == "Núcleo químico" { (conceitos::NUCLEO) }
                                @else if nome == "Barreiras ruminais" { (conceitos::BARREIRAS) }
                                @else if nome == "Ingredientes" { (conceitos::INGREDIENTES) }
                                @else if nome == "Contaminantes" { (conceitos::CONTAMINANTES) }
                                @else if nome == "Inclusão e estoque" {
                                    (conceitos::INCLUSAO_ESTOQUE)
                                }
                                @else if nome == "Fibra física e micronutrientes" {
                                    (conceitos::FIBRA_FISICA)
                                }
                            }
                            strong { (estado.replace('_', " ")) }
                        }
                    }
                }
                @for aviso in &p.avisos { p.alert { (aviso) } }
            }
            section.card.table-wrap {
                div.section-title {
                    div.concept-label { h2 { "Balanço nutricional" } (conceitos::BALANCO) }
                    span.pill {
                        "BASE: MATÉRIA SECA" } }
                table {
                    thead { tr { th { "Nutriente" } th { "Oferta (kg/dia)" } th {
                                "Requisito" }
                            th { "Balanço" } th { "Déficit" } th { "Excesso" } th {
                                "Cobertura" } } }
                    tbody {
                        @for (i, nome) in NUTRIENTES.iter().enumerate().take(4) {
                            @let requisito = p.requisitos.map(|r| r[i]);
                            @let valor = p.totais[i];
                            tr {
                                td { strong { (nome) } }
                                td { (valor.map(|v|
                                    format!("{v:.5}")).unwrap_or("Desconhecido".into())) }
                                td { (requisito.map(|v|
                                    format!("{v:.5}")).unwrap_or("—".into())) }
                                td { (valor.zip(requisito).map(|(v, r)| format!("{:+.5}",
                                    v-r))
                                    .unwrap_or("—".into())) }
                                td { (valor.zip(requisito)
                                    .map(|(v,r)|format!("{:.5}",(r-v).max(0.0)))
                                    .unwrap_or("—".into())) }
                                td { (valor.zip(requisito)
                                    .map(|(v,r)|format!("{:.5}",(v-r).max(0.0)))
                                    .unwrap_or("—".into())) }
                                td { (valor.zip(requisito).filter(|(_, r)| *r > 0.0)
                                    .map(|(v, r)| format!("{:.1}%",
                                    100.0*v/r)).unwrap_or("—".into())) }
                            }
                        }
                    }
                }
                p.muted { "Balanço negativo = déficit; positivo = excesso sobre o mínimo. \
                        Excesso não é diagnóstico de intoxicação." }
            }
            section.card.table-wrap {
                div.concept-label.section-concept {
                    h2 { "Quantidades de fornecimento" } (conceitos::FORNECIMENTO)
                }
                p.muted {
                    "Consumo estimado e oferta são grandezas diferentes. MN: matéria natural." }
                table {
                    thead { tr { th { "Ingrediente" } th { "Consumo kg MS / animal" }
                            th { "Oferta kg MN / animal" } th { "Oferta kg MN / lote / dia" }
                            th { "Bruto kg MN / horizonte" } th {
                                "Oferta prática kg MN / animal" } } }
                    tbody {
                        @for f in &p.fornecimento {
                            tr { td { (&f.ingrediente) }
                                @for v in [f.consumo_ms, f.oferta_mn, f.oferta_lote,
                                f.necessidade_bruta, f.pratico_mn] { td {
                                        (format!("{v:.4}")) } }
                            }
                        }
                    }
                }
                p.alert { @if p.pratico_valido {
                        "Quantidades compatíveis com o núcleo e restrições \
                        avaliadas. Dimensões não avaliadas permanecem indicadas." }
                    @else {
                        "Fornecimento sem recomendação prática validada. Consulte as pendências \
                            e restrições; uma solução contínua pode continuar disponível." }
                }
            }
            @if let Some(busca) = &p.busca {
                section.card {
                    div.concept-label.section-concept {
                        h2 { "Formulação: " (busca) } (conceitos::OTIMO)
                    }
                    p { (&e.conhecimento.metodo) }
                    p.muted {
                        "Ótimo se refere ao modelo contínuo. Não prova mínimo global de \
                            ingredientes ou ótimo das quantidades arredondadas." }
                    @for s in &p.subproblemas {
                        p { strong { "FDN de forragem ≥ " (s.limiar) "%: " (&s.estado) }
                            " · " (&s.motivo) }
                    }
                }
            }
            section.card {
                h2 { "Premissas e economia" }
                dl.details-grid {
                    dt { "Consumo de referência" }
                    dd { (p.consumo_referencia.map(|v| format!("{v:.4} kg MS/animal/dia"))
                        .unwrap_or("Fora da Tabela B; informar consumo".into())) }
                    dt { "Origem do consumo" } dd { (&c.origem_consumo) }
                    dt { "Sobras planejadas" } dd { (format!("{:.2}% · {}", c.sobras*100.0,
                        c.origem_sobras)) }
                    dt { "Base da comparação" } dd { (&c.base_comparacao) }
                    dt { "Custo no horizonte" } dd { (p.custo.map(|v|
                        dinheiro(v*c.lote.animais as f64
                        * c.horizonte as f64)).unwrap_or("—".into())) }
                    @if let Some(custo) = p.custo {
                        @if c.lote.producao > 0.0 {
                            dt { "Custo por kg de leite" } dd { (dinheiro(custo /
                                c.lote.producao)) }
                        }
                        @if let Some(preco) = c.preco_leite.as_deref().and_then(preco_f64) {
                            dt { "Margem sobre alimentação / animal / dia" }
                            dd { (dinheiro(preco*c.lote.producao-custo))
                                " · não é lucro total" }
                        }
                    }
                }
            }
            (super::expansivel::Expansivel { id: "calculos-regras",
                aberto: false, titulo: html! { "Cálculos e regras aplicadas" }, conteudo: html! {
                p.muted { "Entradas → normalização → cálculo → comparação → conclusão. \
                        Todas as verificações independentes são preservadas." }
                ol { @for etapa in &p.caminho { li { (etapa) } } }
                @for (indice, v) in p.verificacoes.iter().enumerate() {
                    (super::expansivel::Expansivel { id: &format!("regra-{indice}-{}", v.regra),
                        aberto:
                        false, titulo: html! { span class=(if v.estado == Estado::Viola {
                                "pill danger" }
                            else { "pill" }) { (v.estado.rotulo()) }
                            " " strong { (&v.regra) } " · " (&v.dimensao)
                        }, conteudo: html! {
                            p { (&v.explicacao) }
                            @if let (Some(valor), Some(limite)) = (v.valor, v.limite) {
                                p.code { "Calculado: " (format!("{valor:.8}")) " · limite: "
                                    (format!("{limite:.8}")) " · diferença: "
                                    (format!("{:+.8}", valor-limite)) " " (&v.unidade) }
                            }
                            p.muted { "Fonte: " (&v.fonte) " · perfil "
                                (&e.conhecimento.perfil_id) }
                            p.muted { "Dependências: " (v.dependencias.join(" → ")) }
                        }, })
                }
            }, })
            (super::expansivel::Expansivel { id: "entradas-preservadas",
                aberto: false, titulo: html! { "Entradas preservadas nesta execução" },
                conteudo: html! { (super::entradas::Entradas(e)) }, })
            (super::expansivel::Expansivel { id: "fontes-versoes",
                aberto: false, titulo: html! { "Fontes e versões preservadas" }, conteudo: html! {
                p.code { (&e.conhecimento.perfil_id) " · versão " (&e.conhecimento.versao)
                    " · aplicação " (&e.conhecimento.versao_app) }
                @for f in &e.conhecimento.fontes {
                    (super::expansivel::Expansivel { id: &format!("fonte-{}", f.id), aberto:
                        false, titulo: html! { (&f.id) " · " (&f.titulo) }, conteudo:
                        html! {
                            p { (&f.edicao) " · " (&f.localizador) }
                            p { (&f.conteudo) } p.code { "SHA-256 " (&f.hash) }
                        }, })
                }
            }, })
        }
    }
}

//! Exibição das entradas preservadas em uma execução.
use crate::dominio::Conhecido;
use crate::persistencia::historico::Execucao;
use maud::{Markup, Render, html};

pub struct Entradas<'a>(pub &'a Execucao);
fn estado(v: Conhecido) -> &'static str {
    match v {
        Conhecido::Sim => "Sim",
        Conhecido::Nao => "Não",
        Conhecido::Desconhecido => "Desconhecido",
    }
}
impl Render for Entradas<'_> {
    fn render(&self) -> Markup {
        let c = &self.0.cenario;
        let l = &c.lote;
        html! {
            div {
                (super::expansivel::Expansivel { id: "lote-premissas",
                    aberto: false,
                    titulo: html! { "Lote, enquadramento e premissas" },
                    conteudo: html! {
                        dl.details-grid {
                            @for (nome,valor) in [
                            ("Lote",l.nome.clone()),
                            ("Revisão",l.revisao_id.clone()),
                            ("Raça",l.raca.clone()),
                            ("Animais",l.animais.to_string()),
                            ("Peso kg",l.peso.to_string()),
                            ("Produção kg/animal/dia",l.producao.to_string()),
                            ("Gordura %",l.gordura.to_string()),
                            ("Dias em lactação",l.dias_lactacao.to_string()),
                            ("Estado produtivo",format!("{:?}",l.estado_produtivo)),
                            ("Destinação econômica",l.destinacao.clone()),
                            ("Idade meses",l.idade_meses.map(|v|v.to_string())
                            .unwrap_or("Desconhecida".into())),
                            ("Adulta",estado(l.adulta).into()),
                            ("Gestante",estado(l.gestante).into()),
                            ("Crescimento",estado(l.crescimento).into()),
                            ("Peso estável",estado(l.peso_estavel).into()),
                            ("Confinada / dieta total",estado(l.confinada).into()),
                            ("Correção ambiental",estado(l.correcao_ambiente).into()),
                            ("Correção por atividade",estado(l.correcao_atividade).into()),
                            ("Acesso à água",estado(l.agua).into()),
                            ("Observações",l.observacoes.clone()),
                            ("Data de avaliação",c.data_avaliacao.clone()),
                            ("Data de aplicação das regras",c.data_aplicacao.clone()),
                            ("Referência UE ativada",c.referencia_ue.to_string()),
                            ("Limite da formulação",format!("{} segundos",c.limite_segundos)),
                            ] { dt { (nome) } dd { (valor) } }
                        }
                        @if let Some(m)=&c.medicao {
                            p { "Pesagem observada do lote: " (m.oferta_ms)
                                " kg MS de oferta − "
                                (m.sobras_ms) " kg MS de sobras, em "
                                (m.periodo_dias) " dias, "
                                (m.animais)
                                " animais. Composição ingerida não é inferida desta diferença." }
                        }
                    },
                })
                @if let Some(p) = &l.procedencia {
                    (super::modelos_preenchimento::Procedencia(p)) }
                @for item in &c.itens {
                    @let a=&item.alimento;
                    (super::expansivel::Expansivel { id: &format!("entrada-{}",
                        a.composicao_id), aberto: false,
                        titulo: html! { (&a.nome) " · " (&a.natureza) },
                        conteudo: html! {
                            @if let Some(p) = &a.procedencia {
                                (super::modelos_preenchimento::Procedencia(p)) }
                            p.code { "Composição " (&a.composicao_id) " · fonte "
                                (&a.fonte_id) }
                            dl.details-grid {
                                dt { "Quantidade original e significado" }
                                dd { (item.quantidade) " · "
                                    (format!("{:?}",item.base)) }
                                dt { "MS no cenário" } dd {
                                    (format!("{}% MN",a.ms*100.0)) }
                                dt { "Preço normalizado" } dd { (&item.preco)
                                    " R$/kg MN" }
                                @if let Some(cotacao)=&item.cotacao_original {
                                    dt { "Cotação original" }
                                    dd { (&cotacao.valor) " R$/" (&cotacao.unidade)
                                        @if let
                                        Some(massa)=cotacao.massa_embalagem {
                                            " · embalagem " (massa) " kg MN"
                                        }
                                    }
                                }
                                dt { "Origem e data da cotação" } dd {
                                    (&item.origem_preco) }
                                dt { "Estoque bruto kg MN" }
                                dd {
                                    (item.estoque.map(|v|v.to_string()).unwrap_or_else(|| {
                                        if item.sem_limite {
                                            "Hipótese: sem limite".into() }
                                        else { "Desconhecido".into() }
                                    })) }
                                dt { "Perda anterior ao fornecimento" }
                                dd { (format!("{}%",item.perda*100.0)) }
                                dt { "Inclusão kg MS/animal/dia" }
                                dd { "Mínimo " (item.minimo_ms) " · máximo "
                                    (item.maximo_ms.map(|v|v.to_string())
                                    .unwrap_or("Sem restrição adicional declarada".into())) }
                                dt { "Rótulo máximo kg MS/animal/dia" }
                                dd { (a.rotulo_max_ms.map(|v|v.to_string())
                                    .unwrap_or("Desconhecido".into())) }
                                dt { "Resolução de oferta kg MN/animal" } dd {
                                    (item.resolucao_mn) }
                                dt { "Origem e processamento" } dd { (&a.origem)
                                    " · "
                                    (&a.processamento) }
                                dt { "Método / data" } dd { (&a.metodo) " · "
                                    (&a.data) }
                                dt { "Produto / fornecedor / lote físico" }
                                dd { (&a.fornecedor) " · " (&a.lote_fisico) }
                                dt { "Evidência de autorização" } dd {
                                    (&a.evidencia) }
                                dt { "Origem animal / exceção legal" }
                                dd { (estado(a.origem_animal)) " / "
                                    (estado(a.excecao_animal)) }
                                dt { "NNP adicionado / medicamento" }
                                dd { (estado(a.nnp_adicionado)) " / "
                                    (estado(a.medicamento)) }
                            }
                            p { @for (i,nome) in
                                ["NDT","PB","Ca","P","FDN","Amido","EE"]
                                .iter().enumerate() {
                                    (nome) ": "
                                    (a.nutrientes[i].map(|v|format!("{}% MS",v*100.0))
                                    .unwrap_or("Desconhecido".into())) " · "
                                } }
                            @for laudo in &a.contaminantes {
                                p { (&laudo.analito) " · " (&laudo.matriz) " · "
                                    (&laudo.qualificador)
                                    " " (laudo.valor) " " (&laudo.unidade) " · MS "
                                    (laudo.ms*100.0)
                                    "% · método " (&laudo.metodo) " · laudo "
                                    (&laudo.documento) }
                            }
                        },
                    })
                }
            }
        }
    }
}

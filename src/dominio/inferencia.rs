use std::collections::BTreeSet;
use std::time::Instant;

use super::calculos::{NUTRIENTES, consumo_item, custo_item};
use super::unidades::TOLERANCIA;
use super::*;

/// Agenda finita e determinística: cada etapa deriva fatos uma vez, sem ciclos nem estado antigo.
pub fn verificar(c: &Cenario, base: &Conhecimento) -> Parecer {
    let inicio = Instant::now();
    let mut p = Parecer::default();
    let erros = validacao::validar(c);
    if !erros.is_empty() || c.perfil_id != base.perfil_id {
        p.condicao = "INVALIDO".into();
        for e in erros {
            p.registrar(
                "DAT-01",
                "Dados",
                Estado::Viola,
                format!("{}: {}", e.campo, e.mensagem),
            );
        }
        if c.perfil_id != base.perfil_id {
            p.registrar(
                "DAT-01",
                "Dados",
                Estado::Viola,
                "Perfil incompatível com artefatos.".into(),
            );
        }
        return p;
    }
    let mut fatos = BTreeSet::from([0]);
    for etapa in 0..8 {
        if !fatos.contains(&etapa) {
            break;
        }
        match etapa {
            0 => {
                p.lcg4 = calculos::lcg4(c.lote.producao, c.lote.gordura);
                if elegibilidade::avaliar(c, base, &mut p) {
                    p.requisitos =
                        calculos::requisitos(base, c.lote.peso, c.lote.producao, c.lote.gordura);
                }
                elegibilidade::ingredientes(c, base, &mut p);
            }
            1 => calcular(c, &mut p),
            2 => nutrientes(&mut p),
            3 => barreiras(base, &mut p),
            4 => restricoes(c, &mut p),
            5 => contaminantes::avaliar(c, base, &mut p),
            6 => {
                p.registrar(
                    "RUM-03",
                    "Fibra física e micronutrientes",
                    Estado::NaoAplicavel,
                    "peNDF, água, vitaminas, aminoácidos e micronutrientes sem perfil completo \
                     permanecem não avaliados; peneiras não são convertidas automaticamente."
                        .into(),
                );
            }
            7 => concluir(&mut p),
            _ => break,
        }
        fatos.insert(etapa + 1);
    }
    p.duracao_ms = inicio.elapsed().as_millis() as u64;
    p
}

fn calcular(c: &Cenario, p: &mut Parecer) {
    let (mut totais, consumo, fdnf) = calculos::totais(c);
    if c.itens
        .iter()
        .any(|i| i.alimento.natureza.starts_with("GARANTIA") && consumo_item(i, c.sobras) > 0.0)
    {
        totais = [None; 7];
        p.registrar(
            "DAT-02",
            "Dados",
            Estado::Desconhecido,
            "Garantia mínima/máxima não é composição pontual. Use análise ou média identificada."
                .into(),
        );
    }
    p.totais = totais;
    p.consumo_ms = consumo;
    p.fdn_forragem = fdnf;
    if consumo <= 0.0 {
        p.registrar(
            "DAT-01",
            "Dados",
            Estado::Viola,
            "A dieta deve ter MS total positiva.".into(),
        );
        p.condicao = "INVALIDO".into();
    }
    p.custo = Some(0.0);
    for item in &c.itens {
        let x = consumo_item(item, c.sobras);
        let custo = custo_item(item, x, c.sobras);
        p.custo = p.custo.zip(custo).map(|(a, b)| a + b);
        let mn = x / (item.alimento.ms * (1.0 - c.sobras));
        let bruto = mn * c.lote.animais as f64 * c.horizonte as f64 / (1.0 - item.perda);
        let pratico = if item.resolucao_mn > 0.0 {
            (mn / item.resolucao_mn).round() * item.resolucao_mn
        } else {
            mn
        };
        p.fornecimento.push(Fornecimento {
            ingrediente: item.alimento.nome.clone(),
            consumo_ms: x,
            oferta_mn: mn,
            oferta_lote: mn * c.lote.animais as f64,
            necessidade_bruta: bruto,
            pratico_mn: pratico,
            custo: custo.unwrap_or(0.0),
        });
    }
    if let Some(referencia) = p.consumo_referencia {
        let diferenca = (consumo - referencia).abs() / referencia;
        if diferenca > 0.10 && !super::unidades::proximos(diferenca, 0.10) {
            p.avisos.push(format!(
                "MS da dieta difere {:.2}% da referência; aviso operacional, \
                sem diagnóstico de capacidade de consumo.",
                100.0 * diferenca
            ));
        }
    }
    if c.itens.iter().any(|i| i.base == BaseQuantidade::OfertaMn) {
        p.avisos.push(
            "Ingestão estimada sob hipótese de mistura homogênea; não é consumo medido.".into(),
        );
    }
}

fn nutrientes(p: &mut Parecer) {
    if let Some(requisitos) = p.requisitos {
        for (i, limite) in requisitos.iter().enumerate() {
            let texto = format!(
                "{}: oferta − requisito = balanço; déficit/excesso e cobertura \
                calculados pela Tabela A. Excesso sobre mínimo não significa intoxicação.",
                NUTRIENTES[i]
            );
            p.comparar(Comparacao {
                regra: &format!("NUT-{:02}", i + 1),
                dimensao: "Núcleo químico",
                valor: p.totais[i],
                limite: *limite,
                maximo: false,
                sanitario: false,
                unidade: "kg/animal/dia",
                texto: &texto,
            });
        }
    } else {
        p.registrar(
            "NUT-01",
            "Núcleo químico",
            Estado::Desconhecido,
            "Exigências indisponíveis: resolva o enquadramento; não extrapolar o perfil.".into(),
        );
    }
}

fn barreiras(base: &Conhecimento, p: &mut Parecer) {
    if p.consumo_ms <= 0.0 {
        return;
    }
    let consumo = p.consumo_ms;
    let pct = |v: Option<f64>| v.map(|x| 100.0 * x / consumo);
    let fdnf = pct(p.fdn_forragem);
    let linha = fdnf.and_then(|v| {
        base.coeficientes
            .iter()
            .filter(|c| c.tabela == "fibra" && (c.eixo <= v || unidades::proximos(c.eixo, v)))
            .max_by(|a, b| a.eixo.total_cmp(&b.eixo))
    });
    p.comparar(Comparacao {
        regra: "RUM-01",
        dimensao: "Barreiras ruminais",
        valor: fdnf,
        limite: 15.0,
        maximo: false,
        sanitario: false,
        unidade: "% MS",
        texto: "FDN de forragem em % da MS total; maior limiar atendido da Tabela C.",
    });
    if let Some(linha) = linha {
        for (i, limite, maximo, nome) in [
            (
                4,
                linha.valores[0].max(base.fdn_min * 100.0),
                false,
                "FDN total",
            ),
            (
                5,
                linha.valores[1].min(base.amido_max * 100.0),
                true,
                "Amido",
            ),
            (6, base.ee_max * 100.0, true, "EE"),
        ] {
            p.comparar(Comparacao {
                regra: "RUM-02",
                dimensao: "Barreiras ruminais",
                valor: pct(p.totais[i]),
                limite,
                maximo,
                sanitario: false,
                unidade: "% MS",
                texto: &format!(
                    "{nome}: linha efetiva {} da Tabela C e POLITICA_MVP \
                    v1 (FDN ≥30%, amido ≤25%, EE ≤6%); não é limite legal brasileiro.",
                    linha.eixo
                ),
            });
        }
    }
}

fn restricoes(c: &Cenario, p: &mut Parecer) {
    for item in &c.itens {
        let x = consumo_item(item, c.sobras);
        for (valor, limite, maximo, nome) in [
            (Some(x), item.minimo_ms, false, "mínimo obrigatório"),
            (
                Some(x),
                item.maximo_ms.unwrap_or(p.consumo_ms),
                true,
                "máximo de inclusão",
            ),
        ] {
            p.comparar(Comparacao {
                regra: "FOR-01",
                dimensao: "Inclusão e estoque",
                valor,
                limite,
                maximo,
                sanitario: false,
                unidade: "kg MS/animal/dia",
                texto: &format!("{}: {nome}.", item.alimento.nome),
            });
        }
        if item.alimento.classe == Classe::Suplemento && x > TOLERANCIA {
            if let Some(teto) = item.alimento.rotulo_max_ms {
                p.comparar(Comparacao {
                    regra: "FOR-01",
                    dimensao: "Inclusão e estoque",
                    valor: Some(x),
                    limite: teto,
                    maximo: true,
                    sanitario: false,
                    unidade: "kg MS/animal/dia",
                    texto: &format!("{}: teto do rótulo.", item.alimento.nome),
                });
            } else {
                p.registrar(
                    "FOR-01",
                    "Inclusão e estoque",
                    Estado::Desconhecido,
                    format!(
                        "{}: falta limite de rótulo; não adotar inclusão livre.",
                        item.alimento.nome
                    ),
                );
            }
        }
        if let Some(estoque) = item.estoque {
            let bruto = x * c.lote.animais as f64 * c.horizonte as f64
                / (item.alimento.ms * (1.0 - c.sobras) * (1.0 - item.perda));
            p.comparar(Comparacao {
                regra: "FOR-01",
                dimensao: "Inclusão e estoque",
                valor: Some(bruto),
                limite: estoque,
                maximo: true,
                sanitario: false,
                unidade: "kg MN bruta/horizonte",
                texto: &format!(
                    "{}: estoque após perdas e sobras, contabilizadas uma vez.",
                    item.alimento.nome
                ),
            });
        } else if !item.sem_limite && x > TOLERANCIA {
            p.registrar(
                "FOR-01",
                "Inclusão e estoque",
                Estado::Desconhecido,
                format!(
                    "{}: estoque desconhecido; não equivale a disponibilidade infinita.",
                    item.alimento.nome
                ),
            );
        }
    }
}

fn concluir(p: &mut Parecer) {
    if p.condicao == "EM_AVALIACAO" {
        p.condicao = if p.dimensao("Ingredientes") != "ATENDE" {
            "BLOQUEADO"
        } else {
            "AVALIADO"
        }
        .into();
    }
    p.pratico_valido = [
        "Núcleo químico",
        "Barreiras ruminais",
        "Ingredientes",
        "Inclusão e estoque",
    ]
    .iter()
    .all(|d| p.dimensao(d) == "ATENDE")
        && p.condicao == "AVALIADO";
    p.verificacoes.sort_by(|a, b| a.regra.cmp(&b.regra));
}

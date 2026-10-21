use super::calculos::{NUTRIENTES, consumo_item};
use super::unidades::TOLERANCIA;
use super::*;

pub(crate) fn nutrientes(p: &mut Parecer) {
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

pub(crate) fn barreiras(base: &Conhecimento, p: &mut Parecer) {
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

pub(crate) fn restricoes(c: &Cenario, p: &mut Parecer) {
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
            if let Some(teto) = item.teto_suplemento() {
                p.comparar(Comparacao {
                    regra: "FOR-01",
                    dimensao: "Inclusão e estoque",
                    valor: Some(x),
                    limite: teto,
                    maximo: true,
                    sanitario: false,
                    unidade: "kg MS/animal/dia",
                    texto: &format!(
                        "{}: {}.",
                        item.alimento.nome,
                        if item.alimento.rotulo_max_ms.is_some() {
                            "teto do rótulo"
                        } else {
                            "hipótese explícita de teto no cenário sintético"
                        }
                    ),
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

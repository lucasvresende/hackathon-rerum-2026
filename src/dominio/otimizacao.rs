use std::time::{Duration, Instant};

use microlp::{ComparisonOp, OptimizationDirection, Problem, SolutionStatus, SolveOutcome};

use super::calculos::custo_item;
use super::unidades::{TOLERANCIA, proximos};
use super::*;

pub fn formular(c: &Cenario, base: &Conhecimento) -> Parecer {
    let inicio = Instant::now();
    let mut sonda = c.clone();
    sonda.formular = false;
    let mut diagnostico = inferencia::verificar(&sonda, base);
    let alvo = c.consumo_alvo.or(diagnostico.consumo_referencia);
    if !validacao::validar(c).is_empty() || alvo.is_none() || diagnostico.requisitos.is_none() {
        diagnostico.busca = Some("INCONCLUSIVA".into());
        return diagnostico;
    }
    let alvo = alvo.expect("consumo verificado");
    let requisitos = diagnostico.requisitos.expect("requisitos verificados");
    let mut ordenado = c.clone();
    ordenado
        .itens
        .sort_by(|a, b| a.alimento.id.cmp(&b.alimento.id));
    let mut candidatas: Vec<(f64, Vec<f64>, Parecer)> = Vec::new();
    let mut estados = Vec::new();
    for linha in base.coeficientes.iter().filter(|r| r.tabela == "fibra") {
        let restante = c.limite_segundos - inicio.elapsed().as_secs_f64();
        let mut estado = Subproblema {
            limiar: linha.eixo,
            estado: "INCONCLUSIVA".into(),
            motivo: String::new(),
        };
        if restante <= 0.0 {
            estado.motivo = "Limite total de tempo atingido.".into();
            estados.push(estado);
            continue;
        }
        let mut problema = Problem::new(OptimizationDirection::Minimize);
        problema.set_time_limit(Duration::from_secs_f64(restante));
        let mut variaveis = Vec::new();
        let mut invalido = false;
        for item in &ordenado.itens {
            let mut teste = ordenado.clone();
            teste.itens = vec![item.clone()];
            let mut seguranca = Parecer::default();
            elegibilidade::ingredientes(&teste, base, &mut seguranca);
            let excluido = seguranca.dimensao("Ingredientes") != "ATENDE";
            let mut teto = item.maximo_ms.unwrap_or(alvo).min(alvo);
            if item.alimento.classe == Classe::Suplemento {
                if let Some(rotulo) = item.alimento.rotulo_max_ms {
                    teto = teto.min(rotulo);
                } else {
                    invalido = true;
                }
            }
            if let Some(estoque) = item.estoque {
                teto = teto.min(
                    estoque * item.alimento.ms * (1.0 - c.sobras) * (1.0 - item.perda)
                        / (c.lote.animais as f64 * c.horizonte as f64),
                );
            } else if !item.sem_limite {
                invalido = true;
            }
            if excluido {
                teto = 0.0;
            }
            if item.alimento.nutrientes.iter().any(Option::is_none)
                || item.alimento.natureza.starts_with("GARANTIA")
            {
                invalido = true;
            }
            if item.minimo_ms > teto {
                estado.estado = "INVIAVEL_NO_MODELO".into();
                estado.motivo = "Mínimo obrigatório excede estoque, rótulo ou exclusão.".into();
                invalido = true;
            }
            variaveis.push(problema.add_var(
                custo_item(item, 1.0, c.sobras).unwrap_or(0.0),
                (item.minimo_ms, teto.max(item.minimo_ms)),
            ));
        }
        if invalido {
            if estado.motivo.is_empty() {
                estado.motivo =
                    "Dados obrigatórios, autorização, composição ou estoque pendentes.".into();
            }
            estados.push(estado);
            continue;
        }
        let expressao = |coef: &dyn Fn(&Item) -> f64| {
            variaveis
                .iter()
                .zip(&ordenado.itens)
                .map(|(v, i)| (*v, coef(i)))
                .collect::<Vec<_>>()
        };
        problema.add_constraint(expressao(&|_| 1.0), ComparisonOp::Eq, alvo);
        for (i, requisito) in requisitos.iter().enumerate() {
            problema.add_constraint(
                expressao(&|it| it.alimento.nutrientes[i].unwrap_or(0.0)),
                ComparisonOp::Ge,
                *requisito,
            );
        }
        for (i, op, limite) in [
            (
                4,
                ComparisonOp::Ge,
                (linha.valores[0] / 100.0).max(base.fdn_min),
            ),
            (
                5,
                ComparisonOp::Le,
                (linha.valores[1] / 100.0).min(base.amido_max),
            ),
            (6, ComparisonOp::Le, base.ee_max),
        ] {
            problema.add_constraint(
                expressao(&|it| it.alimento.nutrientes[i].unwrap_or(0.0)),
                op,
                alvo * limite,
            );
        }
        problema.add_constraint(
            expressao(&|it| {
                if it.alimento.classe == Classe::Forragem {
                    it.alimento.nutrientes[4].unwrap_or(0.0)
                } else {
                    0.0
                }
            }),
            ComparisonOp::Ge,
            alvo * linha.eixo / 100.0,
        );
        match problema.solve() {
            Ok(SolveOutcome::Solution(solucao)) => {
                let mut candidata = ordenado.clone();
                let mut quantidades = Vec::new();
                for (item, v) in candidata.itens.iter_mut().zip(&variaveis) {
                    item.quantidade = solucao.var_value(*v);
                    item.base = BaseQuantidade::ConsumoMs;
                    quantidades.push(item.quantidade);
                }
                let mut p = inferencia::verificar(&candidata, base);
                if p.pratico_valido && proximos(p.consumo_ms, alvo) {
                    estado.estado = if solucao.status() == SolutionStatus::Optimal {
                        "OTIMO_NO_MODELO"
                    } else {
                        "VIAVEL_SEM_OTIMO"
                    }
                    .into();
                    estado.motivo = format!(
                        "{:?}; candidata revalidada pela linha efetiva.",
                        solucao.termination_reason()
                    );
                    p.busca = Some(estado.estado.clone());
                    candidatas.push((p.custo.unwrap_or(f64::INFINITY), quantidades, p));
                } else {
                    estado.motivo = "Candidata não passou na revalidação independente.".into();
                }
            }
            Ok(SolveOutcome::Interrupted(interrupcao)) => {
                estado.motivo = format!("Interrompida: {:?}", interrupcao.termination_reason());
            }
            Err(microlp::Error::Infeasible) => {
                estado.estado = "INVIAVEL_NO_MODELO".into();
                estado.motivo = "Inviabilidade comprovada pelo solver.".into();
            }
            Err(erro) => {
                estado.motivo = format!("Falha numérica/método: {erro}");
            }
        }
        estados.push(estado);
    }
    candidatas.sort_by(|a, b| {
        if !proximos(a.0, b.0) {
            return a.0.total_cmp(&b.0);
        }
        let positivos = |v: &[f64]| v.iter().filter(|q| **q > TOLERANCIA).count();
        positivos(&a.1).cmp(&positivos(&b.1)).then_with(|| {
            a.1.iter()
                .zip(&b.1)
                .map(|(x, y)| x.total_cmp(y))
                .find(|o| !o.is_eq())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    });
    let comprovados = estados
        .iter()
        .all(|s| s.estado == "OTIMO_NO_MODELO" || s.estado == "INVIAVEL_NO_MODELO");
    let mut p = if let Some((_, quantidades, p)) = candidatas.into_iter().next() {
        let mut pratica = ordenado.clone();
        for ((item, q), f) in pratica
            .itens
            .iter_mut()
            .zip(quantidades)
            .zip(&p.fornecimento)
        {
            item.quantidade = q;
            item.base = BaseQuantidade::ConsumoMs;
            if item.resolucao_mn > 0.0 {
                item.quantidade = f.pratico_mn * item.alimento.ms * (1.0 - c.sobras);
            }
        }
        let verificada = inferencia::verificar(&pratica, base);
        let mut p = p;
        p.pratico_valido = verificada.pratico_valido && proximos(verificada.consumo_ms, alvo);
        if !p.pratico_valido {
            p.avisos.push(
                "FORNECIMENTO_INCOMPATIVEL: arredondamento viola o modelo; \
                solução contínua preservada. Não há certificado de ótimo para quantidades práticas."
                    .into(),
            );
        }
        p.busca = Some(
            if comprovados {
                "OTIMO_NO_MODELO"
            } else {
                "VIAVEL_SEM_OTIMO"
            }
            .into(),
        );
        p
    } else {
        diagnostico.busca = Some(
            if estados.iter().all(|s| s.estado == "INVIAVEL_NO_MODELO") {
                "INVIAVEL_NO_MODELO"
            } else {
                "INCONCLUSIVA"
            }
            .into(),
        );
        diagnostico.pratico_valido = false;
        diagnostico
    };
    p.subproblemas = estados;
    p.duracao_ms = inicio.elapsed().as_millis() as u64;
    p
}

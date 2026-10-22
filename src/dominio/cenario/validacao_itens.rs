//! Validação de ingredientes, quantidades, preços, disponibilidade e limites do cenário.
use super::validacao::{Problema, problema, validar_alimento};
use super::{Cenario, unidades::preco_f64};
use std::collections::BTreeSet;

pub fn validar(c: &Cenario) -> Vec<Problema> {
    let mut erros = vec![];
    let mut ids = BTreeSet::new();
    for item in &c.itens {
        let id = &item.alimento.composicao_id;
        let campo = |prefixo: &str| format!("{prefixo}-{id}");
        if !ids.insert(&item.alimento.id) {
            erros.push(problema(
                &campo("usar"),
                "Selecione uma composição por ingrediente.",
            ));
        }
        erros.extend(
            validar_alimento(&item.alimento)
                .into_iter()
                .map(|mut erro| {
                    erro.campo = if erro.campo == "ms" {
                        campo("ms")
                    } else {
                        format!("ingrediente-{id}")
                    };
                    erro
                }),
        );
        for (prefixo, valor) in [
            ("q", item.quantidade),
            ("min", item.minimo_ms),
            ("resolucao", item.resolucao_mn),
        ] {
            if !valor.is_finite() || valor < 0.0 {
                erros.push(problema(
                    &campo(prefixo),
                    "Informe valor finito não negativo.",
                ));
            }
        }
        if !item.perda.is_finite() || !(0.0..1.0).contains(&item.perda) {
            erros.push(problema(
                &campo("perda"),
                "Perda deve ser de 0 a 100% exclusive.",
            ));
        }
        if item
            .maximo_ms
            .is_some_and(|v| !v.is_finite() || v < item.minimo_ms)
        {
            erros.push(problema(
                &campo("max"),
                "Máximo deve ser finito e não inferior ao mínimo.",
            ));
        }
        if item.estoque.is_some_and(|v| !v.is_finite() || v < 0.0) {
            erros.push(problema(
                &campo("estoque"),
                "Estoque deve ser finito e não negativo.",
            ));
        }
        if preco_f64(&item.preco).is_none_or(|v| v < 0.0) {
            erros.push(problema(
                &campo("preco"),
                "Informe preço decimal não negativo.",
            ));
        }
        if item.sem_limite && item.estoque.is_some() {
            erros.push(problema(
                &campo("estoque"),
                "Escolha estoque ou hipótese sem limite.",
            ));
        }
    }
    erros
}

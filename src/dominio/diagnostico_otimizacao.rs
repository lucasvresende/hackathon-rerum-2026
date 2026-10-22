//! Explicações de inviabilidade apoiadas por limites superiores matemáticos conservadores.
use super::{Cenario, Classe};

fn numero(v: f64) -> String {
    format!("{v:.4}").replace('.', ",")
}

/// Só atribui uma causa quando um limite superior já é inferior ao mínimo exigido.
/// Sem essa prova, orienta a revisão do conjunto, sem inventar uma restrição responsável.
pub(crate) fn inviabilidade(c: &Cenario, alvo: f64, requisitos: [f64; 4], limiar: f64) -> String {
    let forragens: Vec<_> = c
        .itens
        .iter()
        .filter(|i| i.alimento.classe == Classe::Forragem)
        .collect();
    let max_fibra: f64 = forragens
        .iter()
        .map(|i| {
            let estoque = i.estoque.map(|s| {
                s * i.alimento.ms * (1.0 - c.sobras) * (1.0 - i.perda)
                    / (c.lote.animais as f64 * c.horizonte as f64)
            });
            let teto = i
                .maximo_ms
                .unwrap_or(alvo)
                .min(alvo)
                .min(estoque.unwrap_or(alvo));
            teto * i.alimento.nutrientes[4].unwrap_or(0.0)
        })
        .sum();
    let minimo = alvo * limiar / 100.0;
    if max_fibra < minimo && !super::unidades::proximos(max_fibra, minimo) {
        let nomes = forragens
            .iter()
            .map(|i| i.alimento.nome.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        return format!(
            "Inviabilidade comprovada. Com os limites de inclusão e estoque de {}, \
            a FDN de forragem não supera {} kg MS/animal/dia; este subproblema exige {}. \
            Revise a disponibilidade ou a seleção de forragens.",
            if nomes.is_empty() {
                "nenhuma forragem selecionada"
            } else {
                &nomes
            },
            numero(max_fibra),
            numero(minimo)
        );
    }
    for (j, requisito) in requisitos.iter().enumerate() {
        let teor_maximo = c
            .itens
            .iter()
            .filter_map(|i| i.alimento.nutrientes[j])
            .fold(0.0_f64, f64::max);
        let maximo = alvo * teor_maximo;
        if maximo < *requisito && !super::unidades::proximos(maximo, *requisito) {
            return format!(
                "Inviabilidade comprovada. Mesmo o maior teor de {} disponível não permite \
                superar {} kg/animal/dia no consumo-alvo; o requisito é {}. \
                Revise o consumo-alvo e os ingredientes selecionados.",
                super::calculos::NUTRIENTES[j],
                numero(maximo),
                numero(*requisito)
            );
        }
    }
    "Inviabilidade comprovada pelo solver para o conjunto de restrições. \
    Revise consumo-alvo, disponibilidade, inclusão e composição dos ingredientes. \
    Não foi isolada uma única restrição responsável; nenhum limite foi relaxado."
        .into()
}

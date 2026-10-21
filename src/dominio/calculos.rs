use super::unidades::{MassaMn, MassaMs, preco_f64};
use super::{BaseQuantidade, Cenario, Conhecimento, Item};

pub const NUTRIENTES: [&str; 7] = ["NDT", "PB", "Ca", "P", "FDN", "Amido", "EE"];

pub fn lcg4(producao: f64, gordura: f64) -> f64 {
    producao * (0.4 + 0.15 * gordura)
}

/// Interpola linearmente entre nós e extrapola linearmente pelas bordas extremas.
pub fn interpolar(pontos: &[(f64, f64)], eixo: f64) -> Option<f64> {
    if !eixo.is_finite() || pontos.is_empty() {
        return None;
    }
    if let Some(ponto) = pontos.iter().find(|(x, _)| *x == eixo) {
        return Some(ponto.1);
    }
    if pontos.len() == 1 {
        return Some(pontos[0].1);
    }
    let primeiro = &pontos[0];
    if eixo < primeiro.0 {
        let segundo = &pontos[1];
        let inclinacao = (segundo.1 - primeiro.1) / (segundo.0 - primeiro.0);
        return Some(primeiro.1 + inclinacao * (eixo - primeiro.0));
    }
    let ultimo = &pontos[pontos.len() - 1];
    if eixo > ultimo.0 {
        let penultimo = &pontos[pontos.len() - 2];
        let inclinacao = (ultimo.1 - penultimo.1) / (ultimo.0 - penultimo.0);
        return Some(ultimo.1 + inclinacao * (eixo - ultimo.0));
    }
    pontos.windows(2).find_map(|par| {
        let [(x0, y0), (x1, y1)] = par else {
            return None;
        };
        if eixo > *x0 && eixo < *x1 {
            Some(y0 + (y1 - y0) * (eixo - x0) / (x1 - x0))
        } else {
            None
        }
    })
}

pub fn requisitos(base: &Conhecimento, peso: f64, leite: f64, gordura: f64) -> Option<[f64; 4]> {
    let mut valores = [0.0; 4];
    for (i, valor) in valores.iter_mut().enumerate() {
        let pontos = |tabela: &str| {
            let mut p: Vec<_> = base
                .coeficientes
                .iter()
                .filter(|c| c.tabela == tabela)
                .map(|c| (c.eixo, c.valores[i]))
                .collect();
            p.sort_by(|a, b| a.0.total_cmp(&b.0));
            p
        };
        *valor =
            interpolar(&pontos("mantenca"), peso)? + leite * interpolar(&pontos("leite"), gordura)?;
    }
    Some(valores)
}

pub fn consumo_referencia(base: &Conhecimento, peso: f64, lcg: f64) -> Option<f64> {
    let mut linhas = Vec::new();
    for producao in [5.0, 10.0, 15.0, 20.0, 25.0, 30.0] {
        let mut pontos: Vec<_> = base
            .coeficientes
            .iter()
            .filter(|c| c.tabela == "consumo" && c.eixo == producao)
            .map(|c| (c.eixo2, c.valores[0]))
            .collect();
        pontos.sort_by(|a, b| a.0.total_cmp(&b.0));
        linhas.push((producao, interpolar(&pontos, peso)?));
    }
    Some(peso * interpolar(&linhas, lcg)? / 100.0)
}

pub fn consumo_item(item: &Item, sobra: f64) -> f64 {
    match item.base {
        BaseQuantidade::ConsumoMs => item.quantidade,
        BaseQuantidade::ConsumoMn => MassaMn(item.quantidade).materia_seca(item.alimento.ms).0,
        BaseQuantidade::OfertaMn => item.quantidade * item.alimento.ms * (1.0 - sobra),
    }
}

pub fn custo_item(item: &Item, consumo_ms: f64, sobra: f64) -> Option<f64> {
    Some(
        MassaMs(consumo_ms).materia_natural(item.alimento.ms).0 * preco_f64(&item.preco)?
            / ((1.0 - sobra) * (1.0 - item.perda)),
    )
}

pub fn totais(cenario: &Cenario) -> ([Option<f64>; 7], f64, Option<f64>) {
    let mut total = [Some(0.0); 7];
    let mut consumo = 0.0;
    let mut fdn_forragem = Some(0.0);
    for item in &cenario.itens {
        let x = consumo_item(item, cenario.sobras);
        consumo += x;
        if x == 0.0 {
            continue;
        }
        for (i, valor) in total.iter_mut().enumerate() {
            *valor = valor
                .zip(item.alimento.nutrientes[i])
                .map(|(s, a)| s + x * a);
        }
        if item.alimento.classe == super::Classe::Forragem {
            fdn_forragem = fdn_forragem
                .zip(item.alimento.nutrientes[4])
                .map(|(s, a)| s + x * a);
        }
    }
    (total, consumo, fdn_forragem)
}

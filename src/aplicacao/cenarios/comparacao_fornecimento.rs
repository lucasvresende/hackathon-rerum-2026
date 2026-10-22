//! Associação de resultados preservados, sem reconstruir quantidades da entrada do solver.
use crate::dominio::{Fornecimento, Item};
use crate::persistencia::historico::Execucao;

pub fn tem_resultado(e: &Execucao) -> bool {
    !e.cenario.formular
        || matches!(
            e.parecer.busca.as_deref(),
            Some("OTIMO_NO_MODELO" | "VIAVEL_SEM_OTIMO")
        )
}

/// O nome só identifica registros legados quando é único tanto na entrada quanto no parecer.
pub fn fornecimento<'a>(
    e: &'a Execucao,
    item: &Item,
    pratico: bool,
) -> Result<&'a Fornecimento, &'static str> {
    if !tem_resultado(e) {
        return Err("Indisponível: busca sem candidata");
    }
    let p = if pratico {
        if item.resolucao_mn <= 0.0 {
            return Err("Indisponível: resolução de pesagem não definida");
        }
        e.parecer
            .pratico
            .as_deref()
            .ok_or("Indisponível: pesagem não definida")?
    } else {
        &e.parecer
    };
    let id = &item.alimento.composicao_id;
    let exatos: Vec<_> = p
        .fornecimento
        .iter()
        .filter(|f| !id.is_empty() && f.composicao_id == *id)
        .collect();
    match exatos.as_slice() {
        [f] => return Ok(f),
        [] => {}
        _ => return Err("Indisponível: associação ambígua da composição"),
    }
    let legados: Vec<_> = p
        .fornecimento
        .iter()
        .filter(|f| f.composicao_id.is_empty() && f.ingrediente == item.alimento.nome)
        .collect();
    let nomes = e
        .cenario
        .itens
        .iter()
        .filter(|i| i.alimento.nome == item.alimento.nome)
        .count();
    match legados.as_slice() {
        [f] if nomes == 1 => Ok(f),
        [] => Err("Indisponível: resultado da composição não encontrado"),
        _ => Err("Indisponível: associação ambígua pelo nome"),
    }
}

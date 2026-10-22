//! Copia entradas ou resultados preservados sem confundir consumo com oferta arredondada.
use crate::dominio::{BaseQuantidade, Cenario};
use crate::persistencia::historico::Execucao;
use serde::Deserialize;

#[derive(Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrigemDieta {
    #[default]
    Simular,
    Continua,
    Pratica,
}

/// A associação usa a composição, nunca a posição das linhas ordenadas pelo otimizador.
/// Históricos sem ID só admitem nome único. Quantidades práticas continuam sendo oferta MN.
pub fn copiar(e: &Execucao, origem: OrigemDieta) -> Result<Cenario, &'static str> {
    let candidata = e.cenario.formular
        && matches!(
            e.parecer.busca.as_deref(),
            Some("OTIMO_NO_MODELO" | "VIAVEL_SEM_OTIMO")
        );
    if origem != OrigemDieta::Simular && !candidata {
        return Err("Esta execução não contém uma solução otimizada para reutilizar.");
    }
    if origem == OrigemDieta::Pratica
        && (e.parecer.pratico.is_none() || e.parecer.resolucao_pendente(&e.cenario))
    {
        return Err("Defina a resolução dos ingredientes e recalcule antes de copiar a pesagem.");
    }
    let mut c = e.cenario.clone();
    c.original_id = Some(e.cenario_id.clone());
    let sufixo = match origem {
        OrigemDieta::Simular => " · simulação",
        OrigemDieta::Continua => " · verificar solução contínua",
        OrigemDieta::Pratica => " · verificar quantidades práticas",
    };
    c.nome = format!(
        "{}{}",
        c.nome
            .chars()
            .take(150 - sufixo.chars().count())
            .collect::<String>(),
        sufixo
    );
    if candidata {
        for item in &mut c.itens {
            let linhas: Vec<_> = e
                .parecer
                .fornecimento
                .iter()
                .filter(|f| {
                    if f.composicao_id.is_empty() {
                        f.ingrediente == item.alimento.nome
                            && e.cenario
                                .itens
                                .iter()
                                .filter(|i| i.alimento.nome == f.ingrediente)
                                .count()
                                == 1
                    } else {
                        f.composicao_id == item.alimento.composicao_id
                    }
                })
                .collect();
            let [f] = linhas.as_slice() else {
                return Err(
                    "Não foi possível identificar unicamente as quantidades da composição.",
                );
            };
            if origem == OrigemDieta::Pratica {
                item.quantidade = f.pratico_mn;
                item.base = BaseQuantidade::OfertaMn;
            } else {
                item.quantidade = f.consumo_ms;
                item.base = BaseQuantidade::ConsumoMs;
            }
        }
    }
    if origem != OrigemDieta::Simular {
        c.formular = false;
    }
    Ok(c)
}

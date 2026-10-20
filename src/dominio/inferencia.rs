use std::collections::BTreeSet;
use std::time::Instant;

use super::avaliacoes::{barreiras, nutrientes, restricoes};
use super::calculos::{consumo_item, custo_item};
use super::*;

/// Agenda finita e determinística: cada etapa deriva fatos uma vez, sem ciclos nem estado antigo.
pub fn verificar(c: &Cenario, base: &Conhecimento) -> Parecer {
    let mut p = verificar_continua(c, base);
    if c.itens.iter().any(|i| i.resolucao_mn > 0.0) && p.pratico_valido {
        let mut pratico = c.clone();
        for (item, f) in pratico.itens.iter_mut().zip(&p.fornecimento) {
            item.quantidade = f.pratico_mn * item.alimento.ms * (1.0 - c.sobras);
            item.base = BaseQuantidade::ConsumoMs;
            item.resolucao_mn = 0.0;
        }
        let revalidado = verificar_continua(&pratico, base);
        p.pratico_valido = revalidado.pratico_valido;
        if !p.pratico_valido {
            p.avisos.push(
                "FORNECIMENTO_INCOMPATIVEL: quantidades arredondadas \
                violam restrições. Solução contínua preservada."
                    .into(),
            );
        }
    }
    p
}

pub(crate) fn verificar_continua(c: &Cenario, base: &Conhecimento) -> Parecer {
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
    p.registrar(
        "DAT-01",
        "Dados",
        Estado::Atende,
        "Entradas finitas, unidades canônicas e referências do cenário validadas.".into(),
    );
    let mut fatos = BTreeSet::from([0]);
    for etapa in 0..8 {
        if !fatos.contains(&etapa) {
            break;
        }
        p.caminho.push(
            match etapa {
                0 => "Validar e enquadrar lote e ingredientes",
                1 => "Normalizar e calcular a dieta",
                2 => "Comparar exigências químicas",
                3 => "Selecionar degrau e barreiras ruminais",
                4 => "Conferir inclusão, rótulo e estoque",
                5 => "Avaliar laudos e referências ativas",
                6 => "Identificar dimensões não avaliadas",
                _ => "Concluir por dimensão e preservar traço",
            }
            .into(),
        );
        match etapa {
            0 => {
                p.lcg4 = calculos::lcg4(c.lote.producao, c.lote.gordura);
                if elegibilidade::avaliar(c, base, &mut p) {
                    p.requisitos =
                        calculos::requisitos(base, c.lote.peso, c.lote.producao, c.lote.gordura);
                }
                elegibilidade::ingredientes(c, base, &mut p);
            }
            1 => calcular(c, base, &mut p),
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

fn calcular(c: &Cenario, base: &Conhecimento, p: &mut Parecer) {
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
        if diferenca > base.discrepancia && !super::unidades::proximos(diferenca, base.discrepancia)
        {
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
    if p.verificacoes.iter().any(|v| v.regra == "CON-01") {
        p.pratico_valido &= p.dimensao("Contaminantes") == "ATENDE";
    }
    p.verificacoes.sort_by(|a, b| a.regra.cmp(&b.regra));
}

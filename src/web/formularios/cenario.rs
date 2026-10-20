use super::*;
use crate::dominio::{
    Alimento, BaseQuantidade, Cenario, CotacaoOriginal, Item, Lote, MedicaoConsumo,
};

pub fn ler_cenario(
    c: &Campos,
    lotes: &[Lote],
    alimentos: &[Alimento],
) -> Result<Cenario, Vec<Problema>> {
    let mut erros = Vec::new();
    let Some(lote) = lotes.iter().find(|l| l.id == texto(c, "lote")) else {
        return Err(vec![problema("lote", "Selecione um lote cadastrado.")]);
    };
    let base = match texto(c, "base") {
        "CONSUMO_MN" => BaseQuantidade::ConsumoMn,
        "OFERTA_MN" => BaseQuantidade::OfertaMn,
        "CONSUMO_MS" => BaseQuantidade::ConsumoMs,
        _ => {
            erros.push(problema(
                "base",
                "Declare a unidade e o significado da quantidade.",
            ));
            BaseQuantidade::ConsumoMs
        }
    };
    let mut itens = Vec::new();
    for a in alimentos {
        let chave = |prefixo: &str| format!("{prefixo}-{}", a.composicao_id);
        if texto(c, &chave("usar")) != "on" {
            continue;
        }
        let mut alimento = a.clone();
        alimento.ms = obrigatorio(c, &chave("ms"), &mut erros) / 100.0;
        let mut quantidade = numero(c, &chave("q"), &mut erros).unwrap_or(0.0);
        if alimento.ms > 0.0 {
            if texto(c, "base_comparacao") == "Quantidade de MN fixa"
                && base == BaseQuantidade::ConsumoMs
            {
                quantidade *= alimento.ms / a.ms;
            } else if texto(c, "base_comparacao") == "Consumo de MS fixo"
                && base != BaseQuantidade::ConsumoMs
            {
                quantidade *= a.ms / alimento.ms;
            }
        }
        let cotacao = CotacaoOriginal {
            valor: texto(c, &chave("preco")).replace(',', "."),
            unidade: texto(c, &chave("unidade-preco")).into(),
            massa_embalagem: numero(c, &chave("embalagem"), &mut erros),
        };
        let preco = crate::dominio::unidades::normalizar_preco(&cotacao).unwrap_or_else(|| {
            erros.push(problema(
                &chave("preco"),
                "Preço ou unidade comercial inválida.",
            ));
            String::new()
        });
        itens.push(Item {
            alimento,
            quantidade,
            base,
            preco,
            cotacao_original: Some(cotacao),
            origem_preco: texto(c, &chave("origem")).into(),
            perda: obrigatorio(c, &chave("perda"), &mut erros) / 100.0,
            estoque: numero(c, &chave("estoque"), &mut erros),
            sem_limite: texto(c, &chave("livre")) == "on",
            minimo_ms: numero(c, &chave("min"), &mut erros).unwrap_or(0.0),
            maximo_ms: numero(c, &chave("max"), &mut erros),
            resolucao_mn: numero(c, &chave("resolucao"), &mut erros).unwrap_or(0.0),
        });
    }
    let cenario = Cenario {
        nome: texto(c, "nome").into(),
        original_id: c.get("original_id").filter(|v| !v.is_empty()).cloned(),
        lote: lote.clone(),
        perfil_id: "E2020_NRC1989_RUMEN_MVP_v1".into(),
        data_avaliacao: texto(c, "data_avaliacao").into(),
        data_aplicacao: texto(c, "data_aplicacao").into(),
        horizonte: inteiro(c, "horizonte", &mut erros),
        sobras: obrigatorio(c, "sobras", &mut erros) / 100.0,
        origem_sobras: texto(c, "origem_sobras").into(),
        consumo_alvo: numero(c, "consumo_alvo", &mut erros),
        medicao: if texto(c, "oferta_observada").is_empty() {
            None
        } else {
            Some(MedicaoConsumo {
                oferta_ms: obrigatorio(c, "oferta_observada", &mut erros),
                sobras_ms: obrigatorio(c, "sobras_observadas", &mut erros),
                periodo_dias: obrigatorio(c, "periodo_observado", &mut erros),
                animais: lote.animais,
            })
        },
        origem_consumo: texto(c, "origem_consumo").into(),
        referencia_ue: texto(c, "ue") == "on",
        preco_leite: c
            .get("preco_leite")
            .filter(|v| !v.is_empty())
            .map(|v| v.replace(',', ".")),
        base_comparacao: texto(c, "base_comparacao").into(),
        formular: texto(c, "modo") == "formular",
        limite_segundos: 10.0,
        itens,
    };
    erros.extend(crate::dominio::validacao::validar(&cenario));
    if erros.is_empty() {
        Ok(cenario)
    } else {
        Err(erros)
    }
}

pub fn campos_cenario(c: &Cenario) -> Campos {
    let mut campos = campos_serializados(c);
    campos.insert("lote".into(), c.lote.id.clone());
    campos.insert(
        "modo".into(),
        if c.formular { "formular" } else { "verificar" }.into(),
    );
    campos.insert("sobras".into(), (c.sobras * 100.0).to_string());
    if c.referencia_ue {
        campos.insert("ue".into(), "on".into());
    }
    let base = c
        .itens
        .first()
        .map(|i| i.base)
        .unwrap_or(BaseQuantidade::ConsumoMs);
    campos.insert(
        "base".into(),
        serde_json::to_value(base).unwrap().as_str().unwrap().into(),
    );
    for i in &c.itens {
        let id = &i.alimento.composicao_id;
        for (nome, valor) in [
            ("usar", "on".into()),
            ("q", i.quantidade.to_string()),
            (
                "preco",
                i.cotacao_original
                    .as_ref()
                    .map(|c| c.valor.clone())
                    .unwrap_or(i.preco.clone()),
            ),
            (
                "unidade-preco",
                i.cotacao_original
                    .as_ref()
                    .map(|c| c.unidade.clone())
                    .unwrap_or("kg".into()),
            ),
            (
                "embalagem",
                i.cotacao_original
                    .as_ref()
                    .and_then(|c| c.massa_embalagem)
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
            ),
            ("ms", (i.alimento.ms * 100.0).to_string()),
            ("perda", (i.perda * 100.0).to_string()),
            ("min", i.minimo_ms.to_string()),
            (
                "max",
                i.maximo_ms.map(|v| v.to_string()).unwrap_or_default(),
            ),
            (
                "estoque",
                i.estoque.map(|v| v.to_string()).unwrap_or_default(),
            ),
            ("resolucao", i.resolucao_mn.to_string()),
            ("origem", i.origem_preco.clone()),
        ] {
            campos.insert(format!("{nome}-{id}"), valor);
        }
        if i.sem_limite {
            campos.insert(format!("livre-{id}"), "on".into());
        }
    }
    if let Some(m) = &c.medicao {
        campos.insert("oferta_observada".into(), m.oferta_ms.to_string());
        campos.insert("sobras_observadas".into(), m.sobras_ms.to_string());
        campos.insert("periodo_observado".into(), m.periodo_dias.to_string());
    }
    campos
}

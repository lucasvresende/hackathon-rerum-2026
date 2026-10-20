use uuid::Uuid;

use super::*;
use crate::dominio::{Alimento, Classe, Laudo, Lote};

pub fn ler_lote(c: &Campos) -> Result<Lote, Vec<Problema>> {
    let mut erros = Vec::new();
    let lote = Lote {
        id: if texto(c, "id").is_empty() {
            Uuid::new_v4().to_string()
        } else {
            texto(c, "id").into()
        },
        revisao_id: String::new(),
        nome: texto(c, "nome").into(),
        animais: inteiro(c, "animais", &mut erros),
        raca: texto(c, "raca").into(),
        idade_meses: numero(c, "idade_meses", &mut erros).map(|v| v as u32),
        peso: obrigatorio(c, "peso", &mut erros),
        producao: obrigatorio(c, "producao", &mut erros),
        gordura: obrigatorio(c, "gordura", &mut erros),
        dias_lactacao: inteiro(c, "dias_lactacao", &mut erros),
        adulta: estado(c, "adulta"),
        gestante: estado(c, "gestante"),
        crescimento: estado(c, "crescimento"),
        peso_estavel: estado(c, "peso_estavel"),
        confinada: estado(c, "confinada"),
        correcao_ambiente: estado(c, "correcao_ambiente"),
        correcao_atividade: estado(c, "correcao_atividade"),
        agua: estado(c, "agua"),
        ordem_parto: numero(c, "ordem_parto", &mut erros).map(|v| v as u32),
        dispersao_peso: numero(c, "dispersao_peso", &mut erros),
        condicao_corporal: numero(c, "condicao_corporal", &mut erros),
        observacoes: texto(c, "observacoes").into(),
    };
    erros.extend(crate::dominio::validacao::validar_lote(&lote));
    if erros.is_empty() {
        Ok(lote)
    } else {
        Err(erros)
    }
}

pub fn campos_lote(lote: &Lote) -> Campos {
    campos_serializados(lote)
}

pub fn ler_alimento(c: &Campos) -> Result<Alimento, Vec<Problema>> {
    let mut erros = Vec::new();
    let mut nutrientes = [None; 7];
    for (i, nome) in ["ndt", "pb", "ca", "p", "fdn", "amido", "ee"]
        .iter()
        .enumerate()
    {
        nutrientes[i] = numero(c, nome, &mut erros).map(|v| v / 100.0);
    }
    let id = if texto(c, "id").is_empty() {
        Uuid::new_v4().to_string()
    } else {
        texto(c, "id").into()
    };
    let classe = match texto(c, "classe") {
        "FORRAGEM" => Classe::Forragem,
        "SUPLEMENTO" => Classe::Suplemento,
        _ => Classe::Concentrado,
    };
    let mut contaminantes = Vec::new();
    for i in 0..3 {
        let campo = |nome: &str| format!("laudo_{i}_{nome}");
        if texto(c, &campo("analito")).is_empty() {
            continue;
        }
        contaminantes.push(Laudo {
            analito: texto(c, &campo("analito")).into(),
            matriz: "MATERIA_PRIMA".into(),
            valor: obrigatorio(c, &campo("valor"), &mut erros),
            unidade: texto(c, &campo("unidade")).into(),
            ms: obrigatorio(c, &campo("ms"), &mut erros) / 100.0,
            qualificador: texto(c, &campo("qualificador")).into(),
            loq: numero(c, &campo("loq"), &mut erros),
            metodo: texto(c, &campo("metodo")).into(),
            documento: texto(c, &campo("documento")).into(),
        });
    }
    let a = Alimento {
        id,
        composicao_id: String::new(),
        nome: texto(c, "nome").into(),
        classe,
        ms: obrigatorio(c, "ms", &mut erros) / 100.0,
        nutrientes,
        fonte_id: texto(c, "fonte_id").into(),
        natureza: texto(c, "natureza").into(),
        origem: texto(c, "origem").into(),
        processamento: texto(c, "processamento").into(),
        metodo: texto(c, "metodo").into(),
        data: texto(c, "data").into(),
        origem_animal: estado(c, "origem_animal"),
        excecao_animal: estado(c, "excecao_animal"),
        nnp_adicionado: estado(c, "nnp_adicionado"),
        medicamento: estado(c, "medicamento"),
        autorizado: estado(c, "autorizado"),
        evidencia: texto(c, "evidencia").into(),
        fornecedor: texto(c, "fornecedor").into(),
        lote_fisico: texto(c, "lote_fisico").into(),
        rotulo_max_ms: numero(c, "rotulo_max_ms", &mut erros),
        pendf: None,
        metodo_fisico: String::new(),
        contaminantes,
    };
    erros.extend(crate::dominio::validacao::validar_alimento(&a));
    if erros.is_empty() { Ok(a) } else { Err(erros) }
}

pub fn campos_alimento(a: &Alimento) -> Campos {
    let mut c = campos_serializados(a);
    c.insert("ms".into(), (a.ms * 100.0).to_string());
    for (i, nome) in ["ndt", "pb", "ca", "p", "fdn", "amido", "ee"]
        .iter()
        .enumerate()
    {
        c.insert(
            (*nome).into(),
            a.nutrientes[i]
                .map(|v| (v * 100.0).to_string())
                .unwrap_or_default(),
        );
    }
    for (i, laudo) in a.contaminantes.iter().enumerate() {
        for (nome, valor) in campos_serializados(laudo) {
            c.insert(
                format!("laudo_{i}_{nome}"),
                if nome == "ms" {
                    (laudo.ms * 100.0).to_string()
                } else {
                    valor
                },
            );
        }
    }
    c
}

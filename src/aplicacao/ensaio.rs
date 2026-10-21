use std::time::Instant;
use uuid::Uuid;

use super::{demonstracao, execucao};
use crate::Resultado;
use crate::persistencia::{banco, cadastros, conhecimento};

/// Ensaio isolado: mede cálculo e persistência com 20 ingredientes e formulação com 10.
pub async fn executar() -> Resultado<()> {
    let diretorio = std::env::temp_dir().join(format!("feedyourcow-ensaio-{}", Uuid::new_v4()));
    std::fs::create_dir(&diretorio)?;
    let pool = banco::preparar(&diretorio.join("ensaio.sqlite3")).await?;
    demonstracao::preparar(&pool).await?;
    let lote = cadastros::lotes(&pool)
        .await?
        .into_iter()
        .find(|l| l.nome == "Lote demonstração")
        .ok_or("Lote demonstração ausente")?;
    let ingredientes = cadastros::alimentos(&pool)
        .await?
        .into_iter()
        .filter(|a| ["silagem", "milho", "soja", "mineral"].contains(&a.id.as_str()))
        .collect();
    let base = conhecimento::ler(&pool).await?;
    let original = demonstracao::cenario(lote, ingredientes, false);
    let mut medidas = std::collections::BTreeMap::new();
    for (nome, quantidade, formular, meta) in [
        ("verificacao", 20, false, 1000.0),
        ("formulacao", 10, true, 5000.0),
    ] {
        let mut c = original.clone();
        c.formular = formular;
        c.itens.clear();
        for i in 0..quantidade {
            let mut item = original.itens[i % original.itens.len()].clone();
            item.alimento.id = Uuid::new_v4().to_string();
            item.alimento.nome = format!("Sintético de ensaio {i}");
            cadastros::salvar_alimento(&pool, &mut item.alimento).await?;
            let ocorrencias = (0..quantidade).filter(|n| n % 4 == i % 4).count();
            item.quantidade /= ocorrencias as f64;
            c.itens.push(item);
        }
        for _ in 0..5 {
            execucao::executar(&pool, &c, &base).await?;
        }
        let mut tempos = Vec::new();
        for _ in 0..100 {
            let inicio = Instant::now();
            let id = execucao::executar(&pool, &c, &base).await?;
            tempos.push(inicio.elapsed().as_secs_f64() * 1000.0);
            let salvo = crate::persistencia::historico::obter(&pool, &id).await?;
            if formular && salvo.parecer.busca.as_deref() != Some("OTIMO_NO_MODELO") {
                return Err("Ensaio não produziu formulação ótima validada.".into());
            }
        }
        tempos.sort_by(f64::total_cmp);
        medidas.insert(
            nome,
            serde_json::json!({
                "ingredientes": quantidade, "execucoes": 100, "aquecimento": 5,
                "p95_ms": tempos[94], "maximo_ms": tempos[99], "meta_ms": meta,
                "atingiu_meta": tempos[94] <= meta,
            }),
        );
        if tempos[94] > meta {
            return Err(format!("Meta de {nome} não atingida.").into());
        }
    }
    println!("{}", serde_json::to_string_pretty(&medidas)?);
    pool.close().await;
    std::fs::remove_dir_all(&diretorio)?;
    Ok(())
}

use feed_your_cow::{
    aplicacao::demonstracao,
    dominio::{Alimento, Cenario, Lote},
    persistencia::{banco, cadastros},
    web::entradas::{Campos, campos_cenario, ler_cenario},
};

async fn exemplo() -> (Campos, Vec<Lote>, Vec<Alimento>) {
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("teste.sqlite3"))
        .await
        .unwrap();
    demonstracao::preparar(&pool).await.unwrap();
    let lotes = cadastros::lotes(&pool).await.unwrap();
    let alimentos = cadastros::alimentos(&pool).await.unwrap();
    let lote = lotes
        .iter()
        .find(|l| l.nome == "Lote demonstração")
        .unwrap()
        .clone();
    let mut c = demonstracao::cenario(
        lote,
        alimentos
            .iter()
            .filter(|a| !a.id.starts_with("ex-"))
            .cloned()
            .collect(),
        false,
    );
    c.formular = true;
    (campos_cenario(&c), lotes, alimentos)
}

#[tokio::test]
async fn qualquer_parte_da_medicao_exige_o_conjunto_completo() {
    for campo in ["oferta_observada", "sobras_observadas", "periodo_observado"] {
        let (mut campos, lotes, alimentos) = exemplo().await;
        campos.insert(campo.into(), "10".into());
        let erros = ler_cenario(&campos, &lotes, &alimentos).unwrap_err();
        for faltante in ["oferta_observada", "sobras_observadas", "periodo_observado"] {
            if faltante != campo {
                assert!(erros.iter().any(|e| e.campo == faltante));
            }
        }
    }
}

#[tokio::test]
async fn formulacao_ignora_quantidade_e_atualiza_apenas_premissas_geradas() {
    let (mut campos, lotes, alimentos) = exemplo().await;
    for (nome, valor) in &mut campos {
        if nome.starts_with("q-") {
            *valor = "-1".into();
        }
    }
    campos.insert("consumo_alvo".into(), "20".into());
    campos.insert("sobras".into(), "10".into());
    let c: Cenario = ler_cenario(&campos, &lotes, &alimentos).unwrap();
    assert!(c.itens.iter().all(|i| i.quantidade == 0.0));
    assert_eq!(c.origem_consumo, "Informado pelo operador");
    assert!(!c.origem_sobras.contains("zero"));
    campos.insert("origem_consumo".into(), "Planejamento da fazenda".into());
    campos.insert("origem_sobras".into(), "Pesagem em 21/10/2026".into());
    let c = ler_cenario(&campos, &lotes, &alimentos).unwrap();
    assert_eq!(c.origem_consumo, "Planejamento da fazenda");
    assert_eq!(c.origem_sobras, "Pesagem em 21/10/2026");
    assert_eq!(c.tipo_consumo(), "Informado pelo operador");
}

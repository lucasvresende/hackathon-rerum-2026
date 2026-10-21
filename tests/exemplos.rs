use feed_your_cow::{
    aplicacao::{cadastros as servico, exemplos},
    dominio::{inferencia, validacao},
    persistencia::{banco, cadastros, conhecimento, exemplos as registros, historico, recuperacao},
};

#[tokio::test]
async fn catalogo_completo_misturas_e_primeiro_calculo() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let lotes = cadastros::lotes(&pool).await.unwrap();
    let alimentos = cadastros::alimentos(&pool).await.unwrap();
    assert_eq!(lotes.len(), 6);
    assert_eq!(alimentos.len(), 15);
    for l in &lotes {
        assert_eq!(l.animais, 1);
        assert!(validacao::validar_lote(l).is_empty());
        assert!(l.procedencia.is_some());
    }
    for a in &alimentos {
        assert!(validacao::validar_alimento(a).is_empty());
        assert!(a.nutrientes.iter().all(Option::is_some));
        assert_eq!(a.natureza, "SINTETICA");
        assert!(a.procedencia.is_some());
        assert!(a.contaminantes.is_empty());
    }
    let obter = |id: &str| alimentos.iter().find(|a| a.id == id).unwrap();
    for (id, pesos) in [
        (
            "ex-a14",
            vec![("ex-a06", 0.7), ("ex-a07", 0.27), ("ex-a12", 0.03)],
        ),
        (
            "ex-a15",
            vec![
                ("ex-a06", 0.5),
                ("ex-a07", 0.25),
                ("ex-a10", 0.22),
                ("ex-a12", 0.03),
            ],
        ),
    ] {
        let mistura = obter(id);
        let ms: f64 = pesos.iter().map(|(id, p)| obter(id).ms * p).sum();
        assert!((mistura.ms - ms).abs() < 1e-12);
        for j in 0..7 {
            let esperado: f64 = pesos
                .iter()
                .map(|(id, p)| {
                    let a = obter(id);
                    a.ms * p * a.nutrientes[j].unwrap()
                })
                .sum::<f64>()
                / ms;
            assert!((mistura.nutrientes[j].unwrap() - esperado).abs() < 1e-12);
        }
    }
    let c = exemplos::cenario_inicial(&pool).await.unwrap();
    assert!(validacao::validar(&c).is_empty());
    let base = conhecimento::ler(&pool).await.unwrap();
    let p = inferencia::verificar(&c, &base);
    assert_eq!(p.dimensao("Elegibilidade"), "ATENDE");
    assert_eq!(
        p.dimensao("Núcleo químico"),
        "ATENDE",
        "{}",
        serde_json::to_string(&p.verificacoes).unwrap()
    );
    assert_eq!(p.dimensao("Inclusão e estoque"), "ATENDE");
    assert_eq!(p.dimensao("Barreiras ruminais"), "ATENDE");
    let id = historico::salvar(&pool, &c, &p, &base).await.unwrap();
    assert!(
        historico::obter(&pool, &id)
            .await
            .unwrap()
            .cenario
            .lote
            .procedencia
            .is_some()
    );
}

#[tokio::test]
async fn inicializacao_nao_recria_exclusoes_e_restauracao_preserva_historico() {
    let d = tempfile::tempdir().unwrap();
    let caminho = d.path().join("app.sqlite3");
    let pool = banco::preparar(&caminho).await.unwrap();
    let mut lote = servico::obter_lote(&pool, "ex-l03").await.unwrap();
    let revisao = lote.revisao_id.clone();
    lote.peso = 580.0;
    servico::salvar_lote(&pool, &mut lote, false, "", "")
        .await
        .unwrap();
    assert!(
        lote.procedencia
            .as_ref()
            .unwrap()
            .campos
            .iter()
            .any(|c| c.campo == "peso" && c.alterado)
    );
    registros::desativar(&pool, "alimento", "ex-a01")
        .await
        .unwrap();
    let reaberto = banco::preparar(&caminho).await.unwrap();
    assert_eq!(cadastros::lotes(&reaberto).await.unwrap().len(), 6);
    assert_eq!(cadastros::alimentos(&reaberto).await.unwrap().len(), 14);
    assert_eq!(
        servico::obter_lote(&reaberto, "ex-l03").await.unwrap().peso,
        580.0
    );
    assert!(
        exemplos::restaurar(&pool, "ex-l03", "automatico")
            .await
            .is_err()
    );
    exemplos::restaurar(&pool, "ex-l03", "copiar")
        .await
        .unwrap();
    assert_eq!(cadastros::lotes(&pool).await.unwrap().len(), 7);
    assert_eq!(
        servico::obter_lote(&pool, "ex-l03").await.unwrap().peso,
        580.0
    );
    exemplos::restaurar(&pool, "ex-l03", "revisar")
        .await
        .unwrap();
    let restaurado = servico::obter_lote(&pool, "ex-l03").await.unwrap();
    assert_eq!(restaurado.peso, 550.0);
    assert_ne!(restaurado.revisao_id, revisao);
    exemplos::restaurar(&pool, "ex-a01", "revisar")
        .await
        .unwrap();
    assert!(
        cadastros::alimentos(&pool)
            .await
            .unwrap()
            .iter()
            .any(|a| a.id == "ex-a01")
    );
    registros::desativar(&pool, "lote", "ex-l02").await.unwrap();
    let backup = d.path().join("backup");
    recuperacao::backup(&pool, &backup).await.unwrap();
    let destino = d.path().join("restaurado");
    recuperacao::restaurar(&backup, &destino).await.unwrap();
    let pool2 = banco::preparar(&destino.join("aplicacao.sqlite3"))
        .await
        .unwrap();
    assert!(
        !cadastros::lotes(&pool2)
            .await
            .unwrap()
            .iter()
            .any(|l| l.id == "ex-l02")
    );
    assert_eq!(registros::listar(&pool2).await.unwrap().len(), 21);
}

#[tokio::test]
async fn falha_na_inicializacao_reverte_cadastros_e_marcador() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::abrir(&d.path().join("app.sqlite3"), true)
        .await
        .unwrap();
    banco::MIGRACOES.run(&pool).await.unwrap();
    for sql in [
        include_str!("../dados_referencia/001_conhecimento.sql"),
        include_str!("../dados_referencia/005_exemplos.sql"),
    ] {
        conhecimento::carregar(&pool, sql).await.unwrap();
    }
    sqlx::raw_sql(sqlx::AssertSqlSafe(
        "CREATE TRIGGER falhar_exemplo BEFORE INSERT ON lotes \
        WHEN NEW.id = 'ex-l02' BEGIN SELECT RAISE(ABORT, 'falha simulada'); END;"
            .to_owned(),
    ))
    .execute(&pool)
    .await
    .unwrap();
    assert!(exemplos::inicializar(&pool).await.is_err());
    assert!(cadastros::lotes(&pool).await.unwrap().is_empty());
    assert!(cadastros::alimentos(&pool).await.unwrap().is_empty());
    assert!(registros::inicial(&pool).await.unwrap().is_none());
}

#[tokio::test]
async fn teto_demonstrativo_nao_substitui_rotulo_em_composicao_real() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let mut c = exemplos::cenario_inicial(&pool).await.unwrap();
    let base = conhecimento::ler(&pool).await.unwrap();
    assert_eq!(
        inferencia::verificar(&c, &base).dimensao("Inclusão e estoque"),
        "ATENDE"
    );
    let mineral = c
        .itens
        .iter_mut()
        .find(|i| i.alimento.id == "ex-a12")
        .unwrap();
    mineral.alimento.natureza = "ANALISE".into();
    let p = inferencia::verificar(&c, &base);
    assert_eq!(p.dimensao("Inclusão e estoque"), "INCONCLUSIVO");
    assert!(!p.pratico_valido);
}

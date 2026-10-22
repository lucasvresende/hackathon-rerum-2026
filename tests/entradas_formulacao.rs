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
    campos.insert("origem_consumo".into(), "Informado pelo operador".into());
    campos.remove("consumo_alvo");
    let c = ler_cenario(&campos, &lotes, &alimentos).unwrap();
    assert_eq!(c.origem_consumo, "Estimado pelo perfil");
}

#[tokio::test]
async fn resultado_sem_candidata_separa_diagnostico_e_expoe_alvo() {
    use feed_your_cow::dominio::otimizacao::formular;
    use feed_your_cow::persistencia::{conhecimento, historico::Execucao};
    use feed_your_cow::web::templates::componentes::resultado::ResultadoExecucao;
    use maud::Render;
    let (mut campos, lotes, alimentos) = exemplo().await;
    campos.insert("consumo_alvo".into(), "1".into());
    let c = ler_cenario(&campos, &lotes, &alimentos).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("base.sqlite3"))
        .await
        .unwrap();
    let base = conhecimento::ler(&pool).await.unwrap();
    let p = formular(&c, &base);
    assert_eq!(p.busca.as_deref(), Some("INVIAVEL_NO_MODELO"));
    let mut e = Execucao {
        id: "teste".into(),
        cenario_id: "teste".into(),
        criada_em: String::new(),
        cenario: c,
        parecer: p,
        conhecimento: base,
    };
    let html = ResultadoExecucao {
        execucao: &e,
        token: "teste",
    }
    .render()
    .into_string();
    let antes = html.split("id=\"diagnostico-entrada\"").next().unwrap();
    assert!(antes.contains("Nenhuma combinação validada"));
    assert!(antes.contains("class=\"alert error\""));
    assert!(!antes.contains("Custo / animal / dia"));
    assert!(html.contains("1,0000 kg MS/animal/dia"));
    assert!(html.contains("Informado pelo operador"));
    assert!(html.contains("data-busca=\"INVIAVEL_NO_MODELO\""));
    e.cenario.consumo_alvo = None;
    for i in &mut e.cenario.itens {
        i.resolucao_mn = 1.0;
    }
    e.parecer = formular(&e.cenario, &e.conhecimento);
    let html = ResultadoExecucao {
        execucao: &e,
        token: "teste",
    }
    .render()
    .into_string();
    assert!(html.contains("id=\"resultado-pratico\""));
    assert!(html.contains("Custo prático por animal/dia"));
    assert!(html.contains("OPT-ALVO"));
    assert!(html.contains("10,800 kg MS"));
    assert!(html.contains("Formulação: Ótimo no modelo"));
}

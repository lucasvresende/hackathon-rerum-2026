use feed_your_cow::aplicacao::{comparacao::Comparacao, demonstracao};
use feed_your_cow::dominio::inferencia::verificar;
use feed_your_cow::persistencia::{banco, conhecimento, historico::Execucao};

async fn execucao() -> Execucao {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let conhecimento = conhecimento::ler(&pool).await.unwrap();
    let cenario = demonstracao::cenario(demonstracao::lote(), vec![], false);
    let parecer = verificar(&cenario, &conhecimento);
    Execucao {
        id: "referencia".into(),
        cenario_id: "cenario".into(),
        criada_em: String::new(),
        cenario,
        parecer,
        conhecimento,
    }
}

#[tokio::test]
async fn economia_exige_dois_custos_e_percentual_exige_referencia_positiva() {
    let mut a = execucao().await;
    let mut b = a.clone();
    a.parecer.custo = Some(10.0);
    b.parecer.custo = Some(8.0);
    b.parecer.pratico_valido = false;
    let economia = Comparacao::entre(a.clone(), b.clone()).economia.unwrap();
    assert_eq!(economia.absoluta, 2.0);
    assert_eq!(economia.percentual, Some(20.0));
    assert!(!economia.alternativa_validada);
    a.parecer.custo = Some(0.0);
    let economia = Comparacao::entre(a.clone(), b.clone()).economia.unwrap();
    assert_eq!(economia.absoluta, -8.0);
    assert_eq!(economia.percentual, None);
    b.parecer.custo = None;
    assert!(Comparacao::entre(a, b).economia.is_none());
}

#[tokio::test]
async fn equivalencia_preserva_criterios_independentemente_do_custo() {
    let a = execucao().await;
    assert!(Comparacao::entre(a.clone(), a.clone()).equivalentes);
    for alterar in [
        |b: &mut Execucao| b.cenario.perfil_id.push_str("-outro"),
        |b: &mut Execucao| b.cenario.lote.producao += 1.0,
        |b: &mut Execucao| b.parecer.consumo_ms += 1.0,
        |b: &mut Execucao| b.cenario.horizonte += 1,
        |b: &mut Execucao| b.cenario.referencia_ue = !b.cenario.referencia_ue,
    ] {
        let mut b = a.clone();
        alterar(&mut b);
        assert!(!Comparacao::entre(a.clone(), b).equivalentes);
    }
    let mut b = a.clone();
    b.cenario.lote.producao += f64::EPSILON;
    b.cenario.sobras = 0.05;
    b.parecer.custo = Some(1.0);
    assert!(Comparacao::entre(a, b).equivalentes);
}

#[tokio::test]
async fn busca_sem_candidata_nao_apresenta_economia_da_entrada() {
    let mut a = execucao().await;
    a.parecer.custo = Some(10.0);
    let mut b = a.clone();
    b.parecer.custo = Some(0.0);
    for estado in ["INVIAVEL_NO_MODELO", "INCONCLUSIVA"] {
        b.parecer.busca = Some(estado.into());
        assert!(b.parecer.custo_resultado().is_none());
        assert!(Comparacao::entre(a.clone(), b.clone()).economia.is_none());
    }
}

#[tokio::test]
async fn comparacao_expoe_resolucao_inclusao_e_diagnostico_pratico() {
    use feed_your_cow::aplicacao::exemplos;
    use feed_your_cow::dominio::otimizacao;
    use feed_your_cow::web::templates::paginas::comparacao::Comparacao as Tela;
    use maud::Render;
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("teste.sqlite3"))
        .await
        .unwrap();
    let mut a = execucao().await;
    a.cenario = exemplos::cenario_inicial(&pool).await.unwrap();
    a.cenario.formular = true;
    a.parecer = otimizacao::formular(&a.cenario, &a.conhecimento);
    let mut b = a.clone();
    for i in &mut b.cenario.itens {
        i.resolucao_mn = 1.0;
    }
    b.parecer = otimizacao::formular(&b.cenario, &b.conhecimento);
    let comparacao = Comparacao::entre(a, b);
    let html = Tela {
        comparacao: &comparacao,
    }
    .render()
    .into_string();
    assert!(html.contains("Resolução de oferta"));
    assert!(html.contains("Inclusão mínima"));
    assert!(html.contains("Inclusão máxima"));
    assert!(html.contains("16,6300"));
    assert!(html.contains("R$ 17,35"));
    assert!(!html.contains("16.868500000000004"));
    assert!(html.contains("não usar como recomendação"));
}

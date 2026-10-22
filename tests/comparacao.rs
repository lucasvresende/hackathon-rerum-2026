use feed_your_cow::aplicacao::{comparacao::Comparacao, demonstracao};
use feed_your_cow::dominio::inferencia::verificar;
use feed_your_cow::persistencia::{banco, conhecimento, historico::Execucao};

async fn execucao() -> Execucao {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let conhecimento = conhecimento::ler(&pool).await.unwrap();
    let mut cenario = demonstracao::cenario(demonstracao::lote(), vec![], false);
    cenario.lote.revisao_id = "revisao-original".into();
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

#[tokio::test]
async fn recusa_outro_lote_ou_revisao_sem_calcular_economia() {
    use feed_your_cow::web::templates::paginas::comparacao::Comparacao as Tela;
    use maud::Render;
    let a = execucao().await;
    for alterar in [
        |b: &mut Execucao| b.cenario.lote.id.push_str("-outro"),
        |b: &mut Execucao| b.cenario.lote.revisao_id.push_str("-nova"),
        |b: &mut Execucao| b.cenario.lote.revisao_id.clear(),
    ] {
        let mut b = a.clone();
        alterar(&mut b);
        b.parecer.custo = Some(1.0);
        let c = Comparacao::entre(a.clone(), b);
        assert!(!c.mesma_revisao);
        assert!(!c.equivalentes);
        assert!(c.economia.is_none());
        let html = Tela { comparacao: &c }.render().into_string();
        assert!(html.contains("mesma revisão do lote"));
        assert!(!html.contains("Diferença de custo"));
        assert!(!html.contains("Ingredientes e premissas"));
    }
}

#[tokio::test]
async fn http_recusa_comparacao_de_revisoes_distintas() {
    use axum::{body::Body, http::Request};
    use feed_your_cow::{
        aplicacao::exemplos,
        persistencia::{cadastros, historico},
        web::{estado::EstadoApp, rotas::app},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("http.sqlite3"))
        .await
        .unwrap();
    let c = exemplos::cenario_inicial(&pool).await.unwrap();
    let base = conhecimento::ler(&pool).await.unwrap();
    let p = verificar(&c, &base);
    let a = historico::salvar(&pool, &c, &p, &base).await.unwrap();
    let mut outra = c.clone();
    outra.lote = cadastros::lotes(&pool)
        .await
        .unwrap()
        .into_iter()
        .find(|l| l.id != c.lote.id)
        .unwrap();
    let b = historico::salvar(&pool, &outra, &verificar(&outra, &base), &base)
        .await
        .unwrap();
    let mut revisada = c.clone();
    revisada.lote.peso += 10.0;
    cadastros::salvar_lote(&pool, &mut revisada.lote, false)
        .await
        .unwrap();
    let nova = historico::salvar(&pool, &revisada, &verificar(&revisada, &base), &base)
        .await
        .unwrap();
    let app = app(EstadoApp::novo(pool, "http://127.0.0.1:3000".into()));
    for (alternativa, status) in [(&a, 200), (&b, 422), (&nova, 422)] {
        let r = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/comparar?a={a}&b={alternativa}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r.status(), status);
        let body = r.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert_eq!(html.contains("Diferença de custo"), status == 200);
    }
}

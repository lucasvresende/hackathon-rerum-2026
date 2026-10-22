use feed_your_cow::{
    aplicacao::{
        exemplos,
        reutilizacao::{OrigemDieta, copiar},
    },
    dominio::unidades::proximos,
    dominio::{BaseQuantidade, inferencia, otimizacao},
    persistencia::{
        banco, conhecimento,
        historico::{self, Execucao},
    },
    web::entradas::{campos_cenario, ler_cenario},
};

async fn exemplo(resolucao: f64) -> Execucao {
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("app.sqlite3"))
        .await
        .unwrap();
    let mut c = exemplos::cenario_inicial(&pool).await.unwrap();
    c.formular = true;
    c.sobras = 0.1;
    for i in &mut c.itens {
        i.resolucao_mn = resolucao;
        i.perda = 0.05;
    }
    let base = conhecimento::ler(&pool).await.unwrap();
    let p = otimizacao::formular(&c, &base);
    assert_eq!(p.busca.as_deref(), Some("OTIMO_NO_MODELO"));
    let id = historico::salvar(&pool, &c, &p, &base).await.unwrap();
    historico::obter(&pool, &id).await.unwrap()
}

#[tokio::test]
async fn reutiliza_solucao_por_composicao_e_preserva_original() {
    let mut e = exemplo(0.0).await;
    let original = serde_json::to_string(&e).unwrap();
    e.parecer.fornecimento.reverse();
    let c = copiar(&e, OrigemDieta::Continua).unwrap();
    assert!(!c.formular);
    assert_eq!(c.original_id.as_deref(), Some(e.cenario_id.as_str()));
    assert!(c.itens.iter().all(|i| i.base == BaseQuantidade::ConsumoMs));
    let p = inferencia::verificar(&c, &e.conhecimento);
    assert!(proximos(p.consumo_ms, e.parecer.consumo_ms));
    assert!(proximos(p.custo.unwrap(), e.parecer.custo.unwrap()));
    for (a, b) in p.totais.iter().zip(e.parecer.totais) {
        assert!(proximos(a.unwrap(), b.unwrap()));
    }
    e.parecer.fornecimento.reverse();
    assert_eq!(serde_json::to_string(&e).unwrap(), original);
    let simular = copiar(&e, OrigemDieta::Simular).unwrap();
    assert!(simular.formular);
    assert!(simular.itens.iter().any(|i| i.quantidade > 0.0));
    assert_eq!(simular.itens[0].preco, e.cenario.itens[0].preco);
}

#[tokio::test]
async fn pratica_reproduz_sobras_perdas_e_custo_sem_converter_duas_vezes() {
    let e = exemplo(0.1).await;
    let c = copiar(&e, OrigemDieta::Pratica).unwrap();
    assert!(!c.formular);
    assert!(c.itens.iter().all(|i| i.base == BaseQuantidade::OfertaMn));
    let lotes = vec![c.lote.clone()];
    let alimentos = c
        .itens
        .iter()
        .map(|i| i.alimento.clone())
        .collect::<Vec<_>>();
    let relida = ler_cenario(&campos_cenario(&c), &lotes, &alimentos).unwrap();
    let p = inferencia::verificar(&relida, &e.conhecimento);
    let anterior = e.parecer.pratico.as_ref().unwrap();
    assert!(proximos(p.consumo_ms, anterior.consumo_ms));
    assert!(proximos(p.custo.unwrap(), anterior.custo.unwrap()));
    for (a, b) in p.totais.iter().zip(anterior.totais) {
        assert!(proximos(a.unwrap(), b.unwrap()));
    }
    for (i, original) in c.itens.iter().zip(&e.cenario.itens) {
        assert_eq!(i.preco, original.preco);
        assert_eq!(i.perda, original.perda);
        assert_eq!(i.estoque, original.estoque);
        assert_eq!(i.minimo_ms, original.minimo_ms);
        assert_eq!(i.maximo_ms, original.maximo_ms);
    }
}

#[tokio::test]
async fn pratica_incompativel_pode_ser_corrigida_mas_nao_e_aprovada_na_copia() {
    let e = exemplo(1.0).await;
    assert!(!e.parecer.pratico_valido);
    let c = copiar(&e, OrigemDieta::Pratica).unwrap();
    let p = inferencia::verificar(&c, &e.conhecimento);
    assert!(!p.pratico_valido);
    assert_eq!(p.dimensao("Núcleo químico"), "NAO_ATENDE");
}

#[tokio::test]
async fn rejeita_pesagem_ausente_busca_sem_solucao_e_historico_ambiguo() {
    let mut e = exemplo(0.0).await;
    assert!(copiar(&e, OrigemDieta::Pratica).is_err());
    e.parecer.busca = Some("INVIAVEL_NO_MODELO".into());
    assert!(copiar(&e, OrigemDieta::Continua).is_err());
    assert!(copiar(&e, OrigemDieta::Simular).unwrap().formular);
    e.parecer.busca = Some("OTIMO_NO_MODELO".into());
    e.parecer.fornecimento[0].composicao_id.clear();
    assert!(copiar(&e, OrigemDieta::Continua).is_ok());
    e.cenario.itens.push(e.cenario.itens[0].clone());
    assert!(copiar(&e, OrigemDieta::Continua).is_err());
}

#[tokio::test]
async fn duplicacao_de_verificacao_preserva_base_mn_e_limita_nome() {
    let mut e = exemplo(0.0).await;
    e.cenario.formular = false;
    e.cenario.nome = "á".repeat(150);
    e.cenario.itens[0].base = BaseQuantidade::OfertaMn;
    e.cenario.itens[0].quantidade = 42.0;
    let c = copiar(&e, OrigemDieta::Simular).unwrap();
    assert_eq!(c.nome.chars().count(), 150);
    assert_eq!(c.itens[0].quantidade, 42.0);
    assert_eq!(c.itens[0].base, BaseQuantidade::OfertaMn);
}

#[tokio::test]
async fn http_valida_origem_e_nao_oferece_pesagem_inexistente() {
    use axum::{body::Body, http::Request};
    use feed_your_cow::web::{estado::EstadoApp, rotas::app};
    use http_body_util::BodyExt;
    use tower::ServiceExt;
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("http.sqlite3"))
        .await
        .unwrap();
    let mut c = exemplos::cenario_inicial(&pool).await.unwrap();
    c.formular = true;
    let base = conhecimento::ler(&pool).await.unwrap();
    let p = otimizacao::formular(&c, &base);
    let id = historico::salvar(&pool, &c, &p, &base).await.unwrap();
    let app = app(EstadoApp::novo(
        pool.clone(),
        "http://127.0.0.1:3000".into(),
    ));
    for (sufixo, status, texto) in [
        ("", 200, "Refazer otimização"),
        (
            "/duplicar?origem=continua",
            200,
            "verificar solução contínua",
        ),
        ("/duplicar?origem=pratica", 422, "Defina a resolução"),
        ("/duplicar?origem=inexistente", 400, "unknown variant"),
    ] {
        let r = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/execucoes/{id}{sufixo}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r.status(), status);
        let body = r.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains(texto), "{html}");
        if sufixo.is_empty() {
            assert!(html.contains("Ajustar e verificar solução contínua"));
            assert!(!html.contains("Ajustar e verificar quantidades práticas"));
        }
    }
    assert_eq!(
        historico::obter(&pool, &id).await.unwrap().parecer.custo,
        p.custo
    );
}

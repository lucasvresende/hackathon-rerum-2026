use axum::{body::Body, http::Request};
use feed_your_cow::{
    aplicacao::{cadastros as servico, exemplos},
    persistencia::{banco, cadastros},
    web::{entradas, estado::EstadoApp, rotas::app},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn copiar_limpar_rascunho_e_proteger_restauracao() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let e = EstadoApp::novo(pool.clone(), "http://127.0.0.1:3000".into());
    let app = app(e.clone());
    let post = |url: &str, body: String| {
        Request::builder()
            .method("POST")
            .uri(url)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(body))
            .unwrap()
    };
    let resposta = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/lotes/ex-l02/copiar")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resposta.status(), 200);
    let bytes = resposta.into_body().collect().await.unwrap().to_bytes();
    let html = std::str::from_utf8(&bytes).unwrap();
    assert!(html.contains("name=\"id\" value=\"\""));
    assert!(html.contains("name=\"origem_copia\" value=\"ex-l02\""));
    assert!(html.contains("name=\"modelo_aplicado\" value=\"ex-l02\""));
    let resposta = app
        .clone()
        .oneshot(post(
            "/cadastros/alimento/modelo",
            format!(
                "csrf={}&modelo=ex-a07&acao_modelo=aplicar&confirmar_modelo=on\
        &laudo_0_documento=LAUDO-ANTIGO&rotulo_max_ms=99&fornecedor=ANTERIOR",
                e.csrf
            ),
        ))
        .await
        .unwrap();
    assert_eq!(resposta.status(), 200);
    let bytes = resposta.into_body().collect().await.unwrap().to_bytes();
    let html = std::str::from_utf8(&bytes).unwrap();
    assert!(!html.contains("LAUDO-ANTIGO"));
    assert!(!html.contains("value=\"99\""));
    assert!(!html.contains("ANTERIOR"));
    assert!(html.contains("name=\"fornecedor\""));
    assert!(html.contains("Demonstração"));
    for (body, status) in [(String::new(), 403), (format!("csrf={}", e.csrf), 400)] {
        assert_eq!(
            app.clone()
                .oneshot(post("/exemplos", body))
                .await
                .unwrap()
                .status(),
            status
        );
    }
    let resposta = app
        .clone()
        .oneshot(post(
            "/cadastros/lote/ex-l02/remover",
            format!("csrf={}&confirmar=on", e.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(resposta.status(), 303);
    assert!(
        !cadastros::lotes(&pool)
            .await
            .unwrap()
            .iter()
            .any(|l| l.id == "ex-l02")
    );
    let resposta = app
        .oneshot(post(
            "/exemplos",
            format!("csrf={}&confirmar=on&exemplo=ex-l02&modo=revisar", e.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(resposta.status(), 303);
    assert!(
        cadastros::lotes(&pool)
            .await
            .unwrap()
            .iter()
            .any(|l| l.id == "ex-l02")
    );
}

#[tokio::test]
async fn cenario_preenchido_passa_pelo_mesmo_formulario_e_copia_preserva_origem() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let inicial = exemplos::cenario_inicial(&pool).await.unwrap();
    let campos = entradas::campos_cenario(&inicial);
    let lotes = cadastros::lotes(&pool).await.unwrap();
    let alimentos = cadastros::alimentos(&pool).await.unwrap();
    let c = entradas::ler_cenario(&campos, &lotes, &alimentos).unwrap();
    let base = feed_your_cow::persistencia::conhecimento::ler(&pool)
        .await
        .unwrap();
    let p = feed_your_cow::dominio::inferencia::verificar(&c, &base);
    assert_eq!(p.dimensao("Núcleo químico"), "ATENDE");
    assert_eq!(p.dimensao("Barreiras ruminais"), "ATENDE");
    assert_eq!(p.dimensao("Inclusão e estoque"), "ATENDE");
    let original = alimentos.iter().find(|a| a.id == "ex-a06").unwrap();
    let mut copia = original.clone();
    copia.id = uuid::Uuid::new_v4().to_string();
    copia.natureza = "ANALISE".into();
    copia.nutrientes[1] = Some(0.1);
    servico::salvar_alimento(&pool, &mut copia, "", &original.composicao_id)
        .await
        .unwrap();
    assert_eq!(copia.natureza, "SINTETICA");
    assert_ne!(copia.id, original.id);
    let p = copia.procedencia.unwrap();
    assert_eq!(p.modelo.id, "ex-a06");
    assert!(p.campos.iter().any(|v| v.campo == "pb" && v.alterado));
    assert_eq!(
        servico::obter_alimento(&pool, &original.composicao_id)
            .await
            .unwrap()
            .nutrientes[1],
        Some(0.09)
    );
}

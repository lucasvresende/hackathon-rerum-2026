use axum::{body::Body, http::Request};
use feed_your_cow::{
    persistencia::banco,
    web::{estado::EstadoApp, rotas::app},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn salvar_preserva_redirecionamento_http_e_htmx() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let estado = EstadoApp::novo(pool, "http://127.0.0.1:3000".into());
    let app = app(estado.clone());
    for htmx in [false, true] {
        let mut pedido = Request::builder()
            .method("POST")
            .uri("/lotes")
            .header("content-type", "application/x-www-form-urlencoded");
        if htmx {
            pedido = pedido.header("hx-request", "true");
        }
        let resposta = app.clone().oneshot(pedido.body(Body::from(format!(
            "csrf={}&nome=Teste&animais=12&peso=400&producao=10&gordura=4&dias_lactacao=150",
            estado.csrf))).unwrap()).await.unwrap();
        assert_eq!(resposta.status(), if htmx { 200 } else { 303 });
        let cabecalho = if htmx { "hx-redirect" } else { "location" };
        assert_eq!(resposta.headers()[cabecalho], "/cadastros/salvo?tipo=lote");
        assert_eq!(resposta.headers()["x-frame-options"], "DENY");
    }
}

#[tokio::test]
async fn modelos_preservam_fragmento_pagina_e_protecao_de_escrita() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let estado = EstadoApp::novo(pool, "http://127.0.0.1:3000".into());
    let app = app(estado.clone());
    for htmx in [false, true] {
        let mut pedido = Request::builder()
            .method("POST")
            .uri("/cadastros/alimento/modelo")
            .header("content-type", "application/x-www-form-urlencoded");
        if htmx {
            pedido = pedido.header("hx-request", "true");
        }
        let resposta = app
            .clone()
            .oneshot(
                pedido
                    .body(Body::from(format!(
                        "csrf={}&modelo=silagem-milho-v1&nome=Rascunho",
                        estado.csrf
                    )))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resposta.status(), 200);
        let bytes = resposta.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&bytes).unwrap();
        assert_eq!(html.contains("<!DOCTYPE html>"), !htmx);
        assert!(html.contains("template-preview"));
        assert!(html.contains("value=\"Rascunho\""));
    }
    for (uri, corpo, origem, status) in [
        (
            "/cadastros/alimento/modelo",
            String::new(),
            "http://127.0.0.1:3000",
            403,
        ),
        (
            "/cadastros/alimento/modelo",
            format!("csrf={}", estado.csrf),
            "https://outro",
            403,
        ),
        (
            "/cadastros/invalido/modelo",
            format!("csrf={}", estado.csrf),
            "http://127.0.0.1:3000",
            404,
        ),
    ] {
        let resposta = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(uri)
                    .header("content-type", "application/x-www-form-urlencoded")
                    .header("origin", origem)
                    .body(Body::from(corpo))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resposta.status(), status);
    }
    let resposta = app
        .oneshot(
            Request::builder()
                .uri("/pagina-ausente")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resposta.status(), 404);
}

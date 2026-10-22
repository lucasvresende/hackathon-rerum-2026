use axum::{body::Body, http::Request};
use feed_your_cow::{
    aplicacao::demonstracao,
    persistencia::banco,
    web::{estado::EstadoApp, rotas::app},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn exemplo_exige_csrf_confirmacao_e_preserva_modo_sem_javascript() {
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("teste.sqlite3"))
        .await
        .unwrap();
    demonstracao::preparar(&pool).await.unwrap();
    let estado = EstadoApp::novo(pool, "http://127.0.0.1:3000".into());
    let app = app(estado.clone());
    for (acao, confirmado, csrf, status) in [
        ("aplicar", "on", "invalido", 403),
        ("previa", "", estado.csrf.as_str(), 200),
        ("aplicar", "", estado.csrf.as_str(), 200),
        ("cancelar", "", estado.csrf.as_str(), 200),
        ("aplicar", "on", estado.csrf.as_str(), 200),
    ] {
        let resposta = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/cenarios/exemplo?dieta=B&acao={acao}"))
                    .header("content-type", "application/x-www-form-urlencoded")
                    .body(Body::from(format!(
                        "csrf={csrf}&modo=formular&nome=Rascunho\
                &confirmar_exemplo={confirmado}&preco-antigo=1234&ue=on"
                    )))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resposta.status(), status);
        if status != 200 {
            continue;
        }
        let bytes = resposta.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&bytes).unwrap();
        assert!(html.contains("<!DOCTYPE html>"));
        if acao == "aplicar" && confirmado == "on" {
            assert!(html.contains("Dieta B · demonstração"));
            assert!(!html.contains("preco-antigo"));
            assert!(!html.contains("name=\"ue\" checked"));
        } else {
            assert!(html.contains("value=\"Rascunho\""));
            assert_eq!(
                html.contains("Substituir pelo exemplo B?"),
                acao != "cancelar"
            );
        }
        assert!(html.contains("value=\"formular\" selected"));
    }
}

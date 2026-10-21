use axum::{body::Body, http::Request};
use feed_your_cow::{
    persistencia::banco,
    web::{estado::EstadoApp, rotas::app},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn visao_geral_explica_conceitos_com_catalogo_inicial() {
    let diretorio = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&diretorio.path().join("app.sqlite3"))
        .await
        .unwrap();
    let app = app(EstadoApp::novo(pool, "http://127.0.0.1:3000".into()));
    for rota in ["/", "/ajuda", "/lotes"] {
        let resposta = app
            .clone()
            .oneshot(Request::builder().uri(rota).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(resposta.status(), 200);
        let bytes = resposta.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&bytes).unwrap();
        if rota == "/" {
            assert!(html.contains("Nenhum cenário calculado."));
            assert_eq!(html.matches("<strong>6</strong>").count(), 2);
            assert!(html.contains("<strong>15</strong>"));
            for id in ["lotes", "animais", "composicoes", "cenarios", "perfil"] {
                assert!(html.contains(&format!("popovertarget=\"dica-{id}\"")));
                assert!(html.contains(&format!("aria-describedby=\"dica-{id}\"")));
                assert!(html.contains(&format!("id=\"dica-{id}\" popover=\"auto\"")));
            }
            assert!(!html.contains("Execuções recentes"));
            assert!(!html.contains("dica-execucoes"));
            assert!(html.contains("Últimos cenários"));
            assert!(html.contains("não garante adequação nutricional completa"));
        } else {
            assert!(!html.contains("class=\"concept-trigger\""));
            if rota == "/ajuda" {
                assert!(html.contains("2 lotes e 50 animais"));
                assert!(html.contains("estar no histórico não significa que a dieta foi aprovada"));
            }
        }
    }
}

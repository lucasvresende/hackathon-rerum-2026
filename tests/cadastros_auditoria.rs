use axum::{body::Body, http::Request};
use feed_your_cow::{
    aplicacao::exemplos,
    dominio::validacao::validar_alimento,
    persistencia::banco,
    web::{entradas, estado::EstadoApp, rotas::app},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn data_da_composicao_aceita_vazio_e_bissexto_mas_rejeita_datas_impossiveis() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let mut a = exemplos::cenario_inicial(&pool)
        .await
        .unwrap()
        .itens
        .remove(0)
        .alimento;
    for (data, valida) in [
        ("", true),
        ("2024-02-29", true),
        ("2026-02-28", true),
        ("2026-02-29", false),
        ("31/02/2026", false),
        ("2026-2-01", false),
    ] {
        a.data = data.into();
        assert_eq!(
            !validar_alimento(&a).iter().any(|e| e.campo == "data"),
            valida
        );
    }
    let mut campos = entradas::campos_alimento(&a);
    campos.insert("data_antiga".into(), "31/02/2026".into());
    campos.insert("data".into(), String::new());
    assert!(
        entradas::ler_alimento(&campos)
            .unwrap_err()
            .iter()
            .any(|e| e.campo == "data")
    );
    campos.insert("remover_data".into(), "on".into());
    assert!(entradas::ler_alimento(&campos).unwrap().data.is_empty());
    campos.insert("data".into(), "2026-02-28".into());
    campos.remove("remover_data");
    assert_eq!(entradas::ler_alimento(&campos).unwrap().data, "2026-02-28");
}

#[tokio::test]
async fn cancelamento_preserva_tipo_e_aceita_url_antiga() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let app = app(EstadoApp::novo(pool, "http://127.0.0.1:3000".into()));
    for (tipo, destino) in [
        ("lote&cancelado=sim", "lotes"),
        ("alimento&cancelado=sim", "alimentos"),
        ("cancelado", "alimentos"),
    ] {
        let r = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/cadastros/salvo?tipo={tipo}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let bytes = r.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&bytes).unwrap();
        assert!(html.contains("Cadastro cancelado"));
        assert!(html.contains(&format!("href=\"/{destino}/novo\"")));
        assert!(!html.contains("data-cadastro-salvo"));
    }
}

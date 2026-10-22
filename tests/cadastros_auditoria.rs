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
        ("0000-01-01", false),
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

#[tokio::test]
async fn post_rejeita_data_invalida_preservando_dados_e_revisao_legada_exibe_original() {
    use feed_your_cow::web::templates::formularios::DadosCadastro;
    use feed_your_cow::web::templates::formularios::alimento::FormularioAlimento;
    use maud::Render;
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let a = exemplos::cenario_inicial(&pool)
        .await
        .unwrap()
        .itens
        .remove(0)
        .alimento;
    let mut campos = entradas::campos_alimento(&a);
    campos.insert("data".into(), "31/02/2026".into());
    campos.insert("nome".into(), "Rascunho preservado".into());
    let tela = FormularioAlimento(DadosCadastro {
        campos: &campos,
        erros: &[],
        modelos: &[],
        token: "teste",
    })
    .render()
    .into_string();
    assert!(tela.contains("Data original inválida: 31/02/2026"));
    assert!(tela.contains("name=\"data_antiga\" value=\"31/02/2026\""));
    assert!(tela.contains("name=\"data\" type=\"date\" value=\"\""));
    let estado = EstadoApp::novo(pool, "http://127.0.0.1:3000".into());
    campos.insert("csrf".into(), estado.csrf.to_string());
    let codificar = |s: &str| s.bytes().map(|b| format!("%{b:02X}")).collect::<String>();
    let corpo = campos
        .iter()
        .map(|(k, v)| format!("{}={}", codificar(k), codificar(v)))
        .collect::<Vec<_>>()
        .join("&");
    let r = app(estado)
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/alimentos")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(corpo))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(r.status(), 200);
    assert!(r.headers().get("location").is_none());
    let bytes = r.into_body().collect().await.unwrap().to_bytes();
    let html = std::str::from_utf8(&bytes).unwrap();
    assert!(html.contains("Informe uma data civil válida"));
    assert!(html.contains("value=\"Rascunho preservado\""));
}

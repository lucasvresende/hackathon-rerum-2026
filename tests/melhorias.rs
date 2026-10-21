use axum::body::Body;
use axum::http::Request;
use feed_your_cow::aplicacao::demonstracao;
use feed_your_cow::dominio::validacao::{validar_alimento, validar_lote};
use feed_your_cow::persistencia::{banco, cadastros};
use feed_your_cow::web::componentes::estrutura::Campo;
use feed_your_cow::web::rotas::{EstadoApp, app};
use http_body_util::BodyExt;
use maud::Render;
use tower::ServiceExt;

#[test]
fn obrigatorio_e_erro_acessivel_acima_do_controle() {
    let html = Campo {
        nome: "animais",
        rotulo: "Animais",
        valor: "0",
        tipo: "number",
        dica: "Inteiro positivo",
        obrigatorio: true,
        erro: Some("Corrija a quantidade"),
    }
    .render()
    .into_string();
    assert!(html.contains("required"));
    assert!(html.contains("aria-hidden=\"true\"> *</span>"));
    assert!(html.contains("step=\"1\" min=\"1\""));
    assert!(html.contains("aria-describedby=\"animais-erro animais-dica\""));
    assert!(html.find("Corrija a quantidade").unwrap() < html.find("<input").unwrap());
}

#[tokio::test]
async fn erros_indicam_campos_e_post_invalido_nao_grava() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    demonstracao::preparar(&pool).await.unwrap();
    let mut alimento = cadastros::alimentos(&pool).await.unwrap().remove(0);
    alimento.origem.clear();
    alimento.nutrientes[1] = Some(2.0);
    alimento.rotulo_max_ms = Some(-1.0);
    let campos: Vec<_> = validar_alimento(&alimento)
        .into_iter()
        .map(|e| e.campo)
        .collect();
    for nome in ["origem", "pb", "rotulo_max_ms"] {
        assert!(campos.iter().any(|c| c == nome), "{nome}: {campos:?}");
    }
    let mut lote = cadastros::lotes(&pool).await.unwrap().remove(0);
    lote.peso = 700.01;
    assert!(
        validar_lote(&lote).is_empty(),
        "Fora do perfil não é cadastro inválido"
    );
    let quantidade = cadastros::lotes(&pool).await.unwrap().len();
    let estado = EstadoApp::novo(pool.clone(), "http://127.0.0.1:3000".into());
    let corpo = format!(
        "csrf={}&nome=Preservado&animais=1.5&peso=-2&producao=10&gordura=4&dias_lactacao=150",
        estado.csrf
    );
    let resposta = app(estado)
        .oneshot(
            Request::builder()
                .uri("/lotes")
                .method("POST")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(corpo))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resposta.status(), 200);
    let bytes = resposta.into_body().collect().await.unwrap().to_bytes();
    let html = std::str::from_utf8(&bytes).unwrap();
    assert!(html.contains("value=\"Preservado\""));
    assert!(html.contains("href=\"#animais\""));
    assert!(html.contains("aria-invalid=\"true\""));
    assert_eq!(cadastros::lotes(&pool).await.unwrap().len(), quantidade);
}

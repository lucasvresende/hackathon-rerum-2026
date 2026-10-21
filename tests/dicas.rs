use std::collections::BTreeSet;

use axum::{body::Body, http::Request};
use feed_your_cow::{
    aplicacao::demonstracao,
    dominio::{inferencia::verificar, otimizacao::formular},
    persistencia::{banco, cadastros, conhecimento, historico},
    web::{estado::EstadoApp, rotas::app},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn conferir_ids(html: &str) {
    let ids: Vec<_> = html
        .split(" id=\"")
        .skip(1)
        .map(|s| s.split('"').next().unwrap())
        .collect();
    assert_eq!(ids.len(), ids.iter().collect::<BTreeSet<_>>().len());
    for trecho in html.split("popovertarget=\"").skip(1) {
        let id = trecho.split('"').next().unwrap();
        assert!(
            html.contains(&format!("id=\"{id}\" popover=\"auto\"")),
            "{id}"
        );
        assert!(html.contains(&format!("aria-describedby=\"{id}\"")), "{id}");
    }
}

#[tokio::test]
async fn dicas_em_listas_e_formularios_preservam_ids_e_rotulos() {
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("app.sqlite3"))
        .await
        .unwrap();
    demonstracao::preparar(&pool).await.unwrap();
    let app = app(EstadoApp::novo(pool, "http://127.0.0.1:3000".into()));
    for (rota, esperados) in [
        (
            "/alimentos",
            vec![
                "Composição do alimento",
                "Análises e revisões",
                "Procedência",
            ],
        ),
        (
            "/alimentos/novo",
            vec!["conceito-ms", "conceito-natureza", "conceito-fonte_id"],
        ),
        (
            "/cenarios/novo?dieta=B",
            vec!["conceito-horizonte", "conceito-base", "conceito-sobras"],
        ),
    ] {
        let resposta = app
            .clone()
            .oneshot(Request::builder().uri(rota).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(resposta.status(), 200);
        let bytes = resposta.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&bytes).unwrap();
        conferir_ids(html);
        for texto in esperados {
            assert!(html.contains(texto), "{rota}: {texto}");
        }
        if rota.contains("cenarios") {
            assert!(html.contains("label for=\"horizonte\""));
            assert!(html.contains("aria-describedby=\"horizonte-erro horizonte-dica\""));
            assert!(html.contains("Perda anterior ao fornecimento"));
            assert!(html.contains("Limites de inclusão"));
        }
    }
}

#[tokio::test]
async fn pareceres_explicam_estados_e_otimo_sem_modificar_calculos() {
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("app.sqlite3"))
        .await
        .unwrap();
    demonstracao::preparar(&pool).await.unwrap();
    let lote = cadastros::lotes(&pool)
        .await
        .unwrap()
        .into_iter()
        .find(|l| l.nome == "Lote demonstração")
        .unwrap();
    let alimentos = cadastros::alimentos(&pool)
        .await
        .unwrap()
        .into_iter()
        .filter(|a| !a.id.starts_with("ex-"))
        .collect::<Vec<_>>();
    let base = conhecimento::ler(&pool).await.unwrap();
    let mut cenario = demonstracao::cenario(lote, alimentos, false);
    let app = app(EstadoApp::novo(
        pool.clone(),
        "http://127.0.0.1:3000".into(),
    ));
    for formulacao in [false, true] {
        cenario.formular = formulacao;
        let parecer = if formulacao {
            formular(&cenario, &base)
        } else {
            verificar(&cenario, &base)
        };
        let id = historico::salvar(&pool, &cenario, &parecer, &base)
            .await
            .unwrap();
        let resposta = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/execucoes/{id}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resposta.status(), 200);
        let bytes = resposta.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&bytes).unwrap();
        conferir_ids(html);
        for id in ["estados", "nucleo", "balanco", "fornecimento"] {
            assert!(html.contains(&format!("popovertarget=\"dica-{id}\"")));
        }
        assert_eq!(html.contains("popovertarget=\"dica-otimo\""), formulacao);
        let preservado = historico::obter(&pool, &id).await.unwrap();
        assert_eq!(preservado.parecer.custo, parecer.custo);
        assert_eq!(preservado.parecer.condicao, parecer.condicao);
    }
}

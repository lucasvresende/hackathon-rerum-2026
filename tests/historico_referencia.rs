use axum::{body::Body, extract::Query, http::Request};
use feed_your_cow::{
    aplicacao::{exemplos, historico::FiltroHistorico},
    dominio::inferencia::verificar,
    persistencia::{banco, conhecimento, historico},
    web::{estado::EstadoApp, rotas::app},
};
use http_body_util::BodyExt;
use std::collections::BTreeMap;
use tower::ServiceExt;

#[test]
fn links_preservam_filtros_com_acentos_e_separadores() {
    let f = FiltroHistorico::novo(Some(2), "Lote São & + #?".into(), "2026-10-22".into());
    let uri = f.url(3, "ref&1").parse().unwrap();
    let Query(q) = Query::<BTreeMap<String, String>>::try_from_uri(&uri).unwrap();
    assert_eq!(q["lote"], f.busca);
    assert_eq!(q["data"], f.data);
    assert_eq!(q["referencia"], "ref&1");
    assert_eq!(q["pagina"], "3");
}

#[tokio::test]
async fn referencia_fora_da_pagina_e_ausencia_sao_tratadas_sem_perder_filtros() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let mut c = exemplos::cenario_inicial(&pool).await.unwrap();
    let base = conhecimento::ler(&pool).await.unwrap();
    c.nome = "Referência antiga".into();
    let id = historico::salvar(&pool, &c, &verificar(&c, &base), &base)
        .await
        .unwrap();
    for n in 0..101 {
        c.nome = format!("Alternativa {n}");
        historico::salvar(&pool, &c, &verificar(&c, &base), &base)
            .await
            .unwrap();
    }
    let app = app(EstadoApp::novo(pool, "http://127.0.0.1:3000".into()));
    for (referencia, existe) in [(&id[..], true), ("nao-existe", false)] {
        let uri =
            FiltroHistorico::novo(Some(1), "Girolando".into(), String::new()).url(1, referencia);
        let r = app
            .clone()
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(r.status(), 200);
        let bytes = r.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&bytes).unwrap();
        assert_eq!(html.contains("Referência fixada"), existe);
        assert_eq!(html.contains("Referência não encontrada"), !existe);
        assert!(html.contains("pagina=2&amp;lote=Girolando"));
        assert!(html.contains(&format!("name=\"referencia\" value=\"{referencia}\"")));
        if existe {
            assert!(html.contains("Referência antiga"));
            assert!(html.contains(&format!("option value=\"{id}\"")));
            assert!(html.contains("Remover referência"));
        }
    }
}

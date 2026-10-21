use axum::body::Body;
use axum::http::{Request, StatusCode};
use feed_your_cow::aplicacao::demonstracao;
use feed_your_cow::dominio::inferencia::verificar;
use feed_your_cow::persistencia::{banco, cadastros, conhecimento, historico, recuperacao};
use feed_your_cow::web::rotas::{EstadoApp, app};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn ambiente() -> (tempfile::TempDir, sqlx::SqlitePool) {
    let d = tempfile::tempdir().unwrap();
    let p = banco::preparar(&d.path().join("banco.sqlite3"))
        .await
        .unwrap();
    demonstracao::preparar(&p).await.unwrap();
    (d, p)
}

#[tokio::test]
async fn ct21_ct22_preserva_revisao_e_reproduz_execucao() {
    let (_d, pool) = ambiente().await;
    let mut lote = cadastros::lotes(&pool).await.unwrap().remove(0);
    let alimentos = cadastros::alimentos(&pool).await.unwrap();
    let c = demonstracao::cenario(lote.clone(), alimentos.clone(), false);
    let base = conhecimento::ler(&pool).await.unwrap();
    let resultado = verificar(&c, &base);
    let id = historico::salvar(&pool, &c, &resultado, &base)
        .await
        .unwrap();
    lote.peso = 500.0;
    cadastros::salvar_lote(&pool, &mut lote, false)
        .await
        .unwrap();
    let mut a = alimentos[0].clone();
    a.ms = 0.5;
    cadastros::salvar_alimento(&pool, &mut a).await.unwrap();
    let e = historico::obter(&pool, &id).await.unwrap();
    assert_eq!(e.cenario.lote.peso, 400.0);
    assert_eq!(e.cenario.itens[0].alimento.ms, c.itens[0].alimento.ms);
    let novo = verificar(&e.cenario, &e.conhecimento);
    assert_eq!(novo.custo, e.parecer.custo);
    assert_eq!(
        serde_json::to_string(&novo.verificacoes).unwrap(),
        serde_json::to_string(&e.parecer.verificacoes).unwrap()
    );
    assert_eq!(cadastros::alimentos(&pool).await.unwrap().len(), 5);
}

#[tokio::test]
async fn ct23_carga_idempotente_colisao_fonte_ausente_e_unidade() {
    let (d, pool) = ambiente().await;
    let carga = include_str!("../dados_referencia/001_conhecimento.sql");
    conhecimento::carregar(&pool, carga).await.unwrap();
    assert!(
        conhecimento::carregar(&pool, &format!("{carga}\n-- mudou"))
            .await
            .is_err()
    );
    let limpo = banco::abrir(&d.path().join("limpo.sqlite3"), true)
        .await
        .unwrap();
    banco::MIGRACOES.run(&limpo).await.unwrap();
    let ruim = carga.replace("'kg/dia'", "'litros'");
    assert!(conhecimento::carregar(&limpo, &ruim).await.is_err());
    assert!(conhecimento::ler(&limpo).await.is_err());
    let semfonte = carga.replace("'C01',\n    'Manual", "'FALTANTE',\n    'Manual");
    assert!(conhecimento::carregar(&limpo, &semfonte).await.is_err());
    let incompleta = carga.replacen("'mantenca'", "'leite'", 1);
    assert!(conhecimento::carregar(&limpo, &incompleta).await.is_err());
    assert!(conhecimento::ler(&limpo).await.is_err());
}

#[tokio::test]
async fn ct24_falha_no_trace_reverte_execucao_inteira() {
    let (_d, pool) = ambiente().await;
    let c = demonstracao::cenario(
        cadastros::lotes(&pool).await.unwrap().remove(0),
        cadastros::alimentos(&pool).await.unwrap(),
        false,
    );
    let base = conhecimento::ler(&pool).await.unwrap();
    sqlx::raw_sql(
        "CREATE TRIGGER falha_trace BEFORE INSERT ON verificacoes \
        BEGIN SELECT RAISE(ABORT, 'falha injetada'); END;",
    )
    .execute(&pool)
    .await
    .unwrap();
    assert!(
        historico::salvar(&pool, &c, &verificar(&c, &base), &base)
            .await
            .is_err()
    );
    assert!(historico::listar(&pool).await.unwrap().is_empty());
}

#[tokio::test]
async fn ct25_backup_restaura_e_nao_sobrescreve() {
    let (d, pool) = ambiente().await;
    let c = demonstracao::cenario(
        cadastros::lotes(&pool).await.unwrap().remove(0),
        cadastros::alimentos(&pool).await.unwrap(),
        false,
    );
    let base = conhecimento::ler(&pool).await.unwrap();
    let id = historico::salvar(&pool, &c, &verificar(&c, &base), &base)
        .await
        .unwrap();
    let backup = d.path().join("backup");
    let destino = d.path().join("restaurado");
    recuperacao::backup(&pool, &backup).await.unwrap();
    recuperacao::restaurar(&backup, &destino).await.unwrap();
    assert!(recuperacao::restaurar(&backup, &destino).await.is_err());
    let restaurado = banco::abrir(&destino.join("aplicacao.sqlite3"), false)
        .await
        .unwrap();
    let e = historico::obter(&restaurado, &id).await.unwrap();
    assert_eq!(
        verificar(&e.cenario, &e.conhecimento).custo,
        e.parecer.custo
    );
    std::fs::write(backup.join("anexo.md"), "adulterado").unwrap();
    assert!(
        recuperacao::restaurar(&backup, &d.path().join("outro"))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn ct27_protege_escrita_e_escapa_texto() {
    let (_d, pool) = ambiente().await;
    let estado = EstadoApp::novo(pool.clone(), "http://127.0.0.1:3000".into());
    let roteador = app(estado.clone());
    let resposta = roteador
        .clone()
        .oneshot(
            Request::builder()
                .uri("/lotes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resposta.status(), StatusCode::OK);
    let resposta = roteador
        .clone()
        .oneshot(
            Request::builder()
                .uri("/lotes")
                .method("POST")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from("nome=intruso"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resposta.status(), StatusCode::FORBIDDEN);
    let resposta = roteador
        .clone()
        .oneshot(
            Request::builder()
                .uri("/lotes")
                .method("POST")
                .header("origin", "https://site-externo.example")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(format!("csrf={}", estado.csrf)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resposta.status(), StatusCode::FORBIDDEN);
    let mut lote = cadastros::lotes(&pool).await.unwrap().remove(0);
    lote.nome = "<script>alert(1)</script>".into();
    cadastros::salvar_lote(&pool, &mut lote, false)
        .await
        .unwrap();
    let resposta = roteador
        .oneshot(
            Request::builder()
                .uri("/lotes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let corpo = String::from_utf8(
        resposta
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .to_vec(),
    )
    .unwrap();
    assert!(corpo.contains("&lt;script&gt;"));
    assert!(!corpo.contains("<script>alert(1)</script>"));
}

#[tokio::test]
async fn ct01_estrutura_incompativel_impede_inicializacao_silenciosa() {
    let (_d, pool) = ambiente().await;
    banco::verificar_estrutura(&pool).await.unwrap();
    sqlx::raw_sql("UPDATE _sqlx_migrations SET checksum = X'00' WHERE version = 1;")
        .execute(&pool)
        .await
        .unwrap();
    assert!(banco::verificar_estrutura(&pool).await.is_err());
}

#[tokio::test]
async fn ajuda_disponivel_com_secoes_e_links_contextuais() {
    let (_d, pool) = ambiente().await;
    let app = app(EstadoApp::novo(pool, "http://127.0.0.1:3000".into()));
    for (rota, secao) in [
        ("/", "inicio"),
        ("/lotes", "lotes"),
        ("/lotes/novo", "lotes"),
        ("/alimentos", "alimentos"),
        ("/alimentos/novo", "alimentos"),
        ("/cenarios/novo", "cenarios"),
        ("/historico", "historico"),
        ("/conhecimento", "conhecimento"),
        ("/ajuda", "inicio"),
        ("/inexistente", "inicio"),
    ] {
        let resposta = app
            .clone()
            .oneshot(Request::builder().uri(rota).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let esperado = if rota == "/inexistente" { 404 } else { 200 };
        assert_eq!(resposta.status().as_u16(), esperado);
        let corpo = resposta.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&corpo).unwrap();
        assert!(html.contains(&format!("href=\"/ajuda#{secao}\"")), "{rota}");
        assert!(
            html.contains(&format!(
                "href=\"/ajuda#{secao}\" target=\"_blank\" rel=\"noopener\""
            )),
            "{rota}"
        );
        if rota == "/ajuda" {
            for id in [
                "inicio",
                "lotes",
                "alimentos",
                "cenarios",
                "verificar",
                "formular",
                "resultados",
                "historico",
                "comparacao",
                "conhecimento",
            ] {
                assert!(html.contains(&format!("id=\"{id}\"")));
                assert!(html.contains(&format!("aria-controls=\"{id}-conteudo\"")));
            }
            assert!(html.contains("hx-live:class=\"{ 'help-open': data.aberta, "));
            assert!(html.contains("'help-ready': data.pronta }\""));
            assert!(html.contains("hx-live:inert=\"!data.aberta\""));
        }
    }
}

use axum::body::Body;
use axum::http::Request;
use feed_your_cow::persistencia::{banco, cadastros, conhecimento, recuperacao, templates};
use feed_your_cow::web::rotas::{EstadoApp, app};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn carga_versionada_converte_unidades_e_rejeita_ativacao_parcial() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let carga = include_str!("../dados_referencia/004_modelos.sql");
    conhecimento::carregar(&pool, carga).await.unwrap();
    let modelos = templates::ler(&mut pool.acquire().await.unwrap())
        .await
        .unwrap();
    assert_eq!(modelos.len(), 25);
    let silagem = modelos.iter().find(|m| m.id == "silagem-milho-v1").unwrap();
    let ca = silagem.valores.iter().find(|v| v.campo == "ca").unwrap();
    assert_eq!(ca.valor, "0.19");
    assert_eq!(ca.valor_original, "1.90");
    assert_eq!(ca.unidade_original, "g/kg MS");
    assert!(!silagem.valores.iter().any(|v| v.campo == "ndt"));
    assert!(
        conhecimento::carregar(&pool, &format!("{carga}\n-- mudou"))
            .await
            .is_err()
    );
    let invalida = "-- versao: teste-invalido\n\
        INSERT INTO modelos_preenchimento VALUES\
        ('invalido','1','Inválido','alimento','contexto','lacunas','TPL-FEED-SILAGEM');\n\
        INSERT INTO valores_modelo VALUES\
        ('invalido','ms','32','litros','32','litros','MEDIA_TABELADA','TPL-FEED-SILAGEM');";
    assert!(conhecimento::carregar(&pool, invalida).await.is_err());
    assert_eq!(
        templates::ler(&mut pool.acquire().await.unwrap())
            .await
            .unwrap()
            .len(),
        25
    );
    let sem_fonte = invalida
        .replace("teste-invalido", "teste-sem-fonte")
        .replace("TPL-FEED-SILAGEM", "FONTE-INEXISTENTE");
    assert!(conhecimento::carregar(&pool, &sem_fonte).await.is_err());
    let seguranca = invalida
        .replace("teste-invalido", "teste-seguranca")
        .replace(
            "'ms','32','litros','32','litros'",
            "'autorizado','SIM','texto','SIM','texto'",
        );
    assert!(conhecimento::carregar(&pool, &seguranca).await.is_err());
}

#[tokio::test]
async fn template_editado_preserva_original_e_procedencia_no_backup() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let estado = EstadoApp::novo(pool.clone(), "http://127.0.0.1:3000".into());
    let app = app(estado.clone());
    let modelo = format!(
        "csrf={}&modelo=silagem-milho-v1&acao_modelo=aplicar",
        estado.csrf
    );
    let post = |uri: &str, corpo: String| {
        Request::builder()
            .uri(uri)
            .method("POST")
            .header("content-type", "application/x-www-form-urlencoded")
            .header("hx-request", "true")
            .body(Body::from(corpo))
            .unwrap()
    };
    let resposta = app
        .clone()
        .oneshot(post("/cadastros/alimento/modelo", modelo.clone()))
        .await
        .unwrap();
    let bytes = resposta.into_body().collect().await.unwrap().to_bytes();
    let html = std::str::from_utf8(&bytes).unwrap();
    assert!(html.contains("Confirme a substituição"));
    assert_eq!(cadastros::alimentos(&pool).await.unwrap().len(), 15);
    let resposta = app
        .clone()
        .oneshot(post(
            "/cadastros/alimento/modelo",
            format!("{modelo}&confirmar_modelo=on&ndt=99"),
        ))
        .await
        .unwrap();
    let bytes = resposta.into_body().collect().await.unwrap().to_bytes();
    let html = std::str::from_utf8(&bytes).unwrap();
    assert!(html.contains("value=\"32.5\""));
    assert!(
        !html.contains("value=\"99\""),
        "Lacuna não herda valor de outro template"
    );
    let salvar = format!(
        "csrf={}&modelo_aplicado=silagem-milho-v1&nome=Silagem&classe=FORRAGEM\
        &ms=32.5&pb=8&fdn=44.3&ca=0.19&p=0.17&amido=29.1&ee=2.8\
        &fonte_id=TPL-FEED-SILAGEM&natureza=MEDIA_TABELADA&origem=Operador&metodo=Referencia",
        estado.csrf
    );
    let resposta = app.oneshot(post("/alimentos", salvar)).await.unwrap();
    assert!(resposta.headers().contains_key("hx-redirect"));
    let alimento = cadastros::alimentos(&pool)
        .await
        .unwrap()
        .into_iter()
        .find(|a| !a.id.starts_with("ex-"))
        .unwrap();
    assert_eq!(alimento.nutrientes[0], None);
    let base = conhecimento::ler(&pool).await.unwrap();
    let mut c = feed_your_cow::aplicacao::demonstracao::cenario(
        feed_your_cow::aplicacao::demonstracao::lote(),
        vec![alimento.clone()],
        false,
    );
    c.itens[0].quantidade = 10.0;
    let parecer = feed_your_cow::dominio::inferencia::verificar(&c, &base);
    assert_ne!(parecer.dimensao("Núcleo químico"), "ATENDE");
    let p = alimento.procedencia.unwrap();
    assert!(p.campos.iter().find(|c| c.campo == "pb").unwrap().alterado);
    assert_eq!(
        p.modelo
            .valores
            .iter()
            .find(|v| v.campo == "pb")
            .unwrap()
            .valor,
        "6.9"
    );
    let modelos = templates::ler(&mut pool.acquire().await.unwrap())
        .await
        .unwrap();
    assert_eq!(
        modelos
            .iter()
            .find(|m| m.id == "silagem-milho-v1")
            .unwrap()
            .valores
            .iter()
            .find(|v| v.campo == "pb")
            .unwrap()
            .valor,
        "6.9"
    );
    let backup = d.path().join("backup");
    recuperacao::backup(&pool, &backup).await.unwrap();
    let destino = d.path().join("restaurado");
    recuperacao::restaurar(&backup, &destino).await.unwrap();
    let restaurado = banco::abrir(&destino.join("aplicacao.sqlite3"), false)
        .await
        .unwrap();
    assert_eq!(
        templates::ler(&mut restaurado.acquire().await.unwrap())
            .await
            .unwrap()
            .len(),
        25
    );
    let a = cadastros::alimentos(&restaurado)
        .await
        .unwrap()
        .into_iter()
        .find(|a| !a.id.starts_with("ex-"))
        .unwrap();
    assert_eq!(
        a.procedencia
            .unwrap()
            .campos
            .iter()
            .find(|c| c.campo == "pb")
            .unwrap()
            .utilizado,
        "8"
    );
}

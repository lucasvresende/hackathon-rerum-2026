use feed_your_cow::persistencia::{banco, cadastros};
use feed_your_cow::web::entradas::{completar_cenario, padroes_cenario};
use feed_your_cow::web::templates::formularios::cenario::FormularioCenario;
use maud::Render;

#[tokio::test]
async fn padroes_nao_inventam_cadastros_cotacoes_ou_disponibilidade() {
    let mut campos = padroes_cenario();
    assert_eq!(campos["horizonte"], "1");
    assert_eq!(campos["sobras"], "0");
    assert_eq!(campos["base"], "CONSUMO_MS");
    assert!(campos["origem_sobras"].contains("Hipótese"));
    let hoje = chrono::Utc::now()
        .with_timezone(&chrono_tz::America::Sao_Paulo)
        .format("%Y-%m-%d")
        .to_string();
    assert_eq!(campos["data_avaliacao"], hoje);
    assert_eq!(campos["data_aplicacao"], hoje);
    for nome in ["nome", "lote", "consumo_alvo"] {
        assert!(!campos.contains_key(nome));
    }
    let diretorio = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&diretorio.path().join("teste.sqlite3"))
        .await
        .unwrap();
    let alimentos = cadastros::alimentos(&pool).await.unwrap();
    completar_cenario(&mut campos, &alimentos);
    let id = &alimentos[0].composicao_id;
    assert_eq!(campos[&format!("perda-{id}")], "0");
    for prefixo in ["usar", "preco", "estoque", "livre"] {
        assert!(!campos.contains_key(&format!("{prefixo}-{id}")));
    }
    campos.insert(format!("perda-{id}"), "7".into());
    completar_cenario(&mut campos, &alimentos);
    assert_eq!(campos[&format!("perda-{id}")], "7");
}

#[tokio::test]
async fn apenas_selecionados_exibem_editores_e_rascunho_e_preservado() {
    let diretorio = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&diretorio.path().join("teste.sqlite3"))
        .await
        .unwrap();
    let alimentos = cadastros::alimentos(&pool).await.unwrap();
    let mut campos = padroes_cenario();
    completar_cenario(&mut campos, &alimentos);
    let id = &alimentos[0].composicao_id;
    let renderizar = |campos: &_| {
        FormularioCenario {
            campos,
            alimentos: &alimentos,
            lotes: &[],
            erros: &[],
            token: "teste",
        }
        .render()
        .into_string()
    };
    let html = renderizar(&campos);
    assert!(!html.contains("class=\"ingredient\""));
    assert!(html.contains("Selecionar ingredientes"));
    campos.insert(format!("usar-{id}"), "on".into());
    campos.insert(format!("q-{id}"), "3.5".into());
    let html = renderizar(&campos);
    assert_eq!(html.matches("class=\"ingredient\"").count(), 1);
    assert!(html.contains("Configurações avançadas"));
    campos.remove(&format!("usar-{id}"));
    let html = renderizar(&campos);
    assert!(!html.contains("class=\"ingredient\""));
    assert!(html.contains(&format!("type=\"hidden\" name=\"q-{id}\" value=\"3.5\"")));
}

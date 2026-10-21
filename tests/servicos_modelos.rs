use feed_your_cow::aplicacao::{demonstracao, modelos_preenchimento as modelos};
use feed_your_cow::dominio::templates::TipoCadastro;
use feed_your_cow::persistencia::{banco, cadastros};

#[tokio::test]
async fn carga_parcial_limpa_lacunas_e_nao_confirma_estados_nem_grava() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let previa = modelos::preparar(
        &pool,
        TipoCadastro::Alimento,
        "silagem-milho-v1",
        modelos::AcaoModelo::Aplicar { confirmado: false },
    )
    .await
    .unwrap();
    assert!(previa.previa);
    assert!(previa.substituicoes.is_empty());
    assert_eq!(previa.erros[0].campo, "confirmar_modelo");
    for (tipo, id, lacuna, estado) in [
        (
            TipoCadastro::Alimento,
            "silagem-milho-v1",
            "ndt",
            "autorizado",
        ),
        (
            TipoCadastro::Lote,
            "jersey-estudo-v1",
            "gordura",
            "gestante",
        ),
    ] {
        let carga = modelos::preparar(
            &pool,
            tipo,
            id,
            modelos::AcaoModelo::Aplicar { confirmado: true },
        )
        .await
        .unwrap();
        assert!(!carga.previa);
        assert_eq!(carga.aplicado.as_deref(), Some(id));
        assert!(carga.erros.is_empty());
        assert!(
            carga
                .substituicoes
                .iter()
                .any(|(c, v)| c == lacuna && v.is_empty())
        );
        assert!(
            carga
                .substituicoes
                .iter()
                .any(|(c, v)| c == estado && v == "DESCONHECIDO")
        );
    }
    let incompativel = modelos::preparar(
        &pool,
        TipoCadastro::Lote,
        "silagem-milho-v1",
        modelos::AcaoModelo::Previa,
    )
    .await
    .unwrap();
    assert!(!incompativel.previa);
    assert_eq!(incompativel.erros[0].campo, "modelo");
    assert!(cadastros::lotes(&pool).await.unwrap().is_empty());
    assert!(cadastros::alimentos(&pool).await.unwrap().is_empty());
}

#[tokio::test]
async fn procedencia_tipadas_preservam_unidades_lacunas_e_valores_originais() {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    demonstracao::preparar(&pool).await.unwrap();
    let mut a = cadastros::alimentos(&pool).await.unwrap().remove(0);
    a.ms = 0.325;
    a.nutrientes[0] = Some(0.7);
    a.nutrientes[1] = Some(0.069);
    let p = modelos::procedencia_alimento(&pool, "silagem-milho-v1", &a)
        .await
        .unwrap()
        .unwrap();
    assert!(!p.campos.iter().find(|c| c.campo == "ms").unwrap().alterado);
    assert!(!p.campos.iter().find(|c| c.campo == "pb").unwrap().alterado);
    let ndt = p.campos.iter().find(|c| c.campo == "ndt").unwrap();
    assert!(ndt.alterado);
    assert_eq!(ndt.utilizado, "70");
    assert!(!p.modelo.valores.iter().any(|v| v.campo == "ndt"));
    let mut lote = demonstracao::lote();
    lote.raca = "Jersey".into();
    lote.peso = 389.0;
    lote.producao = 21.0;
    let p = modelos::procedencia_lote(&pool, "jersey-estudo-v1", &lote)
        .await
        .unwrap()
        .unwrap();
    let peso = p.campos.iter().find(|c| c.campo == "peso").unwrap();
    assert!(!peso.alterado);
    assert_eq!(
        peso.utilizado, "389.0",
        "Preserva a representação histórica do lote"
    );
    assert!(
        p.campos
            .iter()
            .find(|c| c.campo == "gordura")
            .unwrap()
            .alterado
    );
    assert!(
        modelos::procedencia_lote(&pool, "silagem-milho-v1", &lote)
            .await
            .is_err()
    );
    assert!(
        modelos::procedencia_lote(&pool, "", &lote)
            .await
            .unwrap()
            .is_none()
    );
}

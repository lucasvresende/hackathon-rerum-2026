use feed_your_cow::aplicacao::demonstracao;
use feed_your_cow::dominio::otimizacao::formular;
use feed_your_cow::dominio::unidades::proximos;
use feed_your_cow::dominio::*;
use feed_your_cow::persistencia::{banco, cadastros, conhecimento};

async fn exemplo() -> (Cenario, Conhecimento) {
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("teste.sqlite3"))
        .await
        .unwrap();
    demonstracao::preparar(&pool).await.unwrap();
    let mut c = demonstracao::cenario(
        cadastros::lotes(&pool).await.unwrap().remove(0),
        cadastros::alimentos(&pool).await.unwrap(),
        false,
    );
    c.formular = true;
    let base = conhecimento::ler(&pool).await.unwrap();
    pool.close().await;
    (c, base)
}

#[tokio::test]
async fn ct13_resolve_cinco_subproblemas_e_revalida() {
    let (c, base) = exemplo().await;
    let p = formular(&c, &base);
    assert_eq!(
        p.busca.as_deref(),
        Some("OTIMO_NO_MODELO"),
        "{:#?}",
        p.subproblemas
    );
    assert_eq!(p.subproblemas.len(), 5);
    assert!(p.custo.unwrap() < 11.792177413230045);
    assert!(proximos(p.consumo_ms, 10.8));
    assert!(p.pratico_valido);
    assert_eq!(p.dimensao("Núcleo químico"), "ATENDE");
    assert_eq!(p.dimensao("Barreiras ruminais"), "ATENDE");
}

#[tokio::test]
async fn ct13_oraculo_pequeno_com_otimo_conhecido() {
    let (mut c, base) = exemplo().await;
    c.itens.retain(|i| i.alimento.id == "silagem");
    let i = &mut c.itens[0];
    i.alimento.nutrientes = [
        Some(0.8),
        Some(0.2),
        Some(0.01),
        Some(0.01),
        Some(0.5),
        Some(0.1),
        Some(0.03),
    ];
    let p = formular(&c, &base);
    assert_eq!(p.busca.as_deref(), Some("OTIMO_NO_MODELO"));
    assert!(proximos(p.fornecimento[0].consumo_ms, 10.8));
    assert!(proximos(p.custo.unwrap(), 10.8 * 0.25 / 0.35));
}

#[tokio::test]
async fn ct14_distingue_inviabilidade_e_interrupcao() {
    let (mut c, base) = exemplo().await;
    for i in &mut c.itens {
        i.maximo_ms = Some(0.0);
    }
    assert_eq!(
        formular(&c, &base).busca.as_deref(),
        Some("INVIAVEL_NO_MODELO")
    );
    for i in &mut c.itens {
        i.maximo_ms = None;
    }
    c.limite_segundos = 1e-12;
    assert_eq!(formular(&c, &base).busca.as_deref(), Some("INCONCLUSIVA"));
}

#[tokio::test]
async fn ct15_minimo_excluido_e_estoque_nao_sao_relaxados() {
    let (mut c, base) = exemplo().await;
    c.itens[0].minimo_ms = 1.0;
    c.itens[0].alimento.nnp_adicionado = Conhecido::Sim;
    assert_eq!(
        formular(&c, &base).busca.as_deref(),
        Some("INVIAVEL_NO_MODELO")
    );
    c.itens[0].minimo_ms = 0.0;
    c.itens[0].alimento.nnp_adicionado = Conhecido::Nao;
    for i in &mut c.itens {
        i.estoque = Some(0.0);
        i.sem_limite = false;
    }
    assert_eq!(
        formular(&c, &base).busca.as_deref(),
        Some("INVIAVEL_NO_MODELO")
    );
}

#[tokio::test]
async fn ct16_desempate_deterministico_e_ct17_arredondamento() {
    let (mut c, base) = exemplo().await;
    let a = formular(&c, &base);
    c.itens.reverse();
    let b = formular(&c, &base);
    assert!(proximos(a.custo.unwrap(), b.custo.unwrap()));
    for (x, y) in a.fornecimento.iter().zip(&b.fornecimento) {
        assert_eq!(x.ingrediente, y.ingrediente);
        assert!(proximos(x.consumo_ms, y.consumo_ms));
    }
    for i in &mut c.itens {
        i.resolucao_mn = 100.0;
    }
    let p = formular(&c, &base);
    assert_eq!(p.busca.as_deref(), Some("OTIMO_NO_MODELO"));
    assert!(!p.pratico_valido);
    assert!(
        p.avisos
            .iter()
            .any(|v| v.contains("FORNECIMENTO_INCOMPATIVEL"))
    );
}

use feed_your_cow::{
    aplicacao::exemplos,
    dominio::{BaseQuantidade, inferencia, otimizacao},
    persistencia::{banco, conhecimento},
    web::formatacao::{balanco, medida, quantidade_pratica},
};

#[tokio::test]
async fn recomendacao_exige_resolucao_apenas_dos_ingredientes_utilizados() {
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("teste.sqlite3"))
        .await
        .unwrap();
    let mut c = exemplos::cenario_inicial(&pool).await.unwrap();
    let base = conhecimento::ler(&pool).await.unwrap();
    c.formular = true;
    let continuo = otimizacao::formular(&c, &base);
    assert_eq!(continuo.busca.as_deref(), Some("OTIMO_NO_MODELO"));
    assert!(continuo.pratico_valido);
    assert_eq!(continuo.estado_fornecimento(&c), "RESOLUCAO_PENDENTE");
    assert!(!continuo.fornecimento_recomendado(&c));
    for item in &mut c.itens {
        let f = continuo
            .fornecimento
            .iter()
            .find(|f| f.composicao_id == item.alimento.composicao_id)
            .unwrap();
        item.resolucao_mn = f.oferta_mn;
    }
    let p = otimizacao::formular(&c, &base);
    assert!(c.itens.iter().any(|i| i.resolucao_mn == 0.0));
    assert!(p.fornecimento_recomendado(&c));
    let mut copiada = c.clone();
    copiada.formular = false;
    for item in &mut copiada.itens {
        let f = p
            .fornecimento
            .iter()
            .find(|f| f.composicao_id == item.alimento.composicao_id)
            .unwrap();
        item.quantidade = quantidade_pratica(f.pratico_mn)
            .replace(',', ".")
            .parse()
            .unwrap();
        item.base = BaseQuantidade::ConsumoMn;
        item.resolucao_mn = 0.0;
    }
    let revalidada = inferencia::verificar(&copiada, &base);
    assert_eq!(revalidada.dimensao("Núcleo químico"), "ATENDE");
    assert!(revalidada.pratico_valido);
    c.consumo_alvo = Some(1.0);
    let inviavel = otimizacao::formular(&c, &base);
    assert_eq!(inviavel.estado_fornecimento(&c), "SEM_SOLUCAO");
}

#[test]
fn diferencas_que_causam_violacao_nao_desaparecem_na_formatacao() {
    assert_eq!(medida(0.000000353, 5), "3,530e-7");
    assert_eq!(balanco(2.313199647, 2.3132), "-3,530e-7");
    assert_eq!(balanco(2.3132 - f64::EPSILON, 2.3132), "0,00000");
}

#[tokio::test]
async fn pendencias_e_inviabilidade_identificam_o_ingrediente_e_a_restricao() {
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("base.sqlite3"))
        .await
        .unwrap();
    let mut c = exemplos::cenario_inicial(&pool).await.unwrap();
    let base = conhecimento::ler(&pool).await.unwrap();
    c.formular = true;
    let i = c
        .itens
        .iter_mut()
        .find(|i| i.alimento.id == "ex-a01")
        .unwrap();
    i.sem_limite = false;
    let p = otimizacao::formular(&c, &base);
    assert_eq!(p.busca.as_deref(), Some("INCONCLUSIVA"));
    assert!(
        p.subproblemas.iter().all(
            |s| s.motivo.contains("Silagem de milho") && s.motivo.contains("informe o estoque")
        )
    );
    c.itens
        .iter_mut()
        .find(|i| i.alimento.id == "ex-a01")
        .unwrap()
        .estoque = Some(100.0);
    let p = otimizacao::formular(&c, &base);
    assert_eq!(p.busca.as_deref(), Some("INVIAVEL_NO_MODELO"));
    assert!(
        p.subproblemas
            .iter()
            .all(|s| s.motivo.contains("Silagem de milho") && s.motivo.contains("FDN de forragem"))
    );
}

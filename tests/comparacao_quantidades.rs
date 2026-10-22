use feed_your_cow::{
    aplicacao::{
        comparacao::{Comparacao, fornecimento},
        exemplos,
    },
    dominio::{inferencia::verificar, otimizacao::formular},
    persistencia::{banco, conhecimento, historico::Execucao},
    web::templates::paginas::comparacao::Comparacao as Tela,
};
use maud::Render;

async fn exemplo(formular_dieta: bool) -> Execucao {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("app.sqlite3"))
        .await
        .unwrap();
    let conhecimento = conhecimento::ler(&pool).await.unwrap();
    let mut cenario = exemplos::cenario_inicial(&pool).await.unwrap();
    cenario.formular = formular_dieta;
    let parecer = if formular_dieta {
        formular(&cenario, &conhecimento)
    } else {
        verificar(&cenario, &conhecimento)
    };
    Execucao {
        id: "teste".into(),
        cenario_id: "cenario".into(),
        criada_em: String::new(),
        cenario,
        conhecimento,
        parecer,
    }
}

#[tokio::test]
async fn associacao_usa_id_e_so_aceita_nome_legado_unico() {
    let mut e = exemplo(false).await;
    let item = e.cenario.itens[0].clone();
    e.parecer.fornecimento.reverse();
    let f = fornecimento(&e, &item, false).unwrap();
    assert_eq!(f.composicao_id, item.alimento.composicao_id);
    for f in &mut e.parecer.fornecimento {
        f.composicao_id.clear();
    }
    assert!(fornecimento(&e, &item, false).is_ok());
    e.cenario.itens.push(item.clone());
    assert!(
        fornecimento(&e, &item, false)
            .unwrap_err()
            .contains("ambígua")
    );
    e.cenario.itens.pop();
    let linha = fornecimento(&e, &item, false).unwrap().clone();
    e.parecer.fornecimento.push(linha);
    assert!(
        fornecimento(&e, &item, false)
            .unwrap_err()
            .contains("ambígua")
    );
}

#[tokio::test]
async fn usa_candidata_sem_inventar_resultado_ou_pesagem() {
    let mut e = exemplo(true).await;
    let item = e.cenario.itens[0].clone();
    let salvo = fornecimento(&e, &item, false).unwrap().consumo_ms;
    e.cenario.itens[0].quantidade = 9876.0;
    assert_eq!(fornecimento(&e, &item, false).unwrap().consumo_ms, salvo);
    assert!(fornecimento(&e, &item, true).is_err());
    let c = Comparacao::entre(e.clone(), e.clone());
    let html = Tela { comparacao: &c }.render().into_string();
    assert!(html.contains("Consumo calculado (kg MS)"));
    assert!(html.contains("Custo do ingrediente"));
    assert!(html.contains("Balanço nutricional"));
    assert!(html.contains("Resultados por dimensão"));
    assert!(!html.contains("9876"));
    for estado in ["INVIAVEL_NO_MODELO", "INCONCLUSIVA"] {
        e.parecer.busca = Some(estado.into());
        assert!(
            fornecimento(&e, &item, false)
                .unwrap_err()
                .contains("sem candidata")
        );
    }
}

#[tokio::test]
async fn composicao_revisada_e_ingredientes_ausentes_ficam_explicitos() {
    let a = exemplo(false).await;
    let mut b = a.clone();
    let id = b.cenario.itens[0].alimento.composicao_id.clone();
    b.cenario.itens[0].alimento.composicao_id = "revisao-nova".into();
    b.parecer
        .fornecimento
        .iter_mut()
        .find(|f| f.composicao_id == id)
        .unwrap()
        .composicao_id = "revisao-nova".into();
    b.cenario.itens.remove(1);
    let c = Comparacao::entre(a, b);
    let html = Tela { comparacao: &c }.render().into_string();
    assert!(html.contains("Revisões de composição diferentes"));
    assert!(html.contains("revisao-nova"));
    assert!(html.contains("Ausente"));
}

#[tokio::test]
async fn mostra_pratico_revalidado_mesmo_incompativel() {
    let mut e = exemplo(true).await;
    for i in &mut e.cenario.itens {
        i.resolucao_mn = 1.0;
    }
    e.parecer = formular(&e.cenario, &e.conhecimento);
    let item = &e.cenario.itens[0];
    let f = fornecimento(&e, item, true).unwrap();
    assert_eq!(f.oferta_mn.fract(), 0.0);
    let c = Comparacao::entre(e.clone(), e);
    let html = Tela { comparacao: &c }.render().into_string();
    assert!(html.contains("Oferta prática (kg MN)"));
    assert!(html.contains("não usar como recomendação"));
}

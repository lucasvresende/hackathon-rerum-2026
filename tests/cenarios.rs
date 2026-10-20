use feed_your_cow::aplicacao::demonstracao;
use feed_your_cow::dominio::inferencia::verificar;
use feed_your_cow::dominio::unidades::{normalizar_preco, proximos};
use feed_your_cow::dominio::*;
use feed_your_cow::persistencia::{banco, cadastros, conhecimento};
use feed_your_cow::web::formularios::{campos_cenario, ler_cenario};

async fn exemplo() -> (Cenario, Conhecimento) {
    let d = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&d.path().join("teste.sqlite3"))
        .await
        .unwrap();
    demonstracao::preparar(&pool).await.unwrap();
    let c = demonstracao::cenario(
        cadastros::lotes(&pool).await.unwrap().remove(0),
        cadastros::alimentos(&pool).await.unwrap(),
        false,
    );
    let b = conhecimento::ler(&pool).await.unwrap();
    pool.close().await;
    (c, b)
}

#[test]
fn ct05_logica_trivalente_nao_inventa_informacao() {
    use Conhecido::*;
    assert_eq!(Desconhecido.negar(), Desconhecido);
    assert_eq!(Desconhecido.e(Nao), Nao);
    assert_eq!(Desconhecido.e(Sim), Desconhecido);
    assert_eq!(Desconhecido.ou(Sim), Sim);
    assert_eq!(Desconhecido.ou(Nao), Desconhecido);
}

#[test]
fn ct01_precos_comerciais_preservam_normalizacao_decimal() {
    let mut c = CotacaoOriginal {
        valor: "1300.00".into(),
        unidade: "t".into(),
        massa_embalagem: None,
    };
    assert_eq!(normalizar_preco(&c).as_deref(), Some("1.3"));
    c.valor = "65.00".into();
    c.unidade = "embalagem".into();
    c.massa_embalagem = Some(50.0);
    assert_eq!(normalizar_preco(&c).as_deref(), Some("1.3"));
    c.massa_embalagem = None;
    assert!(normalizar_preco(&c).is_none());
    c.unidade = String::new();
    assert!(normalizar_preco(&c).is_none());
}

#[tokio::test]
async fn ct04_aviso_de_consumo_so_acima_de_dez_porcento() {
    let (mut c, b) = exemplo().await;
    for i in &mut c.itens {
        i.quantidade *= 1.1;
    }
    assert!(
        !verificar(&c, &b)
            .avisos
            .iter()
            .any(|a| a.contains("difere"))
    );
    c.itens[0].quantidade += 0.0001;
    assert!(
        verificar(&c, &b)
            .avisos
            .iter()
            .any(|a| a.contains("difere"))
    );
}

#[tokio::test]
async fn ct12_fibra_incompativel_nao_aprova_dimensao() {
    let (mut c, b) = exemplo().await;
    c.itens[0].alimento.pendf = Some(0.5);
    c.itens[0].alimento.metodo_fisico = "Peneiras incompatíveis".into();
    let p = verificar(&c, &b);
    assert_eq!(p.dimensao("Núcleo químico"), "ATENDE");
    assert_eq!(p.dimensao("Fibra física e micronutrientes"), "NAO_AVALIADO");
}

#[tokio::test]
async fn ct17_revalida_tambem_arredondamento_da_verificacao() {
    let (mut c, b) = exemplo().await;
    for i in &mut c.itens {
        i.resolucao_mn = 100.0;
    }
    let p = verificar(&c, &b);
    assert!(!p.pratico_valido);
    assert_eq!(p.dimensao("Núcleo químico"), "ATENDE");
    assert!(
        p.avisos
            .iter()
            .any(|a| a.contains("FORNECIMENTO_INCOMPATIVEL"))
    );
}

#[tokio::test]
async fn ct20_simulacao_ms_fixa_e_mn_fixa() {
    let (c, b) = exemplo().await;
    let lotes = vec![c.lote.clone()];
    let alimentos: Vec<_> = c.itens.iter().map(|i| i.alimento.clone()).collect();
    let mut campos = campos_cenario(&c);
    let id = &c.itens[0].alimento.composicao_id;
    campos.insert(
        format!("ms-{id}"),
        (c.itens[0].alimento.ms * 50.0).to_string(),
    );
    let fixams = ler_cenario(&campos, &lotes, &alimentos).unwrap();
    assert!(proximos(
        verificar(&fixams, &b).consumo_ms,
        verificar(&c, &b).consumo_ms
    ));
    assert!(verificar(&fixams, &b).custo.unwrap() > verificar(&c, &b).custo.unwrap());
    campos.insert("base_comparacao".into(), "Quantidade de MN fixa".into());
    let fixamn = ler_cenario(&campos, &lotes, &alimentos).unwrap();
    assert!(verificar(&fixamn, &b).consumo_ms < verificar(&c, &b).consumo_ms);
    assert!(proximos(
        verificar(&fixamn, &b).custo.unwrap(),
        verificar(&c, &b).custo.unwrap()
    ));
    assert!(proximos(c.itens[0].alimento.ms, alimentos[0].ms));
}

#[tokio::test]
async fn ct08_observacao_de_consumo_nao_e_dieta_ingerida_medida() {
    let (mut c, b) = exemplo().await;
    c.medicao = Some(MedicaoConsumo {
        oferta_ms: 120.0,
        sobras_ms: 12.0,
        periodo_dias: 1.0,
        animais: 12,
    });
    assert!(proximos(c.consumo_informado().unwrap(), 9.0));
    assert!(validacao::validar(&c).is_empty());
    c.medicao.as_mut().unwrap().sobras_ms = 121.0;
    assert!(!validacao::validar(&c).is_empty());
    assert_eq!(verificar(&c, &b).condicao, "INVALIDO");
}

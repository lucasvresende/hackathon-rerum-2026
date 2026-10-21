use feed_your_cow::aplicacao::demonstracao;
use feed_your_cow::dominio::calculos::*;
use feed_your_cow::dominio::inferencia::verificar;
use feed_your_cow::dominio::unidades::{MassaMn, proximos};
use feed_your_cow::dominio::*;
use feed_your_cow::persistencia::{banco, cadastros, conhecimento};

async fn exemplo(a: bool) -> (tempfile::TempDir, Cenario, Conhecimento) {
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("teste.sqlite3"))
        .await
        .unwrap();
    demonstracao::preparar(&pool).await.unwrap();
    let c = demonstracao::cenario(
        cadastros::lotes(&pool)
            .await
            .unwrap()
            .into_iter()
            .find(|l| l.nome == "Lote demonstração")
            .unwrap(),
        cadastros::alimentos(&pool)
            .await
            .unwrap()
            .into_iter()
            .filter(|a| !a.id.starts_with("ex-"))
            .collect::<Vec<_>>(),
        a,
    );
    let base = conhecimento::ler(&pool).await.unwrap();
    pool.close().await;
    (dir, c, base)
}

#[test]
fn ct01_converte_materia_natural_e_nutriente() {
    let ms = MassaMn(20.0).materia_seca(0.35);
    assert!(proximos(ms.0, 7.0));
    assert!(proximos(ms.0 * 0.08, 0.56));
}

#[tokio::test]
async fn ct02_ct03_requisitos_e_interpolacao() {
    let (_dir, c, base) = exemplo(false).await;
    let p = verificar(&c, &base);
    assert!(proximos(p.lcg4, 10.0));
    assert!(proximos(p.consumo_referencia.unwrap(), 10.8));
    for (obtido, esperado) in p
        .requisitos
        .unwrap()
        .iter()
        .zip([6.35, 1.218, 0.04810, 0.031])
    {
        assert!(proximos(*obtido, esperado), "{obtido} != {esperado}");
    }
    assert!(proximos(
        consumo_referencia(&base, 450.0, 12.5).unwrap(),
        12.7125
    ));
    assert!(proximos(
        requisitos(&base, 425.0, 0.0, 3.75).unwrap()[0],
        3.275
    ));
    assert!(proximos(
        requisitos(&base, 425.0, 1.0, 3.75).unwrap()[0] - 3.275,
        0.3115
    ));
}

#[tokio::test]
async fn ct04_bordas_e_ausencia_de_extrapolacao() {
    let (_dir, mut c, base) = exemplo(false).await;
    for peso in [400.0, 450.0, 500.0, 550.0, 600.0, 650.0, 700.0] {
        assert!(requisitos(&base, peso, 10.0, 4.0).is_some());
    }
    for gordura in [3.0, 3.5, 4.0, 4.5, 5.0] {
        assert!(requisitos(&base, 400.0, 10.0, gordura).is_some());
    }
    assert!(consumo_referencia(&base, 650.0, 10.0).is_none());
    c.lote.peso = 650.0;
    assert_eq!(verificar(&c, &base).condicao, "INCONCLUSIVO");
    c.lote.peso = 700.0;
    c.consumo_alvo = Some(15.0);
    assert_ne!(verificar(&c, &base).condicao, "FORA_DO_PERFIL");
    c.lote.peso = 700.01;
    assert_eq!(verificar(&c, &base).condicao, "FORA_DO_PERFIL");
    c.lote.peso = 400.0;
    c.lote.gordura = 2.99;
    assert_eq!(verificar(&c, &base).condicao, "FORA_DO_PERFIL");
    c.lote.gordura = 5.01;
    assert_eq!(verificar(&c, &base).condicao, "FORA_DO_PERFIL");
    c.lote.gordura = 4.0;
    c.lote.dias_lactacao = 59;
    assert_eq!(verificar(&c, &base).condicao, "FORA_DO_PERFIL");
    c.lote.dias_lactacao = 251;
    assert_eq!(verificar(&c, &base).condicao, "FORA_DO_PERFIL");
}

#[tokio::test]
async fn ct05_invalidos_e_desconhecidos() {
    let (_dir, mut c, base) = exemplo(false).await;
    for peso in [f64::NAN, f64::INFINITY, -1.0, 0.0] {
        c.lote.peso = peso;
        assert_eq!(verificar(&c, &base).condicao, "INVALIDO");
    }
    c.lote.peso = 400.0;
    c.lote.gestante = Conhecido::Desconhecido;
    let p = verificar(&c, &base);
    assert!(p.requisitos.is_none());
    assert_eq!(p.dimensao("Elegibilidade"), "INCONCLUSIVO");
}

#[tokio::test]
async fn ct06_dieta_a_acumula_violacoes() {
    let (_dir, mut c, base) = exemplo(true).await;
    let p = verificar(&c, &base);
    assert!(proximos(
        100.0 * p.totais[5].unwrap() / p.consumo_ms,
        26.77777777777778
    ));
    assert_eq!(p.dimensao("Barreiras ruminais"), "NAO_ATENDE");
    for i in &mut c.itens {
        i.alimento.nutrientes[1] = Some(0.0);
    }
    let p = verificar(&c, &base);
    assert_eq!(p.dimensao("Núcleo químico"), "NAO_ATENDE");
    assert_eq!(p.dimensao("Barreiras ruminais"), "NAO_ATENDE");
    assert!(
        p.verificacoes
            .iter()
            .any(|v| v.explicacao.contains("POLITICA_MVP"))
    );
}

#[tokio::test]
async fn ct07_ct18_ct19_dieta_b_nutrientes_custo_e_escala() {
    let (_dir, mut c, base) = exemplo(false).await;
    let p = verificar(&c, &base);
    for (a, b) in p.totais[..4].iter().zip([7.052, 1.464, 0.06964, 0.04684]) {
        assert!(proximos(a.unwrap(), b));
    }
    assert_eq!(p.dimensao("Núcleo químico"), "ATENDE");
    assert_eq!(p.dimensao("Fibra física e micronutrientes"), "NAO_AVALIADO");
    assert!(proximos(p.custo.unwrap(), 11.792177413230045));
    for f in &p.fornecimento {
        assert!(proximos(f.necessidade_bruta, f.oferta_mn * 12.0 * 7.0));
    }
    c.sobras = 0.1;
    let sobras = verificar(&c, &base);
    assert!(proximos(sobras.custo.unwrap(), p.custo.unwrap() / 0.9));
}

#[tokio::test]
async fn ct08_tabela_de_fibra_seleciona_degrau_efetivo() {
    let (_dir, mut c, base) = exemplo(false).await;
    for (valor, limiar) in [
        (19.0, 19.0),
        (18.7, 18.0),
        (18.0, 18.0),
        (17.0, 17.0),
        (16.0, 16.0),
        (15.0, 15.0),
    ] {
        for item in &mut c.itens {
            item.alimento.classe = Classe::Forragem;
            item.alimento.nutrientes[4] = Some(valor / 100.0);
        }
        let p = verificar(&c, &base);
        assert!(
            p.verificacoes
                .iter()
                .any(|v| v.explicacao.contains(&format!("linha efetiva {limiar}")))
        );
    }
    for item in &mut c.itens {
        item.alimento.nutrientes[4] = Some(0.149);
    }
    assert_eq!(
        verificar(&c, &base).dimensao("Barreiras ruminais"),
        "NAO_ATENDE"
    );
}

#[tokio::test]
async fn ct09_excecao_legal_nao_remove_exclusao_do_mvp() {
    let (_dir, mut c, base) = exemplo(false).await;
    c.data_aplicacao = "2026-10-20".into();
    c.itens[0].alimento.origem_animal = Conhecido::Sim;
    c.itens[0].alimento.excecao_animal = Conhecido::Nao;
    let p = verificar(&c, &base);
    assert!(
        p.verificacoes
            .iter()
            .any(|v| v.regra == "LEG-01" && v.estado == Estado::Viola)
    );
    c.itens[0].alimento.excecao_animal = Conhecido::Sim;
    let p = verificar(&c, &base);
    assert!(
        !p.verificacoes
            .iter()
            .any(|v| v.regra == "LEG-01" && v.estado == Estado::Viola)
    );
    assert!(
        p.verificacoes
            .iter()
            .any(|v| v.regra == "MVP-01" && v.estado == Estado::Viola)
    );
    assert_eq!(p.condicao, "BLOQUEADO");
}

#[tokio::test]
async fn ct10_vigencia_e_ativacao_estrangeira() {
    let (_dir, mut c, base) = exemplo(false).await;
    let norma = base.normas.iter().find(|n| n.id == "LEG-ANIMAL").unwrap();
    assert!(!norma.vigente("2004-03-24"));
    assert!(norma.vigente("2004-03-25"));
    assert!(!norma.vigente("2027-03-01"));
    assert!(
        !verificar(&c, &base)
            .verificacoes
            .iter()
            .any(|v| v.regra == "CON-01")
    );
    c.referencia_ue = true;
    assert!(
        verificar(&c, &base)
            .verificacoes
            .iter()
            .any(|v| v.regra == "CON-01" && v.estado == Estado::Desconhecido)
    );
}

#[test]
fn ct11_converte_laudo_sem_inventar_analito_ou_zero() {
    let mut laudo = Laudo {
        analito: "AFB1".into(),
        matriz: "MATERIA_PRIMA".into(),
        valor: 10.0,
        unidade: "µg/kg MN".into(),
        ms: 0.35,
        qualificador: "EXATO".into(),
        loq: None,
        metodo: "HPLC".into(),
        documento: "laudo-teste".into(),
    };
    let ms = contaminantes::base_ms(&laudo).unwrap();
    assert!(proximos(ms, 28.571428571428573));
    assert!(proximos(ms * 0.88, 25.142857142857146));
    laudo.qualificador = "<LOQ".into();
    assert!(contaminantes::base_ms(&laudo).is_none());
    laudo.loq = Some(20.0);
    assert!(proximos(
        contaminantes::base_ms(&laudo).unwrap(),
        20.0 / 0.35
    ));
}

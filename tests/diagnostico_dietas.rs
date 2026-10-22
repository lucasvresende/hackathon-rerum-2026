//! Regressões do diagnóstico desktop: conversões, recomendação e semântica do parecer.
use feed_your_cow::{
    aplicacao::exemplos,
    dominio::{BaseQuantidade, Cenario, Conhecimento, inferencia, otimizacao, unidades::proximos},
    persistencia::{banco, conhecimento, historico::Execucao},
    web::{
        entradas::{campos_cenario, ler_cenario},
        formatacao::quantidade_pratica,
        templates::componentes::resultado::ResultadoExecucao,
    },
};
use maud::Render;

async fn exemplo() -> (Cenario, Conhecimento) {
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("teste.sqlite3"))
        .await
        .unwrap();
    let c = exemplos::cenario_inicial(&pool).await.unwrap();
    let b = conhecimento::ler(&pool).await.unwrap();
    pool.close().await;
    (c, b)
}

fn html(c: Cenario, b: Conhecimento) -> String {
    let parecer = if c.formular {
        otimizacao::formular(&c, &b)
    } else {
        inferencia::verificar(&c, &b)
    };
    ResultadoExecucao {
        execucao: &Execucao {
            id: "teste".into(),
            cenario_id: "teste".into(),
            criada_em: String::new(),
            cenario: c,
            conhecimento: b,
            parecer,
        },
        token: "teste",
    }
    .render()
    .into_string()
}

#[tokio::test]
async fn duplicacoes_preservam_conversao_e_nova_alteracao_usa_ms_da_copia() {
    for base in [
        BaseQuantidade::ConsumoMs,
        BaseQuantidade::ConsumoMn,
        BaseQuantidade::OfertaMn,
    ] {
        let (mut c, b) = exemplo().await;
        let lotes = vec![c.lote.clone()];
        let alimentos: Vec<_> = c.itens.iter().map(|i| i.alimento.clone()).collect();
        for i in &mut c.itens {
            if base != BaseQuantidade::ConsumoMs {
                i.quantidade /= i.alimento.ms;
            }
            i.base = base;
        }
        c.base_comparacao = if base == BaseQuantidade::ConsumoMs {
            "Quantidade de MN fixa"
        } else {
            "Consumo de MS fixo"
        }
        .into();
        let id = c
            .itens
            .iter()
            .find(|i| i.alimento.id == "ex-a01")
            .unwrap()
            .alimento
            .composicao_id
            .clone();
        let mut campos = campos_cenario(&c);
        campos.insert(format!("ms-{id}"), "20".into());
        let mut convertida = ler_cenario(&campos, &lotes, &alimentos).unwrap();
        let esperado = inferencia::verificar(&convertida, &b);
        for _ in 0..3 {
            convertida = ler_cenario(&campos_cenario(&convertida), &lotes, &alimentos).unwrap();
            let p = inferencia::verificar(&convertida, &b);
            assert!(proximos(p.consumo_ms, esperado.consumo_ms));
            assert!(proximos(p.custo.unwrap(), esperado.custo.unwrap()));
            for (a, z) in p.totais.iter().zip(esperado.totais) {
                assert!(proximos(a.unwrap(), z.unwrap()));
            }
        }
        let i = convertida
            .itens
            .iter()
            .find(|i| i.alimento.id == "ex-a01")
            .unwrap();
        let quantidade = i.quantidade;
        let mut campos = campos_cenario(&convertida);
        campos.insert(format!("ms-{id}"), "25".into());
        let nova = ler_cenario(&campos, &lotes, &alimentos).unwrap();
        let i = nova
            .itens
            .iter()
            .find(|i| i.alimento.id == "ex-a01")
            .unwrap();
        let fator = if base == BaseQuantidade::ConsumoMs {
            0.25 / 0.20
        } else {
            0.20 / 0.25
        };
        assert!(proximos(i.quantidade, quantidade * fator));
        campos.insert(format!("referencia_ms-{id}"), "0".into());
        assert!(ler_cenario(&campos, &lotes, &alimentos).is_err());
    }
}

#[tokio::test]
async fn verificacao_distingue_pesagem_pendente_dados_ausentes_violacao_e_invalidade() {
    let (mut c, b) = exemplo().await;
    let p = inferencia::verificar(&c, &b);
    assert_eq!(p.dimensao("Núcleo químico"), "ATENDE");
    assert_eq!(p.estado_fornecimento(&c), "RESOLUCAO_PENDENTE");
    assert!(!p.fornecimento_recomendado(&c));
    for i in &mut c.itens {
        i.resolucao_mn = p
            .fornecimento
            .iter()
            .find(|f| f.composicao_id == i.alimento.composicao_id)
            .unwrap()
            .oferta_mn;
    }
    assert!(inferencia::verificar(&c, &b).fornecimento_recomendado(&c));
    c.itens[0].sem_limite = false;
    let p = inferencia::verificar(&c, &b);
    assert_eq!(p.estado_fornecimento(&c), "PENDENCIAS_FORNECIMENTO");
    let texto = html(c.clone(), b.clone());
    let resumo = texto
        .split("id=\"situacao-fornecimento\"")
        .nth(1)
        .unwrap()
        .split("</ul>")
        .next()
        .unwrap();
    assert!(resumo.contains(&c.itens[0].alimento.nome));
    assert!(resumo.contains("estoque desconhecido"));
    c.itens[0].estoque = Some(0.0);
    assert_eq!(
        inferencia::verificar(&c, &b).estado_fornecimento(&c),
        "FORNECIMENTO_INCOMPATIVEL"
    );
    for i in &mut c.itens {
        i.quantidade = 0.0;
    }
    let texto = html(c, b);
    assert!(texto.contains("data-fornecimento=\"INVALIDO\""));
    assert!(texto.contains("A dieta deve ter MS total positiva"));
}

#[tokio::test]
async fn parecer_identifica_dieta_informada_consumo_estimado_e_piso_global() {
    let (mut c, b) = exemplo().await;
    c.consumo_alvo = Some(25.0);
    c.sobras = 0.1;
    for i in &mut c.itens {
        i.quantidade /= i.alimento.ms;
        i.base = BaseQuantidade::OfertaMn;
    }
    let texto = html(c.clone(), b.clone());
    assert!(texto.contains("Consumo estimado (kg/animal/dia)"));
    assert!(texto.contains("10,53000"));
    assert!(texto.contains("o consumo-alvo não impõe igualdade"));
    assert!(!texto.contains("Consumo-alvo utilizado"));
    assert!(texto.contains("15,3900 kg MS/animal/dia"));
    assert!(texto.contains("FDN de forragem (piso global do perfil)"));
    assert!(texto.contains("governa os limites de FDN total e amido"));
    assert!(!texto.contains("Consumo contínuo"));
    assert!(!texto.contains("Oferta contínua"));
    assert!(texto.contains("da dieta informada"));
    c.sobras = 0.0;
    for i in &mut c.itens {
        i.resolucao_mn = 1.0;
    }
    let texto = html(c, b);
    assert!(texto.contains("da dieta arredondada"));
    assert!(!texto.contains("O ótimo contínuo"));
    // A auditoria dá contexto ao título e prioriza a primeira violação.
    let auditoria = texto.split("id=\"calculos-regras\"").nth(1).unwrap();
    assert!(auditoria.contains("Silagem de milho: mínimo obrigatório"));
}

#[test]
fn pesagem_remove_ruido_sem_arredondar_quantidades_precisas() {
    assert_eq!(quantidade_pratica(0.1 * 3.0), "0,3");
    assert_eq!(quantidade_pratica(42.5), "42,5");
    assert_eq!(quantidade_pratica(0.000000001), "0,000000001");
    let valor = 0.3157894736842105;
    let lido: f64 = quantidade_pratica(valor).replace(',', ".").parse().unwrap();
    assert!((valor - lido).abs() <= f64::EPSILON * 4.0 * valor);
}

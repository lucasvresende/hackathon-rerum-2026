//! Criação idempotente de cadastros e cenários sintéticos de demonstração.
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::Resultado;
use crate::dominio::*;

pub fn lote() -> Lote {
    Lote {
        procedencia: None,
        id: Uuid::new_v4().to_string(),
        revisao_id: String::new(),
        nome: "Lote demonstração".into(),
        animais: 12,
        raca: "Sintética / não determinada".into(),
        idade_meses: Some(48),
        estado_produtivo: EstadoProdutivo::Lactacao,
        destinacao: "Produção de leite".into(),
        peso: 400.0,
        producao: 10.0,
        gordura: 4.0,
        dias_lactacao: 150,
        adulta: Conhecido::Sim,
        gestante: Conhecido::Nao,
        crescimento: Conhecido::Nao,
        peso_estavel: Conhecido::Sim,
        confinada: Conhecido::Sim,
        correcao_ambiente: Conhecido::Desconhecido,
        correcao_atividade: Conhecido::Desconhecido,
        agua: Conhecido::Desconhecido,
        ordem_parto: None,
        dispersao_peso: None,
        condicao_corporal: None,
        observacoes: "Exemplo sintético; não é um rebanho real.".into(),
    }
}

pub fn cenario(lote: Lote, alimentos: Vec<Alimento>, dieta_a: bool) -> Cenario {
    let hoje = Utc::now()
        .with_timezone(&chrono_tz::America::Sao_Paulo)
        .date_naive()
        .to_string();
    Cenario {
        nome: if dieta_a {
            "Dieta A · demonstração"
        } else {
            "Dieta B · demonstração"
        }
        .into(),
        original_id: None,
        lote,
        perfil_id: "E2020_NRC1989_RUMEN_MVP_v1".into(),
        data_avaliacao: hoje.clone(),
        data_aplicacao: hoje,
        horizonte: 7,
        sobras: 0.0,
        origem_sobras: "Hipótese explícita: zero sobras; não há medição.".into(),
        consumo_alvo: None,
        medicao: None,
        origem_consumo: "Referência Tabela B; consumo estimado.".into(),
        referencia_ue: false,
        preco_leite: None,
        base_comparacao: "Consumo de MS fixo".into(),
        formular: false,
        limite_segundos: 10.0,
        itens: alimentos
            .into_iter()
            .map(|a| {
                let (quantidade, preco) = match a.id.as_str() {
                    "silagem" => (if dieta_a { 7.2 } else { 8.2 }, "0.25"),
                    "milho" => (if dieta_a { 1.8 } else { 0.8 }, "1.30"),
                    "soja" => (1.6, "2.20"),
                    _ => (0.2, "4.00"),
                };
                Item {
                    alimento: a,
                    quantidade,
                    base: BaseQuantidade::ConsumoMs,
                    preco: preco.into(),
                    cotacao_original: None,
                    origem_preco: "Cotação sintética do exemplo 9.1".into(),
                    perda: 0.0,
                    estoque: None,
                    sem_limite: true,
                    minimo_ms: 0.0,
                    maximo_ms: None,
                    resolucao_mn: 0.0,
                }
            })
            .collect(),
    }
}

pub async fn preparar(pool: &SqlitePool) -> Resultado<()> {
    crate::persistencia::conhecimento::carregar(
        pool,
        include_str!("../../dados_referencia/002_demonstracao.sql"),
    )
    .await?;
    if crate::persistencia::cadastros::lotes(pool)
        .await?
        .iter()
        .all(|l| l.nome != "Lote demonstração")
    {
        let mut lote = lote();
        crate::persistencia::cadastros::salvar_lote(pool, &mut lote, true).await?;
    }
    Ok(())
}

pub async fn cenarios_salvos(pool: &SqlitePool) -> Resultado<()> {
    use crate::persistencia::{cadastros, conhecimento, historico};
    if !historico::listar(pool).await?.is_empty() {
        return Ok(());
    }
    let lote = cadastros::lotes(pool)
        .await?
        .into_iter()
        .find(|l| l.nome == "Lote demonstração")
        .ok_or("Lote demonstração ausente")?;
    let alimentos: Vec<_> = cadastros::alimentos(pool)
        .await?
        .into_iter()
        .filter(|a| ["silagem", "milho", "soja", "mineral"].contains(&a.id.as_str()))
        .collect();
    let base = conhecimento::ler(pool).await?;
    let mut cenarios = vec![
        cenario(lote.clone(), alimentos.clone(), true),
        cenario(lote.clone(), alimentos.clone(), false),
    ];
    let mut extrapolado = self::lote();
    extrapolado.nome = "Lote extrapolado · sintético".into();
    extrapolado.peso = 650.0;
    cadastros::salvar_lote(pool, &mut extrapolado, true).await?;
    let mut cenario_extrapolado = cenario(extrapolado, alimentos.clone(), false);
    cenario_extrapolado.nome = "Aviso · consumo extrapolado acima de 600 kg".into();
    cenarios.push(cenario_extrapolado);
    let mut bloqueio = cenario(lote, alimentos, false);
    bloqueio.nome = "Bloqueio · NNP adicionado sintético".into();
    let item = &mut bloqueio.itens[0];
    item.alimento.id = "demo-nnp".into();
    item.alimento.nome = "Ingrediente com NNP · sintético".into();
    item.alimento.nnp_adicionado = Conhecido::Sim;
    item.alimento.evidencia = "Hipótese sintética para demonstrar exclusão pelo MVP.".into();
    cadastros::salvar_alimento(pool, &mut item.alimento).await?;
    cenarios.push(bloqueio);
    for c in cenarios {
        let p = crate::dominio::inferencia::verificar(&c, &base);
        historico::salvar(pool, &c, &p, &base).await?;
    }
    Ok(())
}

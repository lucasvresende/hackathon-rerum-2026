//! Preparação e execução de cenários, sem decodificação de formulários.
use super::{demonstracao, execucao};
use crate::persistencia::{cadastros, conhecimento, historico};
use crate::{
    Resultado,
    dominio::{Alimento, Cenario, Lote},
};
use sqlx::SqlitePool;

pub struct OpcoesCenario {
    pub lotes: Vec<Lote>,
    pub alimentos: Vec<Alimento>,
}

pub async fn opcoes(pool: &SqlitePool) -> Resultado<OpcoesCenario> {
    Ok(OpcoesCenario {
        lotes: cadastros::lotes(pool).await?,
        alimentos: cadastros::alimentos(pool).await?,
    })
}

pub enum DietaDemonstracao {
    A,
    B,
}

pub fn exemplo(opcoes: &OpcoesCenario, dieta: DietaDemonstracao) -> Option<Cenario> {
    let lote = opcoes
        .lotes
        .iter()
        .find(|l| l.nome == "Lote demonstração")?;
    let mut selecionados = std::collections::BTreeSet::new();
    let alimentos = opcoes
        .alimentos
        .iter()
        .filter(|a| {
            ["silagem", "milho", "soja", "mineral"].contains(&a.id.as_str())
                && a.natureza == "SINTETICA"
                && selecionados.insert(a.id.clone())
        })
        .cloned()
        .collect();
    Some(demonstracao::cenario(
        lote.clone(),
        alimentos,
        matches!(dieta, DietaDemonstracao::A),
    ))
}

/// Distingue indisponibilidade da base de falha recuperável na execução do formulário.
pub enum FalhaCenario {
    Conhecimento(crate::Erro),
    Execucao(crate::Erro),
}

pub async fn executar_novo(
    pool: &SqlitePool,
    mut cenario: Cenario,
) -> Result<String, FalhaCenario> {
    let base = conhecimento::ler(pool)
        .await
        .map_err(FalhaCenario::Conhecimento)?;
    cenario.perfil_id = base.perfil_id.clone();
    execucao::executar(pool, &cenario, &base)
        .await
        .map_err(FalhaCenario::Execucao)
}

pub async fn duplicar(pool: &SqlitePool, id: &str) -> Resultado<Cenario> {
    let anterior = historico::obter(pool, id).await?;
    let mut cenario = anterior.cenario;
    cenario.original_id = Some(anterior.cenario_id);
    cenario.nome = format!("{} · simulação", cenario.nome);
    Ok(cenario)
}

/// Reproduz com conhecimento e entradas congelados, preservando o histórico original.
pub async fn reproduzir(pool: &SqlitePool, id: &str) -> Resultado<String> {
    let anterior = historico::obter(pool, id).await?;
    let mut cenario = anterior.cenario;
    cenario.original_id = Some(anterior.cenario_id);
    execucao::executar(pool, &cenario, &anterior.conhecimento).await
}

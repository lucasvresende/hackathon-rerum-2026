use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

use crate::Resultado;
use crate::dominio::{Coeficiente, Conhecimento, Fonte, Norma};

pub fn hash(conteudo: &str) -> String {
    format!("{:x}", Sha256::digest(conteudo.as_bytes()))
}

pub async fn carregar(pool: &SqlitePool, sql: &str) -> Resultado<()> {
    let versao = sql
        .lines()
        .next()
        .and_then(|l| l.strip_prefix("-- versao: "))
        .filter(|v| !v.is_empty())
        .ok_or("Carga sem identificador de versão.")?;
    let resumo = hash(sql);
    let mut tx = pool.begin().await?;
    if let Some(anterior) = sqlx::query_file!("queries/carga.sql", versao)
        .fetch_optional(&mut *tx)
        .await?
    {
        if anterior.hash == resumo {
            return Ok(());
        }
        return Err("Versão colidente: mesmo identificador com conteúdo diferente.".into());
    }
    // Arquivo SQL administrativo local: nunca recebe conteúdo de formulários ou da rede.
    sqlx::raw_sql(sqlx::AssertSqlSafe(sql.to_owned()))
        .execute(&mut *tx)
        .await?;
    let contagem = sqlx::query_file!("queries/verificar_base.sql")
        .fetch_one(&mut *tx)
        .await?;
    if contagem.perfis != 1 || contagem.fontes == 0 {
        return Err("Carga sem perfil único ou fontes.".into());
    }
    let base = ler_conexao(&mut tx).await?;
    validar(&base)?;
    sqlx::query_file!("queries/inserir_carga.sql", versao, resumo, sql)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn ler(pool: &SqlitePool) -> Resultado<Conhecimento> {
    let mut conexao = pool.acquire().await?;
    ler_conexao(&mut conexao).await
}

async fn ler_conexao(conexao: &mut sqlx::SqliteConnection) -> Resultado<Conhecimento> {
    let perfil = sqlx::query_file!("queries/perfil.sql")
        .fetch_one(&mut *conexao)
        .await?;
    let coeficientes = sqlx::query_file!("queries/coeficientes.sql", perfil.id)
        .fetch_all(&mut *conexao)
        .await?
        .into_iter()
        .map(|r| Coeficiente {
            tabela: r.tabela,
            eixo: r.eixo,
            eixo2: r.eixo2,
            valores: [r.a, r.b, r.c, r.d],
            unidade: r.unidade,
        })
        .collect();
    let fontes = sqlx::query_file!("queries/fontes.sql")
        .fetch_all(&mut *conexao)
        .await?
        .into_iter()
        .map(|r| Fonte {
            id: r.id,
            titulo: r.titulo,
            edicao: r.edicao,
            localizador: r.localizador,
            natureza: r.natureza,
            conteudo: r.conteudo,
            hash: r.hash,
        })
        .collect();
    let normas = sqlx::query_file!("queries/normas.sql")
        .fetch_all(&mut *conexao)
        .await?
        .into_iter()
        .map(|r| Norma {
            id: r.id,
            fonte_id: r.fonte_id,
            natureza: r.natureza,
            jurisdicao: r.jurisdicao,
            publicacao: r.publicacao,
            inicio: r.inicio,
            fim: r.fim,
            transicao: r.transicao,
            condicoes: r.condicoes,
            excecoes: r.excecoes,
            evidencia: r.evidencia,
            acao: r.acao,
        })
        .collect();
    let conhecimento = Conhecimento {
        perfil_id: perfil.id,
        versao: perfil.versao,
        fdn_min: perfil.fdn_min,
        amido_max: perfil.amido_max,
        ee_max: perfil.ee_max,
        discrepancia: perfil.discrepancia,
        coeficientes,
        fontes,
        normas,
        versao_app: env!("CARGO_PKG_VERSION").into(),
        metodo: "microlp-0.6.0 / cinco subproblemas contínuos / RN-24".into(),
    };
    validar(&conhecimento)?;
    Ok(conhecimento)
}

pub fn validar(c: &Conhecimento) -> Resultado<()> {
    for (tabela, n, unidade) in [
        ("mantenca", 7, "kg/dia"),
        ("leite", 5, "kg/kg leite"),
        ("consumo", 18, "% PV"),
        ("fibra", 5, "% MS"),
    ] {
        let linhas: Vec<_> = c
            .coeficientes
            .iter()
            .filter(|v| v.tabela == tabela)
            .collect();
        if linhas.len() != n || linhas.iter().any(|v| v.unidade != unidade) {
            return Err(format!("Tabela {tabela} incompleta ou unidade incompatível.").into());
        }
    }
    if c.fontes.iter().any(|f| hash(&f.conteudo) != f.hash) {
        return Err("Fonte com conteúdo/hash incompatível.".into());
    }
    Ok(())
}

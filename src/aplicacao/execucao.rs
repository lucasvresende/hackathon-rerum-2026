//! Execução do motor e persistência do cenário e do parecer resultante.
use std::process::Stdio;
use std::time::{Duration, Instant};

use sqlx::SqlitePool;
use tokio::io::AsyncWriteExt;

use crate::Resultado;
use crate::dominio::motor::PedidoMotor;
use crate::dominio::{Cenario, Conhecimento, Parecer, motor};

/// O processo filho é encerrado e aguardado ao expirar o orçamento; não sobra cálculo em thread.
pub async fn calcular(cenario: Cenario, conhecimento: Conhecimento) -> Resultado<Parecer> {
    calcular_no_executavel(cenario, conhecimento, &std::env::current_exe()?).await
}

pub async fn calcular_no_executavel(
    cenario: Cenario,
    conhecimento: Conhecimento,
    executavel: &std::path::Path,
) -> Resultado<Parecer> {
    if !cenario.formular {
        return Ok(motor::avaliar(&cenario, &conhecimento));
    }
    let limite = cenario.limite_segundos;
    if !crate::dominio::validacao::validar(&cenario).is_empty() {
        return Ok(motor::avaliar(&cenario, &conhecimento));
    }
    let prazo = tokio::time::Instant::now() + Duration::from_secs_f64(limite);
    let mut filho = tokio::process::Command::new(executavel)
        .arg("__motor")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()?;
    let bytes = serde_json::to_vec(&PedidoMotor {
        cenario,
        conhecimento,
    })?;
    if let Some(mut entrada) = filho.stdin.take() {
        let envio = tokio::time::timeout_at(prazo, async {
            entrada.write_all(&bytes).await?;
            entrada.shutdown().await
        })
        .await;
        match envio {
            Ok(resultado) => resultado?,
            Err(_) => {
                filho.kill().await?;
                filho.wait().await?;
                return Ok(interrompido());
            }
        }
    }
    let stdout = filho.stdout.take().ok_or("Saída do motor indisponível")?;
    let leitura = tokio::spawn(async move {
        use tokio::io::AsyncReadExt;
        let mut stdout = stdout;
        let mut bytes = Vec::new();
        stdout.read_to_end(&mut bytes).await.map(|_| bytes)
    });
    match tokio::time::timeout_at(prazo, filho.wait()).await {
        Ok(Ok(status)) if status.success() => {
            let bytes = leitura.await??;
            Ok(serde_json::from_slice(&bytes)?)
        }
        Ok(_) => {
            leitura.abort();
            Err("Falha no processo de formulação.".into())
        }
        Err(_) => {
            filho.kill().await?;
            filho.wait().await?;
            leitura.abort();
            Ok(interrompido())
        }
    }
}

/// Executa o motor no processo isolado quando configurado e persiste o parecer e as entradas.
pub async fn executar(
    pool: &SqlitePool,
    cenario: &Cenario,
    base: &Conhecimento,
) -> Resultado<String> {
    let inicio = Instant::now();
    let mut parecer = calcular(cenario.clone(), base.clone()).await?;
    parecer.duracao_ms = inicio.elapsed().as_millis() as u64;
    let id = crate::persistencia::historico::salvar(pool, cenario, &parecer, base).await?;
    tracing::info!(execucao_id = %id, duracao_ms = inicio.elapsed().as_millis(),
        metodo = %base.metodo, "Execução persistida com entradas e traço");
    Ok(id)
}

fn interrompido() -> Parecer {
    Parecer {
        condicao: "INCONCLUSIVO".into(),
        busca: Some("INCONCLUSIVA".into()),
        avisos: vec!["Formulação encerrada efetivamente pelo limite de tempo.".into()],
        ..Parecer::default()
    }
}

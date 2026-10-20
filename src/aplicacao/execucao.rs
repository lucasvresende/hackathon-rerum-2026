use std::process::Stdio;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tokio::io::AsyncWriteExt;

use crate::Resultado;
use crate::dominio::{Cenario, Conhecimento, Parecer};

#[derive(Serialize, Deserialize)]
pub struct PedidoMotor {
    pub cenario: Cenario,
    pub conhecimento: Conhecimento,
}

/// O processo filho é encerrado e aguardado ao expirar o orçamento; não sobra cálculo em thread.
pub async fn calcular(cenario: Cenario, conhecimento: Conhecimento) -> Resultado<Parecer> {
    if !cenario.formular {
        return Ok(crate::dominio::inferencia::verificar(
            &cenario,
            &conhecimento,
        ));
    }
    let limite = cenario.limite_segundos;
    if !crate::dominio::validacao::validar(&cenario).is_empty() {
        return Ok(crate::dominio::otimizacao::formular(
            &cenario,
            &conhecimento,
        ));
    }
    let mut filho = tokio::process::Command::new(std::env::current_exe()?)
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
        entrada.write_all(&bytes).await?;
        entrada.shutdown().await?;
    }
    let stdout = filho.stdout.take().ok_or("Saída do motor indisponível")?;
    let leitura = tokio::spawn(async move {
        use tokio::io::AsyncReadExt;
        let mut stdout = stdout;
        let mut bytes = Vec::new();
        stdout.read_to_end(&mut bytes).await.map(|_| bytes)
    });
    match tokio::time::timeout(Duration::from_secs_f64(limite), filho.wait()).await {
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
            Ok(Parecer {
                condicao: "INCONCLUSIVO".into(),
                busca: Some("INCONCLUSIVA".into()),
                avisos: vec!["Formulação encerrada efetivamente pelo limite de tempo.".into()],
                ..Parecer::default()
            })
        }
    }
}

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

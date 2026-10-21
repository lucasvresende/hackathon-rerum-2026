#![forbid(unsafe_code)]
//! Ponto de entrada: inicializa tracing, prepara o banco e despacha comandos locais ou o servidor.

use std::path::PathBuf;

use feed_your_cow::Resultado;
use feed_your_cow::aplicacao::{demonstracao, execucao::PedidoMotor};
use feed_your_cow::persistencia::{banco, conhecimento, recuperacao};

#[tokio::main]
async fn main() -> Resultado<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let comando = args.first().map(String::as_str).unwrap_or("servir");
    if comando == "__motor" {
        let pedido: PedidoMotor = serde_json::from_reader(std::io::stdin())?;
        let parecer =
            feed_your_cow::dominio::otimizacao::formular(&pedido.cenario, &pedido.conhecimento);
        serde_json::to_writer(std::io::stdout(), &parecer)?;
        return Ok(());
    }
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or("info".into()),
        )
        .with_writer(std::io::stderr)
        .init();
    let caminho = PathBuf::from(
        std::env::var("FEEDYOURCOW_BANCO").unwrap_or("dados/aplicacao.sqlite3".into()),
    );
    match comando {
        "ensaiar" => feed_your_cow::aplicacao::ensaio::executar().await?,
        "preparar" | "demonstrar" => {
            let pool = banco::preparar(&caminho).await?;
            if comando == "demonstrar" {
                demonstracao::preparar(&pool).await?;
                demonstracao::cenarios_salvos(&pool).await?;
            }
            conhecimento::ler(&pool).await?;
            println!("FeedYourCow preparado em {}", caminho.display());
            pool.close().await;
        }
        "carregar" => {
            let arquivo = args.get(1).ok_or("Uso: carregar <arquivo.sql>")?;
            let pool = banco::abrir(&caminho, false).await?;
            conhecimento::carregar(&pool, &std::fs::read_to_string(arquivo)?).await?;
            println!("Carga validada e aplicada.");
        }
        "backup" => {
            let destino = args.get(1).ok_or("Uso: backup <diretorio-novo>")?;
            let pool = banco::abrir(&caminho, false).await?;
            recuperacao::backup(&pool, &PathBuf::from(destino)).await?;
            println!("Backup consistente criado em {destino}");
        }
        "restaurar" => {
            let origem = args
                .get(1)
                .ok_or("Uso: restaurar <backup> <diretorio-novo>")?;
            let destino = args
                .get(2)
                .ok_or("Uso: restaurar <backup> <diretorio-novo>")?;
            recuperacao::restaurar(&PathBuf::from(origem), &PathBuf::from(destino)).await?;
            println!("Restauração verificada em {destino}");
        }
        "servir" => {
            let endereco: std::net::SocketAddr = std::env::var("FEEDYOURCOW_ENDERECO")
                .unwrap_or("127.0.0.1:3000".into())
                .parse()?;
            if !endereco.ip().is_loopback() {
                return Err("O MVP aceita somente endereço de loopback.".into());
            }
            let pool = banco::abrir(&caminho, false).await?;
            banco::verificar_estrutura(&pool).await?;
            conhecimento::ler(&pool).await?;
            let estado =
                feed_your_cow::web::estado::EstadoApp::novo(pool, format!("http://{endereco}"));
            let listener = tokio::net::TcpListener::bind(endereco).await?;
            tracing::info!(%endereco, "FeedYourCow disponível");
            axum::serve(listener, feed_your_cow::web::rotas::app(estado))
                .with_graceful_shutdown(async {
                    let _ = tokio::signal::ctrl_c().await;
                })
                .await?;
        }
        _ => {
            return Err(
                "Comandos: preparar, demonstrar, servir, carregar, backup, restaurar, ensaiar"
                    .into(),
            );
        }
    }
    Ok(())
}

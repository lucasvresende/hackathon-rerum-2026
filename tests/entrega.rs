use feed_your_cow::persistencia::{banco, historico};
use std::process::Command;

#[tokio::test]
async fn ct18_ct28_demonstracao_cli_calcula_quatro_cenarios_e_e_idempotente() {
    let d = tempfile::tempdir().unwrap();
    let caminho = d.path().join("cli.sqlite3");
    for _ in 0..2 {
        let saida = Command::new(env!("CARGO_BIN_EXE_feed-your-cow"))
            .arg("demonstrar")
            .env("FEEDYOURCOW_BANCO", &caminho)
            .output()
            .unwrap();
        assert!(
            saida.status.success(),
            "{}",
            String::from_utf8_lossy(&saida.stderr)
        );
    }
    let pool = banco::abrir(&caminho, false).await.unwrap();
    let execucoes = historico::listar(&pool).await.unwrap();
    assert_eq!(execucoes.len(), 4);
    assert!(execucoes.iter().any(|e| {
        e.parecer
            .avisos
            .iter()
            .any(|aviso| aviso.contains("Tabela B"))
    }));
    assert!(execucoes.iter().any(|e| e.parecer.condicao == "BLOQUEADO"));
    assert!(
        execucoes
            .iter()
            .any(|e| e.parecer.dimensao("Barreiras ruminais") == "NAO_ATENDE")
    );
    assert!(execucoes.iter().any(|e| e.parecer.pratico_valido));
    let filtradas = historico::filtrar(&pool, Some("%extrapolado%"), None, None, 0)
        .await
        .unwrap();
    assert_eq!(filtradas.len(), 1);
    assert!(
        historico::filtrar(&pool, None, None, None, 100)
            .await
            .unwrap()
            .is_empty()
    );
}

#[test]
fn ct27_base_ausente_e_rede_compartilhada_tem_orientacao() {
    let d = tempfile::tempdir().unwrap();
    let saida = Command::new(env!("CARGO_BIN_EXE_feed-your-cow"))
        .arg("servir")
        .env("FEEDYOURCOW_BANCO", d.path().join("ausente.sqlite3"))
        .env("FEEDYOURCOW_ENDERECO", "127.0.0.1:3000")
        .output()
        .unwrap();
    assert!(!saida.status.success());
    assert!(String::from_utf8_lossy(&saida.stderr).contains("preparar"));
    let saida = Command::new(env!("CARGO_BIN_EXE_feed-your-cow"))
        .arg("servir")
        .env("FEEDYOURCOW_ENDERECO", "0.0.0.0:3000")
        .output()
        .unwrap();
    assert!(!saida.status.success());
    assert!(String::from_utf8_lossy(&saida.stderr).contains("loopback"));
}

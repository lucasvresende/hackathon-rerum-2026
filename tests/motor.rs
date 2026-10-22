use feed_your_cow::aplicacao::exemplos;
use feed_your_cow::dominio::{inferencia, motor, otimizacao};
use feed_your_cow::persistencia::{banco, conhecimento};

#[tokio::test]
async fn entrada_do_motor_preserva_os_dois_modos() {
    let dir = tempfile::tempdir().unwrap();
    let pool = banco::preparar(&dir.path().join("motor.sqlite3"))
        .await
        .unwrap();
    let mut cenario = exemplos::cenario_inicial(&pool).await.unwrap();
    let base = conhecimento::ler(&pool).await.unwrap();

    let verificado = motor::avaliar(&cenario, &base);
    let esperado = inferencia::verificar(&cenario, &base);
    assert_eq!(verificado.condicao, esperado.condicao);
    assert_eq!(
        serde_json::to_value(&verificado.verificacoes).unwrap(),
        serde_json::to_value(&esperado.verificacoes).unwrap()
    );

    cenario.formular = true;
    let formulado = motor::avaliar(&cenario, &base);
    let esperado = otimizacao::formular(&cenario, &base);
    assert_eq!(formulado.busca, esperado.busca);
    assert_eq!(formulado.subproblemas.len(), esperado.subproblemas.len());
    assert_eq!(formulado.custo, esperado.custo);

    let pedido = motor::PedidoMotor {
        cenario,
        conhecimento: base,
    };
    let copiado: motor::PedidoMotor =
        serde_json::from_slice(&serde_json::to_vec(&pedido).unwrap()).unwrap();
    assert!(copiado.cenario.formular);
    assert_eq!(
        copiado.conhecimento.perfil_id,
        pedido.conhecimento.perfil_id
    );
    pool.close().await;
}

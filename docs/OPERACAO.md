# Operação e distribuição

Este guia distingue a instalação de desenvolvimento do pacote nativo de release. O
[README principal](../README.md) contém o caminho mínimo; [Uso e telas](USO.md) explica o fluxo
no navegador.

## Pré-requisitos

- Desenvolvimento e build: Rust estável, Cargo e ferramentas de compilação/linker C.
- Empacotamento: `sh`, `tar`, `gzip` e `sha256sum`.
- Testes de navegador: Node 24, npm e os navegadores instalados pelo Playwright.
- Testes auxiliares e teste do pacote: Python 3.12 ou posterior.

O programa distribuído não precisa de Node, Python, Cargo, CDN ou serviço externo. Ele exige
bibliotecas de sistema compatíveis com as da máquina que compilou o binário.

## Desenvolvimento

```sh
cargo build --locked
cargo run -- preparar
cargo run -- servir
```

O comando `preparar` cria o banco, aplica migrações SQLx e carrega o perfil e o catálogo inicial.
É seguro repeti-lo para cargas pendentes: alterações do operador e exclusões de exemplos são
preservadas. `servir` exige banco previamente preparado. O servidor escuta em
`127.0.0.1:3000` e recusa endereços fora do loopback.

Para inspecionar as demonstrações A/B e quatro execuções de referência, em um banco de teste:

```sh
cargo run -- demonstrar
cargo run -- servir
```

`demonstrar` não é necessário para o primeiro uso. O cenário sintético inicial já fica disponível
após `preparar`. Nunca use dados de demonstração como medição, cotação ou laudo reais.

### Recarga e logs

```sh
cargo run --features desenvolvimento -- servir
```

A feature ativa `tower-livereload`. Se `bacon` estiver instalado, `bacon servir` acompanha o
servidor e `bacon testar` acompanha os testes. As tarefas do Bacon estão em `bacon.toml`.
`RUST_LOG=debug` aumenta o detalhamento dos logs de tracing.

## Configuração

| Variável               | Padrão                    | Função                          |
| ---------------------- | ------------------------- | ------------------------------- |
| `FEEDYOURCOW_BANCO`    | `dados/aplicacao.sqlite3` | Arquivo SQLite local.           |
| `FEEDYOURCOW_ENDERECO` | `127.0.0.1:3000`          | Endereço e porta do HTTP local. |
| `RUST_LOG`             | `info`                    | Filtro de tracing.              |

O aplicativo não lê `.env` automaticamente. É possível exportar apenas as variáveis desejadas.
No Bash, para carregar o exemplo de configuração no terminal atual:

```sh
set -a
. ./.env.example
set +a
```

O banco padrão é relativo ao diretório de trabalho. `iniciar.sh` muda para a pasta da instalação,
por isso o pacote mantém o banco padrão ali. Ao usar um banco externo, informe caminho absoluto
antes de preparar **e** antes de servir:

```sh
export FEEDYOURCOW_BANCO="$HOME/feedyourcow-dados/aplicacao.sqlite3"
```

## Comandos administrativos

No checkout, use `cargo run -- <comando>`. No pacote, use `./iniciar.sh <comando>`.

| Comando                               | Efeito                                                            |
| ------------------------------------- | ----------------------------------------------------------------- |
| `preparar`                            | Migra banco e carrega conhecimento e exemplos pendentes.          |
| `demonstrar`                          | Prepara e adiciona fixtures de demonstração e execuções de teste. |
| `servir`                              | Inicia a interface local.                                         |
| `carregar <arquivo.sql>`              | Valida e aplica uma carga local versionada.                       |
| `backup <diretorio-novo>`             | Cria cópia consistente e manifesto SHA-256.                       |
| `restaurar <backup> <diretorio-novo>` | Verifica e restaura em destino novo.                              |
| `ensaiar`                             | Mede a execução do motor com casos controlados.                   |

A carga SQL não deve sobrescrever silenciosamente uma versão distribuída. Para evoluir o
conhecimento, crie uma carga com nova versão e execute testes antes de ativá-la.

### Backup e restauração

```sh
cargo run -- backup /tmp/feedyourcow-backup
cargo run -- restaurar /tmp/feedyourcow-backup /tmp/feedyourcow-restaurado
```

Os dois destinos devem ser novos. O backup usa `VACUUM INTO`, preserva arquivos documentais e
cargas de referência, e grava hashes. A restauração confere o manifesto, a integridade SQLite e
as referências entre tabelas. Para abrir o banco restaurado, aponte `FEEDYOURCOW_BANCO` para
`/tmp/feedyourcow-restaurado/aplicacao.sqlite3`. A restauração não troca automaticamente o banco
ativo.

## Construir e testar o release

```sh
sh scripts/build_producao.sh
python3 scripts/testar_producao.py dist/feedyourcow-<alvo>.tar.gz
```

Substitua `<alvo>` pelo nome mostrado pelo script. O build usa `--locked`, `--release`,
`--no-default-features` e SQLx offline. O pacote inclui o executável, `iniciar.sh`, recursos
estáticos, migrações, cargas de referência e a documentação necessária para uso e auditoria.
Não inclui bancos operacionais, backups nem ferramentas de desenvolvimento.

O teste de produção confere hash e conteúdo do arquivo, prepara um banco temporário, exercita o
motor, backup/restauração e rotas HTTP do binário otimizado. Também verifica que os documentos
esperados acompanham o pacote. Um teste de navegador contra esse binário pode ser executado com:

```sh
FEEDYOURCOW_BINARIO="$PWD/target/<alvo>/release/feed-your-cow" npm test
```

### Instalar e atualizar

Confira o `.sha256` antes de extrair. Execute `./iniciar.sh preparar` uma vez e depois
`./iniciar.sh servir`. O script resolve recursos e banco padrão em relação à instalação mesmo
quando chamado de outro diretório.

Para atualizar, encerre o servidor, faça backup com a versão antiga, extraia a nova versão em
outra pasta, mantenha `FEEDYOURCOW_BANCO` apontando para o mesmo arquivo, execute `preparar` e
então `servir`. Guarde o pacote e o backup anteriores. Para voltar ao binário antigo, restaure o
backup anterior em um destino novo; não abra um banco já migrado com uma versão antiga.

## Problemas frequentes

- **Banco não preparado:** execute `preparar` usando o mesmo caminho de banco que `servir`.
- **Porta ocupada:** configure outra porta de loopback em `FEEDYOURCOW_ENDERECO`.
- **Falha de SQLx no build:** confira `.sqlx/`; `scripts/verificar_sqlx.py` compara o cache com
  as migrações em um banco limpo.
- **Binário não executa no destino:** gere o pacote para o sistema/arquitetura corretos e confira
  a compatibilidade das bibliotecas de sistema.

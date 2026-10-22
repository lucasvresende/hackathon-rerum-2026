# FeedYourCow

Aplicação web local para verificar dietas informadas e formular dietas de menor custo para vacas
adultas em lactação. O resultado mostra o que foi calculado, quais restrições foram atendidas,
quais informações faltam e por que uma recomendação prática pode ser recusada.

Os cadastros iniciais são **sintéticos**. O perfil nutricional implementado cobre NDT, PB, Ca, P,
barreiras de fibra, amido e extrato etéreo. Um resultado favorável nesse perfil não equivale a
adequação nutricional integral nem a avaliação clínica ou sanitária completa.

## Começar em desenvolvimento

Requer Rust estável com Cargo e compilador/linker C. Na raiz do repositório:

```sh
cargo build --locked
cargo run -- preparar
cargo run -- servir
```

Abra <http://127.0.0.1:3000>. `preparar` aplica as migrações e instala o perfil, seis lotes e
quinze alimentos de exemplo. Na **Visão geral**, escolha **Abrir cenário de exemplo**, revise as
premissas e clique em **Calcular e salvar cenário**. Para incluir também as demonstrações A/B e
as execuções históricas de teste, use `cargo run -- demonstrar`.

O servidor usa somente loopback e não exige internet após a instalação. Node e Python são
ferramentas de teste e manutenção, não requisitos para operar o aplicativo.

## Gerar e usar um release

Na máquina de build, com Rust, `sh`, `tar`, `gzip` e `sha256sum`:

```sh
sh scripts/build_producao.sh
```

O arquivo gerado é `dist/feedyourcow-<alvo>.tar.gz`, acompanhado de um `.sha256`. A distribuição é
nativa para o sistema e a arquitetura da máquina de build. Exemplo para Linux x86-64:

```sh
cd dist
sha256sum -c feedyourcow-x86_64-unknown-linux-gnu.tar.gz.sha256
mkdir instalacao
tar -xzf feedyourcow-x86_64-unknown-linux-gnu.tar.gz -C instalacao
./instalacao/feedyourcow/iniciar.sh preparar
./instalacao/feedyourcow/iniciar.sh servir
```

Abra <http://127.0.0.1:3000>. O destino precisa de bibliotecas de sistema compatíveis com o
binário compilado; Rust/Cargo não são necessários nessa máquina. Por padrão, o SQLite é criado em
`feedyourcow/dados/aplicacao.sqlite3` dentro da instalação. Defina `FEEDYOURCOW_BANCO` com um
caminho absoluto antes de `preparar` e `servir` para manter os dados fora dela.

O [guia de operação](docs/OPERACAO.md) detalha configuração, atualização, backup e restauração.

## Documentação

| Pergunta | Leia |
| --- | --- |
| Como preencher e interpretar uma dieta? | [Uso e telas](docs/USO.md) |
| Como instalar, desenvolver e distribuir? | [Operação](docs/OPERACAO.md) |
| Qual é o contrato funcional? | [Requisitos](docs/REQUISITOS.md) |
| Como o motor calcula e decide? | [Motor de inferência](docs/MOTOR.md) |
| Como o código e o banco estão organizados? | [Arquitetura](docs/ARQUITETURA.md) |
| Como verificar a entrega e o CI? | [Qualidade e rastreabilidade](docs/QUALIDADE.md) |
| Quais fontes e hipóteses sustentam o perfil? | [Fontes e limites](docs/FONTES.md) |
| Como a aplicação foi concebida e revisada? | [Processo de desenvolvimento](docs/PROCESSO.md) |

O [índice de `docs/`](docs/README.md) oferece um percurso de leitura. Os comandos e os estados
citados aqui correspondem ao código desta revisão; os testes e o CI verificam o comportamento.

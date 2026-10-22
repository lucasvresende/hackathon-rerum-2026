# Qualidade, testes, CI e cumprimento dos requisitos

A especificação está em [REQUISITOS.md](REQUISITOS.md). Esta página mostra onde procurar as
provas de implementação e como repetir as verificações. Uma linha da matriz indica cobertura
representativa; **não** substitui a execução bem-sucedida dos testes na revisão entregue.

## Camadas de teste

| Camada | O que verifica | Comando principal |
| --- | --- | --- |
| Rust | Domínio, SQLx, HTTP, histórico, cópias e contratos. | `cargo test --locked` |
| Python | Estrutura e scripts auxiliares. | `unittest` em `scripts/` |
| Navegador | Interface real, htmx, erros, impressão e layouts. | `npm run test:e2e` |
| Release | Arquivo, preparo, motor e HTTP. | `scripts/testar_producao.py` |
| Desempenho | Tempo do motor. | `cargo run -- ensaiar` |

Os testes Rust em `tests/*.rs` usam banco temporário quando necessário. Os testes de navegador
criam banco e servidor temporários; `tests/javascript/e2e/navegadores.mjs` copia o executável
antes de iniciar o servidor para estabilizar o processo filho do motor. Capturas ficam em
`output/playwright/`, fora do Git. Imagens representativas e revisadas estão em [USO.md](USO.md).

## Matriz de rastreabilidade

| Requisitos | Implementação | Evidência representativa |
| --- | --- | --- |
| RF-01–05 | `persistencia/`, `web/cadastros/` | `inicio.rs`, `exemplos.rs` |
| RF-06–09 | `web/cenarios/`, `dominio/motor/` | `cenarios.rs`, `nucleo.rs` |
| RF-10–11 | `dominio/motor/formulacao/` | `otimizacao.rs`, `motor.rs` |
| RF-12–15 | `aplicacao/execucoes/` | `comparacao.rs`, `reutilizacao.rs` |
| RF-16–19 | `persistencia/`, `dados_referencia/` | `exemplos.rs`, `entrega.rs` |
| RN-01–10 | `dominio/cenario/`, `dominio/motor/` | `nucleo.rs`, `cenarios.rs`, `motor.rs` |
| RN-11–16 | `dominio/motor/` | `integracao.rs`, `diagnostico_dietas.rs` |
| RN-17–24 | `dominio/motor/`, `dominio/parecer/` | `otimizacao.rs`, `reutilizacao.rs` |
| RNF-01–14 | `web/`, `scripts/`, CI | Rust, Python, Playwright e pacote |

Os nomes de testes Rust da última coluna pertencem a `tests/`. A suíte de navegador cobre,
entre outras áreas, `inicio`, `exemplos`, `cenario-simplificado`, `formulacao`,
`verificacao-desktop`, `resultado-operacional`, `reutilizacao`, `htmx`, `condicionais`, `falhas`,
`guias`, `dicas`, `ajuda` e `fluxos`. Casos verificam Chromium, Firefox e WebKit no CI; a inspeção
visual é complementada por capturas de página.

Casos importantes de regressão incluem: seleção progressiva e rascunho preservado; cálculo sem
JavaScript; CSRF e falhas HTTP; formulação interrompida versus inviável; revalidação prática;
custo com perdas/sobras; comparação recusada para revisões diferentes; histórico imutável;
backup/restauração e primeira simulação em banco limpo.

## Executar a verificação local

```sh
cargo build --locked
cargo test --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo fmt --check
python3 scripts/qualidade.py
python3 scripts/verificar_documentacao.py
python3 -B -m unittest discover -s scripts -p 'test_*.py'
python3 scripts/verificar_estrutura.py
python3 scripts/verificar_sqlx.py
RUST_LOG=warn cargo run --quiet -- ensaiar
```

Para os navegadores:

```sh
npm ci --ignore-scripts
npx playwright install --with-deps chromium firefox webkit
npm run test:e2e
```

`npm test` é equivalente à suíte E2E completa. Para diagnóstico isolado, após compilar:

```sh
FEEDYOURCOW_NAVEGADORES=chromium FEEDYOURCOW_SUITES=formulacao npm run test:e2e
```

`FEEDYOURCOW_SUITES` aceita nomes separados por vírgula. Esse filtro é útil para investigar
falhas; a entrega deve usar a suíte completa nos navegadores definidos pelo CI. Para testar o
binário otimizado, defina `FEEDYOURCOW_BINARIO` com o caminho do executável de release.

## Regras estáticas e desempenho

`cargo fmt --check` valida formatação Rust. `scripts/qualidade.py` limita a 100 caracteres por
linha os arquivos de código/configuração que inspeciona e avisa quando módulos relevantes
passam de 300 linhas. Markdown é revisado separadamente. `scripts/verificar_estrutura.py`
impede mais de dez arquivos `.rs` imediatos por diretório de `src/`.
`scripts/verificar_sqlx.py` compara as macros com `.sqlx/` usando as migrações em banco novo.
`scripts/verificar_documentacao.py` valida links locais e linhas de até 100 caracteres no README
e nos arquivos Markdown de `docs/`.

`ensaiar` executa aquecimento e medições repetidas do motor. As metas do perfil são p95 até
**1 segundo** para verificar 20 ingredientes e **5 segundos** para formular com 10. A meta é
um critério de aceite, não garantia de tempo em qualquer hardware.

## GitHub Actions

`.github/workflows/ci.yml` roda em `push` e `pull_request`:

1. O job `rust` faz build, testes Rust, Clippy com avisos como erro, rustfmt, scripts de
   qualidade, verificação SQLx e ensaio de desempenho. Publica o binário de teste.
2. O job `javascript-e2e` baixa esse binário, instala Node 24 e Playwright, executa os três
   navegadores e publica capturas mesmo se um caso falhar.

O script de produção e seu teste são uma validação local adicional; não fazem parte do workflow
atual. Para aprovar uma entrega final, confira o resultado do CI no commit exato e rode o teste
do pacote gerado para a plataforma de destino.

## Estado desta revisão documental

Em 22/10/2026, os guias foram confrontados com código, migrações, cargas, testes e workflow.
Passaram `cargo test --locked`, Clippy com `-D warnings`, `cargo fmt --check`, os verificadores
de qualidade, estrutura, SQLx e documentação, e os cinco testes Python. `ensaiar` mediu p95 de
29,516 ms na verificação e 37,621 ms na formulação, abaixo das metas do perfil nesta máquina.

A suíte E2E completa passou no Chromium. Após corrigir no observador dos testes o tratamento de
ícones cancelados no fechamento de guias, a suíte completa também passou no Firefox e WebKit.
O caso de reutilização foi repetido isoladamente nos dois navegadores afetados. O pacote Linux
x86-64 gerado por `scripts/build_producao.sh` passou em `scripts/testar_producao.py`, incluindo
preparação, histórico, backup, motor e HTTP. Isso verifica o estado local desta revisão; o
resultado do GitHub Actions no commit final deve ser conferido à parte.

## Regressões da auditoria de fluxos

`tests/cadastros_auditoria.rs` cobre data civil, preservação dos campos, dados legados e
cancelamento. `tests/historico_referencia.rs` verifica referência fora da página e parâmetros
codificados. `tests/comparacao_quantidades.rs` cobre resultado contínuo/prático e associação
por composição; `tests/comparacao.rs` verifica datas e conhecimento preservado completo.
A suíte `auditoria` do navegador cadastra do zero e por exemplo, calcula nos dois modos e
compara entre páginas com mais de 100 execuções, com e sem JavaScript, incluindo capturas de
desktop, mobile e impressão. Os testes usam banco temporário e executável congelado.

Referências de implementação consultadas: [datas civis do chrono][chrono],
[comparação estrutural via serde_json][serde] e [parâmetros GET no Axum][query].

[chrono]: https://docs.rs/chrono/latest/chrono/struct.NaiveDate.html#method.parse_from_str
[serde]: https://docs.rs/serde_json/latest/serde_json/fn.to_value.html
[query]: https://docs.rs/axum/0.8.9/axum/extract/struct.Query.html

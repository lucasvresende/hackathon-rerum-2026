# Testes JavaScript

Os arquivos Rust continuam em `tests/*.rs`, com descoberta automática pelo Cargo.
Os testes JavaScript de integração ficam em `tests/javascript/e2e/`:

- `navegadores.mjs`: prepara banco e servidor temporários e executa os três navegadores.
- `templates.mjs`: referências editáveis, cadastros e componentes dos formulários.
  Confere campos antes ocultados pelo modo simplificado em lotes, alimentos e cenários,
  inclusive após carregar modelos e validar campos; não há alternância de visualização.
- `dicas.mjs`: dicas no cenário, catálogo/cadastro e parecer; IDs únicos, rótulos, ausência
  de envio ao abrir dicas, reativação após htmx, tela móvel, impressão e uso sem JavaScript.
- `inicio.mjs`: três estatísticas e cinco dicas da Visão geral; hover, teclado, Escape,
  clique fora, toque,
  limites da tela, impressão e funcionamento por clique sem JavaScript nos três navegadores.
- `ajuda.mjs`: ajuda contextual em nova guia, preservação da origem, teclado, acessibilidade,
  impressão e modo sem JavaScript.

```sh
cargo build --locked
npm ci --ignore-scripts
npx playwright install --with-deps chromium firefox webkit
npm run test:e2e
```

`npm test` é um atalho compatível. Não compile novamente enquanto a suíte estiver em execução:
o motor de formulação usa o executável do servidor para iniciar o processo filho.
O job JavaScript do CI recebe o binário produzido pelo job Rust.

A suíte usa Playwright e `node:assert/strict`, sem dependências adicionais.
As capturas ficam em `output/playwright/`, fora do Git. O banco operacional não é utilizado.
`FEEDYOURCOW_NAVEGADORES=chromium npm run test:e2e` restringe um diagnóstico; a entrega usa os três.

Testes de DOM e htmx pertencem a `e2e/`. A pasta `unitarios/` e o comando `test:unit` serão
adicionados quando houver funções puras relevantes; não há uma suíte unitária JavaScript atual.

## Contratos de integração

Os casos nomeados usam contextos novos, o htmx real e o servidor temporário:

- `htmx.mjs`: método/URL/cabeçalhos, CSRF, dados e confirmação do modelo; fragmento sem recarga;
  unicidade dos controles; foco na prévia e no resumo de erros; reativação dos comportamentos
  depois do swap; bloqueio de POST inválido; resposta `HX-Redirect` e navegação resultante.
- `condicionais.mjs`: consumo obrigatório acima de 600 kg (com teste na borda de 600 kg),
  ingredientes selecionados, modo formular/verificar, preço por embalagem, laudos parciais
  e abertura de seção recolhida contendo campo inválido.
- `falhas.mjs`: 403 real por CSRF inválido, 500 simulado e interrupção de rede simulada;
  preservação do formulário/rascunho e nova tentativa bem-sucedida em cada caso.
- `guias.mjs`: atualização automática por BroadcastChannel/storage/foco após salvar em outra guia,
  sem clicar
  no botão manual; confere o POST, a inclusão da opção nova e a preservação dos campos.
- `apoio.mjs`: observação dos eventos/requisições e diagnóstico por caso. Falhas HTTP/rede
  são aceitas somente quando esperadas no caminho do caso; exceções de página continuam falhando.

O htmx 4 usado pelo projeto identifica aplicação de modelo com `?acao=aplicar`; a confirmação
vai no corpo do POST. Os testes observam esse contrato efetivo, sem substituir o htmx.
Para comprovar ausência de POST inválido, o observador é instalado antes do clique e aguarda
uma janela negativa de 400 ms; não há pausas fixas para esperar respostas ou trocas de DOM.
Os cenários de rede usam `route.fulfill`/`route.abort`; os demais usam respostas reais.

Para diagnóstico, `FEEDYOURCOW_SUITES` aceita `htmx`, `condicionais`, `falhas`, `guias`,
`inicio`, `dicas` e `fluxos`, separados por vírgula. O padrão executa todos. Exemplo:

```sh
FEEDYOURCOW_NAVEGADORES=chromium FEEDYOURCOW_SUITES=falhas npm run test:e2e
```

A suíte `fluxos` mantém os roteiros anteriores de templates, ajuda e aplicação completa.
O caso anterior que usa o botão “Atualizar cadastros” continua cobrindo a atualização manual;
a nova suíte `guias` verifica exclusivamente a sinalização automática.

Referências de implementação: [rede no Playwright](https://playwright.dev/docs/network) e
[eventos do htmx 4](https://four.htmx.org/reference/events).

Os casos encontraram três regressões: erro HTTP removia o formulário; WebKit tentava focar
um campo de laudo ainda `inert`; e a sinalização somente por storage não chegou à guia de origem
no WebKit testado. As correções preservam o rascunho, adiam o foco até abrir a seção e usam
BroadcastChannel com alternativa por storage e deduplicação. O teste entre guias também verifica
que receber a mesma sinalização pelos dois transportes não dispara outro POST.
A API nativa está documentada em [BroadcastChannel no MDN][broadcast].

[broadcast]: https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel

O observador diferencia o cancelamento tardio de `GET /static/favicon.svg` pelo Firefox
(`NS_BINDING_ABORTED`) durante navegações sucessivas. Essa exceção não se aplica a POSTs,
a outros recursos ou a falhas HTTP, que continuam sendo verificadas.

## Validação local

Em 21/10/2026, `npm test` executou a suíte completa com sucesso em Chromium, Firefox e WebKit:
os dez casos novos passaram em cada navegador (30 execuções), seguidos dos fluxos anteriores.
As capturas de erro de campo em desktop e formulário móvel foram inspecionadas visualmente.
Os 46 testes Rust, build, clippy, rustfmt, limite de linhas e 28 consultas SQLx também passaram;
o ensaio de desempenho atingiu as metas. O YAML do CI foi parseado e os jobs conferidos,
mas a execução remota depende da publicação dos commits.

Após a retirada do preenchimento simplificado, a suíte completa passou novamente em
Chromium, Firefox e WebKit em 21/10/2026, incluindo visibilidade dos campos, templates,
erros, trocas por htmx e uso sem JavaScript. As capturas móvel de alimento e desktop de lote
com erro foram inspecionadas. Build, 53 testes Rust, clippy, rustfmt, qualidade textual e
verificação das 28 consultas SQLx também passaram.

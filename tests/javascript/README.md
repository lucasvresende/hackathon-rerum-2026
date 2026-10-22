# Testes JavaScript

Os arquivos Rust continuam em `tests/*.rs`, com descoberta automática pelo Cargo.
Os testes JavaScript de integração ficam em `tests/javascript/e2e/`:

- `navegadores.mjs`: prepara banco e servidor temporários e executa os três navegadores.
- `formulacao.mjs`: regressões do diagnóstico de menor custo: seleção e edição durante resposta
  atrasada, cálculo pendente, confirmação/cancelamento de demonstração com e sem JavaScript,
  preservação do modo, medição parcial, links de erro, pesquisa, disponibilidade visível,
  ausência de candidata, alvo preservado, números em português e diagnóstico prático no móvel.
  Executar isoladamente com `FEEDYOURCOW_SUITES=formulacao npm run test:e2e`.
  A inspeção pós-correção passou nos três navegadores, junto à suíte completa existente.
  As capturas de seleção, erro de medição, ausência de candidata e fornecimento prático móvel
  são preservadas em `output/playwright/` para inspeção visual.
- `exemplos.mjs`: primeiro cálculo preenchido, remoção e restauração explícita de exemplos,
  identificação sintética, tela móvel e execução com e sem JavaScript nos três navegadores.
- `templates.mjs`: preenchimento com exemplos, cadastros e componentes dos formulários.
  Confere campos de lotes e alimentos e as configurações avançadas de cenários,
  inclusive após carregar modelos e validar campos. No fluxo sem JavaScript, aguarda a resposta
  POST da aplicação do exemplo antes de conferir os valores.
- `dicas.mjs`: dicas no cenário, catálogo/cadastro e parecer; IDs únicos, rótulos, ausência
  de envio ao abrir dicas, reativação após htmx, tela móvel, impressão e uso sem JavaScript.
  Os exemplos da Ajuda aguardam o conteúdo do cartão, inclusive durante a transição no WebKit.
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

`npm test` é um atalho compatível. A suíte copia o binário para seu diretório temporário antes
de iniciar o servidor; o motor filho usa essa cópia estável mesmo que o Cargo recompile o original.
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
- `condicionais.mjs`: consumo-alvo opcional nas bordas de 600 kg, ingredientes selecionados,
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
`inicio`, `dicas`, `fluxos` e `exemplos`, separados por vírgula. O padrão executa todos. Exemplo:

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

Na validação deste checkout em 21/10/2026, `npm test` passou em Chromium, Firefox e WebKit,
incluindo formulários, contratos htmx, erros HTTP/rede, atualização entre guias, fluxos sem
JavaScript, verificação, formulação, simulação, histórico, comparação e impressão. As capturas
de formulário móvel, erro e resultado foram inspecionadas visualmente.

`cargo test --locked` passou com 59 testes; build, clippy, rustfmt, qualidade textual e a
verificação SQLx passaram, com 37 consultas conferidas contra banco limpo. O ensaio de
desempenho passou: p95 de 18,94 ms para verificar 20 ingredientes e 25,71 ms para formular
com 10 ingredientes, abaixo das metas de 1 s e 5 s. A execução remota do GitHub Actions depende
da publicação dos commits.

## Cadastros de exemplo

A suíte `exemplos` verifica primeiro cálculo com cinco ingredientes preenchidos, identificação
sintética, remoção e restauração explícita, filtro por categoria, layout móvel e uso sem JavaScript.
As capturas `*-exemplos-*` e `*-exemplo-*` complementam a inspeção dos cadastros e do parecer.
Os três navegadores passaram na suíte completa de 21/10/2026, incluindo a verificação de exemplos.

## Criação simplificada de dietas

A suíte `cenario-simplificado` cobre defaults, ausência de escolha implícita, seleção progressiva,
remoção e nova seleção com preservação do rascunho, preenchimento incompleto e alternativa
sem JavaScript em Chromium, Firefox e WebKit. Os testes existentes abrem as configurações
avançadas quando precisam alterar suas premissas. O dropdown mantém a ordem do catálogo;
a seleção não move opções enquanto o usuário navega. O estado dos expansíveis é enviado como
texto `true`/`false`, preservando a abertura após atualizar cadastros ou ingredientes.

```sh
FEEDYOURCOW_SUITES=cenario-simplificado npm run test:e2e
```

Os testes de seleção aguardam a aplicação dos valores após o swap, incluindo a transição
`settle` do htmx. A checagem das animações também espera a inicialização do expansível para não
ler seu estilo antes de `expand-ready` no WebKit. Isso evita conferir atributos temporários
durante a troca do HTML. O fluxo de resultado localiza “Cálculos e regras aplicadas” pelo botão
expansível acessível que a interface realmente renderiza.
As dicas em tela móvel são testadas após abrir as configurações que contêm seus controles.

Validação da branch de simplificação em 21/10/2026: suíte completa aprovada em Chromium,
Firefox e WebKit; o teste de seleção também foi repetido com sua espera final ajustada.
Build, testes Rust, clippy sem warnings, rustfmt, qualidade textual e `git diff --check`
aprovados. Capturas móveis de Chromium e WebKit inspecionadas visualmente.

A suíte `cenario-simplificado` também verifica as transições compartilhadas do seletor,
Enter/Espaço, `aria-expanded`, movimento reduzido, impressão e acesso sem JavaScript.
O contexto começa com `reducedMotion: 'no-preference'` para manter determinística a checagem
da transição; em seguida, o teste ativa movimento reduzido e confere que a animação é desativada.

Transições do seletor validadas em 21/10/2026: `cenario-simplificado` e `condicionais`
aprovadas nos três navegadores; testes Rust do cenário, clippy e formatação aprovados.


A suíte `resultado-operacional` cobre solução contínua sem resolução, recusa prática no topo,
valores de fibra, premissas da comparação, estados do histórico, leitura móvel e impressão curta.
Exemplo de execução isolada: `FEEDYOURCOW_SUITES=resultado-operacional npm run test:e2e`.


### Regressão do diagnóstico de dietas em desktop

`verificacao-desktop` usa viewport de 1440 × 1000, sem alternar para celular. Ela verifica
conversão de MS na duplicação, rótulos e unidades, consumo-alvo na verificação, estados e motivos
de pendência/invalidade/violação, sobras, amido, preços, ótimo contínuo e revalidação de pesagem.
Executar após compilar o binário:

```sh
FEEDYOURCOW_NAVEGADORES=chromium FEEDYOURCOW_SUITES=verificacao-desktop npm run test:e2e
```

Os dados são temporários; capturas ficam em `output/playwright/chromium-verificacao-*.png`.
As interações usam a biblioteca Playwright já fixada no projeto, sem dependências adicionais.
Referências consultadas: [assertions Playwright](https://playwright.dev/docs/test-assertions)
e [formatação numérica Rust](https://doc.rust-lang.org/std/fmt/#precision).

### Reutilização e comparação restrita

`FEEDYOURCOW_NAVEGADORES=chromium FEEDYOURCOW_SUITES=reutilizacao npm run test:e2e`
repete os fluxos de otimização, cópia contínua/prática, ajuste e verificação no desktop.
Confere a preservação do original, bases de quantidade, custos com sobras/perdas, comparação
entre execuções da mesma revisão e recusa HTTP 422 para outro lote. As capturas correspondentes
ficam em `output/playwright/chromium-reutilizacao-*.png` e
`output/playwright/chromium-comparacao-outro-lote-recusada.png`.

# Testes JavaScript

Os arquivos Rust continuam em `tests/*.rs`, com descoberta automática pelo Cargo.
Os testes JavaScript de integração ficam em `tests/javascript/e2e/`:

- `navegadores.mjs`: prepara banco e servidor temporários e executa os três navegadores.
- `templates.mjs`: referências editáveis, cadastros e componentes dos formulários.
- `ajuda.mjs`: navegação contextual, teclado, acessibilidade, impressão e modo sem JavaScript.

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

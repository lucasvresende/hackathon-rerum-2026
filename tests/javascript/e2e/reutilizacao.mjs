import assert from 'node:assert/strict';
import { join } from 'node:path';
import { executarCaso, abrirConfiguracoesCenario } from './apoio.mjs';

/** Reproduz o ciclo formular → copiar → verificar e a recusa de outro lote, no desktop. */
export async function testarReutilizacao(browser, url, artefatos, navegador) {
  await executarCaso(browser, url, artefatos, navegador, 'reutilizacao',
    async (page, contexto) => {
      async function calcular(p = page) {
        await p.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
        await p.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
      }
      async function copiar(nome) {
        const [copia] = await Promise.all([contexto.waitForEvent('page'),
          page.getByRole('link', { name: nome, exact: true }).click()]);
        await copia.waitForLoadState();
        return copia;
      }
      await page.goto(`${url}/cenarios/novo?exemplo=inicial&modo=formular`);
      await calcular();
      const original = page.url();
      const stats = await page.locator('.stats').innerText();
      const fornecimento = await page.locator('#fornecimento tbody').innerText();
      assert.equal(await page.getByRole('link',
        { name: 'Ajustar e verificar quantidades práticas' }).count(), 0);
      const refazer = await copiar('Refazer otimização');
      assert.equal(await refazer.locator('#modo').inputValue(), 'formular');
      await refazer.locator('#modo').selectOption('verificar');
      const valores = await refazer.locator('[name^="q-"]')
        .evaluateAll(es => es.map(e => Number(e.value)));
      assert.ok(valores.some(v => v > 0), 'A candidata não pode desaparecer ao trocar o modo');
      await calcular(refazer);
      assert.equal(await refazer.locator('.stats').innerText(), stats);
      await refazer.close();

      const continua = await copiar('Ajustar e verificar solução contínua');
      assert.equal(await continua.locator('#modo').inputValue(), 'verificar');
      const selecao = await continua.locator('[name^="usar-"]:checked').count();
      await continua.getByRole('button', { name: 'Selecionar ingredientes', exact: true }).click();
      await continua.getByRole('searchbox', { name: 'Pesquisar ingrediente' }).fill('milho');
      assert.equal(await continua.locator('[name^="usar-"]:checked').count(), selecao);
      await continua.getByRole('searchbox', { name: 'Pesquisar ingrediente' }).fill('');
      await calcular(continua);
      const alternativa = continua.url();
      assert.equal(await continua.locator('.stats').innerText(), stats);
      await continua.screenshot({ path: join(artefatos,
        `${navegador}-reutilizacao-continua.png`), fullPage: true });
      const [edicao] = await Promise.all([contexto.waitForEvent('page'),
        continua.getByRole('link', { name: 'Duplicar e simular' }).click()]);
      // A edição mantém a verificação preservada na guia anterior.
      await edicao.waitForLoadState();
      const silagem = edicao.locator('.ingredient').filter({ has:
        edicao.getByRole('heading', { name: 'Silagem de milho', exact: true }) });
      const q = silagem.locator('[name^="q-"]');
      await q.fill(String(Number(await q.inputValue()) + 1));
      await calcular(edicao);
      assert.notEqual(await edicao.locator('.stats').innerText(), stats);
      await edicao.close();
      await continua.close();
      await page.reload();
      assert.equal(await page.locator('.stats').innerText(), stats);
      assert.equal(await page.locator('#fornecimento tbody').innerText(), fornecimento);

      await page.goto(`${url}/cenarios/novo?exemplo=inicial&modo=formular`);
      await abrirConfiguracoesCenario(page);
      await page.locator('#sobras').fill('10');
      for (const i of await page.locator('.ingredient').all()) {
        await i.getByRole('button', { name: 'Configurações avançadas do ingrediente' }).click();
        await i.locator('[name^="resolucao-"]').fill('0.1');
        await i.locator('[name^="perda-"]').fill('5');
      }
      await calcular();
      const pratico = await page.locator('#resultado-pratico').innerText();
      const consumo = pratico.match(/Consumo prático por animal\/dia\n([^\n]+)/)[1];
      const custo = pratico.match(/Custo prático por animal\/dia\n([^\n]+)/)[1];
      const pratica = await copiar('Ajustar e verificar quantidades práticas');
      await abrirConfiguracoesCenario(pratica);
      assert.equal(await pratica.locator('#base').inputValue(), 'OFERTA_MN');
      await calcular(pratica);
      const custoEsperado = new RegExp(custo.replace('$', '\\$'));
      assert.match(await pratica.locator('.stats').innerText(), custoEsperado);
      const ms = Number(consumo.split(' ')[0].replace(',', '.'));
      const esperado = ms.toLocaleString('pt-BR', { minimumFractionDigits: 3,
        maximumFractionDigits: 3 });
      assert.ok((await pratica.locator('.stats').innerText()).includes(`${esperado} kg MS`));
      await pratica.screenshot({ path: join(artefatos,
        `${navegador}-reutilizacao-pratica.png`), fullPage: true });
      await pratica.close();

      const a = original.split('/').at(-1), b = alternativa.split('/').at(-1);
      await page.goto(`${url}/historico`);
      await page.getByLabel('Cenário de referência').selectOption(a);
      await page.getByLabel('Cenário alternativo').selectOption(b);
      await page.getByRole('button', { name: 'Comparar premissas e custos' }).click();
      assert.match(await page.locator('main').innerText(), /Diferença de custo/);
      await page.goto(`${url}/cenarios/novo?exemplo=inicial`);
      const lote = page.locator('#lote');
      const atual = await lote.inputValue();
      const outro = await lote.locator('option').evaluateAll((es, id) =>
        es.find(e => e.value && e.value !== id).value, atual);
      await lote.selectOption(outro);
      await calcular();
      const outroId = page.url().split('/').at(-1);
      const resposta = await page.goto(`${url}/comparar?a=${a}&b=${outroId}`);
      assert.equal(resposta.status(), 422);
      const texto = await page.locator('main').innerText();
      assert.match(texto, /mesma revisão do lote/);
      assert.doesNotMatch(texto, /Diferença de custo|Ingredientes e premissas/);
      await page.screenshot({ path: join(artefatos,
        `${navegador}-comparacao-outro-lote-recusada.png`), fullPage: true });
    }, { status: 422, caminho: '/comparar' });

  const semJs = await browser.newContext({ javaScriptEnabled: false,
    viewport: { width: 1440, height: 1000 } });
  try {
    const page = await semJs.newPage();
    await page.goto(`${url}/cenarios/novo?exemplo=inicial&modo=formular`);
    await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    await page.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
    const stats = await page.locator('.stats').innerText();
    const [copia] = await Promise.all([semJs.waitForEvent('page'), page.getByRole('link',
      { name: 'Ajustar e verificar solução contínua', exact: true }).click()]);
    await copia.waitForLoadState();
    assert.equal(await copia.locator('#modo').inputValue(), 'verificar');
    await copia.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    await copia.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
    assert.equal(await copia.locator('.stats').innerText(), stats);
    console.log(`${navegador}: reutilização sem JavaScript passou`);
  } finally { await semJs.close(); }
}

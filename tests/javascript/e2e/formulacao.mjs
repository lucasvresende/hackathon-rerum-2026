import assert from 'node:assert/strict';
import { join } from 'node:path';
import { executarCaso, abrirConfiguracoesCenario } from './apoio.mjs';

export async function testarFormulacao(browser, url, artefatos, navegador) {
  await executarCaso(browser, url, artefatos, navegador, 'formulacao-concorrencia', async page => {
    await page.goto(`${url}/cenarios/novo?dieta=B&modo=formular`);
    await page.getByRole('button', { name: 'Selecionar ingredientes', exact: true }).click();
    let liberar;
    const chegou = new Promise(resolve => {
      page.route('**/cenarios/atualizar', async route => {
        await new Promise(r => { liberar = r; resolve(); });
        await route.continue();
      }, { times: 1 });
    });
    const ids = await page.locator('input[name^="usar-"]:not(:checked)')
      .evaluateAll(es => es.slice(0, 2).map(e => e.id));
    await page.locator(`[id="${ids[0]}"]`).check();
    await chegou;
    await page.locator(`[id="${ids[1]}"]`).check();
    await page.locator('#nome').fill('Edição enquanto aguarda resposta');
    const preco = page.locator('.ingredient input[name^="preco-"]').first();
    const nomePreco = await preco.getAttribute('name');
    await preco.fill('7.75');
    liberar();
    await page.waitForFunction(() => document.querySelectorAll('.ingredient').length === 6);
    await page.waitForFunction(nome => document.querySelector(`[name="${nome}"]`)
      ?.value === '7.75', nomePreco);
    for (const id of ids) assert.equal(await page.locator(`[id="${id}"]`).isChecked(), true);
    assert.equal(await page.locator('#nome').inputValue(), 'Edição enquanto aguarda resposta');
    assert.equal(await page.locator(`[name="${nomePreco}"]`).inputValue(), '7.75');
    await page.screenshot({ path: join(artefatos, `${navegador}-selecao-preservada.png`),
      fullPage: true, animations: 'disabled' });
  });

  await executarCaso(browser, url, artefatos, navegador, 'formulacao-envio-pendente',
    async page => {
    await page.goto(`${url}/cenarios/novo?dieta=B&modo=formular`);
    await page.getByRole('button', { name: 'Selecionar ingredientes', exact: true }).click();
    let liberar;
    const chegou = new Promise(resolve => {
      page.route('**/cenarios/atualizar', async route => {
        await new Promise(r => { liberar = r; resolve(); });
        await route.continue();
      }, { times: 1 });
    });
    await page.locator('#usar-milho-demo-v1').uncheck();
    await chegou;
    await page.locator('#nome').fill('Cálculo depois da seleção');
    await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    liberar();
    await page.waitForURL(/\/execucoes\//);
    assert.match(await page.locator('h1').innerText(), /Cálculo depois da seleção/);
    assert.equal(await page.locator('[data-busca]').getAttribute('data-busca'), 'OTIMO_NO_MODELO');
  });

  for (const javaScriptEnabled of [true, false]) {
    const contexto = await browser.newContext({ javaScriptEnabled });
    const page = await contexto.newPage();
    try {
      await page.goto(`${url}/cenarios/novo?dieta=B&modo=formular`);
      await page.locator('#nome').fill('Rascunho preservado');
      await page.getByRole('button', { name: 'Demonstração A', exact: true }).click();
      await page.locator('#exemplo_pendente').waitFor();
      assert.equal(await page.locator('#nome').inputValue(), 'Rascunho preservado');
      await page.getByRole('button', { name: 'Preencher com exemplo', exact: true }).click();
      await page.locator('#confirmar_exemplo-erro').waitFor({ state: 'visible' });
      assert.equal(await page.locator('#nome').inputValue(), 'Rascunho preservado');
      await page.getByRole('button', { name: 'Manter rascunho', exact: true }).click();
      await page.locator('#exemplo_pendente').waitFor({ state: 'detached' });
      assert.equal(await page.locator('#nome').inputValue(), 'Rascunho preservado');
      await page.getByRole('button', { name: 'Demonstração B', exact: true }).click();
      await page.locator('#confirmar_exemplo').check();
      await page.getByRole('button', { name: 'Preencher com exemplo', exact: true }).click();
      await page.waitForFunction(() => document.querySelector('#nome').value.includes('Dieta B'));
      assert.equal(await page.locator('#modo').inputValue(), 'formular');
      assert.equal(await page.locator('[data-quantidade]:visible').count(), 0);
      assert.equal(await page.locator('.ingredient input[name^="livre-"]:visible').count(), 4);
      if (javaScriptEnabled) {
        await page.getByRole('button', { name: 'Selecionar ingredientes', exact: true }).click();
      }
      await page.locator('#busca-ingredientes').fill('soja');
      await page.getByRole('button', { name: 'Atualizar ingredientes selecionados' }).click();
      await page.locator('[data-alimento]:visible').first().waitFor();
      assert.equal(await page.locator('input[name^="usar-"]:checked').count(), 4);
      assert.equal(await page.locator('[data-alimento]:visible').evaluateAll(es =>
        es.every(e => e.textContent.toLowerCase().includes('soja'))), true);
      console.log(`${navegador}: demonstração e pesquisa com JS=${javaScriptEnabled} passaram`);
    } finally { await contexto.close(); }
  }

  await executarCaso(browser, url, artefatos, navegador, 'formulacao-medicao', async page => {
    await page.goto(`${url}/cenarios/novo?dieta=B&modo=formular`);
    await abrirConfiguracoesCenario(page);
    await page.locator('#medicao .expand-toggle').click();
    await page.locator('#sobras_observadas').fill('10');
    await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    await page.locator('#oferta_observada-erro').waitFor({ state: 'visible' });
    assert.equal(await page.locator('#sobras_observadas').inputValue(), '10');
    await page.locator('#oferta_observada').fill('150');
    await page.locator('#periodo_observado').fill('1');
    await page.locator('#consumo_alvo').fill('10.8');
    await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    await page.locator('#medicao-erro').waitFor({ state: 'visible' });
    await page.locator('#medicao .expand-toggle').click();
    await page.locator('#erros-formulario a[href="#medicao"]').click();
    await page.locator('#oferta_observada').waitFor({ state: 'visible' });
    assert.equal(await page.locator('#medicao .expand-toggle')
      .getAttribute('aria-expanded'), 'true');
    await page.screenshot({ path: join(artefatos, `${navegador}-medicao-corrigir.png`),
      fullPage: true, animations: 'disabled' });
  });

  await executarCaso(browser, url, artefatos, navegador, 'formulacao-resultado', async page => {
    await page.goto(`${url}/cenarios/novo?dieta=B&modo=formular`);
    await abrirConfiguracoesCenario(page);
    await page.locator('#consumo_alvo').fill('1');
    await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    await page.waitForURL(/\/execucoes\//);
    assert.equal(await page.locator('[data-busca]').getAttribute('data-busca'),
      'INVIAVEL_NO_MODELO');
    assert.equal(await page.locator('.stats:visible').count(), 0);
    assert.match(await page.locator('main').innerText(), /Nenhuma combinação validada/);
    await page.locator('#entradas-preservadas .expand-toggle').first().click();
    await page.locator('#lote-premissas .expand-toggle').click();
    await page.locator('#lote-premissas')
      .getByText('1,0000 kg MS/animal/dia', { exact: true }).waitFor();
    await page.waitForFunction(() => document.querySelector('#lote-premissas')
      .innerText.includes('1,0000 kg MS'));
    await page.screenshot({ path: join(artefatos, `${navegador}-sem-candidata.png`),
      fullPage: true, animations: 'disabled' });
    await page.goto(`${url}/cenarios/novo?dieta=B&modo=formular`);
    for (const b of await page.locator('.ingredient .expand-toggle').all()) await b.click();
    for (const f of await page.locator('.ingredient [name^="resolucao-"]').all()) {
      await f.fill('1');
    }
    await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    await page.waitForURL(/\/execucoes\//);
    assert.match(await page.locator('#resultado-pratico').innerText(), /OPT-ALVO/);
    assert.match(await page.locator('#resultado-pratico').innerText(), /Custo prático/);
    assert.match(await page.locator('.stats').innerText(), /10,800 kg MS/);
    await page.setViewportSize({ width: 390, height: 844 });
    assert.equal(await page.evaluate(
      () => document.documentElement.scrollWidth > innerWidth), false);
    await page.screenshot({ path: join(artefatos, `${navegador}-pratico-mobile.png`),
      fullPage: true, animations: 'disabled' });
  });

  await executarCaso(browser, url, artefatos, navegador, 'formulacao-filtros', async page => {
    await page.goto(`${url}/cenarios/novo?dieta=B&modo=formular`);
    await page.locator('#modo').selectOption('verificar');
    assert.equal(await page.locator('[data-quantidade]:visible').count(), 4);
    await page.locator('#modo').selectOption('formular');
    assert.equal(await page.locator('[data-quantidade]:visible').count(), 0);
    await page.getByRole('button', { name: 'Selecionar ingredientes', exact: true }).click();
    await page.locator('#busca-ingredientes').fill('silagem');
    assert.ok(await page.locator('[data-alimento]:visible').count() > 0);
    assert.equal(await page.locator('#usar-soja-demo-v1').isChecked(), true);
    await page.locator('#classe-ingredientes').selectOption('Suplemento');
    assert.equal(await page.locator('[data-alimento]:visible').count(), 0);
    await page.locator('#classe-ingredientes').selectOption('');
    await page.locator('#busca-ingredientes').fill('');
    await page.locator('#contador-ingredientes').waitFor();
    assert.match(await page.locator('#contador-ingredientes').innerText(), /4 selecionados de 10/);
    for (let quantidade = 5; quantidade <= 11; quantidade++) {
      const proximo = page.locator('input[name^="usar-"]:not(:checked)').first();
      const id = (await proximo.getAttribute('name')).slice(5);
      await proximo.check();
      await page.locator(`[data-composicao="${id}"]`).waitFor();
    }
    assert.match(await page.locator('#contador-ingredientes').innerText(), /11 selecionados de 10/);
    for (const b of await page.locator('.ingredient .expand-toggle').all()) await b.click();
    for (const f of await page.locator('.ingredient [name^="preco-"]').all()) await f.fill('1');
    for (const f of await page.locator('.ingredient [name^="origem-"]').all()) {
      await f.fill('Cotação sintética para teste');
    }
    await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    const erro = page.locator('#erros-formulario a[href="#selecao-ingredientes"]');
    await erro.waitFor();
    await page.locator('#selecao-ingredientes .expand-toggle').click();
    await erro.click();
    await page.locator('#busca-ingredientes').waitFor({ state: 'visible' });
  });
}

import assert from 'node:assert/strict';
import { executarCaso, preencherLote, semEnvio } from './apoio.mjs';

export async function testarCondicionais(browser, url, artefatos, navegador) {
  await executarCaso(browser, url, artefatos, navegador, 'condicional-consumo', async page => {
    for (const peso of ['600', '650']) {
      await page.goto(`${url}/lotes/novo`);
      await preencherLote(page, `Consumo ${peso} ${navegador}`, peso);
      await page.getByRole('button', { name: 'Salvar revisão' }).click();
      await page.waitForURL(/cadastros\/salvo/);
    }
    await page.goto(`${url}/cenarios/novo?dieta=B`);
    for (const peso of ['600', '650']) {
      const opcao = page.locator('#lote option')
        .filter({ hasText: `Consumo ${peso} ${navegador}` });
      await page.locator('#lote').selectOption(await opcao.getAttribute('value'));
      assert.equal(await page.locator('#consumo_alvo').evaluate(e => e.required), false);
    }
    await page.locator('#lote').selectOption(await page.locator('#lote option')
      .filter({ hasText: 'Lote demonstração' }).getAttribute('value'));
    assert.equal(await page.locator('#consumo_alvo').evaluate(e => e.required), false);
  });

  await executarCaso(browser, url, artefatos, navegador, 'condicional-ingredientes', async page => {
    await page.goto(`${url}/cenarios/novo?dieta=B`);
    const ingrediente = page.locator('.ingredient').first();
    const usar = ingrediente.locator('input[name^="usar-"]');
    await usar.check();
    const unidade = ingrediente.locator('select[name^="unidade-preco-"]');
    const embalagem = ingrediente.locator('input[name^="embalagem-"]');
    const quantidade = ingrediente.locator('input[name^="q-"]');
    assert.equal(await quantidade.evaluate(e => e.required), true);
    await unidade.selectOption('embalagem');
    assert.equal(await embalagem.evaluate(e => e.required), true);
    await embalagem.fill('0');
    await semEnvio(page, () => page.getByRole('button',
      { name: 'Calcular e salvar cenário' }).click());
    assert.equal(await embalagem.getAttribute('aria-invalid'), 'true');
    await unidade.selectOption('kg');
    assert.equal(await embalagem.evaluate(e => e.required), false);
    await page.locator('#modo').selectOption('formular');
    assert.equal(await quantidade.evaluate(e => e.required), false);
    await page.locator('#modo').selectOption('verificar');
    assert.equal(await quantidade.evaluate(e => e.required), true);
    const ms = ingrediente.locator('input[name^="ms-"]');
    await ms.fill('0');
    assert.equal(await ms.evaluate(e => e.validity.customError), true);
    await usar.uncheck();
    assert.equal(await ms.evaluate(e => e.validity.customError), false);
    for (const prefixo of ['q', 'ms', 'preco', 'perda', 'origem', 'embalagem']) {
      assert.equal(await ingrediente.locator(`input[name^="${prefixo}-"]`)
        .evaluate(e => e.required), false, prefixo);
    }
    await usar.check();
    await ingrediente.locator('.expand-content').waitFor({ state: 'visible' });
    assert.equal(await ms.evaluate(e => e.required), true);
  });

  await executarCaso(browser, url, artefatos, navegador, 'condicional-laudos', async page => {
    await page.goto(`${url}/alimentos/novo`);
    for (const [id, valor] of Object.entries({ nome: 'Laudo parcial', ms: '30',
      fonte_id: 'TPL-FEED-SILAGEM', origem: 'Teste', metodo: 'Teste de preenchimento' })) {
      await page.locator(`#${id}`).fill(valor);
    }
    await page.locator('#laudos .expand-toggle').click();
    const analito = page.locator('[name=laudo_0_analito]');
    await analito.fill('AFB1');
    await analito.press('Tab');
    const campos = ['valor', 'unidade', 'ms', 'qualificador', 'metodo', 'documento'];
    for (const nome of campos) assert.equal(await page.locator(`[name=laudo_0_${nome}]`)
      .evaluate(e => e.required), true, nome);
    await page.locator('#laudos .expand-toggle').click();
    await page.locator('#laudos .expand-content').waitFor({ state: 'hidden' });
    await semEnvio(page, () => page.getByRole('button', { name: 'Salvar revisão' }).click());
    await page.locator('#laudos .expand-content').waitFor({ state: 'visible' });
    await page.waitForFunction(() => document.activeElement?.name === 'laudo_0_valor');
    assert.equal(await page.locator('[name=laudo_0_valor]')
      .getAttribute('aria-invalid'), 'true');
    await analito.fill('');
    await analito.press('Tab');
    for (const nome of campos) assert.equal(await page.locator(`[name=laudo_0_${nome}]`)
      .evaluate(e => e.required), false, nome);
  });
}

import assert from 'node:assert/strict';
import { join } from 'node:path';

export async function testarCenarioSimplificado(browser, url, artefatos, navegador) {
  for (const javaScriptEnabled of [true, false]) {
    const contexto = await browser.newContext({ javaScriptEnabled });
    const page = await contexto.newPage();
    const erros = [];
    page.on('pageerror', erro => erros.push(erro.message));
    try {
      await page.goto(`${url}/cenarios/novo`);
      assert.equal(await page.locator('#horizonte').inputValue(), '1');
      assert.equal(await page.locator('#sobras').inputValue(), '0');
      assert.equal(await page.locator('.ingredient').count(), 0);
      if (javaScriptEnabled) {
        await page.locator('#configuracoes-avancadas-conteudo')
          .waitFor({ state: 'hidden' });
      }
      await page.locator('#nome').fill('Rascunho do teste');
      await page.locator('#selecao-ingredientes summary').click();
      const id = (await page.locator('#selecao-ingredientes input[type=checkbox]')
        .first().getAttribute('name')).slice(5);
      const seletor = page.locator(`#usar-${id}`);
      const atualizar = () => page.getByRole('button', {
        name: 'Atualizar ingredientes selecionados', exact: true,
      }).click();
      await seletor.waitFor({ state: 'visible' });
      await page.waitForFunction(() => !document.querySelector('.htmx-request'));
      await seletor.check();
      if (!javaScriptEnabled) await atualizar();
      await page.locator('.ingredient').waitFor();
      assert.equal(await page.locator('.ingredient').count(), 1);
      assert.equal(await page.locator('#nome').inputValue(), 'Rascunho do teste');
      await page.locator(`[name="q-${id}"]`).fill('3.5');
      // Selecionar outro ingrediente não exige preço nem lote já preenchidos.
      const segundo = page.locator('#selecao-ingredientes input[type=checkbox]').nth(1);
      await segundo.check();
      if (!javaScriptEnabled) await atualizar();
      await page.waitForFunction(() => document.querySelectorAll('.ingredient').length === 2);
      assert.equal(await page.locator(`[name="q-${id}"]`).inputValue(), '3.5');
      await seletor.uncheck();
      if (!javaScriptEnabled) await atualizar();
      await page.locator(`[data-composicao="${id}"]`).waitFor({ state: 'detached' });
      await seletor.waitFor({ state: 'visible' });
      await page.waitForFunction(() => !document.querySelector('.htmx-request'));
      await seletor.check();
      if (!javaScriptEnabled) await atualizar();
      await page.locator(`[data-composicao="${id}"]`).waitFor();
      assert.equal(await page.locator(`[name="q-${id}"]`).inputValue(), '3.5');
      if (javaScriptEnabled) {
        await page.setViewportSize({ width: 390, height: 844 });
        await page.screenshot({ path: join(artefatos, `${navegador}-cenario-simples.png`),
          fullPage: true });
      }
      assert.deepEqual(erros, []);
    } finally { await contexto.close(); }
  }
}

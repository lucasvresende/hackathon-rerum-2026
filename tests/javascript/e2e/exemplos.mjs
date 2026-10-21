import assert from 'node:assert/strict';
import { join } from 'node:path';

export async function testarExemplos(browser, url, artefatos, navegador) {
  for (const javaScriptEnabled of [true, false]) {
    const ctx = await browser.newContext({ javaScriptEnabled,
      viewport: { width: 1280, height: 900 } });
    const page = await ctx.newPage();
    try {
      await page.goto(`${url}/cenarios/novo?exemplo=inicial`);
      assert.match(await page.locator('#nome').inputValue(), /Primeira simulação/);
      assert.equal(await page.locator('input[name^="usar-"]:checked').count(), 5);
      await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
      await page.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
      assert.match(await page.locator('main').innerText(), /Exemplo sintético|exemplo sintético/);
      await page.screenshot({ path: join(artefatos,
        `${navegador}-exemplo-${javaScriptEnabled ? 'js' : 'sem-js'}.png`), fullPage: true });
      await page.goto(`${url}/lotes`);
      const linha = page.locator('tr').filter({ hasText: 'Jersey — exemplo' }).first();
      assert.equal(await linha.getByText('Incluído no sistema').count(), 1);
      await linha.locator('summary').click();
      await linha.locator('[name=confirmar]').check();
      await linha.getByRole('button', { name: 'Confirmar remoção' }).click();
      await page.locator('tr').filter({ hasText: 'Jersey — exemplo' }).waitFor({
        state: 'detached',
      });
      await page.goto(`${url}/exemplos`);
      const card = page.locator('article').filter({ hasText: 'Jersey — exemplo' });
      await card.locator('[name=modo]').selectOption('revisar');
      await card.locator('[name=confirmar]').check();
      await card.getByRole('button', { name: 'Restaurar exemplo', exact: true }).click();
      await page.waitForURL(/salvo=sim/);
      await page.goto(`${url}/lotes`);
      assert.equal(await page.locator('tr').filter({ hasText: 'Jersey — exemplo' }).count(), 1);
      if (javaScriptEnabled) {
        await page.screenshot({ path: join(artefatos, `${navegador}-exemplos-lotes.png`) });
        await page.goto(`${url}/alimentos?classe=Forragem`);
        for (const card of await page.locator('.food-card').all()) {
          assert.match(await card.innerText(), /Forragem/);
        }
        await page.screenshot({ path: join(artefatos, `${navegador}-exemplos-alimentos.png`) });
      }
      await page.setViewportSize({ width: 390, height: 844 });
      await page.goto(`${url}/exemplos`);
      assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth),
        false, 'Restauração deve caber na tela móvel');
      if (javaScriptEnabled) await page.screenshot({
        path: join(artefatos, `${navegador}-exemplos-restaurar-mobile.png`) });
    } finally { await ctx.close(); }
  }
}

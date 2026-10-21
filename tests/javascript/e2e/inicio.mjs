import assert from 'node:assert/strict';
import { join } from 'node:path';

async function conferirLimites(page, dica) {
  const caixa = await dica.boundingBox();
  const tamanho = page.viewportSize();
  assert.ok(caixa && caixa.x >= 0 && caixa.y >= 0);
  assert.ok(caixa.x + caixa.width <= tamanho.width);
  assert.ok(caixa.y + caixa.height <= tamanho.height);
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth), false);
}

export async function testarInicio(browser, url, artefatos, nome) {
  const contexto = await browser.newContext({ viewport: { width: 1440, height: 1000 } });
  const erros = [];
  contexto.on('page', page => page.on('pageerror', erro => erros.push(erro.message)));
  try {
    const page = await contexto.newPage();
    await page.goto(url);
    assert.equal(await page.locator('.overview-stats .stat').count(), 3);
    assert.equal(await page.getByText('Execuções recentes', { exact: true }).count(), 0);
    const indicadores = page.locator('.concept-trigger');
    assert.equal(await indicadores.count(), 5);
    for (const botao of await indicadores.all()) {
      const id = await botao.getAttribute('popovertarget');
      const dica = page.locator(`#${id}`);
      await dica.waitFor({ state: 'hidden' });
      await botao.hover();
      await dica.waitFor({ state: 'visible' });
      await conferirLimites(page, dica);
      // O usuário pode mover o ponteiro até a explicação e selecionar seu texto.
      await dica.hover();
      await dica.waitFor({ state: 'visible' });
      await page.mouse.move(0, 0);
      await dica.waitFor({ state: 'hidden' });
      await botao.focus();
      await dica.waitFor({ state: 'visible' });
      await botao.press('Escape');
      await dica.waitFor({ state: 'hidden' });
      await botao.press('Enter');
      await dica.waitFor({ state: 'visible' });
      await botao.press('Enter');
      await dica.waitFor({ state: 'hidden' });
      await botao.press('Space');
      await dica.waitFor({ state: 'visible' });
      await page.getByRole('heading', { name: 'Visão geral', exact: true }).click();
      await dica.waitFor({ state: 'hidden' });
    }
    const perfil = page.getByRole('button', { name: 'Entender: Perfil ativo', exact: true });
    await perfil.click();
    await page.locator('#dica-perfil').waitFor({ state: 'visible' });
    await page.screenshot({ path: join(artefatos, `${nome}-inicio-dica.png`), fullPage: true });
    await page.emulateMedia({ media: 'print' });
    assert.equal(await page.locator('.concept-trigger:visible, .concept-tip:visible').count(), 0);
    await page.emulateMedia({ media: 'screen' });
    await page.goto(`${url}/ajuda#inicio`);
    await page.locator('#inicio-conteudo').waitFor({ state: 'visible' });
    assert.match(await page.locator('#inicio-conteudo').innerText(), /2 lotes e 50 animais/);
    await page.screenshot({ path: join(artefatos, `${nome}-ajuda-inicio.png`), fullPage: true });
    assert.deepEqual(erros, []);
  } finally { await contexto.close(); }

  // Toque real, bordas do viewport e fallback declarativo sem JavaScript.
  for (const javaScriptEnabled of [true, false]) {
    const mobile = await browser.newContext({ viewport: { width: 390, height: 844 },
      hasTouch: true, javaScriptEnabled });
    try {
      const page = await mobile.newPage();
      await page.goto(url);
      for (const botao of await page.locator('.concept-trigger').all()) {
        const dica = page.locator(`#${await botao.getAttribute('popovertarget')}`);
        await botao.tap();
        await dica.waitFor({ state: 'visible' });
        await conferirLimites(page, dica);
        if (javaScriptEnabled) {
          await page.screenshot({ path: join(artefatos,
            `${nome}-${await dica.getAttribute('id')}-mobile.png`), fullPage: true });
        }
        await page.getByRole('heading', { name: 'Visão geral', exact: true }).tap();
        await dica.waitFor({ state: 'hidden' });
      }
    } finally { await mobile.close(); }
  }
}

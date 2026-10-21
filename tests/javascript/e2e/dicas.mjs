import assert from 'node:assert/strict';
import { join } from 'node:path';
import { executarCaso, esperarPost } from './apoio.mjs';

async function conferirIds(page) {
  const ids = await page.locator('[id]').evaluateAll(els => els.map(e => e.id));
  assert.equal(new Set(ids).size, ids.length);
  assert.equal(await page.locator('label .concept-trigger, button .concept-trigger').count(), 0);
}

async function abrirDica(page, id) {
  const botao = page.locator(`[popovertarget="${id}"]`);
  const dica = page.locator(`#${id}`);
  await botao.scrollIntoViewIfNeeded();
  await botao.focus();
  await dica.waitFor({ state: 'visible' });
  await botao.press('Escape');
  await dica.waitFor({ state: 'hidden' });
  await botao.press('Enter');
  await dica.waitFor({ state: 'visible' });
  const caixa = await dica.boundingBox();
  const viewport = page.viewportSize();
  assert.ok(caixa && caixa.x >= 0 && caixa.y >= 0);
  assert.ok(caixa.x + caixa.width <= viewport.width);
  assert.ok(caixa.y + caixa.height <= viewport.height);
  assert.equal(await botao.evaluate(b => b.type), 'button');
  return { botao, dica };
}

export async function testarDicas(browser, url, artefatos, nome) {
  await executarCaso(browser, url, artefatos, nome, 'dicas-formularios', async page => {
    await page.goto(`${url}/cenarios/novo?dieta=B`);
    let envios = 0;
    page.on('request', r => { if (r.method() === 'POST') envios++; });
    await conferirIds(page);
    const quantidade = await page.locator('[name^="q-"]').first().inputValue();
    for (const id of ['conceito-horizonte', 'conceito-consumo_alvo', 'conceito-sobras',
      'conceito-base', 'conceito-base_comparacao']) {
      const { botao, dica } = await abrirDica(page, id);
      await botao.press('Escape');
      await dica.waitFor({ state: 'hidden' });
    }
    const perda = await page.locator('.concept-trigger[popovertarget^="conceito-perda-"]')
      .filter({ visible: true }).first().getAttribute('popovertarget');
    await abrirDica(page, perda);
    await page.screenshot({ path: join(artefatos, `${nome}-cenario-dica.png`), fullPage: true });
    assert.equal(await page.locator('[name^="q-"]').first().inputValue(), quantidade);
    assert.equal(envios, 0, 'Dicas não podem enviar o formulário');
    const formulario = await page.locator('#formulario').elementHandle();
    const atualizar = esperarPost(page, '/cenarios/atualizar');
    await page.getByRole('button', { name: 'Atualizar cadastros', exact: true }).click();
    await atualizar;
    await page.waitForFunction(anterior => !anterior.isConnected, formulario);
    await page.waitForFunction(() => document.querySelector('#conceito-sobras')
      ?.previousElementSibling?.dataset.dicaPronta === 'true');
    await conferirIds(page);
    const { botao } = await abrirDica(page, 'conceito-sobras');
    await botao.press('Escape');
    assert.equal(envios, 1);

    await page.goto(`${url}/alimentos/novo`);
    await conferirIds(page);
    await page.locator('label[for="ms"]').click();
    assert.equal(await page.locator('#ms').evaluate(e => document.activeElement === e), true);
    for (const id of ['conceito-ms', 'conceito-ndt', 'conceito-pb', 'conceito-natureza',
      'conceito-fonte_id']) {
      const { botao } = await abrirDica(page, id);
      await botao.press('Escape');
    }
    assert.equal(envios, 1);
  });

  await executarCaso(browser, url, artefatos, nome, 'dicas-catalogo-resultados', async page => {
    await page.goto(`${url}/alimentos`);
    await conferirIds(page);
    await abrirDica(page, 'dica-composicao');
    await page.screenshot({ path: join(artefatos, `${nome}-catalogo-dica.png`), fullPage: true });
    await page.goto(`${url}/cenarios/novo?dieta=B&modo=formular`);
    await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    await page.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
    await conferirIds(page);
    const parecer = page.url();
    for (const id of ['dica-estados', 'dica-nucleo', 'dica-balanco', 'dica-fornecimento',
      'dica-otimo']) {
      const { botao } = await abrirDica(page, id);
      await botao.press('Escape');
    }
    await page.setViewportSize({ width: 390, height: 844 });
    await abrirDica(page, 'dica-estados');
    await page.screenshot({ path: join(artefatos, `${nome}-resultado-dica-mobile.png`),
      fullPage: true });
    await page.emulateMedia({ media: 'print' });
    assert.equal(await page.locator('.concept-trigger:visible, .concept-tip:visible').count(), 0);
    await page.emulateMedia({ media: 'screen' });
    for (const javaScriptEnabled of [true, false]) {
      const contextoMobile = await browser.newContext({ javaScriptEnabled,
        hasTouch: true, viewport: { width: 390, height: 844 } });
      try {
        const mobile = await contextoMobile.newPage();
        for (const [rota, id] of [['/alimentos', 'dica-composicao'],
          ['/cenarios/novo?dieta=B', 'conceito-sobras'], [parecer, 'dica-estados']]) {
          await mobile.goto(rota.startsWith('http') ? rota : `${url}${rota}`);
          if (id === 'dica-composicao' && javaScriptEnabled) {
            const cabecalho = mobile.locator('.food-card .expand-heading').first();
            await cabecalho.locator('.expand-toggle').waitFor({ state: 'visible' });
            const expandir = await cabecalho.locator('.expand-toggle').boundingBox();
            const dica = await cabecalho.locator('.concept-trigger').boundingBox();
            assert.ok(expandir && dica && dica.x >= expandir.x + expandir.width);
            assert.ok(Math.abs(dica.y + dica.height / 2 - expandir.y - expandir.height / 2) < 1);
          }
          await mobile.locator(`[popovertarget="${id}"]`).tap();
          await mobile.locator(`#${id}`).waitFor({ state: 'visible' });
          assert.equal(await mobile.evaluate(() => document.documentElement.scrollWidth >
            innerWidth), false);
          if (javaScriptEnabled) {
            const pagina = id === 'dica-composicao' ? 'alimentos'
              : id === 'conceito-sobras' ? 'cenario' : 'resultado';
            await mobile.screenshot({ path: join(artefatos,
              `${nome}-dica-${pagina}-detalhe-mobile.png`) });
          }
          await mobile.touchscreen.tap(2, 2);
          await mobile.locator(`#${id}`).waitFor({ state: 'hidden' });
        }
      } finally { await contextoMobile.close(); }
    }
    for (const [secao, exemplo] of [['alimentos', '0,56 kg'], ['cenarios', '9 kg de MS'],
      ['resultados', 'cobertura é 80%']]) {
      await page.goto(`${url}/ajuda#${secao}`);
      await page.locator(`#${secao}-conteudo`).waitFor({ state: 'visible' });
      assert.ok((await page.locator(`#${secao}-conteudo`).innerText()).includes(exemplo));
    }
  });
}

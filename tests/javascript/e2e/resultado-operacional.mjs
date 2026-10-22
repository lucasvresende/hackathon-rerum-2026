import assert from 'node:assert/strict';
import { join } from 'node:path';
import { executarCaso, abrirConfiguracoesCenario } from './apoio.mjs';

export async function testarResultadoOperacional(browser, url, artefatos, navegador) {
  await executarCaso(browser, url, artefatos, navegador, 'resultado-operacional', async page => {
    await page.goto(`${url}/cenarios/novo?exemplo=inicial&modo=formular`);
    await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    await page.waitForURL(/\/execucoes\//);
    const referencia = page.url().split('/').at(-1);
    assert.equal(await page.locator('[data-fornecimento]').getAttribute('data-fornecimento'),
      'RESOLUCAO_PENDENTE');
    assert.match(await page.locator('#fornecimento').innerText(), /Não definida/);
    assert.match(await page.locator('#barreiras-ruminais').innerText(), /Linha efetiva.*19%/);
    assert.match(await page.locator('#barreiras-ruminais').innerText(), /Amido/);
    await page.goto(`${url}/cenarios/novo?exemplo=inicial&modo=formular`);
    for (const b of await page.locator('.ingredient .expand-toggle').all()) await b.click();
    for (const f of await page.locator('.ingredient [name^="resolucao-"]').all()) await f.fill('1');
    await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    await page.waitForURL(/\/execucoes\//);
    const alternativa = page.url().split('/').at(-1);
    assert.equal(await page.locator('[data-fornecimento]').getAttribute('data-fornecimento'),
      'FORNECIMENTO_INCOMPATIVEL');
    assert.match(await page.locator('#resultado-pratico').innerText(),
      /Não usar como recomendação/);
    const ordem = await page.locator('#situacao-fornecimento, [data-busca], #resultado-pratico')
      .evaluateAll(es => es.map(e => e.id || 'busca'));
    assert.deepEqual(ordem, ['situacao-fornecimento', 'resultado-pratico', 'busca']);
    await page.setViewportSize({ width: 390, height: 844 });
    await page.locator('#fornecimento').scrollIntoViewIfNeeded();
    const largura = await page.locator('#fornecimento').evaluate(e => ({
      largura: e.clientWidth, conteudo: e.scrollWidth,
    }));
    assert.ok(largura.conteudo <= largura.largura + 1, 'Fornecimento não deve exigir arraste');
    assert.equal(await page.locator('#fornecimento tbody tr').first()
      .locator('td').nth(1).getAttribute('data-rotulo'), 'Oferta prática kg MN/animal/dia');
    await page.screenshot({ path: join(artefatos, `${navegador}-fornecimento-operacional.png`),
      fullPage: true, animations: 'disabled' });
    await page.locator('#fornecimento').screenshot({
      path: join(artefatos, `${navegador}-fornecimento-cartoes.png`) });
    await page.setViewportSize({ width: 1440, height: 1000 });
    // Emula a mídia de impressão para verificar a ficha sem abrir diálogo nativo na suíte.
    await page.evaluate(() => document.body.classList.add('impressao-ficha'));
    await page.emulateMedia({ media: 'print' });
    assert.equal(await page.locator('.ficha-operacional').isVisible(), true);
    assert.equal(await page.locator('#calculos-regras').isVisible(), false);
    assert.match(await page.locator('.ficha-operacional').innerText(), /Não usar esta ficha/);
    if (navegador === 'chromium') {
      await page.pdf({ path: join(artefatos, 'chromium-ficha-operacional.pdf'),
        format: 'A4', printBackground: true });
    }
    await page.emulateMedia({ media: 'screen' });
    await page.evaluate(() => window.dispatchEvent(new Event('afterprint')));
    assert.equal(await page.locator('.ficha-operacional').isVisible(), false);
    await page.setViewportSize({ width: 1440, height: 1000 });
    await page.goto(`${url}/comparar?a=${referencia}&b=${alternativa}`);
    const comparacao = await page.locator('main').innerText();
    assert.match(comparacao, /Resolução de oferta/);
    assert.match(comparacao, /Inclusão mínima/);
    assert.match(comparacao, /16,6300/);
    assert.match(comparacao, /R\$ 17,35/);
    assert.doesNotMatch(comparacao, /16\.868500000000004/);
    await page.emulateMedia({ media: 'print' });
    const quebras = await page.locator('.comparacao-relatorio').evaluate(e => ({
      relatorio: getComputedStyle(e).breakInside,
      linhas: [...e.querySelectorAll('tr')].every(l => getComputedStyle(l).breakInside === 'avoid'),
    }));
    assert.equal(quebras.relatorio, 'auto', 'Comparação longa deve fluir entre páginas');
    assert.equal(quebras.linhas, true, 'Linhas curtas devem permanecer juntas na impressão');
    if (navegador === 'chromium') {
      await page.pdf({ path: join(artefatos, 'chromium-comparacao-paginada.pdf'),
        format: 'A4', printBackground: true });
    }
    await page.emulateMedia({ media: 'screen' });
    await page.goto(`${url}/cenarios/novo?exemplo=inicial&modo=formular`);
    await abrirConfiguracoesCenario(page);
    await page.locator('#consumo_alvo').fill('1');
    await page.locator('#nome').fill(`Inviável operacional ${navegador}`);
    await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    await page.waitForURL(/\/execucoes\//);
    await page.goto(`${url}/historico`);
    const linha = page.locator('tbody tr').filter({ hasText: `Inviável operacional ${navegador}` });
    assert.match(await linha.innerText(), /Inviável no modelo/);
    assert.doesNotMatch(await linha.innerText(), /INVALIDO/);
  });
}

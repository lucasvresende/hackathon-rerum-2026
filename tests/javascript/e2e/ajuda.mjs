import assert from 'node:assert/strict';
import { join } from 'node:path';

export async function testarAjuda(page, url, artefatos, navegador) {
  await page.goto(`${url}/ajuda`);
  await page.getByRole('heading', { name: 'Ajuda', exact: true }).waitFor();
  const cartas = page.locator('.help-card');
  assert.equal(await cartas.count(), 10);
  for (const carta of await cartas.all()) {
    const botao = carta.getByRole('button');
    await botao.waitFor();
    await page.waitForFunction(id => document.getElementById(id).classList.contains('help-ready'),
      await carta.getAttribute('id'));
    await carta.locator('.help-content').evaluate(async c => {
      await new Promise(requestAnimationFrame);
      await Promise.all(c.getAnimations().map(a => a.finished));
    });
    assert.equal(await botao.getAttribute('aria-expanded'), 'false');
    await carta.locator('.help-content').waitFor({ state: 'hidden' });
    assert.equal(await carta.evaluate(c => c.classList.contains('help-open')), false);
    assert.equal(await carta.locator('.help-content').evaluate(c => c.inert), true);
    await botao.click();
    await carta.locator('.help-content').waitFor({ state: 'visible' });
    assert.equal(await botao.getAttribute('aria-expanded'), 'true');
    assert.equal(await carta.evaluate(c => c.classList.contains('help-open')), true);
    assert.equal(await carta.locator('.help-content').evaluate(c => c.inert), false);
    await carta.locator('.help-content').evaluate(async c => {
      await Promise.all(c.getAnimations().map(a => a.finished));
    });
    assert.ok(await carta.locator('.help-content').evaluate(c => c.offsetHeight > 0));
    await botao.press('Enter');
    await carta.locator('.help-content').waitFor({ state: 'hidden' });
    assert.equal(await carta.locator('.help-content').evaluate(c => c.offsetHeight), 0);
    assert.equal(await carta.locator('.help-content').evaluate(c => c.inert), true);
    await botao.press('Space');
    await carta.locator('.help-content').waitFor({ state: 'visible' });
  }
  await page.goto(url);
  await conferirNovaGuia(page, url, 'inicio');
  await page.goto(`${url}/cenarios/novo?dieta=B`);
  await page.locator('#nome').fill('Cenário em edição');
  await conferirNovaGuia(page, url, 'cenarios');
  assert.equal(await page.locator('#nome').inputValue(), 'Cenário em edição');
  await page.goto(url);
  await page.goto(`${url}/ajuda#resultados`);
  await page.locator('#resultados-conteudo').waitFor({ state: 'visible' });
  assert.equal(await page.locator('#resultados-botao').getAttribute('aria-expanded'), 'true');
  await page.locator('#lotes-conteudo').waitFor({ state: 'hidden' });
  await page.evaluate(() => scrollTo(0, 0));
  await page.screenshot({ path: join(artefatos, `${navegador}-ajuda.png`), fullPage: true });
  await page.getByRole('navigation', { name: 'Seções da ajuda' })
    .getByRole('link', { name: 'Cadastrar e revisar lotes' }).click();
  await page.locator('#lotes-conteudo').waitFor({ state: 'visible' });
  await page.goBack();
  await page.locator('#resultados-conteudo').waitFor({ state: 'visible' });
  await page.locator('#resultados-botao').click();
  await page.locator('#resultados-conteudo').waitFor({ state: 'hidden' });
  await page.getByRole('navigation', { name: 'Seções da ajuda' })
    .getByRole('link', { name: 'Ler o parecer, imprimir, reproduzir e simular' }).click();
  await page.locator('#resultados-conteudo').waitFor({ state: 'visible' });
  await page.emulateMedia({ media: 'print' });
  assert.equal(await page.locator('.help-content:visible').count(), 10);
  await page.emulateMedia({ media: 'screen' });
  await page.reload();
  await page.locator('#resultados-conteudo').waitFor({ state: 'visible' });
  await page.setViewportSize({ width: 390, height: 844 });
  await conferirLinkAjuda(page);
  await page.evaluate(() => scrollTo(0, 0));
  await page.screenshot({ path: join(artefatos, `${navegador}-ajuda-mobile.png`),
    fullPage: true });
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth), false);
  await page.setViewportSize({ width: 1440, height: 1000 });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  assert.equal(await page.locator('#resultados-conteudo').evaluate(
    c => getComputedStyle(c).transitionDuration), '0s');
  await page.locator('#resultados-botao').click();
  await page.locator('#resultados-conteudo').waitFor({ state: 'hidden' });
  assert.equal(await page.locator('#resultados-conteudo').evaluate(c => c.offsetHeight), 0);
  await page.emulateMedia({ reducedMotion: 'no-preference' });
  const contexto = await page.context().browser().newContext({ javaScriptEnabled: false });
  try {
    const semJs = await contexto.newPage();
    await semJs.goto(`${url}/ajuda#lotes`);
    assert.equal(await semJs.locator('.help-content:visible').count(), 10);
    await semJs.getByRole('heading', { name: 'Cadastrar e revisar lotes' }).waitFor();
    await semJs.goto(`${url}/cenarios/novo?dieta=B`);
    await semJs.locator('#nome').fill('Rascunho sem JavaScript');
    await conferirNovaGuia(semJs, url, 'cenarios');
    assert.equal(await semJs.locator('#nome').inputValue(), 'Rascunho sem JavaScript');
  } finally { await contexto.close(); }
}

export async function conferirLinkAjuda(page) {
  const caminho = new URL(page.url()).pathname;
  const secao = caminho === '/comparar' ? 'comparacao'
    : caminho.startsWith('/execucoes/') ? 'resultados'
    : caminho.startsWith('/lotes') ? 'lotes'
    : caminho.startsWith('/alimentos') ? 'alimentos'
    : caminho.startsWith('/cenarios') ? 'cenarios'
    : caminho === '/historico' ? 'historico'
    : caminho === '/conhecimento' ? 'conhecimento' : 'inicio';
  const link = page.getByRole('link', { name: 'Ajuda nesta página', exact: true });
  assert.equal(await link.getAttribute('href'), `/ajuda#${secao}`);
  assert.equal(await link.getAttribute('target'), '_blank');
  assert.equal(await link.getAttribute('rel'), 'noopener');
  const titulo = await page.getByRole('heading', { level: 1 }).boundingBox();
  const botao = await link.boundingBox();
  assert.ok(titulo && botao, 'Título e ajuda devem estar visíveis');
  assert.ok(botao.x >= titulo.x + titulo.width, 'Ajuda deve ficar à direita do título');
  assert.ok(Math.abs((botao.y + botao.height / 2) - (titulo.y + titulo.height / 2)) < 1,
    'Título e ajuda devem estar centralizados na mesma linha');
}

async function conferirNovaGuia(page, url, secao) {
  const origem = page.url();
  const novaGuia = page.context().waitForEvent('page');
  await page.getByRole('link', { name: 'Ajuda nesta página' }).click();
  const ajuda = await novaGuia;
  try {
    await ajuda.locator(`#${secao}-conteudo`).waitFor({ state: 'visible' });
    assert.equal(ajuda.url(), `${url}/ajuda#${secao}`);
    assert.equal(page.url(), origem, 'A página original deve permanecer aberta');
    assert.equal(await ajuda.evaluate(() => window.opener), null);
  } finally { await ajuda.close(); }
}

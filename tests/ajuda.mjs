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
    assert.equal(await botao.getAttribute('aria-expanded'), 'false');
    assert.equal(await carta.locator('.help-content').isVisible(), false);
    await botao.click();
    await carta.locator('.help-content').waitFor({ state: 'visible' });
    assert.equal(await botao.getAttribute('aria-expanded'), 'true');
    await botao.press('Enter');
    await carta.locator('.help-content').waitFor({ state: 'hidden' });
    await botao.press('Space');
    await carta.locator('.help-content').waitFor({ state: 'visible' });
  }
  await page.goto(url);
  await page.getByRole('link', { name: 'Ajuda nesta página' }).click();
  await page.locator('#inicio-conteudo').waitFor({ state: 'visible' });
  await page.goto(url);
  await page.goto(`${url}/ajuda#resultados`);
  await page.locator('#resultados-conteudo').waitFor({ state: 'visible' });
  assert.equal(await page.locator('#resultados-botao').getAttribute('aria-expanded'), 'true');
  assert.equal(await page.locator('#lotes-conteudo').isVisible(), false);
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
  await page.screenshot({ path: join(artefatos, `${navegador}-ajuda-mobile.png`),
    fullPage: true });
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth), false);
  await page.setViewportSize({ width: 1440, height: 1000 });
  const contexto = await page.context().browser().newContext({ javaScriptEnabled: false });
  try {
    const semJs = await contexto.newPage();
    await semJs.goto(`${url}/ajuda#lotes`);
    assert.equal(await semJs.locator('.help-content:visible').count(), 10);
    await semJs.getByRole('heading', { name: 'Cadastrar e revisar lotes' }).waitFor();
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
}

import assert from 'node:assert/strict';
import { join } from 'node:path';

async function conferirCamposVisiveis(page, campos) {
  assert.equal(await page.locator('[name="visualizacao"], [data-simples]').count(), 0);
  assert.equal(await page.getByRole('button', {
    name: /Mostrar campos complementares|Usar preenchimento simplificado/,
  }).count(), 0);
  for (const campo of campos) {
    assert.equal(await page.locator(`#${campo}`).isVisible(), true, `${campo} deve aparecer`);
  }
}

export async function testarTemplates(page, url, artefatos, navegador) {
  await page.goto(`${url}/alimentos/novo`);
  await conferirCamposVisiveis(page, ['fornecedor', 'lote_fisico', 'data']);
  assert.equal(await page.locator('.sidebar').count(), 0);
  await page.locator('#modelo').selectOption('silagem-milho-v1');
  await page.getByRole('button', { name: 'Ver prévia do template' }).click();
  await page.locator('.template-preview').waitFor();
  assert.equal(await page.locator('#ms').inputValue(), '');
  await page.locator('#confirmar_modelo').check();
  await page.getByRole('button', { name: 'Carregar template para editar' }).click();
  await page.waitForFunction(() => document.getElementById('ms').value === '32.5');
  assert.equal(await page.locator('#ndt').inputValue(), '');
  assert.equal(await page.locator('#autorizado').inputValue(), 'DESCONHECIDO');
  await conferirCamposVisiveis(page, ['fornecedor', 'lote_fisico', 'data']);
  await page.locator('#pb').fill('8');
  await page.locator('#fornecedor').fill(`Teste ${navegador}`);
  assert.equal(await page.locator('#fornecedor').inputValue(), `Teste ${navegador}`);
  await page.getByRole('button', { name: 'Salvar revisão' }).click();
  await page.waitForURL(/cadastros\/salvo/);
  await page.goto(`${url}/alimentos`);
  // O fornecedor fica em procedência; a inspeção de persistência é feita nos testes Rust.
  assert.ok(await page.getByRole('heading', { name: 'Silagem de milho (30–35% MS)' }).count());
  await page.goto(`${url}/lotes`);
  const [cadastro] = await Promise.all([page.context().waitForEvent('page'),
    page.getByRole('link', { name: '+ Cadastrar lote', exact: true }).click()]);
  try {
    await cadastro.waitForLoadState();
    await conferirCamposVisiveis(cadastro, ['idade_meses', 'ordem_parto',
      'dispersao_peso', 'condicao_corporal']);
    assert.equal(await cadastro.locator('.sidebar').count(), 0);
    await cadastro.locator('#modelo').selectOption('jersey-estudo-v1');
    await cadastro.getByRole('button', { name: 'Ver prévia do template' }).click();
    await cadastro.locator('#confirmar_modelo').check();
    await cadastro.getByRole('button', { name: 'Carregar template para editar' }).click();
    await cadastro.waitForFunction(() => document.getElementById('peso').value === '389');
    await cadastro.locator('#peso').fill('450');
    await cadastro.locator('#animais').fill('1.5');
    await cadastro.locator('#nome').fill(`Template editado ${navegador}`);
    await cadastro.locator('#gordura').fill('4');
    await cadastro.locator('#dias_lactacao').fill('150');
    await cadastro.getByRole('button', { name: 'Salvar revisão' }).click();
    assert.equal(await cadastro.locator('#animais').getAttribute('aria-invalid'), 'true');
    await conferirCamposVisiveis(cadastro, ['idade_meses', 'ordem_parto',
      'dispersao_peso', 'condicao_corporal']);
    await cadastro.screenshot({ path: join(artefatos, `${navegador}-template-erro.png`),
      fullPage: true });
    await cadastro.locator('#animais').fill('12');
    assert.equal(await cadastro.locator('#animais').getAttribute('aria-invalid'), null);
    await cadastro.getByRole('button', { name: 'Salvar revisão' }).click();
    await cadastro.waitForURL(/cadastros\/salvo/);
  } finally { await cadastro.close(); }
  await page.goto(`${url}/cenarios/novo?dieta=B`);
  await conferirCamposVisiveis(page, ['preco_leite']);
  await page.locator('#nome').fill('Rascunho preservado');
  const valor = await page.locator('input[name^="q-"]').first().inputValue();
  const [novo] = await Promise.all([page.context().waitForEvent('page'),
    page.getByRole('link', { name: '+ Cadastrar outro lote' }).click()]);
  try {
    await novo.waitForLoadState();
    for (const [id, v] of Object.entries({ nome: `Auxiliar ${navegador}`, animais: '2',
      peso: '450', producao: '15', gordura: '4', dias_lactacao: '150' })) {
      await novo.locator(`#${id}`).fill(v);
    }
    await novo.getByRole('button', { name: 'Salvar revisão' }).click();
    await novo.waitForURL(/cadastros\/salvo/);
    await page.bringToFront();
    await page.getByRole('button', { name: 'Atualizar cadastros' }).click();
    await page.locator('#lote option').filter({ hasText: `Auxiliar ${navegador}` }).waitFor({
      state: 'attached' });
    assert.equal(await page.locator('#nome').inputValue(), 'Rascunho preservado');
    assert.equal(await page.locator('input[name^="q-"]').first().inputValue(), valor);
  } finally { await novo.close(); }
  // Estado acessível, transição e impressão do componente compartilhado.
  const secao = page.locator('.ingredient .expansivel').first();
  const botao = secao.locator('.expand-toggle');
  await page.waitForFunction(() =>
    document.querySelector('.ingredient .expansivel').classList.contains('expand-ready'));
  assert.notEqual(await secao.locator('.expand-content').evaluate(
    e => getComputedStyle(e).transitionDuration), '0s');
  await botao.click();
  await secao.locator('.expand-content').waitFor({ state: 'hidden' });
  assert.equal(await botao.getAttribute('aria-expanded'), 'false');
  assert.equal(await secao.locator('.expand-content').evaluate(e => e.inert), true);
  await botao.press('Space');
  await secao.locator('.expand-content').waitFor({ state: 'visible' });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  assert.equal(await secao.locator('.expand-content').evaluate(
    e => getComputedStyle(e).transitionDuration), '0s');
  await page.emulateMedia({ media: 'print' });
  assert.ok(await page.locator('.expand-content:visible').count());
  await page.emulateMedia({ media: 'screen', reducedMotion: 'no-preference' });
  await page.goto(`${url}/alimentos/novo`);
  await conferirCamposVisiveis(page, ['fornecedor', 'lote_fisico', 'data']);
  await page.setViewportSize({ width: 390, height: 844 });
  await page.screenshot({ path: join(artefatos, `${navegador}-template-mobile.png`),
    fullPage: true });
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth), false);
  await page.setViewportSize({ width: 1440, height: 1000 });
  const semJs = await page.context().browser().newContext({ javaScriptEnabled: false });
  try {
    const p = await semJs.newPage();
    await p.goto(`${url}/alimentos/novo`);
    await conferirCamposVisiveis(p, ['fornecedor', 'lote_fisico', 'data']);
    await p.locator('#modelo').selectOption('soja-farelo-v1');
    await p.getByRole('button', { name: 'Ver prévia do template' }).click();
    await p.locator('#confirmar_modelo').check();
    await p.getByRole('button', { name: 'Carregar template para editar' }).click();
    assert.equal(await p.locator('#ms').inputValue(), '88.46');
    await p.locator('#pb').fill('49');
    assert.equal(await p.locator('#pb').inputValue(), '49');
    assert.ok(await p.locator('.expand-content:visible').count());
  } finally { await semJs.close(); }
}

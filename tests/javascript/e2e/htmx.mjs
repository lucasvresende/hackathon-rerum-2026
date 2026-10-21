import assert from 'node:assert/strict';
import { executarCaso, observarDocumento, esperarPost, conferirPost, conferirFragmento,
  semEnvio, preencherLote } from './apoio.mjs';

export async function testarHtmx(browser, url, artefatos, navegador) {
  await executarCaso(browser, url, artefatos, navegador, 'htmx-modelos', async page => {
    await page.goto(`${url}/alimentos/novo`);
    await page.locator('#nome').fill('Rascunho de composição');
    await page.locator('#modelo').selectOption('silagem-milho-v1');
    const csrf = await page.locator('[name=csrf]').inputValue();
    await observarDocumento(page);
    const previa = esperarPost(page, '/cadastros/alimento/modelo');
    await page.getByRole('button', { name: 'Ver prévia do template' }).click();
    const r = await previa;
    assert.equal(r.status(), 200);
    await conferirPost(r, '/cadastros/alimento/modelo', csrf,
      { modelo: 'silagem-milho-v1', nome: 'Rascunho de composição' });
    assert.doesNotMatch(await r.text(), /<!DOCTYPE|<html/i);
    await conferirFragmento(page, 1);
    assert.equal(await page.locator('#nome').inputValue(), 'Rascunho de composição');
    assert.equal(await page.locator('#ms').inputValue(), '');
    assert.equal(await page.locator('.template-preview').evaluate(e => e ===
      document.activeElement), true, 'A prévia recebe foco depois da troca');
    await page.locator('#confirmar_modelo').check();
    const aplicar = esperarPost(page, '/cadastros/alimento/modelo');
    await page.getByRole('button', { name: 'Carregar template para editar' }).click();
    const resposta = await aplicar;
    assert.equal(new URL(resposta.url()).searchParams.get('acao'), 'aplicar');
    await conferirPost(resposta, '/cadastros/alimento/modelo', csrf,
      { confirmar_modelo: 'on', modelo: 'silagem-milho-v1' });
    await conferirFragmento(page, 2);
    assert.equal(await page.locator('#ms').inputValue(), '32.5');
    assert.equal(await page.locator('#ndt').inputValue(), '');
    assert.equal(await page.locator('#fornecedor').isVisible(), true);
    assert.equal(await page.locator('label').evaluateAll(labels =>
      labels.every(l => l.querySelectorAll('.required').length <= 1)), true);
    await page.locator('#ms').fill('0');
    await semEnvio(page, () => page.getByRole('button', { name: 'Salvar revisão' }).click());
    assert.equal(await page.locator('#ms').getAttribute('aria-invalid'), 'true');
    await page.locator('#ms').fill('32.5');
    assert.equal(await page.locator('#ms').getAttribute('aria-invalid'), null);
    await conferirFragmento(page, 2);
  });

  await executarCaso(browser, url, artefatos, navegador, 'htmx-erros-e-foco', async page => {
    await page.goto(`${url}/cenarios/novo?dieta=B`);
    await page.locator('#nome').fill('Rascunho com erro no servidor');
    const lote = await page.locator('#lote').inputValue();
    const csrf = await page.locator('[name=csrf]').inputValue();
    const quantidade = await page.locator('input[name^="q-"]').first().inputValue();
    // Simula uma seleção obsoleta: não existe lote com esse identificador no servidor.
    await page.locator('#lote').evaluate(e => e.add(new Option('Obsoleto', 'lote-obsoleto')));
    await page.locator('#lote').selectOption('lote-obsoleto');
    await observarDocumento(page);
    const post = esperarPost(page, '/cenarios');
    await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    const resposta = await post;
    assert.equal(resposta.status(), 200);
    await conferirPost(resposta, '/cenarios', csrf, { lote: 'lote-obsoleto' });
    await conferirFragmento(page, 1);
    await page.waitForFunction(() => document.activeElement?.id === 'erros-formulario');
    assert.equal(await page.locator('#preco_leite').isVisible(), true);
    assert.equal(await page.locator('#nome').inputValue(), 'Rascunho com erro no servidor');
    assert.equal(await page.locator('input[name^="q-"]').first().inputValue(), quantidade);
    await page.locator('#lote').selectOption(lote);
    const ingrediente = page.locator('.ingredient').filter({ has: page.locator(
      'input[name^="usar-"]:checked') }).first();
    const q = ingrediente.locator('input[name^="q-"]');
    await page.locator('#modo').selectOption('formular');
    assert.equal(await q.evaluate(e => e.required), false);
    await page.locator('#modo').selectOption('verificar');
    assert.equal(await q.evaluate(e => e.required), true);
    const botao = ingrediente.locator('.expand-toggle');
    await botao.click();
    await ingrediente.locator('.expand-content').waitFor({ state: 'hidden' });
    assert.equal(await botao.getAttribute('aria-expanded'), 'false');
    await botao.press('Space');
    assert.equal(await botao.getAttribute('aria-expanded'), 'true');
    await page.locator('#sobras').fill('100');
    await semEnvio(page, () => page.getByRole('button',
      { name: 'Calcular e salvar cenário' }).click());
    await conferirFragmento(page, 1);
  });

  await executarCaso(browser, url, artefatos, navegador, 'htmx-redirecionamento', async page => {
    await page.goto(`${url}/lotes/novo`);
    await preencherLote(page, `Contrato htmx ${navegador}`);
    const csrf = await page.locator('[name=csrf]').inputValue();
    const post = esperarPost(page, '/lotes');
    await page.getByRole('button', { name: 'Salvar revisão' }).click();
    const resposta = await post;
    await conferirPost(resposta, '/lotes', csrf,
      { nome: `Contrato htmx ${navegador}`, animais: '12', peso: '400' });
    assert.equal(resposta.status(), 200);
    assert.equal((await resposta.allHeaders())['hx-redirect'], '/cadastros/salvo?tipo=lote');
    await page.waitForURL(`${url}/cadastros/salvo?tipo=lote`);
    assert.equal(await page.locator('[data-cadastro-salvo]').count(), 1);
  });
}

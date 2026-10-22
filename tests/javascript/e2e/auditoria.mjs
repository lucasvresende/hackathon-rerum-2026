import assert from 'node:assert/strict';
import { join } from 'node:path';
import { abrirConfiguracoesCenario } from './apoio.mjs';

export async function testarAuditoria(browser, url, artefatos, navegador) {
  const ctx = await browser.newContext({ viewport: { width: 1440, height: 1000 } });
  const p = await ctx.newPage();
  const prefixo = `Auditoria ${navegador}`;
  async function preencher(campos) {
    for (const [id, valor] of Object.entries(campos)) await p.locator(`#${id}`).fill(valor);
  }
  async function salvarCadastro() {
    await p.getByRole('button', { name: 'Salvar revisão', exact: true }).click();
    await p.waitForURL(/cadastros\/salvo/);
  }
  async function aplicarModelo(tipo, id) {
    await p.goto(`${url}/${tipo}/novo`);
    await p.locator('#modelo').selectOption(id);
    await p.getByRole('button', { name: 'Ver prévia do exemplo' }).click();
    await p.locator('#confirmar_modelo').check();
    await p.getByRole('button', { name: 'Preencher com exemplo', exact: true }).click();
    await p.waitForFunction(tipo => tipo === 'lotes'
      ? document.querySelector('#peso')?.value === '550'
      : document.querySelector('#ms')?.value === '32.5', tipo);
  }
  try {
    await p.goto(`${url}/lotes/novo`);
    await p.getByRole('link', { name: 'Cancelar', exact: true }).click();
    assert.equal(await p.getByRole('link', { name: 'Cadastrar outro registro nesta guia' })
      .getAttribute('href'), '/lotes/novo');
    await p.goto(`${url}/lotes/novo`);
    await preencher({ nome: `${prefixo} lote zero`, animais: '2', peso: '550', producao: '18',
      gordura: '4', dias_lactacao: '150' });
    for (const [id, v] of Object.entries({ estado_produtivo: 'LACTACAO', adulta: 'SIM',
      gestante: 'NAO', crescimento: 'NAO', peso_estavel: 'SIM', confinada: 'SIM',
      correcao_ambiente: 'NAO', correcao_atividade: 'NAO', agua: 'SIM' })) {
      await p.locator(`#${id}`).selectOption(v);
    }
    await salvarCadastro();
    await aplicarModelo('lotes', 'ex-l03');
    await preencher({ nome: `${prefixo} lote exemplo`, animais: '3' });
    await salvarCadastro();
    await p.goto(`${url}/alimentos/novo`);
    await preencher({ nome: `${prefixo} alimento zero`, ms: '90', ndt: '70', pb: '12', ca: '0.1',
      p: '0.2', fdn: '20', amido: '45', ee: '3', fonte_id: 'EXEMPLOS-V1',
      origem: 'Hipótese sintética', metodo: 'Teste de regressão', data: '2024-02-29' });
    await p.locator('#natureza').selectOption('SINTETICA');
    for (const [id, v] of Object.entries({ origem_animal: 'NAO', nnp_adicionado: 'NAO',
      medicamento: 'NAO', autorizado: 'SIM' })) await p.locator(`#${id}`).selectOption(v);
    await salvarCadastro();
    await aplicarModelo('alimentos', 'ex-a01');
    await preencher({ nome: `${prefixo} alimento exemplo` });
    await salvarCadastro();
    const execucoes = [];
    for (const tipo of ['zero', 'exemplo']) {
      for (const modo of ['verificar', 'formular']) {
        await p.goto(`${url}/cenarios/novo?exemplo=inicial`);
        await preencher({ nome: `${prefixo} ${tipo} ${modo}` });
        await p.locator('#lote').selectOption(await p.locator('#lote option')
          .filter({ hasText: `${prefixo} lote ${tipo}` }).getAttribute('value'));
        await p.locator('#modo').selectOption(modo);
        await p.locator('#selecao-ingredientes > .expand-heading .expand-toggle').click();
        await p.locator('label[data-alimento]').filter({ hasText: `${prefixo} alimento ${tipo}` })
          .locator('input').check();
        const item = p.locator('.ingredient').filter({ hasText: `${prefixo} alimento ${tipo}` });
        await item.waitFor();
        if (modo === 'verificar') await item.locator('[name^="q-"]').fill('0.1');
        await item.locator('[name^="preco-"]').fill('1');
        await item.locator('[name^="livre-"]').check();
        await item.locator('.expand-toggle').click();
        await item.locator('[name^="origem-"]').fill('Hipótese de teste');
        await p.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
        await p.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
        execucoes.push(p.url().split('/').pop());
      }
    }
    // Mesmo lote/revisão e mesmas quantidades; a diferença normativa precisa ficar visível.
    await p.goto(`${url}/execucoes/${execucoes[0]}/duplicar`);
    await abrirConfiguracoesCenario(p);
    await p.locator('#data_aplicacao').fill('2027-10-22');
    await p.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
    await p.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
    const futura = p.url().split('/').pop();
    await p.goto(`${url}/comparar?a=${execucoes[0]}&b=${futura}`);
    await p.getByText('Data de aplicação das regras diferente', { exact: true }).waitFor();
    assert.match(await p.locator('main').innerText(), /Diferença de custo/);
    await p.screenshot({ path: join(artefatos, `${navegador}-auditoria-datas.png`),
      fullPage: true });
    // Cria o volume pela interface, sem depender da ordem dos IDs ou editar o SQLite.
    for (let i = 0; i < 101; i++) {
      await p.goto(`${url}/cenarios/novo?exemplo=inicial`);
      await p.locator('#nome').fill(`${prefixo} histórico ${i}`);
      await p.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
      await p.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
    }
    for (const javaScriptEnabled of [true, false]) {
      const contexto = await browser.newContext({ javaScriptEnabled,
        viewport: { width: 1440, height: 1000 } });
      const page = await contexto.newPage();
      try {
        await page.goto(`${url}/historico`);
        await page.locator('#filtro-lote').fill(`${prefixo} lote zero`);
        await page.getByRole('button', { name: 'Filtrar histórico' }).click();
        await page.getByRole('row').filter({ has: page.getByRole('cell', {
          name: `${prefixo} zero verificar`, exact: true,
        }) }).getByRole('link', { name: 'Fixar como referência' }).click();
        await page.getByRole('heading', { name: 'Referência fixada' }).waitFor();
        await page.locator('#filtro-lote').fill('Girolando');
        await page.getByRole('button', { name: 'Filtrar histórico' }).click();
        await page.getByRole('link', { name: 'Próxima página →' }).click();
        assert.equal(await page.locator('#filtro-lote').inputValue(), 'Girolando');
        assert.equal(await page.locator('#a').inputValue(), execucoes[0]);
        await page.screenshot({ path: join(artefatos,
          `${navegador}-auditoria-referencia-${javaScriptEnabled}.png`) });
        await page.locator('#filtro-lote').fill(`${prefixo} lote zero`);
        await page.getByRole('button', { name: 'Filtrar histórico' }).click();
        await page.waitForURL(urlAtual => urlAtual.pathname === '/historico'
          && urlAtual.searchParams.get('lote') === `${prefixo} lote zero`
          && urlAtual.searchParams.get('pagina') === null);
        assert.equal(new URL(page.url()).searchParams.get('pagina'), null);
        await page.locator('#b').selectOption(execucoes[1]);
        await page.getByRole('button', { name: 'Comparar premissas e custos' }).click();
        await page.getByRole('heading', { name: 'Comparação de cenários' }).waitFor();
        assert.match(await page.locator('main').innerText(), /Consumo calculado \(kg MS\)/);
        await page.screenshot({ path: join(artefatos,
          `${navegador}-auditoria-comparacao-${javaScriptEnabled}.png`), fullPage: true });
        await page.getByRole('heading', { name: 'Ingredientes e premissas de custo' })
          .scrollIntoViewIfNeeded();
        await page.screenshot({ path: join(artefatos,
          `${navegador}-auditoria-quantidades-${javaScriptEnabled}.png`) });
        await page.emulateMedia({ media: 'print' });
        await page.screenshot({ path: join(artefatos,
          `${navegador}-auditoria-impressao-${javaScriptEnabled}.png`), fullPage: true });
        await page.emulateMedia({ media: 'screen' });
        await page.setViewportSize({ width: 390, height: 844 });
        assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth),
          false, 'Comparação deve caber na tela móvel');
        await page.locator('h1').scrollIntoViewIfNeeded();
        await page.screenshot({ path: join(artefatos,
          `${navegador}-auditoria-mobile-${javaScriptEnabled}.png`) });
      } finally { await contexto.close(); }
    }
    console.log(`${navegador}: auditoria de cadastros, cálculos e comparação passou`);
  } catch (erro) {
    await p.screenshot({ path: join(artefatos, `${navegador}-auditoria-falha.png`),
      fullPage: true });
    console.error(await p.locator('body').innerText());
    throw erro;
  } finally { await ctx.close(); }
}

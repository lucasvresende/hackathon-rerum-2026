import assert from 'node:assert/strict';
import { join } from 'node:path';
import { executarCaso } from './apoio.mjs';

/** Diferencia análises do mesmo alimento antes da escolha, inclusive sem JavaScript. */
export async function testarRevisoesImpressao(browser, url, artefatos, navegador) {
  await executarCaso(browser, url, artefatos, navegador, 'revisoes-impressao', async page => {
    await page.goto(`${url}/alimentos`);
    const original = page.locator('.food-card').filter({ has:
      page.getByRole('heading', { name: 'Farelo de soja', exact: true }) }).first();
    const copiar = await original.getByRole('link', { name: 'Criar cópia' }).getAttribute('href');
    await page.goto(`${url}${copiar}`);
    const nome = `Revisões da soja ${navegador}`;
    await page.locator('#nome').fill(nome);
    // A ausência de data deve continuar explícita, sem inventar data para a análise.
    await page.locator('#data').fill('');
    await page.getByRole('button', { name: 'Salvar revisão', exact: true }).click();
    await page.waitForURL(/cadastros\/salvo/);
    await page.goto(`${url}/alimentos?busca=${encodeURIComponent(nome)}`);
    const revisar = await page.getByRole('link', { name: 'Criar revisão ou nova análise ↗' })
      .getAttribute('href');
    await page.goto(`${url}${revisar}`);
    await page.locator('#pb').fill('45');
    await page.locator('#data').fill('2026-10-22');
    await page.getByRole('button', { name: 'Salvar revisão', exact: true }).click();
    await page.waitForURL(/cadastros\/salvo/);

    for (const javaScriptEnabled of [true, false]) {
      const ctx = await browser.newContext({ javaScriptEnabled,
        viewport: { width: 1440, height: 1000 } });
      try {
        const p = await ctx.newPage();
        await p.goto(`${url}/cenarios/novo?exemplo=inicial`);
        if (javaScriptEnabled) {
          await p.getByRole('button', { name: 'Selecionar ingredientes', exact: true }).click();
        }
        const opcoes = p.locator('label[data-alimento]').filter({ hasText: nome });
        assert.equal(await opcoes.count(), 2);
        const textos = await opcoes.allTextContents();
        assert.equal(new Set(textos).size, 2, 'Análises distintas exigem rótulos distintos');
        assert.match(textos.join(' '), /PB 45,00% MS/);
        assert.match(textos.join(' '), /PB 48,80% MS/);
        assert.match(textos.join(' '), /Data da análise: Não informada/);
        assert.match(textos.join(' '), /Data da análise: 2026-10-22/);
        for (const opcao of await opcoes.all()) {
          const campo = opcao.locator('input');
          const id = (await campo.getAttribute('name')).slice(5);
          const texto = await opcao.innerText();
          assert.ok(texto.includes(`Composição: ${id}`));
          // Toda a identificação integra o nome acessível do controle.
          const controle = p.getByRole('checkbox', { name: texto.replace(/\s+/g, ' ').trim() });
          assert.equal(await controle.count(), 1);
        }
        await p.getByRole('searchbox', { name: 'Pesquisar ingrediente' }).fill(nome);
        if (!javaScriptEnabled) {
          await p.getByRole('button', { name: 'Atualizar ingredientes selecionados' }).click();
        }
        await p.locator('#selecao-ingredientes').screenshot({ path: join(artefatos,
          `${navegador}-revisoes-identificadas-${javaScriptEnabled}.png`) });
        await p.setViewportSize({ width: 390, height: 844 });
        assert.equal(await p.evaluate(() => document.documentElement.scrollWidth > innerWidth),
          false, 'Identificação completa da revisão não deve transbordar no celular');
        const revisada = opcoes.filter({ hasText: 'PB 45,00% MS' });
        await revisada.locator('input').check();
        if (!javaScriptEnabled) {
          await p.getByRole('button', { name: 'Atualizar ingredientes selecionados' }).click();
        }
        const id = (await revisada.locator('input').getAttribute('name')).slice(5);
        await p.locator(`[data-composicao="${id}"]`).waitFor();
        assert.equal(await opcoes.locator('input:checked').count(), 1);
      } finally { await ctx.close(); }
    }
  });
}

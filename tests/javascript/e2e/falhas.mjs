import assert from 'node:assert/strict';
import { executarCaso, observarDocumento, esperarPost, conferirPost } from './apoio.mjs';

export async function testarFalhas(browser, url, artefatos, navegador) {
  const caminho = '/cenarios/atualizar';
  for (const falha of [{ status: 403 }, { status: 500 }, { rede: true }]) {
    const nome = `htmx-falha-${falha.status || 'rede'}`;
    await executarCaso(browser, url, artefatos, navegador, nome, async page => {
      await page.goto(`${url}/cenarios/novo?dieta=B`);
      await page.locator('#nome').fill(`Preservar ${nome}`);
      await page.locator('#sobras').fill('7');
      const quantidade = page.locator('input[name^="q-"]').first();
      await quantidade.fill('5.123');
      const csrf = page.locator('[name=csrf]');
      const token = await csrf.inputValue();
      if (falha.status === 403) {
        // Recusa real do servidor; os demais casos injetam somente a falha operacional.
        await csrf.evaluate(e => { e.value = 'token-invalido'; });
      } else {
        await page.route(`**${caminho}`, route => falha.rede ? route.abort('failed')
          : route.fulfill({ status: 500, contentType: 'text/html',
            body: '<h1>Falha de teste</h1>' }));
      }
      await observarDocumento(page);
      const botao = page.getByRole('button', { name: 'Atualizar cadastros' });
      if (falha.status) {
        const post = esperarPost(page, caminho);
        await botao.click();
        const resposta = await post;
        assert.equal(resposta.status(), falha.status);
        await conferirPost(resposta, caminho, falha.status === 403 ? 'token-invalido' : token,
          { nome: `Preservar ${nome}`, sobras: '7' });
      } else { await botao.click(); }
      await page.waitForFunction(() => window.__testeHtmx.finalizadas === 1);
      assert.equal(await page.evaluate(() => window.__testeHtmx.formulario.isConnected), true);
      assert.equal(await page.evaluate(() => window.__testeHtmx.trocas), 0);
      assert.equal(await page.locator('#nome').inputValue(), `Preservar ${nome}`);
      assert.equal(await page.locator('#sobras').inputValue(), '7');
      assert.equal(await quantidade.inputValue(), '5.123');
      const erros = await page.evaluate(() => window.__testeHtmx.erros);
      if (falha.rede) {
        assert.equal(erros.length, 1);
        assert.match(erros[0], /fetch|network|load/i);
      } else { assert.deepEqual(erros, []); }
      await page.unroute(`**${caminho}`);
      await csrf.evaluate((e, v) => { e.value = v; }, token);
      const repetir = esperarPost(page, caminho);
      await botao.click();
      assert.equal((await repetir).status(), 200);
      await page.waitForFunction(() => window.__testeHtmx.finalizadas === 2);
      assert.equal(await page.evaluate(() => window.__testeHtmx.requisicoes), 2);
      assert.equal(await page.evaluate(() => window.__testeHtmx.trocas), 1);
      assert.equal(await page.locator('#nome').inputValue(), `Preservar ${nome}`);
      assert.equal(await quantidade.inputValue(), '5.123');
    }, { ...falha, caminho });
  }
}

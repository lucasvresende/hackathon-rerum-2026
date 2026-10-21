import assert from 'node:assert/strict';
import { executarCaso, observarDocumento, esperarPost, conferirPost, conferirFragmento,
  preencherLote, semEnvio } from './apoio.mjs';

export async function testarGuias(browser, url, artefatos, navegador) {
  await executarCaso(browser, url, artefatos, navegador, 'atualizacao-automatica',
    async (page, contexto) => {
      await page.goto(`${url}/cenarios/novo?dieta=B`);
      await page.locator('#nome').fill('Rascunho entre guias');
      await page.locator('#sobras').fill('7');
      const quantidade = page.locator('input[name^="q-"]').first();
      await quantidade.fill('6.789');
      const campoQuantidade = await quantidade.getAttribute('name');
      const csrf = await page.locator('[name=csrf]').inputValue();
      await observarDocumento(page);
      await page.evaluate(() => {
        window.__testeHtmx.guias = [];
        for (const tipo of ['storage', 'focus', 'blur', 'visibilitychange']) {
          const alvo = tipo === 'visibilitychange' ? document : window;
          alvo.addEventListener(tipo, e => window.__testeHtmx.guias.push({
            tipo, chave: e.key, foco: document.hasFocus(), visibilidade: document.visibilityState,
          }));
        }
      });
      const [cadastro] = await Promise.all([contexto.waitForEvent('page'),
        page.getByRole('link', { name: '+ Cadastrar outro lote' }).click()]);
      try {
        await cadastro.waitForLoadState();
        await preencherLote(cadastro, `Automático ${navegador}`, '450');
        // O observador é instalado antes de salvar: a resposta pode chegar ainda na outra guia.
        const atualizacao = esperarPost(page, '/cenarios/atualizar').catch(async erro => {
          const estado = await page.evaluate(() => ({ eventos: window.__testeHtmx.guias,
            pendente: typeof atualizarPendente === 'undefined' ? null : atualizarPendente,
            foco: document.hasFocus(), visibilidade: document.visibilityState,
            sinal: localStorage.getItem('cadastros-atualizados') }));
          throw new Error(`${erro.message}: ${JSON.stringify(estado)}`, { cause: erro });
        });
        await cadastro.getByRole('button', { name: 'Salvar revisão' }).click();
        await cadastro.waitForURL(/cadastros\/salvo/);
        await page.bringToFront();
        const resposta = await atualizacao;
        await conferirPost(resposta, '/cenarios/atualizar', csrf, {
          nome: 'Rascunho entre guias', sobras: '7', [campoQuantidade]: '6.789',
        });
        assert.equal(resposta.status(), 200);
        await conferirFragmento(page, 1);
        await page.locator('#lote option').filter({ hasText: `Automático ${navegador}` })
          .waitFor({ state: 'attached' });
        assert.equal(await page.locator('#nome').inputValue(), 'Rascunho entre guias');
        assert.equal(await quantidade.inputValue(), '6.789');
        assert.equal(await page.locator('#sobras').inputValue(), '7');
        // Os dois transportes levam o mesmo sinal; a segunda entrega não deve repetir o POST.
        await semEnvio(page, () => page.locator('#sobras').fill('7'));
        await conferirFragmento(page, 1);
        // Nenhum clique no botão de atualização: comprova storage/foco com troca real de guia.
      } finally { await cadastro.close(); }
    });
}

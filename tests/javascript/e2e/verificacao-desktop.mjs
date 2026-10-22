import assert from 'node:assert/strict';
import { join } from 'node:path';
import { executarCaso, abrirConfiguracoesCenario } from './apoio.mjs';

/** Reproduz o diagnóstico em Chromium desktop usando banco isolado pelo runner. */
export async function testarVerificacaoDesktop(browser, url, artefatos, navegador) {
  await executarCaso(browser, url, artefatos, navegador, 'verificacao-desktop',
    async (page, contexto) => {
      async function abrir(modo = 'verificar') {
        await page.goto(`${url}/cenarios/novo?exemplo=inicial&modo=${modo}`);
        await page.locator('.ingredient').first().waitFor();
      }
      async function calcular(alvo = page) {
        await alvo.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
        await alvo.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
      }
      function ingrediente(nome) {
        return page.locator('.ingredient').filter({ has: page.getByRole('heading',
          { name: nome, exact: true }) });
      }
      async function avancado(nome) {
        const item = ingrediente(nome);
        await item.locator('.expand-toggle').click();
        await item.locator('.expand-content').waitFor({ state: 'visible' });
        return item;
      }
      async function estado(esperado) {
        assert.equal(await page.locator('#situacao-fornecimento')
          .getAttribute('data-fornecimento'), esperado);
      }
      async function captura(nome, alvo = page) {
        await alvo.screenshot({ path: join(artefatos,
          `${navegador}-verificacao-${nome}.png`), fullPage: true });
      }

      await abrir();
      assert.equal(await page.getByLabel(/^Consumo \(kg MS\/animal\/dia\)/)
        .count(), 5);
      assert.doesNotMatch(await page.locator('main').innerText(), /Ignorada ao formular/);
      await abrirConfiguracoesCenario(page);
      await page.locator('#consumo_alvo').fill('25');
      await calcular();
      assert.equal(await page.getByRole('button',
        { name: 'Reproduzir versões originais' }).count(), 0);
      await estado('RESOLUCAO_PENDENTE');
      let texto = await page.locator('main').innerText();
      assert.match(texto, /R\$ 20,81/);
      assert.match(texto, /17,100 kg MS/);
      assert.match(texto, /o consumo-alvo não impõe igualdade/);
      assert.doesNotMatch(texto, /Consumo-alvo utilizado|Origem efetiva do consumo/);
      assert.doesNotMatch(texto, /solução contínua|Consumo contínuo|Oferta contínua/);
      assert.match(texto, /piso global do perfil/);
      assert.match(texto, /Consumo estimado \(kg\/animal\/dia\)/);
      assert.match(await page.locator('#fornecimento').innerText(), /Não definida/);
      await captura('base');

      await abrir();
      await abrirConfiguracoesCenario(page);
      await page.locator('#base_comparacao').selectOption('Quantidade de MN fixa');
      const silagem = await avancado('Silagem de milho');
      await silagem.locator('[name^="ms-"]').fill('20');
      await calcular();
      const referencia = await page.locator('#fornecimento tbody').innerText();
      const custo = await page.locator('.stats').innerText();
      const [copia] = await Promise.all([contexto.waitForEvent('page'),
        page.getByRole('link', { name: 'Duplicar e simular' }).click()]);
      await copia.waitForLoadState();
      await calcular(copia);
      assert.equal(await copia.locator('#fornecimento tbody').innerText(), referencia);
      assert.equal(await copia.locator('.stats').innerText(), custo);
      assert.match(await copia.locator('main').innerText(), /12,869 kg MS/);
      await captura('duplicacao-ms', copia);
      await copia.close();

      await abrir();
      const estoque = ingrediente('Silagem de milho');
      await estoque.locator('[name^="livre-"]').uncheck();
      await calcular();
      await estado('PENDENCIAS_FORNECIMENTO');
      assert.match(await page.locator('#situacao-fornecimento').innerText(),
        /Silagem de milho: estoque desconhecido/);
      await captura('pendencia');
      await abrir();
      await ingrediente('Silagem de milho').locator('[name^="livre-"]').uncheck();
      await ingrediente('Silagem de milho').locator('[name^="estoque-"]').fill('1');
      await calcular();
      await estado('FORNECIMENTO_INCOMPATIVEL');
      assert.match(await page.locator('#situacao-fornecimento').innerText(),
        /Silagem de milho: estoque após perdas e sobras/);

      await abrir();
      for (const q of await page.locator('[name^="q-"]').all()) await q.fill('0');
      await calcular();
      await estado('INVALIDO');
      assert.match(await page.locator('#situacao-fornecimento').innerText(), /MS total positiva/);

      await abrir();
      await abrirConfiguracoesCenario(page);
      await page.locator('#base').selectOption('OFERTA_MN');
      assert.equal(await page.getByLabel(/^Oferta \(kg MN\/animal\/dia\)/)
        .count(), 5);
      await page.locator('#sobras').fill('10');
      for (const item of await page.locator('.ingredient').all()) {
        const q = item.locator('[name^="q-"]');
        const ms = Number(await item.locator('[name^="ms-"]').inputValue()) / 100;
        await q.fill(String(Number(await q.inputValue()) / ms));
      }
      await calcular();
      texto = await page.locator('main').innerText();
      assert.match(texto, /15,390 kg MS/);
      assert.match(texto, /10,53000/);
      assert.match(texto, /não é consumo medido/);

      await abrir();
      await ingrediente('Milho moído').locator('[name^="q-"]').fill('8');
      await calcular();
      await estado('FORNECIMENTO_INCOMPATIVEL');
      assert.match(await page.locator('#barreiras-ruminais').innerText(), /36,7119/);
      await page.locator('#calculos-regras > .expand-heading .expand-toggle').click();
      const auditoria = page.locator('#calculos-regras-conteudo .expand-toggle');
      assert.match(await auditoria.first().innerText(), /Viola.*Amido/s);
      assert.match((await auditoria.allTextContents()).join(' '),
        /Silagem de milho: mínimo obrigatório/);

      await abrir('formular');
      await calcular();
      await estado('RESOLUCAO_PENDENTE');
      assert.match(await page.locator('[data-busca]').innerText(), /Ótimo no modelo/);
      assert.match(await page.locator('.stats').innerText(), /R\$ 18,64/);
      assert.match(await page.locator('#fornecimento').innerText(), /13,8278/);
      await abrir('formular');
      await ingrediente('Silagem de milho').locator('[name^="preco-"]').fill('2');
      await calcular();
      assert.match(await page.locator('.stats').innerText(), /R\$ 51,77/);
      assert.match(await page.locator('#fornecimento').innerText(), /5,7117/);

      await abrir('formular');
      for (const item of await page.locator('.ingredient').all()) {
        await item.locator('.expand-toggle').click();
        await item.locator('[name^="resolucao-"]').fill('0.1');
      }
      await calcular();
      await estado('FORNECIMENTO_INCOMPATIVEL');
      assert.match(await page.locator('#resultado-pratico').innerText(), /OPT-ALVO/);
      assert.doesNotMatch(await page.locator('#fornecimento').innerText(), /000000000000004/);
      await captura('otimizacao-arredondada');

      await abrir('formular');
      for (const item of await page.locator('.ingredient').all()) {
        await item.locator('[name^="livre-"]').uncheck();
        await item.locator('[name^="estoque-"]').fill('1');
      }
      await calcular();
      await estado('SEM_SOLUCAO');
      assert.match(await page.locator('[data-busca]').innerText(), /Inviável no modelo/);
      assert.equal(await page.locator('.stats').isVisible(), false);
      await captura('inviavel');
    });
}

import assert from 'node:assert/strict';
import { testarAjuda, conferirLinkAjuda } from './ajuda.mjs';
import { mkdtempSync, mkdirSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { resolve, join } from 'node:path';
import { spawn, spawnSync } from 'node:child_process';
import { chromium, firefox, webkit } from 'playwright';

const diretorio = mkdtempSync(join(tmpdir(), 'feedyourcow-browser-'));
const binario = resolve(process.env.FEEDYOURCOW_BINARIO || 'target/debug/feed-your-cow');
const url = process.env.FEEDYOURCOW_URL || 'http://127.0.0.1:3107';
const ambiente = { ...process.env, FEEDYOURCOW_BANCO: join(diretorio, 'app.sqlite3'),
  FEEDYOURCOW_ENDERECO: new URL(url).host };
const preparo = spawnSync(binario, ['demonstrar'], { env: ambiente, encoding: 'utf8' });
assert.equal(preparo.status, 0, preparo.stderr);
const servidor = spawn(binario, ['servir'], { env: ambiente, stdio: 'ignore' });
const artefatos = resolve('output/playwright');
mkdirSync(artefatos, { recursive: true });
try {
  for (let i = 0; i < 100; i++) {
    try { if ((await fetch(`${url}/saude`)).ok) break; } catch { /* servidor iniciando */ }
    await new Promise(r => setTimeout(r, 50));
  }
  for (const [nome, navegador] of Object.entries({ chromium, firefox, webkit })) {
    if (process.env.FEEDYOURCOW_NAVEGADORES &&
      !process.env.FEEDYOURCOW_NAVEGADORES.split(',').includes(nome)) continue;
    const executavel = process.env[`PW_${nome.toUpperCase()}_EXECUTAVEL`];
    const browser = await navegador.launch({ headless: true,
      ...(executavel ? { executablePath: executavel } : {}) });
    let page;
    try {
      const contexto = await browser.newContext({ viewport: { width: 1440, height: 1000 } });
      const erros = [];
      contexto.on('page', page => {
        page.on('pageerror', e => erros.push(e.message));
        page.on('console', m => { if (m.type() === 'error') erros.push(m.text()); });
      });
      await contexto.route('**/*', route => {
        if (new URL(route.request().url()).origin !== new URL(url).origin) {
          erros.push(`Recurso externo: ${route.request().url()}`);
          return route.abort();
        }
        return route.continue();
      });
      page = await contexto.newPage();
      async function conferirTela(titulo) {
        await conferirLinkAjuda(page);
        await page.getByRole('heading', { level: 1, name: titulo, exact: true }).waitFor();
        const texto = await page.locator('body').innerText();
        const slogans = ['nutrição com clareza', 'conhecimento que você pode consultar',
          'uma memória para cada decisão', 'mais confiança nas escolhas',
          'decisões explicáveis, do ingrediente'];
        assert.doesNotMatch(texto, new RegExp(slogans.join('|'), 'iu'));
        assert.equal(await page.locator('.hero, .eyebrow, .hero-art').count(), 0);
        await page.getByRole('navigation').getByRole('link', { name: 'Perfil e fontes' }).waitFor();
      }
      await testarAjuda(page, url, artefatos, nome);
      await page.goto(url);
      await conferirTela('Visão geral');
      await page.screenshot({ path: join(artefatos, `${nome}-inicio.png`), fullPage: true });
      await page.getByRole('link', { name: 'Verificar dieta', exact: true }).click();
      await conferirTela('Novo cenário');
      assert.equal(await page.locator('#modo').inputValue(), 'verificar');
      await page.goto(url);
      await page.getByRole('link', { name: 'Formular dieta', exact: true }).click();
      await conferirTela('Novo cenário');
      assert.equal(await page.locator('#modo').inputValue(), 'formular');
      await page.goto(`${url}/conhecimento`);
      await conferirTela('Perfil nutricional e fontes');
      await page.getByRole('heading', { name: 'Coeficientes do perfil' }).waitFor();
      await page.goto(`${url}/lotes/novo`);
      await conferirTela('Cadastro de lote');
      for (const [id, valor] of Object.entries({ nome: `Lote ${nome}`, animais: '20',
        peso: '400', producao: '10', gordura: '4', dias_lactacao: '150' })) {
        await page.locator(`#${id}`).fill(valor);
      }
      for (const [id, valor] of Object.entries({ estado_produtivo: 'LACTACAO', adulta: 'SIM',
        gestante: 'NAO', crescimento: 'NAO',
        peso_estavel: 'SIM', confinada: 'SIM' })) await page.locator(`#${id}`).selectOption(valor);
      await page.getByRole('button', { name: 'Salvar revisão' }).click();
      await page.waitForURL(`${url}/lotes`);
      await conferirTela('Lotes');
      await page.getByText(`Lote ${nome}`, { exact: true }).waitFor();
      await page.goto(`${url}/alimentos`);
      await conferirTela('Catálogo de alimentos');
      await page.screenshot({ path: join(artefatos, `${nome}-alimentos.png`), fullPage: true });
      await page.getByRole('link', { name: 'Criar revisão ou nova análise' }).first().click();
      await conferirTela('Composição do alimento');
      await page.locator('#metodo').fill('Revisão sintética do teste de navegador');
      await page.getByRole('button', { name: 'Salvar revisão' }).click();
      await page.waitForURL(`${url}/alimentos`);
      await page.goto(`${url}/cenarios/novo?dieta=B`);
      const lote = await page.locator('#lote option')
        .filter({ hasText: `Lote ${nome}` }).getAttribute('value');
      await page.locator('#lote').selectOption(lote);
      await page.locator('#nome').fill(`Verificação ${nome}`);
      await page.locator('#sobras').fill('100');
      await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
      await page.locator('#erros-formulario').waitFor();
      await page.waitForFunction(() => document.activeElement.id === 'erros-formulario');
      assert.equal(await page.locator('#nome').inputValue(), `Verificação ${nome}`);
      assert.equal(await page.locator('#sobras').inputValue(), '100');
      await page.screenshot({ path: join(artefatos, `${nome}-erro.png`), fullPage: true });
      await page.locator('#sobras').fill('0');
      await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
      await page.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
      await conferirTela(`Verificação ${nome}`);
      await page.getByRole('heading', { name: 'Balanço nutricional' }).waitFor();
      await page.getByRole('heading', { name: 'Quantidades de fornecimento' }).waitFor();
      await page.getByRole('heading', { name: 'Cálculos e regras aplicadas' }).waitFor();
      assert.match(await page.locator('main').innerText(), /R\$ 11,79/);
      await page.screenshot({ path: join(artefatos, `${nome}-resultado.png`), fullPage: true });
      await page.getByRole('link', { name: 'Duplicar e simular' }).click();
      await page.locator('#nome').fill(`Simulação ${nome}`);
      await page.locator('#sobras').fill('10');
      await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
      await page.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
      assert.match(await page.locator('main').innerText(), /R\$ 13,10/);
      await page.getByRole('link', { name: 'Duplicar e simular' }).click();
      await page.locator('#nome').fill(`Preço e MS ${nome}`);
      const ms = page.locator('input[name^="ms-"]').first();
      const preco = page.locator('input[name^="preco-"]').first();
      await ms.fill(String(Number(await ms.inputValue()) / 2));
      await preco.fill(String(Number(await preco.inputValue()) * 1.1));
      await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
      await page.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
      assert.match(await page.locator('.stats .stat').nth(1).innerText(), /10.800 kg MS/);
      const custo = Number((await page.locator('.stats .stat').first().innerText())
        .match(/R\$ ([0-9,]+)/)[1].replace(',', '.'));
      assert.ok(custo > 13.10, 'Mudanças de preço e MS devem alterar o custo');
      await page.goto(`${url}/cenarios/novo?modo=formular`);
      await page.locator('#nome').fill(`Formulação ${nome}`);
      await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
      await page.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
      await page.getByRole('heading', { name: 'Formulação: OTIMO_NO_MODELO' }).waitFor();
      await page.emulateMedia({ media: 'print' });
      await page.screenshot({ path: join(artefatos, `${nome}-impressao.png`), fullPage: true });
      await page.emulateMedia({ media: 'screen' });
      await page.goto(`${url}/historico`);
      await conferirTela('Histórico de cenários');
      await page.locator('#a').selectOption({ index: 1 });
      await page.getByRole('button', { name: 'Comparar premissas e custos' }).click();
      await conferirTela('Comparação de cenários');
      await page.goto(`${url}/alimentos`);
      await page.getByRole('heading', { name: 'Catálogo de alimentos' }).waitFor();
      await page.setViewportSize({ width: 390, height: 844 });
      await page.screenshot({ path: join(artefatos, `${nome}-alimentos-mobile.png`),
        fullPage: true });
      assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth),
        false, 'Catálogo deve caber na tela móvel');
      await page.goto(`${url}/`);
      await page.screenshot({ path: join(artefatos, `${nome}-mobile.png`), fullPage: true });
      const transbordou = await page.evaluate(
        () => document.documentElement.scrollWidth > innerWidth);
      assert.equal(transbordou, false);
      assert.deepEqual(erros, [], `${nome}: erros de JavaScript ou dependência externa`);
      console.log(`${nome}: cadastro, validação, fragmento, verificação, formulação, simulação, ` +
        'histórico, comparação, impressão, mobile e interface objetiva passaram');
    } catch (erro) {
      if (page) {
        await page.screenshot({ path: join(artefatos, `${nome}-falha.png`), fullPage: true });
        console.error(`${nome}: ${page.url()}\n${await page.locator('body').innerText()}`);
      }
      throw erro;
    } finally { await browser.close(); }
  }
} finally {
  servidor.kill('SIGTERM');
  await new Promise(resolve => servidor.once('exit', resolve));
  rmSync(diretorio, { recursive: true, force: true });
}

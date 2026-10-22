import { testarReutilizacao } from './reutilizacao.mjs';
import { testarVerificacaoDesktop } from './verificacao-desktop.mjs';
import { abrirConfiguracoesCenario } from './apoio.mjs';
import { testarFormulacao } from './formulacao.mjs';
import { testarResultadoOperacional } from './resultado-operacional.mjs';
import { testarCenarioSimplificado } from './cenario-simplificado.mjs';
import { testarExemplos } from './exemplos.mjs';
import { testarDicas } from './dicas.mjs';
import { testarInicio } from './inicio.mjs';
import { testarHtmx } from './htmx.mjs';
import { testarCondicionais } from './condicionais.mjs';
import { testarFalhas } from './falhas.mjs';
import { testarGuias } from './guias.mjs';
import { testarTemplates } from './templates.mjs';
import assert from 'node:assert/strict';
import { testarAjuda, conferirLinkAjuda } from './ajuda.mjs';
import { mkdtempSync, mkdirSync, rmSync, copyFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { resolve, join } from 'node:path';
import { spawn, spawnSync } from 'node:child_process';
import { chromium, firefox, webkit } from 'playwright';

const nomesSuites = ['reutilizacao', 'verificacao-desktop', 'htmx', 'condicionais',
  'falhas', 'guias',
  'inicio', 'dicas',
  'fluxos', 'exemplos', 'cenario-simplificado', 'formulacao', 'resultado-operacional'];
const suites = new Set(process.env.FEEDYOURCOW_SUITES?.split(',') || nomesSuites);
for (const suite of suites) assert.ok(nomesSuites.includes(suite), `Suíte desconhecida: ${suite}`);

const diretorio = mkdtempSync(join(tmpdir(), 'feedyourcow-browser-'));
const origemBinario = resolve(process.env.FEEDYOURCOW_BINARIO || 'target/debug/feed-your-cow');
// Congela o executável: o solver filho não deve depender de um arquivo substituído pelo Cargo.
const binario = join(diretorio, 'feed-your-cow');
copyFileSync(origemBinario, binario);
const url = process.env.FEEDYOURCOW_URL || 'http://127.0.0.1:3107';
const ambiente = { ...process.env, FEEDYOURCOW_BANCO: join(diretorio, 'app.sqlite3'),
  FEEDYOURCOW_ENDERECO: new URL(url).host };
const preparo = spawnSync(binario, ['demonstrar'], { env: ambiente, encoding: 'utf8' });
assert.equal(preparo.status, 0, preparo.stderr);
const servidor = spawn(binario, ['servir'], { env: ambiente, stdio: ['ignore', 'ignore', 'pipe'] });
let logServidor = '';
servidor.stderr.on('data', dados => { logServidor += dados.toString(); });
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
        const ids = await page.locator('[id]').evaluateAll(els => els.map(e => e.id));
        assert.equal(new Set(ids).size, ids.length, 'IDs de controles devem ser únicos');
        await conferirLinkAjuda(page);
        await page.getByRole('heading', { level: 1, name: titulo, exact: true }).waitFor();
        const texto = await page.locator('body').innerText();
        const slogans = ['nutrição com clareza', 'conhecimento que você pode consultar',
          'uma memória para cada decisão', 'mais confiança nas escolhas',
          'decisões explicáveis, do ingrediente'];
        assert.doesNotMatch(texto, new RegExp(slogans.join('|'), 'iu'));
        assert.equal(await page.locator('.hero, .eyebrow, .hero-art').count(), 0);
        if (await page.locator('.pagina-cadastro').count() === 0) {
          await page.getByRole('navigation')
            .getByRole('link', { name: 'Perfil nutricional' }).waitFor();
        } else assert.equal(await page.locator('.sidebar').count(), 0);
      }
      async function novaGuia(link) {
        const [nova] = await Promise.all([contexto.waitForEvent('page'), link.click()]);
        await nova.waitForLoadState();
        page = nova;
      }
      if (suites.has('reutilizacao')) {
        await testarReutilizacao(browser, url, artefatos, nome);
      }
      if (suites.has('verificacao-desktop')) {
        await testarVerificacaoDesktop(browser, url, artefatos, nome);
      }
      if (suites.has('resultado-operacional')) {
        await testarResultadoOperacional(browser, url, artefatos, nome);
      }
      if (suites.has('formulacao')) {
        await testarFormulacao(browser, url, artefatos, nome);
      }
      if (suites.has('cenario-simplificado')) {
        await testarCenarioSimplificado(browser, url, artefatos, nome);
      }
      if (suites.has('dicas')) await testarDicas(browser, url, artefatos, nome);
      if (suites.has('inicio')) await testarInicio(browser, url, artefatos, nome);
      if (suites.has('htmx')) await testarHtmx(browser, url, artefatos, nome);
      if (suites.has('condicionais')) await testarCondicionais(browser, url, artefatos, nome);
      if (suites.has('falhas')) await testarFalhas(browser, url, artefatos, nome);
      if (suites.has('guias')) await testarGuias(browser, url, artefatos, nome);
      if (suites.has('exemplos')) await testarExemplos(browser, url, artefatos, nome);
      if (!suites.has('fluxos')) continue;
      await testarTemplates(page, url, artefatos, nome);
      await testarAjuda(page, url, artefatos, nome);
      await page.goto(url);
      await conferirTela('Visão geral');
      for (const caminho of ['inicio', 'lotes-de-animais', 'alimentos', 'nova-dieta',
        'historico', 'perfil-nutricional', 'ajuda']) {
        const icone = page.locator(`.nav-icon[src="/static/icones/${caminho}.svg"]`);
        await icone.waitFor();
        assert.deepEqual(await icone.evaluate(element => [element.width, element.height]), [21, 21],
          'Todos os ícones do menu devem ter 21 × 21 px');
      }
      await page.screenshot({ path: join(artefatos, `${nome}-inicio.png`), fullPage: true });
      await novaGuia(page.getByRole('link', { name: 'Verificar dieta', exact: true }));
      await conferirTela('Nova dieta');
      assert.equal(await page.locator('#modo').inputValue(), 'verificar');
      await page.goto(url);
      await novaGuia(page.getByRole('link', { name: 'Formular dieta', exact: true }));
      await conferirTela('Nova dieta');
      assert.equal(await page.locator('#modo').inputValue(), 'formular');
      await page.goto(`${url}/conhecimento`);
      await conferirTela('Perfil nutricional');
      await page.getByRole('heading', { name: 'Coeficientes do perfil' }).waitFor();
      assert.equal(await page.getByRole('button', { name: /^Entender:/ }).count(), 8,
        'Cada coluna de coeficientes deve oferecer uma explicação acessível');
      await page.getByRole('heading', { name: 'Fontes de referência' }).waitFor();
      await page.getByText(/Um perfil nutricional é o conjunto versionado/).waitFor();
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
      await page.waitForURL(/cadastros\/salvo/);
      await page.goto(`${url}/lotes`);
      await conferirTela('Lotes de animais');
      await page.getByText(`Lote ${nome}`, { exact: true }).waitFor();
      await page.goto(`${url}/alimentos`);
      await conferirTela('Catálogo de alimentos');
      await page.screenshot({ path: join(artefatos, `${nome}-alimentos.png`), fullPage: true });
      await novaGuia(page.getByRole('link', { name: 'Criar revisão ou nova análise' }).first());
      await conferirTela('Composição do alimento');
      await page.locator('#metodo').fill('Revisão sintética do teste de navegador');
      await page.getByRole('button', { name: 'Salvar revisão' }).click();
      await page.waitForURL(/cadastros\/salvo/);
      await page.goto(`${url}/cenarios/novo?dieta=B`);
    await abrirConfiguracoesCenario(page);
      const lote = await page.locator('#lote option')
        .filter({ hasText: `Lote ${nome}` }).getAttribute('value');
      await page.locator('#lote').selectOption(lote);
      await page.locator('#nome').fill(`Verificação ${nome}`);
      await page.locator('#sobras').fill('100');
      await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
      assert.equal(await page.locator('#sobras').getAttribute('aria-invalid'), 'true');
      assert.equal(await page.locator('#nome').inputValue(), `Verificação ${nome}`);
      assert.equal(await page.locator('#sobras').inputValue(), '100');
      await page.screenshot({ path: join(artefatos, `${nome}-erro.png`), fullPage: true });
      await page.locator('#sobras').fill('0');
      await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
      await page.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
      await conferirTela(`Verificação ${nome}`);
      await page.getByRole('heading', { name: 'Balanço nutricional' }).waitFor();
      await page.getByRole('heading', { name: 'Quantidades de fornecimento' }).waitFor();
      await page.locator('#calculos-regras > .expand-heading > .expand-toggle')
        .waitFor({ state: 'visible' });
      for (const id of ['calculos-regras', 'entradas-preservadas', 'fontes-versoes']) {
        const botao = page.locator(`#${id} > .expand-heading > .expand-toggle`);
        assert.equal(await botao.getAttribute('aria-expanded'),
          'false', `${id} deve iniciar recolhida`);
      }
      await page.emulateMedia({ media: 'print' });
      for (const id of ['calculos-regras', 'entradas-preservadas', 'fontes-versoes']) {
        assert.ok(await page.locator(`#${id} > .expand-content`)
          .evaluate(element => element.getBoundingClientRect().height > 0),
        `${id} deve mostrar seu conteúdo na impressão`);
      }
      await page.emulateMedia({ media: 'screen' });
      assert.match(await page.locator('main').innerText(), /R\$ 11,79/);
      await page.screenshot({ path: join(artefatos, `${nome}-resultado.png`), fullPage: true });
      await novaGuia(page.getByRole('link', { name: 'Duplicar e simular' }));
      await abrirConfiguracoesCenario(page);
      await page.locator('#nome').fill(`Simulação ${nome}`);
      await page.locator('#sobras').fill('10');
      await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
      await page.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
      assert.match(await page.locator('main').innerText(), /R\$ 13,10/);
      await novaGuia(page.getByRole('link', { name: 'Duplicar e simular' }));
      await abrirConfiguracoesCenario(page);
      await page.locator('#nome').fill(`Preço e MS ${nome}`);
      await page.locator('.ingredient .expand-toggle').first().click();
      const ms = page.locator('.ingredient input[name^="ms-"]').first();
      const preco = page.locator('input[name^="preco-"]').first();
      await ms.fill(String(Number(await ms.inputValue()) / 2));
      await preco.fill(String(Number(await preco.inputValue()) * 1.1));
      await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
      await page.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
      assert.match(await page.locator('.stats .stat').nth(1).innerText(), /10,800 kg MS/);
      const custo = Number((await page.locator('.stats .stat').first().innerText())
        .match(/R\$ ([0-9,]+)/)[1].replace(',', '.'));
      assert.ok(custo > 13.10, 'Mudanças de preço e MS devem alterar o custo');
      await page.goto(`${url}/cenarios/novo?dieta=B&modo=formular`);
      await page.locator('#nome').fill(`Formulação ${nome}`);
      await page.getByRole('button', { name: 'Calcular e salvar cenário' }).click();
      await page.waitForURL(/\/execucoes\/[a-f0-9-]+$/);
      await page.getByRole('heading', { name: 'Formulação: Ótimo no modelo' }).waitFor();
      await page.emulateMedia({ media: 'print' });
      await page.screenshot({ path: join(artefatos, `${nome}-impressao.png`), fullPage: true });
      await page.emulateMedia({ media: 'screen' });
      await page.goto(`${url}/historico`);
      await conferirTela('Histórico e análise');
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
      await conferirLinkAjuda(page);
      await page.goto(`${url}/conhecimento`);
      await conferirTela('Perfil nutricional');
      await page.screenshot({ path: join(artefatos, `${nome}-cabecalho-mobile.png`),
        fullPage: true });
      assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth),
        false, 'Cabeçalho com título longo e ajuda deve caber na tela móvel');
      assert.deepEqual(erros, [], `${nome}: erros de JavaScript ou dependência externa`);
      console.log(`${nome}: cadastro, validação, fragmento, verificação, formulação, simulação, ` +
        'histórico, comparação, impressão, mobile e interface objetiva passaram');
    } catch (erro) {
      if (page) {
        await page.screenshot({ path: join(artefatos, `${nome}-falha.png`), fullPage: true });
        console.error(`${nome}: ${page.url()}\n${await page.locator('body').innerText()}`);
      }
      console.error(logServidor.slice(-8000));
      throw erro;
    } finally { await browser.close(); }
  }
} finally {
  if (servidor.exitCode === null) {
    servidor.kill('SIGTERM');
    await new Promise(resolve => servidor.once('exit', resolve));
  }
  rmSync(diretorio, { recursive: true, force: true });
}

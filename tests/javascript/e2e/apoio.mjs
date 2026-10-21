import assert from 'node:assert/strict';
import { join } from 'node:path';

/** Isola cada contrato e aceita erros de rede somente no caso que os provoca explicitamente. */
export async function executarCaso(browser, url, artefatos, navegador, nome, testar, falha) {
  const contexto = await browser.newContext({ viewport: { width: 1440, height: 1000 } });
  contexto.setDefaultTimeout(10000);
  const erros = [], respostasRuins = [], falhasRede = [], consoleErros = [];
  contexto.on('page', p => {
    p.on('pageerror', erro => erros.push(erro.message));
    p.on('console', msg => { if (msg.type() === 'error') consoleErros.push(msg.text()); });
    p.on('response', r => { if (r.status() >= 400) respostasRuins.push(r); });
    p.on('requestfailed', r => {
      // Firefox pode cancelar a busca tardia do favicon ao navegar para a página seguinte.
      const faviconCancelado = navegador === 'firefox' && r.method() === 'GET'
        && new URL(r.url()).pathname === '/static/favicon.svg'
        && r.failure()?.errorText === 'NS_BINDING_ABORTED';
      if (!faviconCancelado) falhasRede.push(r);
    });
  });
  await contexto.route('**/*', route => {
    if (new URL(route.request().url()).origin !== new URL(url).origin) {
      erros.push(`Recurso externo: ${route.request().url()}`);
      return route.abort();
    }
    return route.continue();
  });
  const page = await contexto.newPage();
  try {
    await testar(page, contexto);
    assert.deepEqual(erros, [], 'Não pode haver exceções JavaScript ou recursos externos');
    assert.equal(respostasRuins.length, falha?.status ? 1 : 0);
    for (const r of respostasRuins) {
      assert.equal(new URL(r.url()).pathname, falha.caminho);
      assert.equal(r.status(), falha.status);
    }
    assert.equal(falhasRede.length, falha?.rede ? 1 : 0,
      falhasRede.map(r => `${r.method()} ${r.url()}: ${r.failure()?.errorText}`).join('\n'));
    for (const r of falhasRede) assert.equal(new URL(r.url()).pathname, falha.caminho);
    for (const texto of consoleErros) {
      assert.ok(falha, `Erro de console inesperado: ${texto}`);
      // Chromium/WebKit registram HTTP >=400; htmx também registra o fetch abortado.
      const esperado = falha.rede
        ? /Failed to load resource|htmx: htmx:error:.*(fetch|network|load)/i
        : /Failed to load resource.*(403|500|Forbidden|Internal Server Error)/i;
      assert.match(texto, esperado);
    }
    console.log(`${navegador}: ${nome} passou`);
  } catch (erro) {
    await page.screenshot({ path: join(artefatos, `${navegador}-${nome}-falha.png`),
      fullPage: true }).catch(() => {});
    throw new Error(`${navegador} / ${nome}: ${erro.message}`, { cause: erro });
  } finally { await contexto.close(); }
}

/** Referências e eventos observados, sem substituir funções de htmx ou JavaScript da aplicação. */
export async function observarDocumento(page) {
  await page.evaluate(() => {
    window.__testeHtmx = {
      documento: document, principal: document.querySelector('main'),
      formulario: document.querySelector('#formulario'), requisicoes: 0, trocas: 0,
      finalizadas: 0, erros: [], scripts: document.scripts.length,
    };
    for (const [evento, contador] of [['htmx:before:request', 'requisicoes'],
      ['htmx:after:swap', 'trocas'], ['htmx:finally:request', 'finalizadas']]) {
      document.addEventListener(evento, () => { window.__testeHtmx[contador]++; });
    }
    document.addEventListener('htmx:error', e => {
      window.__testeHtmx.erros.push(e.detail.error?.message || String(e.detail.error));
    });
  });
}

export function esperarPost(page, caminho) {
  return page.waitForResponse(r => r.request().method() === 'POST'
    && new URL(r.url()).pathname === caminho);
}

export async function conferirPost(resposta, caminho, csrf, campos = {}) {
  const pedido = resposta.request();
  assert.equal(pedido.method(), 'POST');
  assert.equal(new URL(pedido.url()).pathname, caminho);
  const headers = await pedido.allHeaders();
  assert.equal(headers['hx-request'], 'true');
  assert.equal(headers['hx-request-type'], 'partial');
  assert.match(headers['content-type'], /^application\/x-www-form-urlencoded/);
  const dados = new URLSearchParams(pedido.postData());
  assert.equal(dados.get('csrf'), csrf);
  for (const [nome, valor] of Object.entries(campos)) assert.equal(dados.get(nome), valor, nome);
  return dados;
}

export async function conferirFragmento(page, quantidade) {
  await page.waitForFunction(n => window.__testeHtmx.finalizadas === n, quantidade);
  const estado = await page.evaluate(() => {
    const t = window.__testeHtmx;
    const ids = [...document.querySelectorAll('[id]')].map(e => e.id);
    return { documento: t.documento === document, principal: t.principal ===
      document.querySelector('main'), anteriorRemovido: !t.formulario.isConnected,
    formularios: document.querySelectorAll('#formulario').length,
    idsUnicos: new Set(ids).size === ids.length, scripts: document.scripts.length === t.scripts,
    requisicoes: t.requisicoes, trocas: t.trocas, erros: t.erros };
  });
  assert.deepEqual(estado, { documento: true, principal: true, anteriorRemovido: true,
    formularios: 1, idsUnicos: true, scripts: true, requisicoes: quantidade,
    trocas: quantidade, erros: [] });
}

/** Janela negativa delimitada: a validação é síncrona e não deve iniciar nenhum POST. */
export async function semEnvio(page, acao) {
  const ausencia = page.waitForRequest(r => r.method() === 'POST', { timeout: 400 })
    .then(() => false, erro => { if (erro.name === 'TimeoutError') return true; throw erro; });
  await acao();
  assert.equal(await ausencia, true, 'Formulário inválido disparou um POST');
}

export async function preencherLote(page, nome, peso = '400') {
  for (const [id, valor] of Object.entries({ nome, peso, animais: '12', producao: '10',
    gordura: '4', dias_lactacao: '150' })) await page.locator(`#${id}`).fill(valor);
}

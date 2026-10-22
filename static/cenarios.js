'use strict';
// Uma resposta de atualização só pode substituir a mesma versão do rascunho que enviou.
const atualizacoesCenario = new WeakMap();
const pedidosCenario = new WeakMap();
function assinaturaCenario(form) {
  return JSON.stringify([...new FormData(form).entries()]);
}
document.addEventListener('htmx:before:request', evento => {
  const ctx = evento.detail.ctx;
  const form = ctx.sourceElement?.closest('form#formulario');
  if (!form || new URL(ctx.request.action, location.href).pathname !== '/cenarios/atualizar') {
    return;
  }
  const estado = { form, assinatura: assinaturaCenario(form), repetir: false, enviar: null };
  pedidosCenario.set(ctx, estado);
  atualizacoesCenario.set(form, estado);
});
document.addEventListener('submit', evento => {
  const estado = atualizacoesCenario.get(evento.target);
  if (!estado) return;
  // Aguarda a atualização sem perder o clique em calcular, filtrar ou carregar exemplo.
  evento.preventDefault();
  evento.stopImmediatePropagation();
  estado.enviar = evento.submitter;
}, true);
document.addEventListener('htmx:before:swap', evento => {
  const estado = pedidosCenario.get(evento.detail.ctx);
  if (!estado || evento.detail.ctx.response?.status >= 400) return;
  if (estado.enviar || assinaturaCenario(estado.form) !== estado.assinatura) {
    evento.preventDefault();
    estado.repetir = true;
  }
});
document.addEventListener('htmx:finally:request', evento => {
  const ctx = evento.detail.ctx;
  const estado = pedidosCenario.get(ctx);
  if (!estado) return;
  atualizacoesCenario.delete(estado.form);
  pedidosCenario.delete(ctx);
  if (!estado.form.isConnected || ctx.response?.status >= 400) return;
  if (!estado.repetir && !estado.enviar) return;
  setTimeout(() => {
    if (!estado.form.isConnected) return;
    const botao = estado.enviar || estado.form.querySelector('#atualizar-cadastros');
    if (botao?.isConnected) botao.click();
  }, 0);
});

function normalizarPesquisa(valor) {
  return valor.normalize('NFD').replace(/[\u0300-\u036f]/g, '').toLowerCase();
}
function filtrarIngredientes() {
  const form = document.querySelector('form#formulario[action="/cenarios"]');
  if (!form) return;
  const busca = normalizarPesquisa(form.querySelector('#busca-ingredientes')?.value || '');
  const classe = form.querySelector('#classe-ingredientes')?.value || '';
  form.querySelectorAll('.selecao-classe').forEach(grupo => {
    grupo.querySelectorAll('[data-alimento]').forEach(linha => {
      linha.hidden = (classe && classe !== grupo.dataset.classe)
        || !normalizarPesquisa(linha.dataset.alimento).includes(busca);
    });
    grupo.hidden = ![...grupo.querySelectorAll('[data-alimento]')].some(l => !l.hidden);
  });
  const quantidade = form.querySelectorAll('input[name^="usar-"]:checked').length;
  const limite = form.querySelector('#modo').value === 'formular' ? 10 : 20;
  const contador = form.querySelector('#contador-ingredientes');
  contador.textContent = `${quantidade} selecionados de ${limite} permitidos.`
    + (quantidade > limite ? ' Reduza a seleção antes de calcular.' : '');
  contador.classList.toggle('field-error', quantidade > limite);
}
for (const evento of ['input', 'change']) {
  document.addEventListener(evento, e => {
    if (e.target.closest('form#formulario')) filtrarIngredientes();
  });
}
document.addEventListener('htmx:after:swap', filtrarIngredientes);
filtrarIngredientes();

// O resumo aponta a controles e grupos reais, abrindo seus ancestrais antes de focar.
document.addEventListener('click', evento => {
  const link = evento.target.closest('#erros-formulario a');
  if (!link) return;
  const alvo = document.getElementById(link.hash.slice(1));
  if (!alvo) return;
  evento.preventDefault();
  abrirSecoes(alvo);
  requestAnimationFrame(() => requestAnimationFrame(() => {
    const controle = alvo.matches('input,select,textarea') ? alvo
      : alvo.querySelector('input:not([type=hidden]),select,textarea,button');
    controle?.focus();
    alvo.scrollIntoView({ block: 'center' });
  }));
});

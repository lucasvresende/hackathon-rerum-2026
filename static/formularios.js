'use strict';
// A API nativa bloqueia envios; o servidor repete a validação antes de persistir.
function abrirSecoes(controle) {
  for (let secao = controle.closest('.expansivel'); secao;
    secao = secao.parentElement.closest('.expansivel')) {
    htmx.live.q(secao).data.aberta = true;
  }
  for (const secao of document.querySelectorAll('details')) {
    if (secao.contains(controle)) secao.open = true;
  }
}
function erroCampo(controle) {
  if (!controle.matches('input:not([type=hidden]), select, textarea')) return;
  let mensagem = '';
  const nome = controle.name;
  const numero = controle.type === 'number' && controle.value !== ''
    ? Number(controle.value) : null;
  const positivo = /^(peso|gordura|consumo_alvo|periodo_observado|oferta_observada|embalagem-)/;
  if (numero !== null && ((positivo.test(nome) || nome === 'ms' || nome.startsWith('ms-'))
    && numero <= 0)) mensagem = 'Informe um valor maior que zero.';
  if (numero !== null && /^(sobras|perda-)/.test(nome) && numero >= 100) {
    mensagem = 'Informe um percentual menor que 100%.';
  }
  if (controle.required && controle.value.trim() === '') mensagem = 'Preencha este campo.';
  controle.setCustomValidity(mensagem);
  const validade = controle.validity;
  if (!mensagem && !validade.valid) {
    if (validade.stepMismatch) mensagem = 'Informe um inteiro válido para este campo.';
    else if (validade.rangeUnderflow) mensagem = `O mínimo permitido é ${controle.min}.`;
    else if (validade.rangeOverflow) mensagem = `O máximo permitido é ${controle.max}.`;
    else if (validade.tooLong) mensagem = `Use até ${controle.maxLength} caracteres.`;
    else mensagem = 'Informe um valor válido para este campo.';
  }
  const campo = controle.closest('.field');
  if (!campo) return;
  let aviso = campo.querySelector('.field-error');
  if (!aviso) {
    aviso = document.createElement('span');
    aviso.className = 'field-error';
    aviso.id = `${controle.id}-erro`;
    controle.before(aviso);
    controle.setAttribute('aria-describedby',
      `${aviso.id} ${controle.getAttribute('aria-describedby') || ''}`.trim());
  }
  aviso.textContent = mensagem;
  aviso.hidden = !mensagem;
  if (mensagem) controle.setAttribute('aria-invalid', 'true');
  else controle.removeAttribute('aria-invalid');
}
function configurarCampos() {
  configurarCondicionais();
  document.querySelectorAll('[aria-invalid="true"]').forEach(abrirSecoes);
  const previa = document.querySelector('.template-preview');
  if (previa) previa.focus();
  document.querySelectorAll('form#formulario select, form#formulario textarea').forEach(c => {
    if (!c.id) c.id = c.name;
  });
  document.querySelectorAll('form#formulario [required]').forEach(c => {
    const label = c.labels?.[0];
    if (label && !label.querySelector('.required')) {
      const marca = document.createElement('span');
      marca.className = 'required';
      marca.setAttribute('aria-hidden', 'true');
      marca.textContent = ' *';
      label.append(marca);
    }
  });
}
let focoInvalidoPendente = false;
document.addEventListener('invalid', evento => {
  // O navegador não pode focar um controle ainda inert/recolhido durante a atualização hx-live.
  // As mensagens próprias continuam visíveis; o primeiro inválido recebe foco após a abertura.
  evento.preventDefault();
  abrirSecoes(evento.target);
  erroCampo(evento.target);
  if (focoInvalidoPendente) return;
  focoInvalidoPendente = true;
  const controle = evento.target;
  requestAnimationFrame(() => requestAnimationFrame(() => {
    focoInvalidoPendente = false;
    if (controle.isConnected) controle.focus();
  }));
}, true);
document.addEventListener('input', evento => erroCampo(evento.target));
document.addEventListener('change', evento => erroCampo(evento.target));
document.addEventListener('htmx:after:swap', configurarCampos);
configurarCampos();
document.addEventListener('submit', evento => {
  if (evento.target.id !== 'formulario'
    || evento.submitter?.hasAttribute('data-sem-validacao')) {
    return;
  }
  evento.target.querySelectorAll('input, select, textarea').forEach(erroCampo);
  if (!evento.target.reportValidity()) {
    evento.preventDefault();
    evento.stopImmediatePropagation();
  }
}, true);

// As guias compartilham somente uma sinalização; os dados continuam sendo lidos do servidor.
let atualizarPendente = false;
let ultimoSinalCadastro = null;
let canalCadastros;
try { canalCadastros = new BroadcastChannel('cadastros-atualizados'); }
catch { /* localStorage e atualização manual continuam disponíveis */ }
function receberAtualizacao(sinal) {
  if (typeof sinal !== 'string' || sinal === ultimoSinalCadastro) return;
  ultimoSinalCadastro = sinal;
  atualizarPendente = true;
  if (document.hasFocus()) atualizarCadastros();
}
window.addEventListener('storage', evento => {
  if (evento.key === 'cadastros-atualizados') receberAtualizacao(evento.newValue);
});
canalCadastros?.addEventListener('message', evento => receberAtualizacao(evento.data));
if (document.querySelector('[data-cadastro-salvo]')) {
  const sinal = String(Date.now());
  try { localStorage.setItem('cadastros-atualizados', sinal); }
  catch { /* sinalização opcional; atualização manual continua disponível */ }
  canalCadastros?.postMessage(sinal);
}
function atualizarCadastros() {
  if (!atualizarPendente) return;
  const botao = document.getElementById('atualizar-cadastros');
  if (botao) { atualizarPendente = false; botao.click(); }
}
window.addEventListener('focus', atualizarCadastros);

function configurarCondicionais() {
  const form = document.getElementById('formulario');
  if (!form) return;
  const consumo = form.querySelector('#consumo_alvo');
  if (consumo) consumo.required = false;
  form.querySelectorAll('.ingredient').forEach(ingrediente => {
    const usar = form.querySelector(`#usar-${ingrediente.dataset.composicao}`);
    const unidade = ingrediente.querySelector('select[name^="unidade-preco-"]');
    ingrediente.querySelectorAll('input:not([type=checkbox])').forEach(c => {
      const prefixo = c.name.split('-')[0];
      c.required = usar.checked && (['ms', 'perda', 'preco'].includes(prefixo)
        || (prefixo === 'q' && form.querySelector('#modo').value !== 'formular')
        || (prefixo === 'embalagem' && unidade.value === 'embalagem'));
      if (prefixo === 'q') {
        const formular = form.querySelector('#modo').value === 'formular';
        c.type = formular ? 'hidden' : 'number';
        c.min = '0'; c.step = 'any';
        const grupo = c.closest('[data-quantidade]');
        grupo.hidden = formular;
        const base = form.querySelector('#base').value;
        grupo.querySelector('label').firstChild.textContent = base === 'OFERTA_MN'
          ? 'Oferta (kg MN/animal/dia)' : base === 'CONSUMO_MN'
            ? 'Consumo (kg MN/animal/dia)' : 'Consumo (kg MS/animal/dia)';
        if (formular) { c.setCustomValidity(''); c.removeAttribute('aria-invalid'); }
      }
      if (!usar.checked) c.setCustomValidity('');
    });
  });
  for (let i = 0; i < 3; i++) {
    const analito = form.querySelector(`[name="laudo_${i}_analito"]`);
    if (!analito) continue;
    ['valor', 'unidade', 'ms', 'qualificador', 'metodo', 'documento'].forEach(sufixo => {
      const c = form.querySelector(`[name="laudo_${i}_${sufixo}"]`);
      if (c) c.required = !!analito.value.trim();
    });
  }
}
document.addEventListener('change', evento => {
  configurarCondicionais();
});

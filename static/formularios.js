'use strict';
// A API nativa bloqueia envios; o servidor repete a validação antes de persistir.
function abrirSecoes(controle) {
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
  if (!controle.validity.valid) mensagem ||= controle.validationMessage;
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
document.addEventListener('invalid', evento => {
  abrirSecoes(evento.target);
  erroCampo(evento.target);
}, true);
document.addEventListener('input', evento => erroCampo(evento.target));
document.addEventListener('change', evento => erroCampo(evento.target));
document.addEventListener('htmx:after:swap', configurarCampos);
configurarCampos();
document.addEventListener('submit', evento => {
  if (evento.target.id !== 'formulario' || evento.submitter?.hasAttribute('data-sem-validacao')) {
    return;
  }
  evento.target.querySelectorAll('input, select, textarea').forEach(erroCampo);
  if (!evento.target.reportValidity()) {
    evento.preventDefault();
    evento.stopImmediatePropagation();
  }
}, true);

// As guias compartilham somente uma sinalização; os dados continuam sendo lidos do servidor.
if (document.querySelector('[data-cadastro-salvo]')) {
  try { localStorage.setItem('cadastros-atualizados', String(Date.now())); } catch { /* opcional */ }
}
let atualizarPendente = false;
window.addEventListener('storage', evento => {
  if (evento.key !== 'cadastros-atualizados') return;
  atualizarPendente = true;
  if (document.hasFocus()) atualizarCadastros();
});
function atualizarCadastros() {
  if (!atualizarPendente) return;
  const botao = document.getElementById('atualizar-cadastros');
  if (botao) { atualizarPendente = false; botao.click(); }
}
window.addEventListener('focus', atualizarCadastros);

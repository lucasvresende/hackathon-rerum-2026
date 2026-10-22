'use strict';
// Comportamentos de apresentação; validação e decisões pertencem ao servidor.
document.addEventListener('click', (evento) => {
  const botao = evento.target.closest('[data-imprimir]');
  if (!botao) return;
  document.body.classList.toggle('impressao-ficha', botao.dataset.imprimir === 'ficha');
  window.print();
});
window.addEventListener('afterprint', () => document.body.classList.remove('impressao-ficha'));
document.addEventListener('htmx:after:swap', () => {
  const erros = document.querySelector('#erros-formulario');
  if (erros) erros.focus();
});
// O fragmento da URL abre a carta; hx-live sincroniza conteúdo e estado acessível.
window.addEventListener('hashchange', () => {
  const carta = document.getElementById(location.hash.slice(1));
  if (!carta?.matches('.help-card')) return;
  htmx.live.q(carta).data.aberta = true;
  carta.scrollIntoView({ block: 'start' });
});
// Erros operacionais não substituem o rascunho; o htmx 4 permite cancelar antes da troca.
document.addEventListener('htmx:before:swap', evento => {
  const contexto = evento.detail.ctx;
  if (contexto.response?.status >= 400
    && contexto.sourceElement?.closest('form#formulario')) evento.preventDefault();
});

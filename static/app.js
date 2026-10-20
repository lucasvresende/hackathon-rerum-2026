'use strict';
// Comportamentos de apresentação; validação e decisões pertencem ao servidor.
document.addEventListener('click', (evento) => {
  if (evento.target.closest('[data-imprimir]')) window.print();
});
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

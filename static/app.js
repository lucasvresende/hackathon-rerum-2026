'use strict';
// Comportamentos de apresentação; validação e decisões pertencem ao servidor.
document.addEventListener('click', (evento) => {
  if (evento.target.closest('[data-imprimir]')) window.print();
});
document.addEventListener('htmx:after:swap', () => {
  const erros = document.querySelector('#erros-formulario');
  if (erros) erros.focus();
});

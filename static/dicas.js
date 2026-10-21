'use strict';
// O popover nativo cuida de Escape e clique fora; aqui acrescentamos hover e posicionamento.
function iniciarDicas() {
  document.querySelectorAll('.concept-trigger:not([data-dica-pronta])').forEach(botao => {
    botao.dataset.dicaPronta = 'true';
    const dica = document.getElementById(botao.getAttribute('popovertarget'));
    let fixada = false;
    let fechamento;
    const aberta = () => dica.matches(':popover-open');
    function posicionar() {
      const origem = botao.getBoundingClientRect();
      const margem = 12;
      const largura = dica.offsetWidth;
      const altura = dica.offsetHeight;
      const esquerda = Math.max(margem,
        Math.min(origem.left, innerWidth - largura - margem));
      const abaixo = origem.bottom + 8;
      const topo = abaixo + altura <= innerHeight - margem ? abaixo : origem.top - altura - 8;
      dica.style.left = `${esquerda}px`;
      dica.style.top = `${Math.max(margem, topo)}px`;
    }
    function mostrar() {
      clearTimeout(fechamento);
      if (!aberta()) dica.showPopover();
      posicionar();
    }
    function agendarFechamento() {
      fechamento = setTimeout(() => {
        if (!fixada && document.activeElement !== botao && !dica.matches(':hover')) {
          dica.hidePopover();
        }
      }, 150);
    }
    botao.addEventListener('pointerenter', evento => {
      if (evento.pointerType === 'mouse') mostrar();
    });
    botao.addEventListener('pointerleave', agendarFechamento);
    botao.addEventListener('focus', mostrar);
    botao.addEventListener('blur', () => { fixada = false; agendarFechamento(); });
    botao.addEventListener('click', evento => {
      evento.preventDefault();
      if (fixada && aberta()) dica.hidePopover();
      else { mostrar(); fixada = true; }
    });
    dica.addEventListener('pointerenter', () => clearTimeout(fechamento));
    dica.addEventListener('pointerleave', agendarFechamento);
    dica.addEventListener('beforetoggle', evento => {
      if (evento.newState === 'closed') fixada = false;
    });
    window.addEventListener('resize', () => { if (aberta()) posicionar(); });
    document.addEventListener('scroll', () => { if (aberta()) posicionar(); }, true);
  });
}
iniciarDicas();
document.addEventListener('htmx:after:swap', iniciarDicas);

# Correções do diagnóstico de dietas — 22/10/2026

Duplicar uma simulação de MS preserva as quantidades já convertidas e usa a MS da cópia como
referência de uma nova alteração. A conversão não é reaplicada contra a composição do catálogo.
A referência acompanha o rascunho, inclusive após atualizar ingredientes ou corrigir erros.

Verificar avalia as quantidades informadas; o alvo opcional não impõe igualdade nesse modo.
O parecer identifica consumo estimado de nutrientes, piso global de FDN de forragem e linha
selecionada. Termos de ótimo e solução contínua ficam restritos à formulação. A auditoria prioriza
violações e pendências e inclui ingrediente ou comparação no título de cada regra.

A pesagem exige resolução positiva para todos os ingredientes utilizados, também na verificação.
Sem resolução, o atendimento químico continua visível, mas não há recomendação de pesagem.
Dados ausentes, entrada inválida e violações comprovadas têm resumos distintos, com os motivos.
Números práticos removem resíduos binários sem arredondar na precisão das tabelas de apresentação.
Históricos preservam suas entradas e resultados; o estado operacional é apresentado pela regra
atual de resolução, inclusive para execuções antigas sem precisão de pesagem.

As regressões estão em `tests/diagnostico_dietas.rs` e na suíte Playwright `verificacao-desktop`.
Para repetir somente o diagnóstico em Chromium desktop (1440 × 1000), com banco temporário:

```sh
cargo build --locked
FEEDYOURCOW_NAVEGADORES=chromium FEEDYOURCOW_SUITES=verificacao-desktop npm run test:e2e
```

As capturas ficam em `output/playwright/chromium-verificacao-*.png`.
A suíte cobre cópia com MS alterada, unidades, sobras, alvo, amido, estoque desconhecido/insuficiente,
MS zero, sensibilidade a preço, inviabilidade e recusa de arredondamento incompatível.

# Auditoria dos fluxos no navegador

Data: 22/10/2026. Código avaliado: `0c6af6f`. Escopo: análise, sem correções funcionais.

Foram encontrados seis problemas reproduzíveis. Os mais relevantes estão na comparação:
quantidades ausentes, datas normativas omitidas e seleção limitada à página do histórico.
Não foi demonstrado erro aritmético do motor nos casos executados.

## Ambiente e método

- Chromium headless com Playwright 1.64.1, viewport principal de 1440 × 1000.
- Build local com `cargo build --locked`; banco SQLite isolado em `/tmp`.
- Banco preparado com `demonstrar`; cadastros e cálculos adicionais feitos pela interface.
- Suíte completa existente executada com `FEEDYOURCOW_NAVEGADORES=chromium npm run test:e2e`.
- Onze testes Rust passaram: `comparacao`, `contratos_http` e `exemplos_http`.
- Screenshots foram capturadas; a comparação A/B também foi inspecionada visualmente.
- Firecrawl foi tentado, mas não conseguiu alcançar o servidor em loopback. As evidências
  funcionais são do navegador local; não houve publicação nem túnel para o banco de teste.

O wrapper Playwright CLI não ficou disponível no ambiente. Foram usados o pacote Playwright
já instalado no projeto, a suíte existente e scripts exploratórios temporários, sem novas
bibliotecas. Chromium precisou executar fora da restrição inicial do sandbox.

O executável do servidor deve ser copiado para um caminho temporário antes de rodar novos
builds. Um rebuild durante esta auditoria substituiu o executável e provocou falha ao iniciar
um solver filho. A formulação foi repetida com executável congelado; essa interferência do
ambiente não integra os seis achados.

## Cobertura dos fluxos pedidos

| Fluxo | Exercício e resultado |
| --- | --- |
| Alimento do zero | Cadastro salvo, reaberto e incluído em verificação e formulação. |
| Alimento de exemplo | Prévia, confirmação, alteração e novo cadastro; uso nos dois cálculos. |
| Lote do zero | Cadastro com condições explícitas; seleção nos dois modos de cálculo. |
| Lote de exemplo | Modelo Girolando, alteração para três animais e uso nos dois cálculos. |
| Exemplos originais | Cenário inicial, demonstrações A/B, verificação e otimização. |
| Comparação | A/B, datas distintas, recusa de outro lote e histórico com 101 novas execuções. |
| Botões | Salvar, cancelar, prévia, aplicar exemplo, atualizar, calcular, comparar e paginar. |

A suíte existente também exercitou reutilização da candidata, atualização entre guias,
remoção/restauração de exemplos, filtros, expansão, ajuda, impressão, teclado, telas móveis,
fluxos sem JavaScript e tratamento de falhas HTTP/rede. Passou integralmente no Chromium.
Essa cobertura não prova o funcionamento de todo botão em toda combinação de estados.

Os cenários exploratórios usam hipóteses sintéticas. Uma verificação bloqueada foi preservada
como bloqueada; uma formulação chegou a ótimo no modelo com quantidades aproximadas. Esses
estados distintos são esperados e não representam, por si, erro do programa.

## 1. P2 — Comparação não mostra as quantidades que explicam o custo

**Reprodução:** preparar demonstrações, abrir Histórico e análise, selecionar Dieta A como
referência e Dieta B como alternativa e clicar em Comparar premissas e custos.

**Observado:** os custos são R$ 12,56 e R$ 11,79 por animal/dia, mas os blocos de cada
ingrediente mostram as mesmas premissas. Não aparecem as quantidades efetivamente consumidas
ou ofertadas por ingrediente, nem o custo de cada participação. O usuário precisa abrir os
dois pareceres e reconstruir a comparação para entender a diferença econômica.

**Impacto:** a principal mudança entre as dietas fica invisível na comparação. A falta de
balanço nutricional lado a lado também dificulta interpretar a mudança de fornecimento.

**Local:** `src/web/comparacao/templates/componentes/comparacao_item.rs`, linhas 12–32;
`src/web/comparacao/templates/paginas/comparacao.rs`, seção de ingredientes.

**Correção sugerida:** mostrar consumo e oferta em MN/MS e custo por ingrediente a partir
do resultado de cada execução. Para formulação, usar a candidata calculada, preservando a
distinção entre solução contínua e quantidades práticas.

[Evidência da comparação A/B](imagens/auditoria-2026-10-22/comparacao-ab.png).

## 2. P2 — Comparação omite datas que determinam a vigência das regras

**Reprodução:** executar duas verificações iguais para a mesma revisão do lote, com datas de
aplicação das regras em `2026-10-22` e `2027-10-22`, e comparar os resultados.

**Observado:** as datas não aparecem na tabela e não é apresentado o aviso de critérios
não equivalentes. O cálculo de `equivalentes` ignora datas de aplicação e avaliação, assim
como versões/hashes do conhecimento preservado; compara somente o identificador do perfil,
produção, consumo, horizonte e ativação da referência UE, além da revisão do lote.

**Impacto:** diferenças no contexto normativo ficam ocultas. Neste experimento não foi
comprovada alteração numérica ou de parecer causada pela data. A omissão visual e a ausência
de aviso foram reproduzidas; o risco com cargas de conhecimento distintas decorre do código.

**Local:** `src/aplicacao/cenarios/comparacao.rs`, linhas 28–33;
`src/web/comparacao/templates/paginas/comparacao.rs`, tabela de premissas.

**Correção sugerida:** exibir as datas e versões efetivas, destacando diferenças. Definir
explicitamente quais mudanças retiram equivalência sem impedir a inspeção dos cenários.

[Evidência com datas distintas](imagens/auditoria-2026-10-22/comparacao-datas-distintas.png).

## 3. P2 — Paginação do histórico apaga os filtros

**Reprodução:** gerar 101 execuções de um lote, filtrar o histórico pelo nome desse lote e
clicar em Próxima página.

**Observado:** o destino é `/historico?pagina=2`, sem `lote` nem `data`. O campo de busca
fica vazio e voltam a aparecer execuções de outros lotes. Página anterior tem o mesmo defeito.

**Impacto:** a navegação abandona a pesquisa silenciosamente e muda o conjunto oferecido para
comparação, especialmente quando o histórico cresce.

**Local:** `src/web/historico/templates/paginas/historico.rs`, linhas 35–41.

**Correção sugerida:** preservar e codificar os filtros nos links anterior/próximo.

[Evidência após avançar](imagens/auditoria-2026-10-22/paginacao-perde-filtro.png).

## 4. P2 — Interface não permite comparar execuções de páginas diferentes

**Reprodução:** no mesmo histórico com mais de 100 execuções da mesma revisão, tentar comparar
a mais recente com uma execução que ficou na página seguinte.

**Observado:** os dois seletores contêm somente as execuções da página atual. Ao navegar,
ambas as listas são substituídas. Não existe seleção persistente nem busca independente para
referência e alternativa. O endpoint aceita IDs explícitos; a limitação está na interface.

**Impacto:** duas execuções válidas para comparação ficam inacessíveis como par pelo fluxo
normal, mesmo sendo da mesma revisão do lote.

**Local:** `src/web/historico/templates/paginas/historico.rs`, linhas 48–59.

**Correção sugerida:** seleção persistente entre páginas ou busca de execuções independente
para cada lado da comparação, com filtro por revisão compatível.

A captura do item 3 mostra a página 2 e os seletores restritos ao seu conjunto de resultados.

## 5. P2 — Data impossível da composição é aceita e preservada

**Reprodução:** cadastrar um alimento com fonte existente, origem e método preenchidos;
informar `31/02/2026` em Data da análise, salvar e reabrir a revisão.

**Observado:** o cadastro é salvo e a data impossível permanece no campo. O controle é texto;
a leitura copia a string e `validar_alimento` não valida a data.

**Impacto:** a procedência da composição pode registrar uma data civil inexistente. Não foi
demonstrado que essa data altera o cálculo nutricional.

**Local:** `src/web/cadastros/entradas.rs`, linha 135;
`src/dominio/cenario/validacao.rs`, função `validar_alimento`.

**Correção sugerida:** validar a data quando preenchida, definir formato civil e usar um
controle de data coerente com a serialização. Manter ausência como ausência, quando permitida.

[Evidência da revisão reaberta](imagens/auditoria-2026-10-22/alimento-data-invalida-persistida.png).

## 6. P3 — Cancelar lote direciona o próximo cadastro para alimento

**Reprodução:** abrir Cadastro de lote, clicar em Cancelar e depois em Cadastrar outro
registro nesta guia.

**Observado:** o link aponta para `/alimentos/novo`. O cancelamento usa `tipo=cancelado`,
perdendo a informação de que a origem era um lote. Não ocorreu salvamento indevido.

**Impacto:** quebra a continuidade do fluxo e leva o usuário ao cadastro de outra entidade.

**Local:** `src/web/cadastros/templates/formularios/cadastro.rs`, linhas 230–231;
`src/web/cadastros/templates/paginas/cadastro_salvo.rs`, escolha da rota pelo tipo.

**Correção sugerida:** transportar separadamente o tipo do cadastro e seu estado cancelado.

[Evidência após cancelar lote](imagens/auditoria-2026-10-22/cancelar-lote.png).

## Barreiras que funcionaram e limites

A comparação de lotes distintos foi recusada com explicação e sem economia. Os seis testes
Rust de comparação também confirmaram recusa de revisões distintas e ausência de economia
quando a busca não tem candidata. Fonte inexistente no alimento foi rejeitada e o formulário
preservou os valores. Não houve falha da suíte Chromium existente.

A diferença econômica A/B usa precisão interna: subtrair preços já arredondados na tela pode
produzir um centavo de diferença. Isso não foi classificado como erro aritmético.

Não foram executados Firefox/WebKit, auditoria clínica do perfil, prova matemática independente
do solver, nem teste exaustivo de todas as combinações de ingredientes e restrições.

As evidências exploratórias completas ficaram em `output/playwright/auditoria/`, ignorado pelo
Git. As cinco capturas referenciadas acima acompanham este relatório no repositório. O banco
habitual da aplicação não foi modificado. Nenhum dos seis problemas foi corrigido nesta análise.

## Resolução dos achados

Atualização de 22/10/2026: os seis achados acima foram corrigidos. O texto original permanece
como registro da inspeção anterior às mudanças.

| Achado | Comportamento corrigido | Regressão |
| --- | --- | --- |
| 1 | Quantidades, custos, revisões e balanço vêm do parecer salvo. | comparacao_quantidades |
| 2 | Datas e conhecimento completo geram motivos específicos. | comparacao |
| 3 | Filtros codificados acompanham anterior/próximo. | historico_referencia |
| 4 | Referência fixada acompanha filtros e páginas pela URL. | historico_referencia |
| 5 | Data civil validada; legado exige correção ou remoção explícita. | cadastros_auditoria |
| 6 | Cancelamento preserva lote/alimento e aceita URL antiga. | cadastros_auditoria |

A suíte Chromium completa passou, incluindo a nova suíte `auditoria`. Ela cadastra lote e
alimento do zero e por exemplo, verifica e formula dietas, testa datas distintas, cria mais
de 100 execuções e compara com referência fixada, com e sem JavaScript. Foram revistas
capturas de desktop; também são geradas evidências de mobile e impressão em
`output/playwright/`. A suíte Rust, Clippy, rustfmt e os verificadores SQLx, estruturais e
documentais passaram. Não houve migração nem alteração das execuções históricas.

[Comparação corrigida: consumo, oferta e custo por ingrediente][quantidades].
[Referência e filtro preservados na segunda página, sem JavaScript][referencia].
[Comparação em viewport móvel, sem JavaScript][mobile].

[quantidades]: imagens/auditoria-2026-10-22/comparacao-corrigida.png
[referencia]: imagens/auditoria-2026-10-22/referencia-corrigida.png

[mobile]: imagens/auditoria-2026-10-22/mobile-corrigido.png

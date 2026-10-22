# Interface e comportamento dos formulários

Este guia complementa [o uso da aplicação](../README.md#usar-a-aplicação).
Registra detalhes de seleção, atualização do rascunho e leitura do parecer.

## Fluxo ilustrado

As capturas foram geradas por testes Playwright com dados sintéticos. Os totais da página inicial
dependem do banco usado na captura e não representam os valores de uma instalação nova.

![Visão geral e atalhos principais](imagens/visao-geral.png)

![Formulário de nova dieta em tela estreita, com ingredientes selecionados](imagens/nova-dieta.png)

A [captura de um parecer com fornecimento incompatível](imagens/fornecimento.png) mostra como a
recusa de recomendação prática aparece sem ocultar o resultado contínuo da formulação.

## Preenchimento de novas dietas

As demonstrações A/B exigem confirmação antes de substituir o rascunho, inclusive sem JavaScript.
“Manter rascunho” cancela a troca. O tipo de cálculo escolhido é preservado.
Na formulação, os campos de quantidade inicial ficam ocultos; ao voltar à verificação na mesma
página, seus valores continuam disponíveis. A hipótese de disponibilidade ilimitada fica visível
junto ao estoque. A seleção oferece pesquisa, agrupamento por classe e contador do limite;
filtrar não remove ingredientes marcados. Sem JavaScript, o botão aplica a pesquisa e a seleção.

Durante uma atualização, respostas obsoletas não substituem o rascunho. Uma nova atualização
usa os valores atuais, preservando seleção, nome e preço digitados enquanto a resposta chegava.
Um clique em calcular durante essa espera é retomado após a atualização. O formulário usa
[`hx-sync`](https://four.htmx.org/reference/attributes/hx-sync) e os
[eventos htmx 4](https://four.htmx.org/reference/events) para coordenar as requisições.
O modificador [`settle:0`](https://four.htmx.org/reference/attributes/hx-swap) evita que a
restauração temporária dos atributos antigos altere valores dos controles focados na substituição.
As transições CSS dos expansíveis continuam disponíveis.

A formulação ignora quantidades iniciais no servidor. Uma pesagem parcialmente preenchida exige
os demais dados, em vez de ser descartada. A origem efetiva do consumo é derivada do alvo ou da
pesagem; comentários próprios são preservados. Textos automáticos de estimativa e taxas zero são
atualizados quando deixam de corresponder às entradas. A revalidação arredondada preserva um
parecer separado, com consumo, custo e violações, sem alterar o ótimo contínuo.
O parecer mostra o estado da busca antes dos números. Sem candidata, o diagnóstico das entradas
fica recolhido e identificado como tal. Consumo-alvo, origem efetiva e valor informado aparecem
nas premissas e nas entradas preservadas. Números e estados dos resultados usam apresentação
em português; os IDs técnicos permanecem em atributos e nos detalhes de auditoria.
Histórico, resumo inicial e comparação não apresentam o custo de uma entrada sem candidata como
custo de solução, nem calculam economia com esse valor. O histórico original permanece preservado.

Novas dietas começam com horizonte de um dia, datas civis atuais de São Paulo, consumo
estimado, quantidades em kg MS/animal/dia e hipóteses explícitas de sobras/perdas zero.
Nome, lote, ingredientes, cotação e disponibilidade exigem escolha ou informação do operador.
As opções de datas, consumo, sobras, bases, preço do leite e referência UE ficam em
“Configurações avançadas”. Os valores das demonstrações e cópias são preservados.

Em “Selecionar ingredientes”, marque as composições desejadas. O htmx atualiza o formulário
sem exigir que os demais campos estejam completos; somente os selecionados exibem editores.
Desmarcar preserva o rascunho para uma nova seleção, mas exclui o ingrediente do cálculo.
Sem JavaScript, use “Atualizar ingredientes selecionados”. As configurações avançadas ficam
expostas nessa alternativa. Preços, estoque e disponibilidade ilimitada não são presumidos.
As opções comerciais, MS, perdas, inclusão e resolução ficam no expansível de cada ingrediente.

A seleção usa o evento `change` do [htmx 4](https://four.htmx.org/reference/attributes/hx-trigger)
e a rota existente de atualização, com validação e persistência somente ao calcular.

O seletor de ingredientes usa o mesmo componente expansível dos demais dropdowns, com
transições de abertura/fechamento, teclado, estado preservado após htmx e respeito à preferência
de movimento reduzido. Sem JavaScript e na impressão, seu conteúdo fica disponível.

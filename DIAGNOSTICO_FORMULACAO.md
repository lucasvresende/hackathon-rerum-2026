# Diagnóstico do fluxo de formulação de menor custo

Inspeção original em 21/10/2026, com código-base `29bdd20`, anterior às correções abaixo.

## Correções da revisão no Chromium — 22/10/2026

A revisão adicional usou banco limpo e trinta execuções. O oráculo independente confirmou
22 soluções viáveis e três casos inviáveis. O solver e as tolerâncias foram preservados;
as correções concentram-se na recomendação, explicação e leitura operacional.

| Achado adicional | Correção e cobertura |
| --- | --- |
| Quantidades visuais incompatíveis com validação | Resolução zero indica aproximação contínua; recomendação exige resolução positiva nos ingredientes usados. Pesagens preservam casas decimais. Teste de cópia e nova verificação em `fornecimento_apresentacao.rs`. |
| Déficit exibido como zero | Diferenças relevantes pequenas usam notação científica; balanços equivalentes pela tolerância mostram zero sem sinal negativo. |
| Histórico confunde estados | Busca e condição de fornecimento são independentes; inviabilidade não é apresentada como entrada inválida. |
| Comparação omite premissas | Mostra resolução, inclusão mínima/máxima, teto de suplemento, consumo/custo práticos e escopo do custo contínuo. |
| Recusa prática escondida | Estado operacional e diagnóstico prático precedem o certificado da busca e os valores contínuos. |
| Pendências genéricas | Identificam alimento e campo; explicações de inviabilidade só atribuem causa com prova matemática conservadora. |
| A/B indisponíveis no banco limpo | Formulário oferece o cenário inicial com confirmação; A/B só aparecem com os cadastros históricos necessários. |
| Barreiras pouco acessíveis | Quadro de FDN, FDN de forragem, amido e EE apresenta valores, limites e linha efetiva. |
| Oferta prática fora da área móvel | Fornecimento por ingrediente no celular; pesagem prática precede aproximações. |
| Unidades e linguagem | Grandezas de fornecimento explicitam animal/dia e lote/horizonte; comparação e avisos usam formatação brasileira. |
| Impressão operacional longa | Ficha curta com condição de uso e quantidades, mantendo o parecer completo para auditoria. |

Decisão confirmada pelo programador: não assumir resolução de pesagem. Zero mantém somente
solução contínua aproximada. Ingrediente com quantidade zero não exige resolução positiva.
Históricos antigos mantêm seus dados; a apresentação distingue os estados sem alterar o registro.

Regressões: `tests/fornecimento_apresentacao.rs`, `tests/comparacao.rs`,
`tests/exemplo_cenario_http.rs` e `tests/javascript/e2e/resultado-operacional.mjs`.
A validação desta revisão passou em build, 72 testes Rust, Clippy, formatação, qualidade textual
e 37 consultas SQLx. A suíte completa passou no Chromium. A tentativa em Firefox parou em
um seletor ambíguo do novo teste; o seletor foi isolado por navegador. As reexecuções adicionais
não foram autorizadas, portanto Firefox/WebKit não têm validação completa desta revisão.
A ficha do exemplo com arredondamento incompatível foi gerada em uma página A4, com recusa
explícita. Artefatos ficam em `output/playwright/`, incluindo `chromium-ficha-operacional.pdf`.

## Acompanhamento das correções

O diagnóstico abaixo registra o comportamento anterior. A implementação posterior passou a
rejeitar medições parciais (F04), ignorar quantidades iniciais na formulação (parte de F10),
atualizar somente textos automáticos de origem (F07) e preservar a revalidação prática completa
(parte de F05). Regressões de domínio e entradas estão em `tests/otimizacao.rs` e
`tests/entradas_formulacao.rs`. A apresentação e os formulários receberam as correções abaixo.
F03, F05, F06 e F13 receberam apresentação específica: ausência de candidata no topo,
diagnóstico parcial recolhido, quadro prático independente e alvo/origem nas entradas.
Os rótulos e números do resultado usam português, preservando os estados canônicos no histórico.

F01, F02, F08, F09, F10, F11 e F12 receberam correções no formulário: atualização de rascunho
protegida contra respostas obsoletas, confirmação processada no servidor, modo preservado,
erros associados a grupos acessíveis, quantidade inicial oculta na formulação, disponibilidade
visível e seleção pesquisável com classes e contador. A regressão de F01 inclui edição do preço
com foco mantido no campo, além de seleção rápida e cálculo solicitado durante a atualização.
F03 também protege custos no histórico, resumo e comparação. F07 cobre a troca de consumo
informado de volta para estimado. Os comentários próprios do operador não são sobrescritos.

Cobertura adicionada: `tests/exemplo_cenario_http.rs`, `tests/comparacao.rs`,
`tests/javascript/e2e/formulacao.mjs`, além dos testes de domínio e entradas citados acima.
Os treze itens seguintes descrevem as falhas originais para reprodução e rastreabilidade.

Validação após as correções: build com lockfile, 67 testes Rust, Clippy com avisos tratados como
erros, formatação e qualidade textual passaram. A suíte completa de interface passou em Chromium,
Firefox e WebKit. As regressões de formulação também cobrem pesquisa e confirmação sem JavaScript,
onze ingredientes, respostas atrasadas e cálculo pendente nos três navegadores.
Capturas finais estão em `output/playwright/`: `*-selecao-preservada.png`,
`*-medicao-corrigir.png`, `*-sem-candidata.png` e `*-pratico-mobile.png`.

Foram executados 23 cenários em Chromium, Firefox e WebKit, além de inspeção móvel nos três
navegadores, seis ensaios de seleção concorrente e casos adicionais de navegação, cópia e estoque.
Os 23 cenários tiveram os mesmos estados de busca e validação nos três navegadores.
Os resultados são observações desta rodada, não uma prova de ausência de outros defeitos.

## Ambiente e reprodução

- Build: `cargo build --locked`, concluído com sucesso.
- Automação: Playwright 1.64.1, navegadores em modo headless, requisições HTTP reais ao Axum.
- Desktop: 1440 × 1000; móvel: 390 × 844; capturas inspecionadas visualmente.
- Banco isolado: `/tmp/diagnostico-formulacao.sqlite3`, preparado pelo comando `demonstrar`.
- Servidor: `http://127.0.0.1:3123`; o histórico existente não recebeu essas execuções.
- Entrada principal: `/cenarios/novo?dieta=B&modo=formular`.
- Lote: demonstração, 12 animais, 400 kg, produção de 10 kg de leite/animal/dia.
- Ingredientes: silagem de milho, milho moído, farelo de soja e mistura mineral das fixtures A/B.
- Referência: 10,8 kg MS/animal/dia; horizonte inicial de sete dias.

Os arquivos locais estão em `output/playwright/diagnostico-formulacao/`, ignorado pelo Git:

- `chromium.json`, `firefox.json`, `webkit.json`: resultados por caso e erros de console.
- `resumo.json`: consolidação dos 72 registros, incluindo as três inspeções móveis.
- `selecao.json`: ensaios de seleção rápida, com e sem atraso de resposta.
- `extra.json`: navegação, perda de rascunho, onze ingredientes, estoque e cópia.
- `<navegador>-<caso>.txt` e `.png`: texto visível e capturas dos casos principais.
- `diagnostico-*.cjs`: roteiros exploratórios usados nesta rodada, com caminhos locais.

As evidências locais dependem deste workspace. As descrições abaixo permitem reproduzir os
problemas sem esses arquivos. Para repetir os roteiros, manter o servidor e o banco acima.

## Problemas de maior prioridade

### F01 — P1: seleção rápida perde o segundo ingrediente

**Reprodução:** abrir o seletor, atrasar a resposta de `/cenarios/atualizar` em 350 ms e marcar
dois ingredientes inicialmente desmarcados antes de a primeira resposta chegar.

**Observado:** após a substituição do formulário, apenas o primeiro permanece marcado.
Reproduzido nos três navegadores. Sem atraso artificial, ambos permaneceram selecionados.
O atraso é uma condição controlada do ensaio; não foi medido como latência normal do servidor.

**Impacto:** o conjunto submetido ao solver pode diferir da escolha recém-feita pelo operador.
**Local:** `src/web/templates/formularios/ingredientes.rs:26`, atualização de todo o formulário.
**Direção de correção:** sincronizar alterações pendentes com as respostas antes de substituir
o formulário. Validar também edições de preço durante uma atualização em andamento.
**Critério:** RNF-03, preservação de contexto. Evidência: `selecao.json`.

### F02 — P1: demonstração sobrescreve rascunho sem confirmação

**Reprodução:** alterar nome e preço no formulário; clicar em “Demonstração A”.
**Observado:** navegação imediata para o exemplo A, descartando as alterações; nenhum diálogo.
**Impacto:** perda de trabalho sem oportunidade de cancelar.
**Local:** `src/web/templates/formularios/cenario.rs:51`, links GET diretos.
**Direção de correção:** proteger a substituição de rascunho alterado, incluindo alternativa
sem JavaScript. **Critério:** RF-19 exige confirmação quando há perda de alterações.
**Evidência:** `extra.json`, caso `perda-rascunho`.

### F03 — P1: busca sem candidata apresenta valores da dieta inicial como resultado

**Reprodução:** usar a demonstração B, informar consumo-alvo de 1 ou 5 kg MS e calcular.
**Observado:** busca `INVIAVEL_NO_MODELO`, mas o topo mostra `AVALIADO`, 10,800 kg MS e
R$ 11,79/animal/dia. Nutrientes aparecem como atendidos e há uma tabela de fornecimento.
Esses números pertencem às quantidades iniciais da demonstração, não a uma solução encontrada.

O mesmo reaproveitamento ocorre com estoque zero, estoque desconhecido e mínimos inviáveis.
Existe um aviso de ausência de recomendação prática, mas ele não identifica a origem dos números.
O estado da busca fica depois das tabelas, distante do resumo inicial.

**Impacto:** interpretação equivocada de custo, atendimento e quantidades da formulação.
**Locais:** `src/dominio/otimizacao.rs:232` devolve o diagnóstico inicial;
`src/web/templates/componentes/resultado.rs:41` apresenta-o no mesmo layout da candidata.
**Direção de correção:** destacar ausência de candidata no topo e identificar explicitamente
qualquer diagnóstico parcial da entrada. Não apresentar seu custo como custo da solução.
**Critérios:** RF-10, RF-13, RN-19, RNF-03.
**Evidências:** `chromium-consumo-1.txt`, `chromium-estoque-desconhecido.png`.

### F04 — P1: medição parcial é descartada silenciosamente

**Reprodução:** abrir consumo observado, preencher somente MS das sobras com 10 kg e calcular.
**Observado:** a aplicação calcula um ótimo usando a referência de 10,8 kg MS; não pede os
demais campos e não informa que descartou o dado de sobras digitado.
**Causa confirmada:** `src/web/entradas/cenario.rs:103` cria a medição somente quando a oferta
observada está preenchida. Sobras ou período preenchidos isoladamente não ativam a validação.
**Impacto:** intenção de usar observações é convertida em cálculo por referência sem aviso.
**Direção de correção:** qualquer campo da medição preenchido deve exigir o conjunto completo.
**Critérios:** RF-07, RF-08, RN-02. Evidência: `*-medicao-parcial.txt` nos três navegadores.

## Outros defeitos funcionais e de explicação

### F05 — P2: revalidação prática não explica as violações encontradas

**Reprodução:** definir resolução de oferta de 1 kg MN para todos os ingredientes.
**Observado:** mistura mineral passa de 0,0839 para 0 kg MN; silagem vai para 28 e soja para 1.
O sistema bloqueia corretamente o fornecimento prático e mantém o ótimo contínuo.
Entretanto, o balanço e as dimensões continuam exibindo os números contínuos como atendidos;
não há balanço prático, custo prático nem identificação das restrições violadas pelo arredondamento.

**Causa:** `src/dominio/otimizacao.rs:220` calcula `verificada`, mas preserva apenas o booleano
de validade e um aviso genérico, descartando o diagnóstico detalhado da candidata prática.
**Direção de correção:** preservar e apresentar a revalidação prática separada da contínua.
**Critérios:** RF-11, RF-13, RN-20. Evidência: `*-resolucao-1.txt` e `.png`.

### F06 — P2: consumo-alvo efetivo não aparece nas entradas preservadas da tela

**Reprodução:** informar consumo-alvo de 1 ou 20 kg MS, calcular e abrir as entradas preservadas.
**Observado:** a interface mostra consumo de referência e consumo da dieta, mas não apresenta
o campo de consumo-alvo informado. No caso inviável, ambos os números visíveis são 10,8 kg MS.
**Causa:** o dado existe em `Cenario`, porém não é renderizado pelo componente `Entradas`.
**Impacto:** não se consegue auditar pela tela qual igualdade de consumo foi enviada ao solver.
**Direção de correção:** exibir alvo utilizado, valor informado/observado e origem efetiva.
**Critérios:** RF-13, RN-07, RN-23. Local: `src/web/templates/componentes/entradas.rs`.

### F07 — P2: mudanças deixam premissas de origem contraditórias

**Reprodução:** no exemplo B, mudar consumo-alvo para 20; em outro cenário, sobras para 10%.
**Observado:** a origem continua “Referência Tabela B; consumo estimado.” no primeiro;
no segundo, aparece “10.00% · Hipótese explícita: zero sobras; não há medição.”.
**Impacto:** o histórico contém valores e justificativas conflitantes sem sinalização.
**Direção de correção:** separar modo/origem estruturados do comentário livre e sinalizar a
necessidade de revisar hipóteses ao alterar os valores a que se referem.
**Critérios:** RF-06, RF-13, RN-02. Evidências: `*-consumo-20.txt`, `*-perdas-sobras-10.txt`.

### F08 — P2: preencher demonstração troca formulação por verificação

**Reprodução:** abrir “Formular dieta” e clicar em “Demonstração B”.
**Observado:** o modo passa para “Verificar as quantidades informadas”.
**Impacto:** o próximo cálculo pode apenas avaliar as quantidades iniciais, embora a tarefa
iniciada pelo operador fosse buscar menor custo.
**Causa:** os links omitem `modo=formular`; o exemplo é carregado como verificação.
**Direção de correção:** preservar o modo escolhido ou explicitar a mudança na ação.
**Local:** `src/web/templates/formularios/cenario.rs:51`. Evidência: `extra.json`.

### F09 — P2: erros de grupo não levam aos controles a corrigir

**Reprodução A:** informar alvo 10,8 e medição com oferta 150, sobras 10 e período 1.
**Observado:** há erro de incompatibilidade no resumo, mas a seção da medição volta recolhida,
sem mensagens nos campos. A âncora `#medicao` não abre a seção.
**Reprodução B:** selecionar onze ingredientes e preencher preços e origens.
**Observado:** servidor rejeita corretamente, mas o resumo aponta para `#itens`, que não existe.
**Impacto:** o operador precisa localizar manualmente a área e interpretar identificadores internos.
**Locais:** `configuracoes_cenario.rs:67` fixa a medição fechada; `resumo_erros.rs` gera âncoras
diretamente dos identificadores de validação.
**Direção de correção:** mapear grupos para seções reais, abri-las e associar erros aos controles.
**Critérios:** RF-07, RNF-03. Evidências: `*-medicao-conflito.png`, `extra-onze-ingredientes.txt`.

## Problemas de interface e continuidade

### F10 — P2: quantidade dita ignorada continua editável e pode impedir a formulação

**Reprodução:** preencher -1 em “Quantidade na base escolhida” no modo formular.
**Observado:** a validação bloqueia o envio apesar de o próprio campo dizer “Ignorada ao formular”.
Vazio ou zero produzem normalmente a mesma candidata ótima.
**Direção de correção:** retirar esse controle do fluxo de formulação ou esclarecer sua função;
alinhar a validação ao comportamento escolhido. **Local:** `ingredientes.rs:62`.
**Evidência:** `*-quantidade-negativa-ignorada.txt`; no móvel há quatro controles sem utilidade.

### F11 — P2: estoque visível conflita com hipótese escondida

**Reprodução:** no exemplo B, informar estoque 100 kg nos controles visíveis e calcular.
**Observado:** erro “Escolha estoque ou hipótese sem limite”. A hipótese já marcada fica
recolhida nas configurações avançadas de cada ingrediente; a mensagem aparece no estoque.
**Direção de correção:** apresentar estoque conhecido, desconhecido e ilimitado como escolha
explícita junto ao próprio estoque. Não exigir descobrir e desmarcar uma opção distante.
**Evidência:** `*-estoque-sem-limite-contraditorio.txt`.

### F12 — P3: seleção não oferece busca nem organização por classe

**Observado:** o seletor apresenta uma lista única de composições, com nomes e `SINTETICA`.
Não há pesquisa, filtro por classe nem contador de selecionados/limite.
É possível preencher onze ingredientes antes de descobrir o limite no envio.
**Impacto:** maior esforço de seleção e preenchimento desnecessário; piora com novas análises.
**Direção de correção:** pesquisa e agrupamento por classe, contador e aviso antecipado do limite.
**Critério:** RNF-14. Local: `src/web/templates/formularios/ingredientes.rs:30`.

### F13 — P3: números e estados técnicos prejudicam a leitura em português

**Observado:** “10.800 kg MS”, “10.00%”, “INVIAVEL_NO_MODELO”, “ProvenOptimal” e “NAO AVALIADO”.
O ponto em 10.800 pode ser interpretado como separador de milhares no contexto brasileiro.
**Direção de correção:** formatar números na apresentação em pt-BR e traduzir rótulos, mantendo
identificadores técnicos nos detalhes de auditoria. **Critérios:** RNF-03 e RNF-14.
**Local:** `src/web/templates/componentes/resultado.rs:51`. Evidência: `chromium-base.png`.

## Matriz de resultados e verificações que funcionaram

Valores abaixo são por animal/dia. O custo exibido nos casos sem candidata é o diagnóstico
da entrada, conforme F03, e não foi tratado como custo de solução.

| Variação | Resultado nos três navegadores |
| --- | --- |
| Demonstração B, formulação | Ótimo; 10,8 kg MS; R$ 9,63 |
| Quantidades iniciais vazias ou zero | Mesma candidata ótima |
| Preços duplicados | Mesmas quantidades; custo não arredondado duplica; R$ 19,27 |
| Todos os preços zero, mantendo origem | Ótimo; R$ 0,00 |
| Preços equivalentes por tonelada ou embalagem de 50 kg | Mesmo ótimo; R$ 9,63 |
| Alvo 1 ou 5 kg MS | Inviável nos cinco subproblemas |
| Alvo 10,8 kg MS | Ótimo; R$ 9,63 |
| Alvo 20 kg MS | Ótimo; R$ 14,29; aviso de discrepância |
| Alvo 50 kg MS | Ótimo; R$ 35,71; aviso de discrepância |
| Estoques zero, sem hipótese ilimitada | Inviável |
| Estoques desconhecidos, sem hipótese ilimitada | Busca inconclusiva |
| Estoque informado e hipótese ilimitada simultâneos | Validação rejeita |
| Mínimo 3 e máximo 2 kg MS para soja | Validação rejeita e preserva os valores |
| Mínimo 20 kg MS em cada ingrediente | Inviável |
| Sobras 10% e perdas 10% em todos | Ótimo; R$ 11,89, fator de custo 1/0,81 |
| MS da silagem reduzida pela metade | Ótimo; R$ 16,61 |
| Resolução de oferta 1 kg MN | Ótimo contínuo; fornecimento prático bloqueado |
| Consumo-alvo e medição completos simultâneos | Validação rejeita |
| Apenas sobras observadas preenchidas | Aceita e descarta a medição: F04 |

Ensaios extras no Chromium: 1.000 kg MN por ingrediente suportam o horizonte de um dia;
sete e trinta dias são inviáveis. Selecionar onze ingredientes é rejeitado no servidor.
Duplicar um ótimo e trocar para verificação recupera as entradas originais: custo R$ 11,79,
em vez das quantidades otimizadas de R$ 9,63. Isso preserva o cenário original, mas falta uma
ação explícita para usar a solução encontrada como dieta a verificar.

Não houve exceções JavaScript nem mensagens de erro no console nos 69 cenários da matriz.
Não houve transbordamento horizontal da página nas três inspeções móveis; as tabelas do
resultado usam rolagem interna. A apresentação móvel foi inspecionada no Chromium.

O aceite de 50 kg MS não foi classificado como defeito: o modelo permite alvo positivo informado
e registra aviso, sem alegar capacidade fisiológica. A ausência de laudos continuou identificada
como inconclusiva; peNDF e micronutrientes permaneceram não avaliados.

Não foram exercitados nesta rodada: interrupção real por dez segundos, indisponibilidade do banco,
conjunto integral de fronteiras nutricionais e todos os ingredientes regulatoriamente excluídos.
Não foi feita validação bibliográfica ou clínica; o foco foi o fluxo implementado e seus requisitos.

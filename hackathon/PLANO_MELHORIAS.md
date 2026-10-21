# Plano de implementação das melhorias de interface

> Atualização de 21/10/2026: o modo simplificado foi retirado. Os formulários exibem todos
> os campos; templates editáveis e seções expansíveis permanecem. As menções abaixo
> registram o planejamento anterior e não definem o comportamento vigente.

Data: 20/10/2026. Estado: substituído pelo [plano revisado](PLANO_MELHORIAS_V2.md).

Base: [sugestões](NOTAS_MELHORIAS.md), [requisitos](../REQUISITOS.md) e código atual.
As decisões abaixo precisam ser alinhadas antes de implementar as etapas correspondentes,
conforme a instrução 2 de [AGENTS.md](../AGENTS.md).

## 1. Diagnóstico do código atual

- `src/web/componentes/estrutura.rs`: `Pagina` sempre inclui o menu lateral. `Campo` já
  oferece `required`, `aria-invalid`, dica e mensagem de erro, mas não mostra asterisco;
  a mensagem individual aparece abaixo do controle.
- `src/web/cadastros.rs`: criação e revisão reutilizam o mesmo formulário. Os POSTs validam
  no servidor e devolvem os valores digitados quando há erro; o sucesso redireciona à listagem.
- `src/web/componentes/cadastro.rs`: lotes e alimentos compartilham parte da renderização,
  mas seletores não usam o componente `Campo` e precisam da mesma apresentação de erros.
- `src/dominio/validacao.rs`: alguns erros são associados a grupos, como `nutrientes`,
  ou a nomes diferentes dos controles, como `rotulo`. Há também campos exigidos pelo servidor,
  como `origem` do alimento, sem indicação equivalente no formulário.
- `src/web/cenarios.rs`: o novo cenário já pode receber dados da demonstração, incluindo
  composições sintéticas, a partir do primeiro lote encontrado. Tornar essa escolha explícita.
- `static/app.css` e `componentes/ajuda.rs`: as cartas de ajuda já têm animação de 180 ms,
  movimento reduzido, impressão e controle acessível por hx-live.
- Outros conteúdos expansíveis usam `details/summary` em cadastros, cenários, catálogo,
  perfil, entradas e resultados. Eles precisam de uma solução reutilizável.
- Existem testes Rust e fluxos Playwright em `tests/navegadores.mjs` e `tests/ajuda.mjs`.

## 2. Decisões que precedem a implementação

### D1 — Nova guia: decisão aprovada

O programador escolheu nova guia com página de formulário sem menu nesta conversa.
Usar links `target="_blank"` para criação e revisão de lotes, alimentos e cenários.

“Sem outras opções” será interpretado como ausência do menu de navegação da aplicação,
mantendo salvar, cancelar e ajuda contextual. Confirmar essa interpretação.
Uma página não controla a barra de ferramentas nem garante o formato de janela do navegador.

Ainda definir confirmação de sucesso, cancelamento, retorno e atualização da seleção de origem,
sem depender de fechamento programático da guia. Proposta: confirmar o cadastro na nova guia
e atualizar as opções na origem ao retornar, preservando os demais valores digitados.
O resultado do cenário deve abrir o parecer na própria guia após cálculo bem-sucedido.

### D2 — Exemplos e modo simplificado

Confirmar quais modelos de lote, alimentos e cenário serão oferecidos e seus valores.
Separar três conceitos: exemplo visual, preenchimento solicitado e dado efetivamente declarado.

Proposta para aprovação:

- Lote: exemplo numérico com 20 animais, 450 kg, 15 kg de leite/dia, gordura de 4% e
  150 dias em lactação. São valores ilustrativos para digitação, sem recomendação de manejo.
- Alimentos: aproveitar inicialmente silagem, milho, soja e mineral da demonstração,
  sempre identificados como sintéticos. Médias tabeladas reais exigem fonte e versão próprias.
- Cenário: escolher lote e versões dos alimentos explicitamente; fornecer exemplos de unidades
  e parâmetros, sem aplicar silenciosamente o primeiro lote ou a dieta de demonstração.
- Modo simplificado: mostrar os campos essenciais e recolher os complementares, mantendo
  acessíveis as informações necessárias para enquadramento, procedência e restrições.

Não pré-confirmar gestação, crescimento, estabilidade de peso, autorização, origem animal,
NNP, medicamentos ou ausência de contaminantes. Não inventar fonte, laudo, preço ou estoque.
Sobras/perdas zero e disponibilidade sem limite só entram como hipóteses explícitas.
Confirmar a apresentação dessas hipóteses antes de adotar preenchimentos automáticos.

### D3 — Dados inválidos e animais fora do perfil

Bloquear envio de valores malformados ou inconsistentes, mas preservar a distinção entre
erro de preenchimento e condição `FORA_DO_PERFIL`, conforme RF-07 e RN-01/RN-02.
Por exemplo: peso negativo é inválido; peso de 800,01 kg excede o perfil do cálculo.
O cadastro aceita peso positivo, enquanto o perfil aplica a faixa operacional de 300–800 kg e
identifica a extrapolação controlada nas suas extensões tabeladas.
Definir também quais campos são obrigatórios para salvar e quais condicionam a recomendação.

## 3. Etapas de implementação

### Etapa 1 — Unificar campos obrigatórios e validação

Prioridade alta; base para todos os cadastros e para o modo simplificado.

1. Inventariar regras de lotes, alimentos e cenários, inclusive campos condicionais e seletores.
2. Mapear erros para o nome exato do controle; manter resumo para problemas gerais.
3. Evoluir `Campo` e extrair componentes de rótulo, mensagem e seleção em módulos pequenos.
4. Exibir `*` e legenda “Campos obrigatórios”, alinhados ao `required` e às regras do servidor.
5. Colocar mensagem acima do controle inválido, borda vermelha e associação por
   `aria-describedby`; manter a dica acessível e o resumo com links para os campos.
6. Aplicar restrições HTML adequadas: `min`, `max`, `step`, `maxlength` e obrigatoriedade
   condicional. Inteiros não devem usar indiscriminadamente `step="any"`.
7. Usar validação nativa e eventos locais para feedback antes do envio, inclusive após
   substituições htmx. Reutilizar hx-live onde já resolver o comportamento necessário.
8. Validar novamente no servidor, inclusive com JavaScript desligado ou POST direto.
   Preservar valores e distinguir erro de campo, pendência e falha operacional.

Aceite: nenhum dado inválido é persistido; controles exibem mensagens claras; a indicação
desaparece após correção; erro de servidor aponta o campo certo e preserva o preenchimento.
Testar MS zero/acima de 100%, inteiros fracionários, nomes vazios, origem ausente,
limites de inclusão contraditórios e campos condicionais.

### Etapa 2 — Isolar os formulários

Prioridade alta; segue a decisão D1 e reutiliza a etapa 1.

1. Separar estrutura de documento, navegação principal e estrutura exclusiva de cadastro.
2. Reutilizar os mesmos componentes Maud em página completa e resposta de fragmento.
3. Adaptar todos os pontos de entrada: início, listagens, revisão, novo cenário,
   duplicação e cadastro de lote a partir do cenário.
4. Abrir os formulários em nova guia sem menu, com ações de salvar, cancelar e ajuda.
5. Manter o formulário isolado também após erros; tratar sucesso conforme a tela de origem.
6. Conservar token/origem de proteção e impedir que parâmetros de retorno aceitem URLs externas.
7. Manter título acessível e navegação por teclado; o acesso direto e sem JavaScript também
   deve renderizar a página exclusiva de formulário.
8. Anunciar a nova guia no link e preservar o cenário da guia original. Atualizar suas opções
   sem abrir outro formulário ao retornar ou reenviar automaticamente o cenário.

Aceite: criar e revisar não exibem o menu principal; cancelar não grava; salvar cria uma
única revisão; um cadastro auxiliar não apaga os dados do cenário em elaboração.
Duplicar cenário continua gerando nova execução e preservando a anterior.

### Etapa 3 — Exemplos e preenchimento simplificado

Prioridade alta; depende de D2/D3 e dos componentes da etapa 1.

1. Oferecer ação explícita “Preencher exemplo”, com identificação e origem do modelo.
2. Reutilizar o catálogo e as estruturas tipadas existentes; não criar uma segunda tabela
   nutricional em JavaScript. Preservar IDs/versionamento das composições selecionadas.
3. Para alimentos sintéticos, preservar a natureza `SINTETICA` e a identificação da fonte;
   nunca apresentá-los como análise do produto do usuário.
4. Criar visualizações simplificada e completa do mesmo formulário e contrato de dados.
   Alternar a visualização deve preservar campos, erros e condições declaradas.
5. Exibir campos adicionais quando exigidos: consumo informado acima de 600 kg,
   limite de rótulo para suplemento e dados do laudo quando houver analito selecionado.
6. Evitar controles obrigatórios ocultos e não focalizáveis; abrir a seção que contém erro.
7. Não substituir valores já alterados ao trocar modo. Em revisões, preservar os dados
   cadastrados; aplicar modelo só por ação explícita e com confirmação de substituição.
8. Retirar a seleção implícita de demonstração do fluxo comum de novo cenário;
   manter uma entrada explícita para os exemplos A/B.

Aceite: obter um formulário mais curto sem eliminar informação necessária ao cálculo;
nenhuma seleção de exemplo grava ou executa automaticamente. Dados desconhecidos permanecem
desconhecidos, e editar um cadastro não altera os resultados históricos.

### Etapa 4 — Identidade e leitura das tabelas

Prioridade baixa; pode avançar independentemente das decisões dos formulários.

1. Verificar a licença e atribuição do SVG indicado nas notas antes de incorporá-lo.
2. Salvar cópia local revisada em `static/`, sem scripts ou referências externas, e registrar
   procedência/licença. Exibir ao lado de FeedYourCow com dimensões fixas.
3. Tratar o ícone como decorativo quando o nome já identificar o link, evitando leitura duplicada.
4. Adicionar variável de cor no início de `static/app.css` e estilo `tbody tr:hover`.
5. Aplicar destaque equivalente com `:focus-within` às linhas com controles ou links;
   preservar contraste, estados de alerta, cabeçalhos e impressão.

Aceite: recurso funciona offline, mantém o tema esmeralda e não cria banner;
hover/foco facilitam a leitura sem esconder cores ou informações de resultado.

### Etapa 5 — Padronizar conteúdos expansíveis

Prioridade média; reutiliza a solução local das cartas de ajuda.

1. Inventariar todos os expansíveis em `paginas.rs` e `componentes/`, incluindo ingredientes,
   laudos, medição de consumo, regras, fontes e entradas preservadas.
2. Extrair componente Maud com título, conteúdo, estado inicial e identificador único.
3. Reutilizar a transição de grid/opacidade e hx-live existentes, mantendo estados acessíveis,
   movimento reduzido, impressão e alternativa sem JavaScript.
4. Definir tratamento específico de ingredientes: seleção do alimento e expansão do conteúdo
   são estados diferentes. Evitar controles interativos dentro do botão de expansão.
5. Preservar `open` inicial, navegação por fragmentos da ajuda e preenchimento dos controles
   após substituição htmx. Expandir seções com erro e impedir foco no conteúdo recolhido.
6. Evitar impor animação diretamente a todos os `details` sem verificar abertura e fechamento
   nos três navegadores. Validar o componente antes de migrar as demais ocorrências.

Aceite: abrir/fechar é suave e funciona com teclado; não há animação inicial indesejada,
perda de dados, IDs duplicados ou informação ausente na impressão.

## 4. Testes, documentação e entrega

Cada etapa deve incluir seus testes e documentação no mesmo commit funcional.
Antes da implementação, conferir documentação oficial das versões utilizadas de Axum, Maud,
htmx/hx-live e APIs do navegador pertinentes à solução escolhida.
O plano parte do código local; essa conferência de APIs permanece como tarefa da implementação.

- Testes Rust: mapeamento de erros, obrigatoriedade condicional, modelos e renderização.
- Integração HTTP/SQLite: rejeição de POST inválido sem gravação, proteção de escrita,
  preservação de valores, criação/revisão e histórico imutável.
- Playwright: criação e revisão de lote/alimento; verificar, formular e duplicar cenário;
  erros, correção, alternância de modo, exemplos e cancelamento.
- Navegadores: Chromium, Firefox e WebKit; testar teclado, foco, 390 px, impressão,
  JavaScript desligado, movimento reduzido e recursos exclusivamente locais.
- Inspeção visual: formulário isolado, obrigatórios, erro acima do controle, tabelas,
  ícone e expansíveis, com capturas por navegador.
- Qualidade: build, testes, clippy, rustfmt, limites de linhas e verificação SQLx conforme
  os comandos já documentados no README e no CI.

Atualizar `REQUISITOS.md` com as decisões aprovadas e novos casos de aceitação, sem mudar
silenciosamente o perfil nutricional. Atualizar README e ajuda contextual em cada entrega.
Relacionar novos testes a RF-02/03/06/07/14/15 e RNF-02/03/05/07/10/11/12/14,
preservando a cobertura de CT-05, CT-21, CT-26, CT-27, CT-29 e CT-30.

Não há necessidade identificada de novas dependências de produção ou alteração de esquema
para as melhorias de apresentação. Caso os modelos exijam novos dados de referência,
usar carga versionada; mudanças estruturais exigem migração SQLx e consultas verificadas.

## 5. Sequência e commits sugeridos

1. Registrar D1 nos requisitos e alinhar os detalhes de retorno e as decisões D2/D3.
2. `feat: padroniza validação dos formulários` — etapa 1.
3. `feat: isola os formulários de cadastro` — etapa 2.
4. `feat: adiciona exemplos e cadastro simplificado` — etapa 3.
5. `style: adiciona ícone e destaque nas tabelas` — etapa 4.
6. `feat: suaviza os conteúdos expansíveis` — etapa 5.

Dependências: etapa 1 precede 2 e 3; etapa 4 é independente; etapa 5 reutiliza a ajuda e
deve verificar interação com formulários. Os commits podem ser menores quando cada parte
estiver completa, documentada e testada. Não é necessário um commit final para reunir mudanças.

O plano estará pronto para execução após as decisões pendentes; as funcionalidades estarão
concluídas somente com os aceites e verificações da respectiva etapa demonstrados.

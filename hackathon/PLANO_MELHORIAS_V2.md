# Plano revisado de melhorias e sugestões com fontes

Data: 20/10/2026. Estado: execução autorizada; validação final em andamento.
Este documento substitui o [plano anterior](PLANO_MELHORIAS.md) para a retomada do trabalho.

## 1. Orientações incorporadas

1. Implementar as melhorias de [NOTAS_MELHORIAS.md](NOTAS_MELHORIAS.md).
2. Abrir criação e revisão em nova guia, com formulário sem menu principal.
3. Após salvar lote/alimento, mostrar confirmação na guia de cadastro. Ao retornar ao cenário
   original, atualizar as opções preservando os demais campos. Fluxo aprovado nesta conversa.
4. Buscar valores típicos de raças ou composições em fontes confiáveis na internet e usá-los
   como sugestões de preenchimento. Essa orientação substitui o uso de dados sintéticos como
   modelos comuns de alimentos. Exemplos A/B continuam exclusivos da demonstração.
5. Retomada autorizada pelo programador após a revisão, incluindo templates editáveis.

O [AGENTS.md](../AGENTS.md) e os [requisitos](../REQUISITOS.md) continuam aplicáveis.
Preservar condições desconhecidas, unidades, versões, fontes e histórico imutável.

## 2. Estado do repositório na pausa

- `cf37d54`: plano inicial e link no README.
- `789137a`: primeira etapa da validação, com asteriscos, mensagens acima dos controles,
  restrições numéricas e ajustes de associação de erros. Inclui documentação e dois novos testes.
- Os 35 testes anteriores passaram; os dois novos testes passaram em execução específica.
  A nova interface ainda não foi validada em Chromium, Firefox e WebKit.
- Há alterações parciais, sem commit e sem validação, para abertura em nova guia e estrutura
  sem menu, em sete arquivos de `src/web/`. Elas foram preservadas ao interromper a execução.
- Exemplos com fontes, modo simplificado, SVG, hover e padronização das expansões ainda não
  foram implementados.

Na retomada, revisar essas alterações parciais antes de completá-las. Não descartá-las nem
tratá-las como funcionalidade concluída. A pausa não implica reversão do commit de validação.

## 3. Pesquisa preliminar e critérios de seleção

Priorizar Embrapa, universidades, CQBAL e Feedipedia. Para cada sugestão, verificar a tabela
original, identidade do alimento ou população animal, processamento, unidades e método.
Registrar edição, página/tabela, URL, data de consulta, valores e limitações preserváveis.
Uma publicação antiga não passa a ser atual por ter sido consultada hoje.

Não juntar silenciosamente nutrientes de tabelas que descrevem produtos diferentes.
Não converter energia digestível/metabolizável em NDT por equivalência presumida.
Não usar CNF como amido nem uma média como garantia mínima ou laudo do produto.
Resultados de busca servem para localizar documentos; os valores precisam ser conferidos
no original.

### 3.1. Valores já encontrados para avaliar como sugestões

Estes são candidatos pesquisados, ainda sujeitos à conferência documental e à revisão de
compatibilidade com o perfil. Não foram carregados no banco nem aprovados para recomendação.

| Candidato | MS, % MN | PB, % MS | FDN, % MS | EE, % MS | Fonte |
| --- | ---: | ---: | ---: | ---: | --- |
| Silagem de milho, classe 30–35% MS | 32,5 | 6,9 | 44,3 | 2,8 | Feedipedia, tabela 12871 |
| Farelo de soja | 88,46 | 48,84 | 15,34 | 2,16 | Embrapa, 2014 |

Para a silagem, a tabela também informa amido de 29,1% MS, Ca de 1,9 g/kg MS e P de
1,7 g/kg MS. As conversões para o formulário são Ca de 0,19% MS e P de 0,17% MS.
A tabela identifica FDN de 44,3% como média obtida por equação; preservar essa informação.
Não há NDT nessa tabela: esse campo deve continuar desconhecido nesse modelo.
A fonte é [Feedipedia, silagem de milho 30–35% MS][silagem].

Para o farelo de soja, a publicação apresenta NDT de 79,42% MS, Ca de 0,47% MS e P de
0,52% MS; amido não aparece na tabela examinada. Conferir página e notas metodológicas antes
de carregar. O número de observações varia por nutriente; por exemplo, PB tem 88 e NDT tem 15.
A fonte é [Embrapa, Tabelas nordestinas de composição de alimentos, 2014][embrapa].

Também foi localizada composição de milho farelo na mesma publicação. Sua MS apresenta grande
dispersão na tabela: não usar esse valor como padrão sem verificar identidade e processamento.
Pesquisar uma entrada mais específica para milho em grão/moído, quando necessário.

Não completar NDT da silagem ou amido da soja com números inventados ou com os exemplos A/B.
Um modelo parcial pode facilitar cadastro e diagnóstico, mas não habilita automaticamente
formulação ou aprovação de dimensões dependentes dos valores ausentes.

### 3.2. Sugestões por raça

Pesquisar Holandesa, Jersey e Girolando, preservando diferenças de sexo, idade, ordem de parto,
grupo genético, manejo e estágio de lactação. Preferir médias/faixas de estudos ou tabelas com
população identificada, em vez de valores máximos de produção ou recordes de animais.

A pesquisa preliminar localizou [Embrapa, Girolando: raça tropical desenvolvida no Brasil][gir].
O documento contém resultados por grupos e produção acumulada em lactação; ainda não foi
estabelecido um conjunto completo de valores adequados ao formulário.
Não dividir produção de lactação por dias para apresentá-la como produção diária observada.

Proposta: oferecer sugestão por raça somente para os campos que tenham evidência suficiente.
Exibir faixa/contexto e deixar o usuário aplicar e corrigir o valor. Não atribuir 450 kg,
15 kg/dia ou 4% de gordura a todas as raças como se fossem médias publicadas.
Número de animais e dias em lactação continuam dados do lote, sem inferência pela raça.
Não adicionar fator racial ao motor nem ampliar sua população coberta.

## 4. Como as sugestões devem funcionar

### Alimentos

1. Mostrar uma lista de alimentos com descrição específica e fonte, separada dos exemplos A/B.
2. Ao escolher, apresentar prévia dos campos, bases, natureza e lacunas; aplicar por ação explícita.
3. Preencher como `MEDIA_TABELADA`, com referência preservada, e manter os valores editáveis.
4. Quando um campo mudar, registrar que houve alteração pelo operador. Não atribuir esse novo
   valor à publicação; manter separadamente referência original e origem do valor utilizado.
5. Se a alteração vier de análise, permitir nova revisão com sua própria fonte/método e data.
6. Não preencher preço, estoque, fornecedor, lote físico, autorização, laudo, limite de rótulo,
   origem animal ou presença de NNP/medicamentos como conclusões sobre o produto do usuário.
7. Não oferecer composição mineral genérica: usar um produto/rótulo identificado quando existir.

### Lotes

1. Permitir selecionar raça/grupo e consultar sugestões com origem e contexto.
2. Aplicar somente valores publicados para campos compatíveis; deixar lacunas explícitas.
3. Não pré-confirmar gestação, crescimento, estabilidade de peso, manejo ou acesso à água.
4. Confirmar os dados reais antes de salvar. Um lote fora da cobertura do perfil pode ser
   cadastrado, mas o cálculo continua distinguindo `FORA_DO_PERFIL` de erro de preenchimento.

### Cenários e modo simplificado

1. Novo cenário começa com escolha explícita de lote, composições e premissas.
2. Oferecer caminhos explícitos para demonstração A/B e para uso de dados cadastrados.
3. Modo simplificado usa o mesmo formulário e contrato do modo completo, recolhendo apenas
   campos complementares. Alternar modo não apaga nem substitui valores.
4. Sobras/perdas zero e disponibilidade sem limite exigem hipótese explícita.
5. Erros e dependências tornam visíveis os campos necessários; nenhum obrigatório fica oculto
   e impossível de focalizar. Não omitir consumo-alvo quando necessário nem limites de rótulo.
6. Trocar modelo sobre formulário preenchido exige confirmação de substituição.

## 5. Dados, procedência e arquitetura

Reutilizar Rust/Axum/Maud, SQLx/SQLite, htmx v4 e hx-live. Não introduzir dependência de
produção sem necessidade identificada. Consultar a documentação oficial das APIs ao implementar.

- Manter sugestões e fontes em carga SQL versionada, separadas da carga sintética.
- Preservar valores originais, unidades e conversões, inclusive g/kg MS para % MS e frações.
- Registrar por campo a fonte, valor de referência, natureza e eventual alteração pelo operador.
  Definir o contrato tipado antes de decidir se o esquema precisa de tabelas adicionais.
- Reutilizar `fontes`, `composicoes`, revisões e o sistema de cargas, em vez de duplicar
  valores de referência em JavaScript ou criar uma base paralela em JSON.
- Se necessário, criar nova migração; não reescrever migrações aplicadas. Manter consultas em
  arquivos SQL com macros verificadas e atualizar o cache offline.
- Preservar fontes e modelos usados no histórico e no backup. Não recalcular execuções antigas
  com sugestões atualizadas. A aplicação deve operar offline após a preparação.
- Não fazer busca automática na internet durante cadastro ou cálculo. Atualizações de modelos
  passam por pesquisa, revisão documental, testes e nova carga versionada.

## 6. Sequência de retomada

### Etapa A — Fechar a pesquisa e os critérios

Conferir os originais, páginas, bases, métodos, dispersão e compatibilidade dos candidatos.
Pesquisar os campos de raça e milho ainda sem evidência suficiente. Produzir matriz de campos
que podem ser preenchidos, campos ausentes e motivos de exclusão.
Revisar a seleção com o programador antes de ativar a carga, conforme AGENTS.md.

Aceite: cada número proposto tem origem verificável; lacunas e estimativas são explícitas.

### Etapa B — Concluir validação e isolamento de formulários

Revisar o commit existente e as alterações interrompidas. Completar seletores, campos condicionais
e associação de erros de cenários. Cobrir envio direto e valores já preenchidos por modelos.
Concluir páginas exclusivas em nova guia, cancelamento, confirmação e atualização da origem.

Aceite: nenhum inválido é persistido; criar/revisar não exibe o menu; voltar ao cenário mantém
valores digitados e disponibiliza os novos registros sem reenvio automático.

### Etapa C — Carga de sugestões e modo simplificado

Implementar o contrato de procedência, a carga versionada, a prévia e a aplicação dos valores.
Separar sugestões publicadas de demonstração e registrar alterações do operador por campo.
Adicionar os modos de visualização com todas as condições necessárias ao cálculo preservadas.

Aceite: fonte correta acompanha cada valor; mudar modelo não grava automaticamente; editar um
valor não falsifica sua origem; ausências não viram zero nem permitem conclusão sem suporte.

### Etapa D — Identidade, tabelas e expansões

Incorporar o SVG local indicado nas notas após registrar licença/atribuição e revisar seu conteúdo.
Adicionar hover/foco nas linhas das tabelas sem encobrir estados. Extrair componente Maud
reutilizável para expansões suaves, aproveitando as cartas de ajuda e o hx-live existentes.
Separar seleção do ingrediente e botão de expansão; manter teclado, impressão e movimento reduzido.

Aceite: visual consistente, funcionamento offline e conteúdo acessível, sem perda de dados.

## 7. Verificação e commits

Cada etapa deve incluir testes, documentação atualizada e commit curto em português.

- Rust: conversões, associação de erros, lacunas, modelos e rastreabilidade por campo.
- Integração: carga idempotente e transacional; versões colidentes, fonte ausente e unidade
  incompatível rejeitadas; salvar revisão não altera histórico; backup preserva os modelos.
- Playwright em Chromium, Firefox e WebKit: nova guia sem menu, prévia, preenchimento,
  substituição confirmada, modo simplificado/completo, correção e cancelamento.
- Testar cadastro auxiliar com cenário parcialmente preenchido, JavaScript desligado,
  teclado/foco, impressão, 390 px e movimento reduzido. Fazer inspeção visual com capturas.
- Comprovar que modelos incompletos mantêm diagnóstico independente e não recebem aprovação
  nas dimensões sem dados. Testar que raça não altera coeficientes do motor.
- Executar build, testes, clippy, rustfmt, verificação SQLx e qualidade textual.
  Corrigir também as violações textuais preexistentes antes da entrega final.

Atualizar requisitos, README, ajuda contextual e manifesto de fontes a cada entrega.
Commits sugeridos: documentação da pesquisa; validação/isolamento; carga/procedência;
interface de sugestões; modo simplificado; identidade/tabelas; expansões.

## 8. Pontos para revisão antes de retomar

1. Aceitar modelos parciais com lacunas visíveis, sem misturar fontes incompatíveis para
   aparentar composição completa. Recomendação: sim, preservando o diagnóstico inconclusivo.
2. Definir quais raças entram inicialmente conforme os resultados da pesquisa, sem exigir
   preenchimento de todo campo por raça. Recomendação: liberar apenas campos com evidência.
3. Aprovar a seleção final de fontes/modelos após a etapa A, antes de carregá-los como sugestões.

A primeira carga foi aprovada nesta conversa: valores verificados, modelos parciais e lacunas
explícitas. A matriz final de fontes está em `fontes/MODELOS.md`. A implementação inclui
isolamento de cadastros, carga/procedência, prévia e edição, modo simplificado e expansões.
Resultados de testes e limitações de entrega devem ser conferidos no README e no relatório final.

[silagem]: https://www.feedipedia.org/node/12871
[embrapa]: https://www.infoteca.cnptia.embrapa.br/infoteca/handle/doc/1007523
[gir]: https://www.infoteca.cnptia.embrapa.br/infoteca/handle/doc/594820

# Valores para preenchimento manual na apresentação

Roteiro conferido em 22/10/2026 com o perfil `E2020_NRC1989_RUMEN_MVP_v1`.
Os números são hipóteses sintéticas para demonstrar o FeedYourCow. Não representam um rebanho,
uma cotação, um laudo ou uma recomendação de dieta reais.

## 1. Preparação e ordem sugerida

Use uma instalação já preparada e abra a aplicação. Para preparar e iniciar pelo terminal,
consulte [Operação](../../docs/OPERACAO.md). O comando `preparar` já disponibiliza o catálogo
inicial; este roteiro não depende do comando `demonstrar` nem dos botões Demonstração A/B.

Durante a apresentação, faça os cadastros abaixo e use nomes iniciados por **Apresentação**.
Se o banco já tiver registros com esses nomes, acrescente um sufixo para identificar o ensaio.
Use ponto nos valores decimais digitados quando o navegador exigir; nas tabelas de entrada,
`0.25` significa vinte e cinco centavos. Nas tabelas de resultado, usa-se a vírgula brasileira.
MN significa matéria natural; MS significa matéria seca.

Percurso principal, para cerca de 10–15 minutos:

1. Cadastrar o lote e um alimento; mostrar os demais alimentos preparados antes da fala.
2. Preencher e executar **01 - Dieta base**; abrir balanço, regras, fontes e fornecimento.
3. Duplicar para **02 - Milho mais barato** e comparar os custos no histórico.
4. Duplicar a base para **07 - Menor custo** e mostrar a formulação.
5. Demonstrar **03 - Sem farelo de soja**, **04 - Estoque insuficiente** e **08 - Pesagem grossa**.
6. Encerrar com histórico, reutilização e impressão. Os demais casos são complementares.

Faça os casos 01, 02 e 07 antes de revisar o lote: a comparação exige a **mesma revisão do
mesmo lote**. Cada caso começa da execução 01, salvo indicação explícita em contrário.
Use **Duplicar e simular**, altere o nome e somente os campos indicados. Não encadeie as
alterações de casos independentes.

## 2. Cadastro do lote principal

Em **Lotes de animais**, abra um novo cadastro e preencha:

| Campo | Valor para digitar ou selecionar |
| --- | --- |
| Nome | Apresentação - Girolando |
| Estado produtivo | Lactação |
| Destinação econômica | Produção de leite |
| Raça / grupo genético | Girolando 5/8 Holandês × 3/8 Gir |
| Número de animais | 12 |
| Peso médio (kg) | 550 |
| Leite (kg/animal/dia) | 22 |
| Gordura do leite (%) | 3.8 |
| Dias em lactação | 120 |
| Idade (meses) | 54 |
| Ordem de parto | 2 |
| Dispersão de peso (kg) | Deixar vazio |
| Condição corporal | 3 |
| Classe adulta | Sim |
| Gestante | Não |
| Em crescimento | Não |
| Peso estável | Sim |
| Confinada com dieta total quantificável | Sim |
| Precisa de correção por ambiente | Não |
| Precisa de correção por atividade | Não |
| Acesso à água | Sim |

Em **Observações e hipóteses complementares**, digite:

> Lote sintético da apresentação. Condições assumidas, sem medição real.
> Condição corporal na escala de 1 a 5. Grupo tratado como homogêneo.

Clique em **Salvar revisão**. Isso demonstra cadastro com condições explícitas e dados
complementares. Os valores são os do exemplo Girolando inicial, com 12 animais para tornar
visível a diferença entre quantidade por animal, por lote e por período.

## 3. Cadastro dos cinco alimentos

Cadastre os alimentos abaixo em **Alimentos**. Para reduzir a digitação durante a fala,
é possível preparar quatro antes e cadastrar apenas a silagem ao vivo. Outra opção é usar
os alimentos correspondentes do catálogo inicial, desde que não tenham sido alterados.
Escolha uma única composição de cada alimento no cenário; não selecione simultaneamente
um cadastro manual e seu equivalente do catálogo.

### 3.1. Identificação e composição

Todos os valores desta tabela são **percentuais**, como o formulário solicita.
Não digite `0.325` no campo MS da silagem: digite `32.5`.
NDT, PB, Ca, P, FDN, amido e EE estão expressos sobre a MS.

| Campo | Silagem | Milho | Farelo | Casca | Mineral |
| --- | --- | --- | --- | --- | --- |
| Classificação | Forragem | Concentrado | Concentrado | Concentrado | Suplemento |
| Matéria seca (% MN) | 32.5 | 88 | 88.5 | 90 | 95 |
| NDT (% MS) | 65 | 86 | 79.4 | 74 | 0 |
| Proteína bruta (% MS) | 6.9 | 9 | 48.8 | 12 | 0 |
| Cálcio (% MS) | 0.19 | 0.03 | 0.47 | 0.6 | 20 |
| Fósforo (% MS) | 0.17 | 0.3 | 0.52 | 0.18 | 8 |
| FDN (% MS) | 44.3 | 12 | 15.3 | 65 | 0 |
| Amido (% MS) | 29.1 | 70 | 2 | 3 | 0 |
| Extrato etéreo (% MS) | 2.8 | 4 | 2.2 | 2 | 0 |
| Máximo do rótulo (kg MS/animal/dia) | Vazio | Vazio | Vazio | Vazio | Vazio |
| Processamento | Ensilado | Moído | Farelo | Seco | Misturado |

Use os nomes **Apresentação - Silagem**, **Apresentação - Milho**, **Apresentação - Farelo**,
**Apresentação - Casca** e **Apresentação - Mineral**, respectivamente.
“Farelo” corresponde a farelo de soja; “Casca”, a casca de soja; “Mineral”, à mistura mineral.
Os zeros do mineral são hipóteses explícitas da composição sintética, não dados ausentes.

### 3.2. Campos comuns aos cinco alimentos

| Campo | Valor |
| --- | --- |
| Origem animal | Não |
| Exceção legal documentada | Não |
| NNP intencionalmente adicionado | Não |
| Contém medicamento | Não |
| Autorização / hipótese do exemplo | Sim |
| Natureza dos valores | SINTETICA |
| Identificador da fonte | EXEMPLOS-V1 |
| Origem | Catálogo sintético do sistema; transcrição para apresentação |
| Método, edição e página | Transcrição dos valores sintéticos do catálogo EXEMPLOS-V1 |
| Data da análise | 2026-10-21 |
| Fornecedor / produto | Deixar vazio |
| Lote físico | Deixar vazio |
| Evidência de autorização | Hipótese sintética da apresentação; não comprova produto real |
| Laudos de contaminantes | Deixar todos os campos vazios |

A fonte `EXEMPLOS-V1` já existe no banco preparado e pode ser consultada em **Conhecimento**.
A data identifica a composição demonstrativa, sem declarar análise laboratorial real.
O mineral não tem rótulo comercial: seu teto será uma hipótese explícita no cenário.
Essa possibilidade depende da natureza `SINTETICA`; não use `ANALISE` para estes exemplos.

Salve cada cadastro. Isso demonstra composição, procedência, unidades e distinção entre
zero informado e campo desconhecido. Preço e estoque serão preenchidos na dieta.

## 4. Caso 01 - Dieta base

Em **Nova dieta**, preencha os dados do cenário:

| Campo | Valor |
| --- | --- |
| Nome do cenário | Apresentação 01 - Dieta base |
| Lote e revisão atual | Apresentação - Girolando |
| Tipo de cálculo | Verificar as quantidades informadas |
| Horizonte (dias) | 7 |
| Data de avaliação | 2026-10-22 |
| Data de aplicação das regras | 2026-10-22 |
| Quantidade informada por ingrediente | Consumo em kg MS/animal/dia |
| Consumo-alvo (kg MS/animal/dia) | Deixar vazio |
| Sobras planejadas (% da MS ofertada) | 0 |
| Preço do leite (R$/kg) | 2.50 |
| Origem do consumo | Estimado pelo perfil |
| Origem das sobras e perdas | Hipótese sintética: zero sobras e zero perdas |
| Consumo observado por pesagem do lote | Deixar os três campos vazios |
| Referência UE adicional | Desmarcada |
| Base da simulação de MS | Consumo de MS fixo |

Marque somente os cinco alimentos e atualize a seleção. Preencha:

| Ingrediente | Consumo kg MS/animal/dia | Preço R$/kg MN | Inclusão máxima kg MS |
| --- | --- | --- | --- |
| Silagem | 11 | 0.25 | Vazio |
| Milho | 0.8 | 1.30 | Vazio |
| Farelo de soja | 3 | 2.20 | Vazio |
| Casca de soja | 2 | 1.10 | Vazio |
| Mineral | 0.3 | 4.00 | 0.3 |

Nas configurações de **cada ingrediente**, confira:

- Unidade comercial: **R$/kg MN**; massa da embalagem: vazia.
- MS neste cenário: respectivamente **32.5, 88, 88.5, 90 e 95**.
- Perda anterior ao fornecimento: **0**; inclusão mínima: **0**.
- Resolução de oferta: **0**, para começar com o resultado contínuo.
- Estoque bruto: vazio; marque **Hipótese explícita: disponibilidade sem limite neste cenário**.
- Origem e data da cotação: **Hipótese sintética da apresentação - 22/10/2026**.

Clique em **Calcular e salvar cenário**. Anote o identificador da execução 01.

### Resultado a mostrar

| Indicador | Resultado esperado aproximado |
| --- | --- |
| Condição da execução | AVALIADO |
| Elegibilidade, ingredientes, inclusão e estoque | ATENDE |
| Núcleo químico e barreiras ruminais | ATENDE |
| Consumo de MS | 17,1000 kg/animal/dia |
| Consumo de referência do perfil | 16,8685 kg MS/animal/dia |
| Leite corrigido a 4% | 21,34 kg/animal/dia |
| Custo por animal/dia | R$ 20,81 |
| Custo do lote/dia | R$ 249,70 |
| Custo do lote em 7 dias | R$ 1.747,92 |
| Receita informada por animal/dia | R$ 55,00 |
| Margem sobre alimentação por animal/dia | R$ 34,19 |
| Contaminantes | INCONCLUSIVO |
| Fibra física e micronutrientes | NÃO AVALIADO |
| Fornecimento | Resolução pendente |

A margem considera apenas receita do leite informado menos alimentação, sem prever produção
nem representar lucro líquido. A verificação preserva os 17,1 kg MS digitados; a referência
de 16,8685 kg não substitui as quantidades informadas.

No balanço, confira os nutrientes em **kg/animal/dia**:

| Nutriente | Ingerido | Exigência |
| --- | --- | --- |
| NDT | 11,70000 | 10,86920 |
| PB | 2,53500 | 2,31320 |
| Ca | 0,10724 | 0,090508 |
| P | 0,06430 | 0,05824 |

No fornecimento contínuo, confira:

| Ingrediente | Oferta kg MN/animal/dia | Necessidade bruta kg MN/7 dias do lote |
| --- | --- | --- |
| Silagem | 33,84615 | 2.843,07692 |
| Milho | 0,90909 | 76,36364 |
| Farelo de soja | 3,38983 | 284,74576 |
| Casca de soja | 2,22222 | 186,66667 |
| Mineral | 0,31579 | 26,52632 |

Abra **Cálculos e regras aplicadas**, **Entradas preservadas** e as fontes/versionamento.
Explique que cada dimensão tem seu próprio estado: atender à composição química não resolve
as dimensões sem dados. A resolução zero também não aprova uma pesagem prática.

## 5. Casos derivados: custos, déficits e fornecimento

Sempre duplique a execução 01 para iniciar cada caso desta seção.

### Caso 02 - Milho mais barato e comparação econômica

Nome: **Apresentação 02 - Milho mais barato**. Troque somente o preço do milho de `1.30`
para `1.00` R$/kg MN e salve. O custo será aproximadamente **R$ 20,54/animal/dia**.
O consumo, o balanço e os estados nutricionais permanecem iguais.

Em **Histórico e análise**, fixe a execução 01 como referência, selecione 02 e clique em
**Comparar premissas e custos**. A redução calculada antes de arredondar é de cerca de
**R$ 0,272727/animal/dia**, ou **R$ 22,91 para 12 animais em 7 dias**. Mostre o preço
alterado e a linha do milho. Use as mesmas datas, lote, revisão e conhecimento ativo.

### Caso 03 - Sem farelo de soja

Nome: **Apresentação 03 - Sem farelo de soja**. Troque somente a quantidade do farelo de
soja de `3` para `0`. Mantenha-o selecionado e salve em modo verificar.

Esperado: consumo **14,1 kg MS/animal/dia**, custo **R$ 13,35/animal/dia**, núcleo químico e
barreiras ruminais **NÃO ATENDE**, fornecimento incompatível. Mostre os déficits e as regras
violadas: um custo menor não torna a dieta aceitável.

### Caso 04 - Estoque insuficiente

Nome: **Apresentação 04 - Estoque insuficiente**. Na silagem, desmarque disponibilidade
sem limite e informe estoque bruto `100` kg MN. Salve em modo verificar.

Esperado: **Inclusão e estoque: NÃO ATENDE**, pois seriam necessários aproximadamente
**2.843,08 kg MN** de silagem para os 12 animais em 7 dias. O núcleo químico permanece
favorável; o fornecimento é incompatível. Mostra a diferença entre viabilidade nutricional
e disponibilidade física.

### Caso 05 - Sobras e perdas

Nome: **Apresentação 05 - Sobras e perdas**. Informe sobras planejadas `5` e perda anterior
ao fornecimento da silagem `10`, ambas em percentual. Nos outros ingredientes, mantenha
perda `0`. Troque a origem das sobras e perdas para **Hipótese sintética: sobras 5%; perda
prévia da silagem 10%**. Conserve a base consumo MS.

Esperado: consumo preservado em **17,1 kg MS**, custo **R$ 22,89/animal/dia**, oferta da
silagem **35,62753 kg MN/animal/dia** e necessidade bruta **3.325,24 kg MN/7 dias do lote**.
Mostra que a retirada do estoque inclui perdas anteriores à oferta e que a oferta inclui
sobras. Não desconte os percentuais manualmente das quantidades de consumo.

### Caso 06 - Pesagem validada

Nome: **Apresentação 06 - Pesagem validada**. Informe resolução de oferta `0.01` kg MN/animal
nos cinco ingredientes e aumente apenas o teto de inclusão do mineral de `0.3` para `0.31`
kg MS. O teto continua sendo uma hipótese sintética, não um rótulo comercial.

Esperado: **FORNECIMENTO_VALIDADO** dentro do perfil avaliado. As quantidades práticas,
em kg MN/animal/dia, são silagem **33,85**, milho **0,91**, farelo **3,39**, casca **2,22**
e mineral **0,32**. A dieta prática consome **17,1042 kg MS** e custa cerca de **R$ 20,83**.
O custo contínuo permanece em R$ 20,81. A resolução de 10 g exige balança compatível.

Mostre a revalidação e **Imprimir ficha de fornecimento**. O teto de `0.31` evita que os
`0.304` kg MS do mineral arredondado excedam o limite original `0.3`. O estado operacional
não elimina a pendência de contaminantes nem aprova dimensões fora do perfil.

## 6. Formulação de menor custo

### Caso 07 - Menor custo

Duplique 01, nomeie **Apresentação 07 - Menor custo** e escolha **Formular combinação de
menor custo**. Mantenha preços, teto do mineral `0.3`, disponibilidade sem limite, consumo-alvo
vazio e resolução `0`. As quantidades da verificação não são ponto inicial da busca.

Esperado: busca **OTIMO_NO_MODELO**, consumo **16,8685 kg MS/animal/dia** e custo aproximado
**R$ 18,64/animal/dia**. O motor usa a referência de consumo e resolve cinco subproblemas.

| Ingrediente | Consumo contínuo kg MS/animal/dia | Oferta contínua kg MN/animal/dia |
| --- | --- | --- |
| Silagem | 13,827762 | 42,546961 |
| Farelo de soja | 2,785009 | 3,146903 |
| Mineral | 0,255729 | 0,269188 |
| Milho e casca de soja | 0 | 0 |

Mostre os estados dos subproblemas, a candidata revalidada e o fornecimento com resolução
pendente. Use **Ajustar e verificar solução contínua** para abrir as quantidades calculadas
em um novo cenário. **Refazer otimização** demonstra o reaproveitamento das premissas.

É possível comparar 01 e 07 por serem da mesma revisão do lote. Explique as diferenças de
modo e consumo antes da redução de custo: a comparação deve expor essas premissas.

### Caso 08 - Pesagem grossa

Parta de 07 usando **Refazer otimização**. Nomeie **Apresentação 08 - Pesagem grossa** e
informe resolução de oferta `1` kg MN/animal nos cinco itens. Mantenha o teto do mineral `0.3`.

Esperado: busca ainda **OTIMO_NO_MODELO**, mas **FORNECIMENTO_INCOMPATIVEL** após arredondar.
Abra o diagnóstico prático e suas violações. A ficha de fornecimento deve indicar recusa;
a solução contínua permanece disponível. Este caso distingue certificado matemático de
quantidade operacional aceitável.

### Caso 09 - Restrições inviáveis

Duplique 01, nomeie **Apresentação 09 - Restrições inviáveis**, escolha formular e informe
inclusão máxima `0.1` kg MS para **cada um dos cinco ingredientes**.

Esperado: **INVIAVEL_NO_MODELO**, sem candidata e sem custo de solução. A soma dos tetos é
apenas 0,5 kg MS/dia, insuficiente para o consumo-alvo de referência. Não interprete eventual
diagnóstico das entradas como dieta encontrada.

### Caso 10 - Estoque desconhecido

Duplique 01, nomeie **Apresentação 10 - Estoque desconhecido**, escolha formular e, somente
na silagem, desmarque disponibilidade sem limite e deixe estoque vazio.

Esperado: busca **INCONCLUSIVA**, com indicação para informar o estoque ou declarar a hipótese.
Isso demonstra que vazio é desconhecido; difere de estoque zero e da inviabilidade do caso 09.

## 7. Perfil, segurança e qualidade dos dados

Para os casos 11, 12 e 14, crie **cópias independentes** do lote principal, com os nomes
indicados. Não salve essas mudanças como revisões do lote usado em 01 e 02.
Duplique a execução 01, use **Atualizar cadastros** quando necessário e selecione a cópia.

| Caso e nome do novo lote | Única alteração na cópia | Resultado a demonstrar |
| --- | --- | --- |
| 11 - Gestante | Gestante: Sim | FORA_DO_PERFIL; elegibilidade NÃO ATENDE |
| 12 - Gestação desconhecida | Gestante: Desconhecido | INCONCLUSIVO; elegibilidade inconclusiva |
| 14 - Peso extrapolado | Peso médio: 650 kg | Aviso de extrapolação do consumo da Tabela B |

No caso 14, mantenha consumo-alvo vazio. O peso está dentro do perfil, porém fora da faixa
400–600 kg tabelada para consumo. O aviso não equivale a bloqueio. Use nomes de cenário
**Apresentação 11**, **Apresentação 12** e **Apresentação 14**, respectivamente.

### Caso 13 - Ingrediente com NNP

Em Alimentos, crie uma cópia independente da silagem chamada **Apresentação - Silagem com NNP**.
Mude apenas **NNP intencionalmente adicionado** para **Sim**. Mantenha a natureza sintética.
Duplique 01 como **Apresentação 13 - Ingrediente com NNP**, desmarque a silagem original e
selecione a cópia. Atualize a seleção e preencha no novo item os valores originais da silagem:
quantidade `11`, preço `0.25`, MS `32.5`, perdas `0`, mínimo `0`, máximo vazio, resolução `0`
e disponibilidade sem limite marcada. Salve em modo verificar.

Esperado: condição **BLOQUEADO**, dimensão Ingredientes **NÃO ATENDE**, regra `MVP-01`.
Demonstra exclusão pela política do MVP; não é uma conclusão de proibição universal do NNP.

### Caso 15 - Consumo observado

Duplique 01 como **Apresentação 15 - Consumo observado**. Deixe consumo-alvo vazio e preencha
**MS ofertada ao lote: 216**, **MS das sobras do lote: 10.8** e **Período: 1 dia**.
Informe origem do consumo **Observado por pesagem do lote**.

Esperado: consumo informado pela medição de **17,1 kg MS/animal/dia**:
`(216 - 10.8) / (12 × 1) = 17.1`. A verificação mantém a dieta de 17,1 kg MS e seu custo.
Os campos observados não substituem automaticamente as sobras planejadas do cenário;
aqui a medição ilustra a origem do consumo. Pesagem do lote não prova ingestão homogênea.

### Caso 16 - Referência de contaminantes sem laudos

Duplique 01 como **Apresentação 16 - Contaminantes sem laudos** e marque a referência UE
adicional. Não preencha laudos nos alimentos.

Esperado: contaminantes **INCONCLUSIVO**, com verificações da referência estrangeira e
pendências para o fornecimento. Ausência de laudo não significa concentração zero. Explique
que a referência adicional não é apresentada como limite legal brasileiro.

### Erros de preenchimento para mostrar antes de salvar

Faça estas tentativas separadamente em um rascunho e corrija depois da mensagem:

| Formulário | Entrada intencionalmente inválida | Comportamento esperado |
| --- | --- | --- |
| Lote | Número de animais: 0 | Rejeitar; corrigir para 12 |
| Alimento | Matéria seca: 101 | Rejeitar percentual; corrigir para o valor da tabela |
| Cenário | Horizonte: 0 | Rejeitar; corrigir para 7 |
| Ingrediente | Perda anterior: 100 | Rejeitar; corrigir para 0 |

A validação pode aparecer primeiro no navegador ou no retorno do servidor. Mostre a indicação
do campo e a preservação do rascunho. Peso `650` ou gestação desconhecida não são erros de
preenchimento: são dados válidos com consequências no parecer.

## 8. Revisões, cópias, histórico e impressão

Depois da comparação 01 × 02, demonstre:

1. Abra o alimento milho e salve uma nova revisão com PB de `9` para `10`% MS. Reabra a execução
   01: suas entradas e o parecer continuam com a composição antiga de PB `9`%.
2. Crie uma cópia do lote principal chamada **Apresentação - Cópia independente** e mostre que
   ela tem identidade própria. Para simular com a cópia, selecione-a em um novo cenário.
3. Em uma cópia de descarte, demonstre a desativação/exclusão. O histórico permanece consultável.
4. Fixe 01 como referência no histórico e localize o caso 11. A alternativa de outro lote fica
   identificada como incompatível e indisponível para seleção. O servidor também recusa essa
   comparação. Uma execução após revisar o lote não é comparável à revisão antiga.
5. Filtre o histórico pelo nome **Apresentação**, abra um resultado e use **Imprimir parecer**.
   Para a versão operacional curta, use a execução 06; a execução 08 mostra a recusa prática.
6. Em **Conhecimento**, mostre perfil, coeficientes, normas e fontes. No parecer, mostre as
   versões preservadas, que registram o contexto daquela execução.
7. Em **Ajuda** e nos indicadores `?`, mostre as explicações de MS/MN, sobras e estados.

Se quiser mostrar **Restaurar exemplos**, use um registro descartável do catálogo e escolha
explicitamente cópia ou revisão. Restaurar pode mudar o cadastro ativo; faça isso depois dos
casos principais. Repetir `preparar` não desfaz edições nem restaura exclusões.

## 9. Conferência e limites de reprodução

Os valores de composição vêm da carga [005_exemplos.sql](../../dados_referencia/005_exemplos.sql).
As quantidades e os preços base seguem o
[cenário inicial](../../src/aplicacao/demonstracao/exemplos.rs).
Os nomes de campos foram conferidos nos formulários de lote, alimento e cenário implementados.

Os 16 casos foram executados no motor, em banco temporário, com 12 animais e os valores deste
roteiro. Foram conferidos estados, custos, balanço, oferta e revalidação prática. Também passaram
17 testes existentes de exemplos, apresentação do fornecimento, comparação e auditoria dos
cadastros. A conferência deste documento não incluiu um novo ensaio manual no navegador.

Os números impressos são aproximados; a aplicação pode mostrar mais casas decimais. Resultados
dependem das composições, revisões, conhecimento e datas usados. Alterações no catálogo ou nas
regras exigem novo ensaio. Para repetir os casos numéricos, mantenha os dados das seções 2–4;
faça as alterações de revisão da seção 8 somente ao final.

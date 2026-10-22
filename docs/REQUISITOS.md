# Requisitos do FeedYourCow

**Versão documental:** 2.0 · **Perfil implementado:** `E2020_NRC1989_RUMEN_MVP_v1`.

Este documento define o comportamento esperado da aplicação local. O
[motor de inferência](MOTOR.md) explica cálculos e estados; [qualidade](QUALIDADE.md) relaciona
requisitos e evidências. Os exemplos numéricos são hipóteses de demonstração, não medições reais.
IDs permanecem estáveis para permitir rastrear regras, testes e pareceres.

## Objetivo e limites

O programa deve responder: (1) uma dieta informada atende ao perfil nutricional implementado?
(2) Qual dieta de menor custo pode ser encontrada com os ingredientes, preços e restrições do
cenário? A resposta deve apresentar entradas, versões, operações, limites, avisos e o caminho que
levou a cada conclusão.

O perfil atende **vacas adultas em lactação**, confinadas, com dieta total quantificável, peso
estável, sem gestação, crescimento, mobilização de reservas nem correção necessária por ambiente
ou atividade. Aceita peso vivo de **300–800 kg**, gordura do leite de **3–5%**, leite corrigido a
4% de **5–30 kg/animal/dia** e **60–250 dias em lactação**. As tabelas originais cobrem peso
400–700 kg para exigências e 400–600 kg para consumo; fora dessas faixas tabeladas, mas dentro do
perfil, o cálculo extrapola linearmente e precisa avisar da menor precisão.

O núcleo avalia NDT, PB, Ca, P, FDN total e de forragem, amido, EE, inclusão, estoque, segurança
de ingredientes e contaminantes quando houver dados e referência aplicável. Não implementa
integralmente NASEM 2021 ou BR-LEITE 2024. Fibra física, vitaminas, aminoácidos, água e outros
micronutrientes não recebem aprovação por inferência. Não há previsão de produção de leite,
diagnóstico clínico, prescrição, integração obrigatória com rede ou cálculo para outras categorias
animais. O sistema não apresenta uma dieta como segura ou adequada em todas as dimensões.

## Contrato de dados

### Lote

Registrar identidade, revisão, número positivo de animais, raça, idade quando conhecida, peso,
produção de leite em kg/animal/dia, gordura em pontos percentuais, dias em lactação, estado
produtivo, destinação e condições de manejo. Gestação, crescimento, classe adulta, peso estável,
confinamento, água e correções por ambiente/atividade usam **sim**, **não** ou **desconhecido**.
Desconhecido não confirma ausência de uma condição. Ordem de parto, dispersão de peso e condição
corporal são complementares. Lotes heterogêneos devem ser separados antes da análise.

### Alimento e produto

Separar identidade do alimento, revisão de composição e produto/lote físico. A composição traz
MS em kg MS/kg MN; NDT, PB, Ca, P, FDN, amido e EE em kg nutriente/kg MS. Preservar valor e
unidade originais, fonte, edição, localizador, natureza, método, data e processamento. Valores
ausentes continuam ausentes. Médias, análises, garantias mínimas/máximas e hipóteses sintéticas
têm significados diferentes; garantia unilateral não alimenta cálculo como ponto exato.

Registrar origem animal, NNP adicionado, medicamento, autorização, evidência de uso, fornecedor,
lote e limites de rótulo quando aplicáveis. Laudos de contaminantes precisam de analito, matriz,
unidade, umidade, método, documento e qualificador. Ausência de laudo não comprova ausência de
contaminação. Não inventar fabricantes, laudos ou autorizações para completar exemplos.

### Cenário

Registrar nome, revisão do lote, perfil, datas de avaliação e aplicação de regras, horizonte
positivo em dias, modo verificar/formular, base das quantidades, ingredientes selecionados,
preço e, opcionalmente, origem e data da cotação, estoque bruto ou hipótese explícita de
disponibilidade sem limite,
perdas, sobras, limites de inclusão e resolução de oferta. Preço pode ser informado por kg MN,
tonelada MN ou embalagem com massa. Consumo-alvo ou medição de oferta e sobras é opcional;
quando ausente, usar a referência do perfil. Preço do leite e referência adicional UE para
contaminantes também são opcionais.

As bases internas são kg, dia, animal, kg MS/kg MN, kg nutriente/kg MS e R$/kg MN. Oferta,
consumo, massa por animal e massa por lote não são intercambiáveis. O cenário admite de **1 a 20
ingredientes na verificação** e de **1 a 10 na formulação**. O limite total de busca é positivo
e não excede 10 segundos. Entradas não finitas, unidades incoerentes, percentuais fora da faixa,
estoque negativo ou preço inválido devem ser rejeitados com indicação de campo. Origem e data
da cotação são opcionais.

## Requisitos funcionais

| ID    | Comportamento e critério observável                                              |
| ----- | -------------------------------------------------------------------------------- |
| RF-01 | `preparar` aplica migrações, perfil e exemplos sem duplicar ou desfazer edições. |
| RF-02 | Cadastrar, revisar, copiar e desativar lotes sem alterar execuções antigas.      |
| RF-03 | Cadastrar e revisar alimentos, composições, fontes e condições de produto.       |
| RF-04 | Informar custos, unidade comercial, perdas, estoque e inclusão por cenário.      |
| RF-05 | Consultar perfil, coeficientes, normas, fontes e versões usados.                 |
| RF-06 | Construir cenários editáveis e escolher verificar ou formular.                   |
| RF-07 | Validar dados e enquadrar lote, ingredientes e perfil antes da conclusão.        |
| RF-08 | Obter consumo-alvo informado/medido ou referência calculada, com origem visível. |
| RF-09 | Verificar dieta informada sem alterar suas quantidades.                          |
| RF-10 | Formular candidata de menor custo e indicar o estado de cada subproblema.        |
| RF-11 | Mostrar oferta e consumo por animal/lote e revalidar resolução de pesagem.       |
| RF-12 | Comparar custos somente entre execuções da mesma revisão do mesmo lote.          |
| RF-13 | Exibir parecer por dimensão, regra, valor, limite, fonte e dependências.         |
| RF-14 | Preservar execução imutável e permitir consulta e impressão do histórico.        |
| RF-15 | Duplicar ou reutilizar candidata em nova simulação sem substituir o original.    |
| RF-16 | Aplicar cargas versionadas de conhecimento com integridade e hash.               |
| RF-17 | Fazer backup consistente e restaurar em destino novo após verificações.          |
| RF-18 | Entregar pacote nativo com recursos, cargas e instruções de operação.            |
| RF-19 | Oferecer catálogo sintético inicial e restauração explícita de exemplos.         |

A consulta ao histórico não oferece reexecução com versões originais. **Duplicar e simular** usa o
conhecimento ativo em uma nova execução. A impressão curta de fornecimento deve indicar recusa
quando as quantidades práticas não estiverem validadas.

## Regras de negócio

- **RN-01–02 — perfil e incerteza:** condição fora do perfil bloqueia recomendação; dado
  desconhecido permanece pendente, enquanto violação demonstrada permanece visível.
- **RN-03–04 — unidades e oferta:** converter MN/MS e preços antes de somar; contabilizar sobras e
  perdas uma vez; preservar dados e unidades originais no parecer.
- **RN-05–08 — exigências e consumo:** interpolar tabelas do perfil, calcular LCG4, escolher
  consumo informado quando disponível e apresentar balanço, déficit e cobertura por nutriente.
- **RN-09–10 — barreiras:** aplicar a linha efetiva de fibra e as políticas do MVP. Fibra física
  sem método/perfil completo e micronutrientes permanecem não avaliados.
- **RN-11–14 — ingredientes e normas:** autorização e proibições dependem de identidade,
  jurisdição, vigência e evidência. NNP adicionado, medicamentos e subprodutos animais são
  excluídos da formulação automática. Laudos só são comparados a analito, matriz, unidade e
  referência compatíveis; a referência UE é opcional e identificada como estrangeira.
- **RN-15–16 — fornecimento:** perdas, sobras, número de animais, horizonte, rótulo e estoque
  limitam a oferta. Estoque vazio não significa ilimitado.
- **RN-17–19 — busca:** resolver cinco problemas lineares, revalidar cada candidata, ordenar por
  custo e desempatar de modo determinístico. Interrupção não prova inviabilidade nem ótimo.
- **RN-20–21 — prática e economia:** resolução zero conserva aproximação contínua; pesagem
  recomendável exige resolução positiva dos itens usados e revalidação. Custo e economia
  dependem de base e premissas comparáveis, sem prever produção futura.
- **RN-22–24 — explicação e histórico:** preservar entrada, regra, fonte, versão e traço.
  Execução concluída é imutável; tolerâncias e formatação não podem esconder violações.

Valores e fórmulas normativos desta versão estão em [MOTOR.md](MOTOR.md). Mudança de perfil,
limite ou regra exige carga versionada, atualização documental e testes de regressão.

## Estados do resultado

Cada verificação usa `ATENDE`, `VIOLA`, `DESCONHECIDO` ou `NAO_APLICAVEL`. Uma dimensão resume
`ATENDE`, `NAO_ATENDE`, `INCONCLUSIVO` ou `NAO_AVALIADO`. A execução também distingue
`INVALIDO`, `FORA_DO_PERFIL`, `BLOQUEADO` e `AVALIADO`, conforme o caso. Na formulação, a busca
usa `OTIMO_NO_MODELO`, `VIAVEL_SEM_OTIMO`, `INVIAVEL_NO_MODELO` ou `INCONCLUSIVA`. O estado de
fornecimento prático é independente da certificação da busca.

## Requisitos não funcionais

- **RNF-01–04:** servidor local em Rust/Axum, páginas Maud, htmx/hx-live locais, CSS plano e
  SQLite via SQLx com migrações e macros sobre arquivos SQL separados.
- **RNF-05–07:** uso offline após instalação, resultados reproduzíveis com fatos/versionamento
  preservados, escrita protegida por POST, CSRF, origem e limites de requisição.
- **RNF-08–09:** meta p95 de até 1 s para verificar 20 ingredientes e 5 s para formular 10,
  timeout/cancelamento efetivo, erros úteis e tracing.
- **RNF-10–12:** testes Rust, navegador e pacote; build, testes, Clippy, rustfmt, SQLx e
  qualidade textual no CI; documentação sincronizada às mudanças.
- **RNF-13–14:** recarga opcional em desenvolvimento, interface de trabalho em português,
  ajuda contextual, teclado, impressão e fluxos essenciais sem JavaScript.
- **RNF-15:** a versão final deve funcionar em Chromium, Firefox e WebKit, com os fluxos
  essenciais validados pela suíte de navegador nos três motores.

## Aceite da entrega

Uma revisão só atende o contrato quando:

1. Um banco novo abre o cenário sintético completo sem cadastro manual prévio.
2. Verificação e formulação apresentam dimensões independentes, restrições e limites.
3. Candidata de formulação tem estado de busca correto e passa por revalidação.
4. Arredondamento incompatível recusa recomendação prática sem apagar o contínuo.
5. Revisões, histórico, comparação restrita e backup preservam dados anteriores.
6. Build, testes Rust, verificações estáticas e testes de navegador do CI passam.
7. Pacote de release prepara, serve e restaura um banco temporário com documentação local.

A [matriz de rastreabilidade](QUALIDADE.md) indica arquivos de teste e limitações da evidência.

## Anexo A — Catálogo inicial sintético

`dados_referencia/005_exemplos.sql` preserva o conteúdo integral, fontes, hipóteses e receitas.
Há seis lotes de um animal (`ex-l01` a `ex-l06`): Holandesa, Jersey, Girolando 5/8 Holandês,
mestiça F1 Holandês/Gir, Gir Leiteiro e Pardo-Suíça leiteira. Todos são exemplos, com condições
fisiológicas assumidas explicitamente.

| IDs               | Alimentos                                                              |
| ----------------- | ---------------------------------------------------------------------- |
| `ex-a01`–`ex-a05` | Silagens, feno de tifton, capim-elefante e cana sem ureia.             |
| `ex-a06`–`ex-a10` | Milho, farelo de soja, farelo de trigo, polpa cítrica e casca de soja. |
| `ex-a11`–`ex-a13` | Caroço de algodão, mistura mineral e calcário calcítico.               |
| `ex-a14`–`ex-a15` | Mistura milho/soja e mistura com casca de soja.                        |

As misturas são composições próprias: proporções em massa natural e nutrientes ponderados pela
matéria seca dos componentes. Atualizar um componente não modifica automaticamente a composição
já cadastrada da mistura. O cenário inicial usa o lote `ex-l03` e cinco ingredientes; suas
datas civis são preenchidas ao abrir o formulário. Preços e estoques são hipóteses do cenário.
A [procedência e as limitações](FONTES.md) explicam o uso dessas referências.

## Complemento de aceite — comparação e cadastros

- A data opcional da composição deve ser civil válida em `AAAA-MM-DD`. Datas inválidas
  legadas exigem correção ou confirmação de remoção ao criar nova revisão.
- O histórico preserva filtros e uma referência fixada entre páginas, inclusive sem JavaScript.
- RF-12 inclui consumo, oferta, custos por ingrediente, revisões de composição, balanço e
  dimensões dos pareceres preservados. Resultado ausente ou associação ambígua é indisponível.
- Datas, versões e toda a estrutura do conhecimento preservado participam da equivalência.
  Diferenças permitem inspeção de custos com aviso específico. A recusa entre lotes ou
  revisões distintos permanece obrigatória; comparar não reexecuta o motor.

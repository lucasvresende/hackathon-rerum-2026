# Auditoria dos 18 fluxos no Chromium

**Data:** 22/10/2026. **Código avaliado:** `5b8b100`.

Foram confirmados dois problemas de interface: revisões indistinguíveis na seleção de
ingredientes (P2) e primeira página quase vazia no PDF da comparação (P3). Não foi identificado
erro aritmético nos cenários conferidos. A suíte completa de navegador passou no Chromium,
assim como 20 testes Rust direcionados. Os dois achados permanecem sem correção nesta auditoria.

A [auditoria anterior](AUDITORIA_FLUXOS.md) registra seis problemas já corrigidos. Esses
problemas antigos não foram contados novamente.

## Ambiente e método

- Build local com `cargo build --locked`, sem mudanças no código da aplicação.
- Chromium instalado pelo Playwright, modo headless, desktop e viewports móveis da suíte.
- Suíte existente em `tests/javascript/e2e/navegadores.mjs`, com todas as suas 14 suítes.
- Inspeção complementar pela CLI do Playwright, com leitura do DOM e screenshots.
- Bancos temporários preparados com `demonstrar`; o banco habitual não foi alterado.
- Inspeção complementar em `http://127.0.0.1:3123`, com banco em
  `/tmp/feedyourcow-auditoria-18.sqlite3` durante a execução.
- PDFs A4 gerados pelo Chromium; extração com Poppler e inspeção visual de páginas selecionadas.
- Conferência independente em Python, lendo entradas e resultados preservados do SQLite.

O Firecrawl hospedado não foi utilizado porque o alvo é um servidor em loopback. O Chromium
precisou de execução fora do sandbox do ambiente. A primeira tentativa de iniciá-lo foi recusada
pelo sandbox; isso não constitui falha da aplicação. Firefox e WebKit não foram executados.

## Cobertura dos fluxos solicitados

| Nº | Fluxo | Exercício e resultado |
| --- | --- | --- |
| 1 | Alimento do zero | Cadastro sintético com composição, fonte e condições; salvo e utilizado. |
| 2 | Alimento de exemplo | Prévia, confirmação, alteração do nome, gravação e uso nos cálculos. |
| 3 | Alimento por cópia | Soja copiada com identidade independente; original preservado. |
| 4 | Modificação de alimento | PB alterada de 48,8% para 45%; ambas as revisões preservadas. |
| 5 | Lote do zero | Dois animais, 550 kg, condições explícitas; verificação e formulação. |
| 6 | Lote de exemplo | Modelo Girolando alterado para três animais e utilizado nos dois modos. |
| 7 | Lote por cópia | Girolando copiado para quatro animais; original segue com um animal. |
| 8 | Modificação de lote | Cópia revisada de 550 para 560 kg; original continua com 550 kg. |
| 9 | Otimização variada | Exemplos, cadastros novos e cópias; preços, estoque, resolução e alvo. |
| 10 | Relatório da otimização | Busca, dimensões, candidata, custos, oferta e restrições. |
| 11 | Verificação variada | Exemplos, novos e cópias; MN/MS, sobras, estoque e amido. |
| 12 | Relatório da verificação | Entradas, balanços, oferta e custos conferidos numericamente. |
| 13 | Comparação | A/B, datas distintas, referência entre páginas e par criado nesta inspeção. |
| 14 | Valores da comparação | Quantidades, nutrientes, custos e diferença econômica conferidos. |
| 15 | Recusa entre lotes | Navegador confirma HTTP 422; Rust também cobre revisões diferentes. |
| 16 | PDF | Pareceres, ficha e comparação gerados; achado de paginação na comparação. |
| 17 | Botões | Cadastros, cenários, filtros, ajuda, expansões, histórico e impressão exercitados. |
| 18 | Reutilização | Duplicar, refazer e verificar candidata contínua/prática preservam origem. |

O item 17 é cobertura dos controles exercitados pelas suítes e pela exploração, não garantia
exaustiva de todo botão em toda combinação de estados.

## Achado 1 — P2: revisões nutricionais indistinguíveis na seleção

**Fluxos afetados:** modificação de alimentos e seleção para verificação/formulação.

**Reprodução:**

1. No catálogo, criar uma cópia do Farelo de soja com nome `QA18 soja copiada`.
2. Salvar a cópia e criar uma revisão mantendo o nome e alterando PB de 48,8% para 45%.
3. Abrir Nova dieta, expandir Selecionar ingredientes e pesquisar `QA18`.

**Observado:** aparecem duas opções com texto exatamente igual:
`QA18 soja copiada · Exemplo sintético`. São composições distintas, com PB distinta, mas o
seletor não exibe versão, data, composição ou link de inspeção para distinguir as opções.
Os controles possuem identificadores diferentes no HTML; essa diferença não é visível ao usuário.

**Esperado:** diferenciar as revisões antes da escolha, com identificação acessível da composição
ou acesso à sua ficha. Preservar revisões antigas é correto; a falha é apresentá-las sem distinção.

**Impacto:** o usuário pode escolher a composição antiga pensando utilizar a análise revisada.
Isso muda a entrada nutricional. Não foi demonstrada troca silenciosa de composição pelo servidor.
Selecionar ambas e preencher os campos obrigatórios foi corretamente recusado com
“Selecione uma composição por ingrediente”.

**Local:** `src/web/cenarios/templates/formularios/selecao_ingredientes.rs`, linhas 61–67.

**Sugestão:** mostrar referência da revisão, data e resumo da análise, com acesso à composição.
A escolha de apresentação deve ser definida antes de implementar a correção.

![Duas revisões com o mesmo rótulo][revisoes]

## Achado 2 — P3: PDF da comparação desperdiça a primeira página

**Fluxos afetados:** comparação e impressão/PDF.

**Reprodução:**

1. Comparar duas execuções da mesma revisão de lote com seis ingredientes.
2. Gerar PDF A4 pelo Chromium, com fundo habilitado e sem cabeçalhos/rodapés adicionais.
3. Conferir a primeira página e o início da segunda.

**Observado:** a primeira página contém somente o título “Comparação de cenários”. A tabela de
premissas começa na segunda página. O documento de teste tem oito páginas e conserva os valores
inspecionados, mas desperdiça quase toda a página inicial.

**Esperado:** iniciar as premissas junto ao título, permitindo quebra adequada do conteúdo longo.

**Causa provável:** `static/app.css`, linha 163, aplica `break-inside: avoid` a todas as `.card`.
A comparação coloca praticamente todo o conteúdo em uma única seção `.card.table-wrap`, no
arquivo `src/web/comparacao/templates/paginas/comparacao.rs`. O agrupamento longo é deslocado
antes de ser fragmentado. A causa é uma inferência do CSS; a página quase vazia foi reproduzida.

**Sugestão:** permitir fragmentação do contêiner longo na impressão e restringir a prevenção de
quebras a blocos curtos, cabeçalhos e linhas quando couberem. Revalidar em A4 com poucos e muitos
ingredientes antes de aplicar uma regra global.

![Primeira página do PDF da comparação][pdf]

## Conferência dos relatórios e valores

Na exploração complementar, o lote copiado e revisado tinha quatro animais de 560 kg. A dieta
utilizou os cinco ingredientes do cenário inicial e 0,1 kg MS da soja copiada. A otimização foi
criada a partir dessa execução, com os mesmos preços e composições.

| Grandeza | Verificação | Otimização |
| --- | --- | --- |
| Consumo kg MS/animal/dia | 17,200000 | 16,854208 |
| Custo R$/animal/dia, precisão interna | 21,0571736718 | 18,6534990705 |
| Custo R$/animal/dia, tela | 21,06 | 18,65 |
| Custo R$/lote/dia, tela | 84,23 | 74,61 |
| Custo em sete dias, tela | 589,60 | 522,30 |
| NDT kg/animal/dia | 11,77940 | 11,19052 |
| PB kg/animal/dia | 2,58000 | 2,31720 |
| Ca kg/animal/dia | 0,10771 | 0,09091 |
| P kg/animal/dia | 0,06482 | 0,05862 |

Foram recalculados por ingrediente: oferta a partir de MS e sobras, necessidade bruta com perdas,
escala por animais e horizonte, custo com preço normalizado e totais dos sete nutrientes.
A conferência coincidiu com os resultados preservados, com tolerância absoluta de `1e-10`.
Na otimização, as quantidades da candidata vieram do resultado salvo; não houve solução
independente do problema linear nem prova externa de mínimo global.

A diferença interna foi `2,4036746013`, apresentada corretamente como **R$ 2,40**, e
`11,4149915785%`, apresentada como **11,41%**. Subtrair os dois valores já arredondados na tela
produz R$ 2,41; isso não é erro de cálculo. A comparação também avisou “Consumo diferente” e
“Alternativa sem fornecimento validado”, evitando tratar menor custo como substituição validada.

## Barreiras e controles que funcionaram

- Quantidades informadas foram conservadas na verificação, mesmo com consumo-alvo diferente.
- Falta de estoque gera pendência; estoque insuficiente impede fornecimento compatível.
- Formulação sem solução não apresenta custo da entrada como custo de candidata.
- Resolução zero distingue quantidades aproximadas de pesagem validada.
- Arredondamento incompatível é recusado sem apagar a solução contínua.
- Sobras, perdas e alteração de MS são preservadas na reutilização.
- Reabrir a execução original após simular conserva seus valores.
- Histórico preserva filtros e referência fixada entre páginas, também sem JavaScript.
- Cadastro rejeita fonte inválida e data civil impossível, preservando os dados para correção.
- Botões de impressão acionam `window.print` com modos distintos; `afterprint` limpa o modo ficha.
  Esse teste observou a chamada, substituindo o diálogo nativo por um registrador.
- A ficha PDF de uma solução incompatível mostra explicitamente que não deve orientar pesagem.

Foram gerados PDFs de verificação (23 páginas), otimização (22 páginas), comparação (oito páginas)
e ficha de fornecimento (uma página). Foram inspecionados visualmente a ficha, a tabela de
fornecimento da verificação e as páginas 1 e 4 da comparação. Não houve revisão visual página a
página dos pareceres completos. A geração usou a API de impressão do Chromium; salvar pelo diálogo
nativo do sistema não foi automatizado.

## Execução e evidências

Comando da suíte completa, usando banco temporário criado pelo próprio runner:

```sh
FEEDYOURCOW_NAVEGADORES=chromium FEEDYOURCOW_URL=http://127.0.0.1:3122 npm run test:e2e
```

Testes Rust direcionados: **20 passaram**, sem falhas:

```sh
cargo test --locked --test comparacao --test comparacao_quantidades \
  --test cadastros_auditoria --test reutilizacao
```

Capturas e PDFs da exploração ficaram em `output/playwright/auditoria-18/`, ignorado pelo Git.
A conferência independente está em `conferencia-numerica.json` nessa pasta. As duas evidências
dos achados acompanham este documento no repositório.

Não foram avaliados desempenho de produção, todos os cruzamentos de parâmetros, conformidade
clínica do perfil ou correção científica integral das referências. Nenhuma alteração funcional
ou correção dos achados foi realizada.

[revisoes]: imagens/auditoria-18-fluxos-2026-10-22/revisoes-indistinguiveis.png
[pdf]: imagens/auditoria-18-fluxos-2026-10-22/comparacao-p1.png

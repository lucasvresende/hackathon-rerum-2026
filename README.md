# FeedYourCow

Sistema especialista local para verificar e formular dietas de vacas adultas em lactação.
Interface em português do Brasil com tema verde-esmeralda, cálculos explicáveis e histórico.
Os requisitos de interface priorizam tarefas, dados e resultados para o operador da fazenda.
FeedYourCow identifica o software; telas, menu e rodapé não devem conter slogans ou publicidade.

## Índice

- [Executar](#executar)
- [Fluxo de uso](#fluxo-de-uso)
- [Ajuda de uso](#ajuda-de-uso)
- [Desenvolvimento](#desenvolvimento)
- [Organização](#organizacao)
- [Cargas e recuperação](#cargas-e-recuperacao)
- [Referências](#referencias)
- [Arquitetura e decisões](#arquitetura-e-decisoes)
- [Validação e rastreabilidade](#validacao-e-rastreabilidade)

## Executar

Requer Rust estável. A compilação usa consultas SQLx verificadas e metadados offline versionados.
Nenhum runtime Node/Python ou solver externo é necessário para usar o aplicativo.

```sh
cargo build --locked
cargo run -- demonstrar
cargo run -- servir
```

Abra <http://127.0.0.1:3000>. `demonstrar` prepara o banco, carrega quatro composições sintéticas,
cria lotes e calcula as dietas A/B, uma pendência e um bloqueio. Dados sintéticos não são análises
nem cotações reais.
Para instalar sem exemplos, use `cargo run -- preparar` no lugar de `demonstrar`.

Configuração por variáveis de ambiente: `FEEDYOURCOW_BANCO` (padrão `dados/aplicacao.sqlite3`),
`FEEDYOURCOW_ENDERECO` (padrão `127.0.0.1:3000`) e `RUST_LOG`. Somente loopback é aceito.
O arquivo `.env.example` serve como referência; exporte as variáveis no ambiente de execução.

<a id="fluxo-de-uso"></a>

## Fluxo de uso

1. Cadastre/revise um lote e confirme os estados fisiológicos conhecidos.
2. Consulte alimentos ou cadastre uma análise com sua fonte e condições de uso.
3. Crie cenário: selecione lote, datas, consumo, ingredientes, preços, estoques e premissas.
4. Verifique quantidades ou solicite formulação de menor custo.
5. Leia as dimensões, balanços, fornecimento, fontes e explicações.
6. Consulte o histórico, duplique para simular, compare e imprima pelo navegador.

A tela “Visão geral” oferece acesso direto a “Verificar dieta” e “Formular dieta”, além de
cadastros e cenários recentes. O menu “Perfil e fontes” reúne o perfil nutricional, coeficientes,
normas e referências. Os textos auxiliares se limitam a orientar operações e interpretar dados.

Atendimento químico não significa adequação nutricional integral. Fibra física, micronutrientes
ou contaminantes sem dados suficientes permanecem identificados. O perfil histórico é
`E2020_NRC1989_RUMEN_MVP_v1`, sem implementação integral de NASEM/BR-LEITE.

## Desenvolvimento

O recurso `desenvolvimento` ativa tower-livereload; a execução normal não inclui autoreload.
Use `bacon` para acompanhar verificações e `bacon servir` para executar com recarga automática.
O motor usa Rust tipado, sem código `unsafe` próprio. Bibliotecas existentes são priorizadas.
As consultas normais usam `query_file!`; SQL administrativo é restrito a cargas locais por CLI.
Os comandos completos de testes estão em
[Validação e rastreabilidade](#validacao-e-rastreabilidade).

<a id="organizacao"></a>

## Organização

- `src/dominio/`: unidades, validações, cálculos, inferência, parecer e otimização.
- `src/aplicacao/`: orquestração, processo de cálculo e exemplos sintéticos.
- `src/persistencia/`: banco, cargas, revisões, histórico e recuperação.
- `src/web/`: rotas, proteção de escrita, formulários e componentes Maud.
- `migrations/`, `queries/`, `.sqlx/`: esquema SQLx, consultas e metadados de compilação.
- `dados_referencia/`, `fontes/`, `hackathon/`: cargas, fontes e anexo numérico preservado.
- `static/`: CSS plano, JavaScript de apresentação e bibliotecas locais.
- `tests/`: casos de aceitação e integração.

O banco ativo fica fora do Git. Cadastros têm revisões; cada execução congela entradas, versões,
fontes e traço em transação. Editar um cadastro não altera resultados anteriores.

<a id="cargas-e-recuperacao"></a>

## Cargas e recuperação

```sh
cargo run -- carregar dados_referencia/001_conhecimento.sql
cargo run -- backup /tmp/feedyourcow-backup
cargo run -- restaurar /tmp/feedyourcow-backup /tmp/feedyourcow-restaurado
```

Cargas têm versão/hash e são idempotentes; colisões de versão são rejeitadas. Nunca edite uma carga
já distribuída.
Backup usa `VACUUM INTO`, preserva documentos locais e gera manifesto de integridade.
Restauração exige diretório novo e verifica hashes, integridade e vínculos do SQLite.

<a id="referencias"></a>

## Referências

`REQUISITOS.md` define o comportamento; o anexo específico em `hackathon/` contém tabelas,
fórmulas e procedência. A implementação foi desenvolvida com auxílio de IA, revisada por
compilador e testes. O cálculo operacional é determinístico e não usa modelos de linguagem.

A documentação de operação, arquitetura, decisões e testes está reunida neste README.
A especificação e os relatórios de pesquisa permanecem como referências nos arquivos indicados.

<a id="ajuda-de-uso"></a>

## Ajuda de uso

Acesse **Ajuda** no menu ou **Ajuda nesta página** para abrir as instruções da tarefa atual.
A página `/ajuda` explica os cadastros, cenários, cálculos, pareceres, simulações, histórico,
comparação e fontes em cartas expansíveis. Use o índice para ir direto ao assunto e clique
no título da carta (ou pressione Enter/Espaço) para expandir ou recolher seu conteúdo.
Links como `/ajuda#resultados` abrem automaticamente a carta relacionada ao parecer.

<a id="arquitetura-e-decisoes"></a>

## Arquitetura e decisões

### Separação de responsabilidades

O monólito separa domínio, aplicação, persistência e web. Tipos carregam a base das quantidades,
conhecimento dos dados e versões. Componentes Maud implementam `Render`; a decisão fica no servidor.
A interface usa htmx 4.0.0 e hx-live 4.0.0, distribuídos localmente sob licença 0BSD.
O tema usa variáveis de cor no início de um único CSS plano, com verde-esmeralda predominante.

A apresentação segue RNF-14: FeedYourCow identifica o software, sem slogans ou banner de promoção.
Os cabeçalhos indicam a tarefa/conteúdo; descrições aparecem apenas quando orientam a operação.
O início oferece atalhos para verificar/formular dietas, cadastros e execuções recentes.
Unidades, hipóteses, fontes, erros e limitações permanecem nos formulários e pareceres.

O motor valida entradas, enquadra o lote, normaliza, deriva fatos, avalia regras independentes e
conclui por dimensão. Uma agenda finita mantém ordem estável e impede reaproveitar fatos antigos.
Lógica desconhecida não equivale a falso; bloqueios não desaparecem após um resultado químico.
Composição ausente mantém pendência de dados e preserva os cálculos de nutrientes independentes.
Os IDs DAT/ESC/LEG/NUT/RUM/CON/FOR/MVP vinculam verificações ao anexo numérico.

### Conhecimento e banco

Migrações SQLx alteram a estrutura; cargas SQL alteram conhecimento e catálogo versionados.
As tabelas A/B/C e políticas vêm do SQLite. Não há arquivo concorrente de parâmetros em JSON.
O JSON tipado congela os objetos completos usados em uma execução; entidades, vínculos, nutrientes,
preços, estoques e verificações também possuem campos relacionais.

A carga administrativa exige versão, hash e transação. Conteúdo diferente com a mesma versão é
rejeitado. Fontes e coeficientes são validados antes do commit. Arquivos de carga são operações
administrativas locais confiáveis, nunca entradas HTTP. Consultas usuais são parametrizadas em
arquivos `.sql`, com macros SQLx verificadas na compilação.

Revisões de lote e composição não são sobrescritas. Execuções, entradas e traços são gravados
atomicamente. Há proteção por chaves estrangeiras e triggers de preservação. O conteúdo das fontes
acompanha cada execução; o hash sozinho não é usado como mecanismo de reconstrução.

### Otimização e precisão

`microlp` 0.6.0 resolve cinco programas lineares contínuos, um por degrau de fibra. O modelo fixa
consumo, composição e coeficientes. Candidatas são conferidas pelo mesmo verificador independente.
Não há relaxamento automático de restrições. Estoque insuficiente e exclusão com mínimo positivo
são conflitos explícitos. Ausência de dados mantém busca inconclusiva.

O cálculo roda em processo filho do mesmo executável Rust. O servidor limita o tempo, encerra o
processo e aguarda sua finalização. Isso evita deixar threads calculando após o timeout e mantém
o executor assíncrono disponível. Um timeout não representa inviabilidade comprovada.

Os limites estrangeiros de contaminantes são parâmetros SQL versionados. Quando ativados,
integram o modelo linear e a revalidação; matéria-prima irregular não é autorizada por diluição.
Um resultado <LOQ insuficiente permanece desconhecido.

O estado de cada subproblema é preservado. A alegação de ótimo depende de todos estarem resolvidos
como ótimos ou inviáveis. Desempate compara custo sem centavos, quantidade de ingredientes positivos
e vetor canônico. Não prova mínimo global de ingredientes entre todas as soluções ótimas.

Grandezas físicas usam `f64` e tolerâncias RN-24. Preços originais usam representação decimal e
conversão por unidade comercial. Arredondamento monetário final: meio para longe de zero.
JSON usa `float_roundtrip` para preservar números ao reabrir dados históricos.
Arredondamento de fornecimento exige revalidação; solução contínua e oferta prática são distintas.

### Operação e segurança

Somente loopback; sem autenticação nem acesso compartilhado no MVP. Escritas exigem POST, token
CSRF e verificação da origem quando enviada. Texto é escapado pelo Maud; corpos têm limite.
Não há interpretador de regras arbitrárias, CDN, APIs ou modelos de linguagem em execução.

Backups usam `VACUUM INTO`, documentos locais e manifesto SHA-256. A restauração exige destino
novo e confere hashes, integridade e referências do banco antes do uso.

### Fontes técnicas consultadas

- [Axum 0.8.9](https://docs.rs/axum/0.8.9/axum/).
- [Maud 0.27.0](https://docs.rs/maud/0.27.0/maud/).
- [SQLx 0.9.0](https://docs.rs/sqlx/0.9.0/sqlx/).
- [microlp 0.6.0](https://docs.rs/microlp/0.6.0/microlp/).
- [htmx v4](https://four.htmx.org/) e
  [hx-live](https://four.htmx.org/extensions/hx-live).

A IA auxiliou leitura, implementação e testes. Dados sintéticos têm identificação explícita.
Nenhuma conclusão operacional é delegada à IA. Limitações científicas seguem `REQUISITOS.md`.

### Ajuda contextual

A rota `/ajuda` renderiza o componente Maud `Ajuda`, separado em `componentes/ajuda.rs`.
Dez cartas explicam visão geral, lotes, alimentos, cenários, verificação, formulação, pareceres,
histórico, comparação e fontes. Cada `Pagina` declara sua seção de ajuda; o layout apresenta
“Ajuda nesta página” com `/ajuda#secao`, inclusive em cadastros, pareceres e telas de erro.

O estado `data-aberta` pertence a cada carta. O hx-live sincroniza `aria-expanded`, o indicador
e a visibilidade do conteúdo; botões nativos permitem usar Enter e Espaço. O fragmento inicial
abre apenas a seção indicada. Mudanças do fragmento (índice e navegação voltar/avançar) abrem a
carta por meio da API `htmx.live.q`, preservando as outras cartas abertas. Sem JavaScript, todo
o conteúdo permanece disponível com títulos; a impressão também inclui as cartas recolhidas.
Não há novas dependências nem requisições para expandir cartas.

Sintaxe conferida na [documentação oficial do hx-live](https://four.htmx.org/extensions/hx-live)
em 20/10/2026: estado compartilhado `data`, bindings `hx-live:*` e API pública `htmx.live.q`.

Os scripts locais htmx e hx-live carregam em ordem no `head`, antes de o documento terminar
de ser analisado. Isso registra a extensão antes da inicialização do htmx 4, inclusive quando
os arquivos chegam em tempos diferentes. O script de apresentação continua usando `defer`.

<a id="validacao-e-rastreabilidade"></a>

## Validação e rastreabilidade

### Executar a validação

O script SQLx confere o cache sem depender da configuração de drivers da SQLx CLI instalada.

```sh
cargo build --locked
cargo test --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo fmt --check
python3 scripts/qualidade.py
python3 scripts/verificar_sqlx.py
npm ci --ignore-scripts
npx playwright install --with-deps chromium firefox webkit
npm test
RUST_LOG=warn cargo run --quiet -- ensaiar
```

As ferramentas Node/Playwright são exclusivas de desenvolvimento. `npm test` cria um banco
isolado, prepara a demonstração, inicia servidor temporário em loopback e limpa esses dados ao
terminar. Não usa o banco operacional. As capturas ficam em `output/playwright/`, fora do Git.
O script registra erros JavaScript e recusa recursos de outras origens durante os fluxos.

O servidor de teste usa a porta 3107. `FEEDYOURCOW_URL` permite escolher outra porta local.
`FEEDYOURCOW_NAVEGADORES=chromium` restringe uma execução de diagnóstico; a entrega usa os três.
Os caminhos `PW_CHROMIUM_EXECUTAVEL`, `PW_FIREFOX_EXECUTAVEL` e `PW_WEBKIT_EXECUTAVEL` são opcionais
para inspeção com instalações locais. A validação final usa as versões do pacote fixado.

`verificar_sqlx.py` recria um banco de compilação em diretório temporário, aplica os arquivos de
migração e compara os metadados recém-gerados com `.sqlx`. Compilar offline não desativa essa
verificação de compatibilidade. Migrações/cargas operacionais são exercitadas pelos testes Rust.

Validação local inicial em 20/10/2026: 34 testes Rust, fluxos nos três navegadores e checks de
build, clippy, formatação, texto e 26 consultas SQLx passaram. O GitHub ainda não executou o
workflow.

### Matriz dos casos de aceitação

| Caso | Evidência automatizada |
| --- | --- |
| CT-01 | Conversão MN/MS, nutriente e normalização decimal de preços. |
| CT-02 | LCG4, consumo e requisitos numéricos do exemplo. |
| CT-03 | Interpolação linear e bilinear com valores conhecidos. |
| CT-04 | Nós, bordas, ausência de extrapolação e aviso de consumo. |
| CT-05 | Não finitos, positivos obrigatórios, desconhecidos e lógica trivalente. |
| CT-06 | Dieta A e acumulação de violações químicas e ruminais. |
| CT-07 | Dieta B, nutrientes e dimensões sem avaliação. |
| CT-08 | Degraus de fibra, inclusive 18,7% e faixa abaixo de 15%. |
| CT-09 | Bloqueio legal, exceção e exclusão independente pelo MVP. |
| CT-10 | Fronteiras de vigência e ativação explícita da referência estrangeira. |
| CT-11 | Bases de umidade, LOQ, laudos e restrições da otimização. |
| CT-12 | Fibra física incompatível não impede cálculo químico nem recebe aprovação. |
| CT-13 | Cinco subproblemas, ótimo conhecido e revalidação independente. |
| CT-14 | Estados da busca, falha, interrupção e cancelamento de processo real. |
| CT-15 | Inclusão obrigatória, exclusão, rótulo, estoque e não diluição de ingrediente. |
| CT-16 | Mesma solução ao permutar ingredientes e agregação determinística. |
| CT-17 | Revalidação do fornecimento arredondado em verificação e formulação. |
| CT-18 | Custos A/B, sobras e demonstração calculada pela CLI. |
| CT-19 | Escala por animais/horizonte e necessidade bruta. |
| CT-20 | MS ou MN fixas, custo, simulação e comparação no navegador. |
| CT-21 | Revisar lote/composição preserva execução anterior. |
| CT-22 | Reprodução com mesmas entradas, versões, valores e traço. |
| CT-23 | Carga idempotente, colisão, fontes, unidades e ativação atômica. |
| CT-24 | Falha injetada no traço reverte a gravação da execução. |
| CT-25 | Backup/restauração reproduz resultado e impede sobrescrita. |
| CT-26 | Fluxos Playwright nos três navegadores, incluindo foco, mobile e impressão. |
| CT-27 | CSRF/origem, escape, base ausente, loopback e ausência de recursos externos. |
| CT-28 | Build, testes, clippy, formatação, SQLx, instalação e ensaio de desempenho. |
| CT-29 | Títulos funcionais, ações de cálculo diretas, sem slogans/banner; desktop e mobile. |
| CT-30 | Ajuda contextual, cartas hx-live, teclado, fragmentos, impressão e uso sem JavaScript. |

Os testes Rust estão agrupados em `nucleo`, `cenarios`, `otimizacao`, `integracao` e `entrega`.
A matriz vincula obrigações aos testes; não representa prova científica ou cobertura de todas as
combinações possíveis. Novas fontes/perfis exigem novos testes e revisão de requisitos.

### Inspeção visual

Capturas de início, catálogo, erro, resultado, impressão e telas de 390 px são produzidas para
Chromium, Firefox e WebKit. Foram verificados o tema verde-esmeralda, legibilidade, estados por
texto, rolagem das tabelas e preservação do foco após erro de fragmento.

CT-29 verifica títulos funcionais nas telas de início, lotes, composição, cenário, resultado,
histórico, comparação e perfil/fontes. Os atalhos “Verificar dieta” e “Formular dieta” devem abrir
o formulário com o modo correspondente selecionado. O teste também verifica a remoção dos slogans
anteriores e das estruturas promocionais. O catálogo e a visão geral não devem transbordar em
390 px; a inspeção das capturas confere a apresentação sem banner e sem subtítulos redundantes.
Na revisão de RNF-14 em 20/10/2026, CT-29 e os fluxos completos passaram nos três navegadores;
os 34 testes Rust, build, clippy, formatação e limite de linhas também passaram.
Falhas no roteiro de navegador registram captura e conteúdo da tela para diagnóstico.

A impressão é verificada por emulação de mídia, com menu oculto e conteúdo do parecer preservado.
O acesso aos recursos locais foi verificado com bloqueio de solicitações externas.

### Desempenho

`ensaiar` prepara banco próprio com ingredientes sintéticos de teste, executa cinco aquecimentos
e mede 100 verificações com 20 ingredientes e 100 formulações com 10 ingredientes. Os tempos
incluem cálculo, comunicação com o processo e persistência. As formulações devem terminar com
ótimo validado; resposta inconclusiva rápida não conta como sucesso do ensaio.

Ambiente de referência: Ubuntu 26.04.1 x86_64, Intel Core i7-8550U, quatro núcleos/oito threads,
Rust 1.98.1, perfil debug. O ensaio final obteve p95 aproximado de 23 ms para verificação e
24 ms para formulação. Os valores medidos dependem do ambiente; o comando falha se p95 ultrapassar
1 segundo ou 5 segundos, respectivamente. A saída JSON permite registrar novas medições.

### Integração contínua

`.github/workflows/ci.yml` reproduz os checks de Rust, texto, SQLx e navegadores, preservando
capturas como artefato. O workflow está configurado para push e pull request; a execução remota
no GitHub depende de publicar os commits. A validação desta entrega é local.

### Ajuda contextual (CT-30)

O teste Rust `ajuda_disponivel_com_secoes_e_links_contextuais` verifica a rota, as dez seções,
os controles acessíveis e os links de ajuda em páginas completas e no erro 404.
`tests/ajuda.mjs`, executado pelo roteiro existente em Chromium, Firefox e WebKit, verifica
expansão/recolhimento de todas as cartas, Enter/Espaço, isolamento do estado, links com fragmento,
índice, retorno do navegador, recarga, conteúdo sem JavaScript e ausência de transbordamento
em 390 px. O roteiro também confere os destinos contextuais nos cadastros, cenário, resultado,
histórico, comparação e fontes. Capturas `*-ajuda.png` e `*-ajuda-mobile.png` apoiam a inspeção.

Validação da ajuda em 20/10/2026: build, 35 testes Rust, clippy com todas as features,
formatação e fluxos completos nos três navegadores passaram. Inspeção visual das capturas de ajuda
em desktop e mobile confirmou legibilidade e expansão. Todos os arquivos desta implementação
respeitaram 100 caracteres por linha. Naquela validação, o check global apontava uma linha
longa em uma alteração preexistente de `hackathon/SEQUENCIA_IMPLEMENTACAO.md`, preservada
na entrega.

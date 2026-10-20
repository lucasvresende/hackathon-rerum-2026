# Validação e rastreabilidade

## Executar a validação

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

Validação local final em 20/10/2026: 34 testes Rust, fluxos nos três navegadores e checks de
build, clippy, formatação, texto e 26 consultas SQLx passaram. O GitHub ainda não executou o
workflow.

## Matriz dos casos de aceitação

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

Os testes Rust estão agrupados em `nucleo`, `cenarios`, `otimizacao`, `integracao` e `entrega`.
A matriz vincula obrigações aos testes; não representa prova científica ou cobertura de todas as
combinações possíveis. Novas fontes/perfis exigem novos testes e revisão de requisitos.

## Inspeção visual

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

## Desempenho

`ensaiar` prepara banco próprio com ingredientes sintéticos de teste, executa cinco aquecimentos
e mede 100 verificações com 20 ingredientes e 100 formulações com 10 ingredientes. Os tempos
incluem cálculo, comunicação com o processo e persistência. As formulações devem terminar com
ótimo validado; resposta inconclusiva rápida não conta como sucesso do ensaio.

Ambiente de referência: Ubuntu 26.04.1 x86_64, Intel Core i7-8550U, quatro núcleos/oito threads,
Rust 1.98.1, perfil debug. O ensaio final obteve p95 aproximado de 23 ms para verificação e
24 ms para formulação. Os valores medidos dependem do ambiente; o comando falha se p95 ultrapassar
1 segundo ou 5 segundos, respectivamente. A saída JSON permite registrar novas medições.

## Integração contínua

`.github/workflows/ci.yml` reproduz os checks de Rust, texto, SQLx e navegadores, preservando
capturas como artefato. O workflow está configurado para push e pull request; a execução remota
no GitHub depende de publicar os commits. A validação desta entrega é local.

## Ajuda contextual (CT-30)

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
respeitam 100 caracteres por linha. O check global de texto ainda aponta uma linha de 122
caracteres em `hackathon/SEQUENCIA_IMPLEMENTACAO.md:18`, em alteração preexistente preservada.

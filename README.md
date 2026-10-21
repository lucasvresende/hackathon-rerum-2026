# FeedYourCow

Sistema especialista local para verificar e formular dietas de vacas adultas em lactação.
Interface em português do Brasil com tema verde-esmeralda, cálculos explicáveis e histórico.
Os requisitos de interface priorizam tarefas, dados e resultados para o operador da fazenda.
FeedYourCow identifica o software; telas, menu e rodapé não devem conter slogans ou publicidade.

O [plano revisado de melhorias](hackathon/PLANO_MELHORIAS_V2.md) detalha as sugestões de
`hackathon/NOTAS_MELHORIAS.md`, os modelos com fontes, os testes e as decisões pendentes.
A implementação foi retomada após aprovação; as funcionalidades estão descritas abaixo.

## Índice

- [Executar](#executar)
- [Build de produção](#build-de-producao)
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

<a id="build-de-producao"></a>

## Build de produção

O pacote é destinado ao uso local previsto no MVP e compilado para o sistema/arquitetura da
máquina de build. O setup usa Rust/Cargo estáveis, compilador C e linker exigidos pelas
dependências,
mais ferramentas usuais de Linux: `sh`, `tar`, `gzip` e `sha256sum`. Não exige Node nem Python
para compilar ou usar a distribuição. Python 3.12+ é usado apenas no teste opcional do pacote.
Não há cross-compilation nem promessa de binário estático: a instalação exige um Linux compatível
com o alvo e as bibliotecas de sistema da máquina de build, incluindo a versão da glibc no alvo GNU.

```sh
sh scripts/build_producao.sh
```

O script pode ser chamado de qualquer diretório. Ele usa `--release --locked --no-default-features`,
fixa o alvo nativo e o diretório de compilação `target/`, e força `SQLX_OFFLINE=true` para validar
consultas com o cache `.sqlx/`, sem banco operacional durante o build. O perfil `release` usa
otimização padrão de nível 3, ThinLTO, uma unidade de geração de código e remoção de símbolos.
O recurso `desenvolvimento` e o autoreload ficam desativados.
Essas opções seguem a [documentação oficial de perfis do Cargo][perfis-cargo].
O build otimizado permanece local; o GitHub Actions não o executa nesta etapa.

[perfis-cargo]: https://doc.rust-lang.org/cargo/reference/profiles.html

A saída é `dist/feedyourcow-<alvo>.tar.gz` e seu arquivo `.sha256`; por exemplo,
`dist/feedyourcow-x86_64-unknown-linux-gnu.tar.gz`. Repetir o build substitui somente esses
artefatos gerados. O pacote contém binário, `iniciar.sh`, `static/` com licenças das bibliotecas,
cargas SQL, migrações, manifesto de fontes, anexo numérico, requisitos, README, `.env.example`
e `BUILD.txt` com versões do compilador e do Cargo. Migrações e cargas de preparação já estão
embutidas no executável; os arquivos acompanham o pacote para consulta e cargas administrativas.
Bancos, backups, `.env`, código-fonte da aplicação e ferramentas de desenvolvimento não entram.
O diretório `dist/` fica fora do Git.

### Instalar e iniciar

Confira o checksum e extraia em um diretório novo e gravável. Exemplo para o alvo Linux x86-64:

```sh
cd dist
sha256sum -c feedyourcow-x86_64-unknown-linux-gnu.tar.gz.sha256
mkdir instalacao
tar -xzf feedyourcow-x86_64-unknown-linux-gnu.tar.gz -C instalacao
./instalacao/feedyourcow/iniciar.sh preparar
./instalacao/feedyourcow/iniciar.sh servir
```

Abra <http://127.0.0.1:3000> e encerre com Ctrl+C. `preparar` aplica migrações e carrega o
conhecimento, sem os exemplos sintéticos. Para uma avaliação com exemplos, use `demonstrar`
em uma instalação de teste. Não é necessário ter Rust/Cargo instalado na máquina de execução.
O wrapper resolve os arquivos estáticos e o banco padrão em relação à instalação, mesmo quando
chamado de outro diretório; executar o binário diretamente exige estar na raiz do pacote.
Ele repassa todos os comandos administrativos, como `backup`, `restaurar` e `carregar`.

O padrão grava em `feedyourcow/dados/aplicacao.sqlite3`. Para separar os dados da instalação,
exporte um caminho absoluto antes de preparar e iniciar:

```sh
export FEEDYOURCOW_BANCO="$HOME/feedyourcow-dados/aplicacao.sqlite3"
export FEEDYOURCOW_ENDERECO=127.0.0.1:3000
export RUST_LOG=info
./instalacao/feedyourcow/iniciar.sh preparar
./instalacao/feedyourcow/iniciar.sh servir
```

`.env.example` é uma referência, não um arquivo carregado automaticamente. O endereço continua
restrito a loopback; o pacote não acrescenta autenticação ou publicação para outros computadores.
Antes de atualizar uma instalação, encerre o servidor, faça backup com a versão anterior,
extraia a nova versão em outra pasta e mantenha o caminho absoluto do banco. Execute `preparar`
para aplicar migrações/cargas antes de `servir`; não substitua uma carga versionada já distribuída.
Preserve o backup e o pacote anterior; recuperar um esquema anterior exige restaurar o backup
em um destino novo, em vez de executar um binário antigo sobre um banco já migrado.

### Testar a distribuição

```sh
python3 scripts/testar_producao.py dist/feedyourcow-x86_64-unknown-linux-gnu.tar.gz
```

Ajuste o nome ao alvo gerado. O teste confere checksum e conteúdo do pacote, extrai em pasta
temporária fora do checkout e valida preparação idempotente, histórico da demonstração,
backup/restauração, formulação no processo filho e HTTP com recursos estáticos locais.
O ensaio exercita o binário otimizado; o teste também confirma ausência de autoreload no HTML.
O servidor usa uma porta livre de loopback e é encerrado ao final, sem usar o banco operacional.
Os testes de interface existentes podem ser executados contra o binário otimizado no checkout:

```sh
FEEDYOURCOW_BINARIO="$PWD/target/x86_64-unknown-linux-gnu/release/feed-your-cow" npm test
```

Validação local em 20/10/2026: pacote Linux x86-64 gerado, checksum conferido e teste isolado
concluído, incluindo preparação, backup/restauração, motor e HTTP. O binário `release` também
passou nos fluxos de Chromium, Firefox e WebKit; os 35 testes Rust, clippy, formatação e qualidade
textual passaram. O setup não alterou o workflow do CI.

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
- `src/aplicacao/`: orquestração, processo de cálculo e exemplos sintéticos; serviços de
  cadastros, modelos de preenchimento, cenários, histórico e comparação econômica.
- `src/persistencia/`: banco, cargas, revisões, histórico e recuperação.
- `src/web/rotas/`: grupos de rotas por recurso, compostos com `Router::merge`.
- `src/web/handlers/`: adaptação HTTP por recurso; `entradas/` decodifica formulários.
  `estado.rs`, `erro.rs`, `resposta.rs` e `seguranca.rs` concentram estado, falhas,
  redirecionamentos htmx/HTTP e proteção de escrita. `apresentacao.rs` monta respostas
  de formulário completas ou parciais para os handlers que compartilham esses fluxos.
- `src/web/templates/`: apresentação em structs com `maud::Render`, separada em `layouts/`
  (principal e cadastro), `paginas/`, `formularios/` e `componentes/` reutilizáveis.
  Templates recebem dados preparados; não consultam o banco nem executam operações de negócio.
  Os módulos HTTP anteriores, `web::componentes` e `web::formularios` têm reexportações
  de compatibilidade; código novo usa `web::templates` e `web::entradas`.
- `migrations/`, `.sqlx/`: esquema SQLx e metadados de compilação.
- `queries/`: consultas agrupadas por recurso: `lotes/`, `alimentos/`, `cenarios/`,
  `execucoes/`, `modelos_preenchimento/`, `conhecimento/` e operações de `banco/`.
  Cada consulta continua em arquivo próprio, validado pelas macros SQLx; as pastas não
  alteram o SQL, o esquema ou as cargas versionadas de `dados_referencia/`.
- `dados_referencia/`, `fontes/`, `hackathon/`: cargas, fontes e anexo numérico preservado.
- `static/`: CSS plano, JavaScript de apresentação e bibliotecas locais.
- `tests/*.rs`: testes Rust, descobertos automaticamente pelo Cargo.
- `tests/javascript/e2e/`: roteiros Playwright; veja [testes JavaScript](tests/javascript/README.md).
- `scripts/`: verificações, build local de produção, inicialização e teste da distribuição.
- `dist/`: pacotes de produção gerados, excluídos do Git.

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
A requisição percorre `rotas → handlers → entradas/aplicacao/persistencia → templates`.
Handlers decodificam HTTP, validam a proteção de escrita e escolhem a resposta. `entradas/`
converte o rascunho em dados tipados e preserva valores para correção de erros. A aplicação
associa procedência aos cadastros, prepara substituições de modelos, escolhe exemplos explícitos,
executa/duplica/reproduz cenários, filtra datas e calcula equivalência/economia entre snapshots.
Os serviços recebem dados tipados e `SqlitePool`, sem `State`, cabeçalhos, HTML ou mapas HTTP.
Consultas simples de leitura podem ser chamadas diretamente pelos handlers.

`templates/` contém apenas apresentação. Cada página, formulário, layout e componente visual é
uma struct que implementa `Render`; recebe dados preparados e não
consulta o banco. `layouts::Principal` inclui o menu; `layouts::Cadastro` mantém a guia isolada.
`formularios::lote::FormularioLote` e `formularios::alimento::FormularioAlimento` compartilham
campos e metadados de cadastro, enquanto `formularios::cenario` compõe seus ingredientes.
`apresentacao.rs` seleciona formulário parcial ou página completa; `resposta.rs` conserva os
redirecionamentos HTTP e htmx. As rotas, status, dados persistidos e snapshots foram preservados.

Os modelos de preenchimento são referências de domínio, distintos dos templates HTML Maud.
Sua seleção, lacunas e procedência ficam em `aplicacao::modelos_preenchimento`; o componente
`templates::componentes::modelos_preenchimento` apresenta a seleção e a referência recebida.
Migrações, cargas de referência e consultas não mudam de responsabilidade com essa separação.

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

A rota `/ajuda` renderiza o componente Maud `Ajuda`, separado em `templates/componentes/ajuda.rs`.
Dez cartas explicam visão geral, lotes, alimentos, cenários, verificação, formulação, pareceres,
histórico, comparação e fontes. O cabeçalho de cada página declara sua seção de ajuda e apresenta
“Ajuda nesta página” com `/ajuda#secao`, inclusive em cadastros, pareceres e telas de erro.
O botão fica na mesma linha do título, alinhado à direita e centralizado verticalmente.
Títulos longos quebram dentro do espaço disponível, preservando o botão na mesma linha no celular.

O estado `data-aberta` pertence a cada carta. O hx-live sincroniza a classe `help-open`,
`aria-expanded`, o indicador e `inert` no conteúdo fechado, impedindo foco em links recolhidos.
CSS puro anima abertura e fechamento por 180 ms com grid e opacidade, sem altura máxima fixa.
A classe `help-ready`, também controlada por hx-live após o primeiro frame, evita animar
os fechamentos da carga inicial. A preferência por movimento reduzido desativa a transição;
a impressão exibe todas as cartas.
Botões nativos permitem usar Enter e Espaço. O fragmento inicial
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
npm run test:e2e
RUST_LOG=warn cargo run --quiet -- ensaiar
```

As ferramentas Node/Playwright são exclusivas de desenvolvimento. `npm test` permanece como
atalho de `npm run test:e2e`. A suíte cria um banco
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
| CT-31 | Templates, procedência, conversões, backup, nova guia e preenchimento editável. |

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

`.github/workflows/ci.yml` separa o job Rust (texto e SQLx incluídos) do job JavaScript E2E.
O segundo recebe o binário compilado pelo primeiro e executa os três navegadores, preservando
capturas como artefato. O workflow está configurado para push e pull request; a execução remota
no GitHub depende de publicar os commits. A validação desta entrega é local.

### Ajuda contextual (CT-30)

O teste Rust `ajuda_disponivel_com_secoes_e_links_contextuais` verifica a rota, as dez seções,
os controles acessíveis e os links de ajuda em páginas completas e no erro 404.
`tests/javascript/e2e/ajuda.mjs`, executado pelo roteiro existente em Chromium, Firefox e WebKit, verifica
expansão/recolhimento de todas as cartas, Enter/Espaço, isolamento do estado, links com fragmento,
índice, retorno do navegador, recarga, conteúdo sem JavaScript e ausência de transbordamento
em 390 px. Também confere a classe de abertura, altura recolhida, `inert` e movimento reduzido.
O roteiro também confere o alinhamento entre título e botão de ajuda, inclusive com título longo
em 390 px, e os destinos contextuais nos cadastros, cenário, resultado,
histórico, comparação e fontes. Capturas `*-ajuda.png` e `*-ajuda-mobile.png` apoiam a inspeção.

Validação da ajuda em 20/10/2026: build, 35 testes Rust, clippy com todas as features,
formatação e fluxos completos nos três navegadores passaram. Inspeção visual das capturas de ajuda
em desktop e mobile confirmou legibilidade e expansão. Todos os arquivos desta implementação
respeitaram 100 caracteres por linha. Naquela validação, o check global apontava uma linha
longa em uma alteração preexistente de `hackathon/SEQUENCIA_IMPLEMENTACAO.md`, preservada
na entrega.

### Melhorias dos formulários

Campos obrigatórios apresentam `*`. Restrições HTML e a API nativa de validação dão feedback
antes do envio; o servidor continua validando POSTs, inclusive sem JavaScript. As mensagens
ficam acima do controle, ligadas por `aria-describedby`, com resumo e links para correção.
Inteiros usam passo 1; condições fora do perfil não se confundem com valores malformados.
`tests/melhorias.rs` cobre renderização acessível, associação de erros e rejeição sem gravação.

Criação e revisão de lotes, alimentos e cenários abrem em nova guia sem menu principal.
Salvar um cadastro mostra confirmação; cancelar não grava. Ao retornar ao cenário original,
as opções são atualizadas após a sinalização entre guias, mantendo os valores do formulário.
O botão “Atualizar cadastros” oferece o mesmo fluxo manual, inclusive sem JavaScript.
Novo cenário exige seleção explícita; as dietas sintéticas A/B têm links de demonstração próprios.

### Templates editáveis e preenchimento simplificado

`preparar` aplica a migração `0004_modelos.sql` e a carga `004_modelos.sql`. No cadastro,
escolha um template, abra a prévia, confirme a substituição e use “Carregar template para editar”.
Os campos continuam editáveis; salvar cria uma revisão do cadastro e preserva o template original.
Não é necessário JavaScript para consultar a prévia, carregar, editar e salvar.

A primeira carga inclui silagem de milho 30–35% MS e milho em grão da Feedipedia, farelo de soja
da Embrapa e um modelo parcial de Jersey baseado em estudo identificado. Fontes, páginas,
unidades, conversões e limitações estão em [fontes/MODELOS.md](fontes/MODELOS.md).
Jersey é exemplo de população estudada, com peso fora do perfil; não é média geral da raça.
NDT ausente na Feedipedia e amido ausente na tabela de soja continuam desconhecidos.
Nenhum template confirma autorização, condições fisiológicas, disponibilidade ou ausência de risco.

As tabelas `modelos_preenchimento` e `valores_modelo` armazenam versões imutáveis e referências
à tabela `fontes`. Rust valida campos e unidades da carga. JavaScript não mantém uma cópia
independente das tabelas nutricionais. Cada cadastro preserva o modelo, valores de referência,
valores utilizados e alterações por campo. O servidor reconstrói essa procedência a partir
do modelo do banco; referências enviadas pelo navegador não são aceitas como fonte científica.
Alterações de MS no cenário também atualizam o valor utilizado no snapshot de procedência.
Backup inclui dados, versões e procedência no SQLite, além do manifesto de fontes e da carga.

“Mostrar campos complementares” alterna a apresentação do mesmo formulário. Os valores são
preservados; condições e informações necessárias continuam disponíveis. Campos inválidos dentro
de uma seção recolhida são revelados para correção. As expansões usam o componente Maud
`Expansivel`, hx-live e CSS com movimento reduzido, impressão e conteúdo aberto sem JavaScript.

O cabeçalho usa uma cópia local do SVG de vaca solicitado; procedência, licença e atribuição
estão em [static/README.md](static/README.md). Tabelas destacam linhas no hover e no foco.
A carga de templates não altera o perfil nutricional nem os exemplos sintéticos A/B.

### Verificação das melhorias

A validação local executou 40 testes Rust, build com lockfile, clippy com todos os alvos/features,
rustfmt, limite de 100 caracteres e conferência de 28 consultas SQLx contra banco limpo.
Os testes novos de persistência exercitam aplicação sem salvar, confirmação de substituição,
edição sem alterar a referência, lacunas, carga inválida/transacional e restauração de procedência.
O ensaio com 100 execuções após aquecimento atingiu as metas: p95 de cerca de 28 ms para
verificação com 20 ingredientes e 23 ms para formulação com 10, no ambiente local em debug.

A suíte Playwright inclui templates nos três navegadores, uso sem JavaScript, formulários sem
menu, nova guia, atualização de cadastros sem perder o cenário, erros em português, expansão
com teclado, movimento reduzido, impressão, layout móvel e unicidade dos IDs.
As capturas ficam em `output/playwright/`; recursos externos e erros JavaScript causam falha.
A inspeção visual inclui o formulário, erro acima do campo, versão móvel e ícone local.

Para atualizar uma instalação existente, encerre o servidor, execute `cargo run -- preparar`
e inicie novamente com `cargo run -- servir`. A preparação aplica a migração e carga de modelos;
o comando de servir continua recusando banco com estrutura incompatível.


### Verificação da refatoração

A separação de apresentação e serviços acrescentou seis testes comportamentais:

- `tests/comparacao.rs`: critérios de equivalência, diferença absoluta, custo desconhecido e
  ausência de percentual quando o custo de referência é zero.
- `tests/servicos_modelos.rs`: aplicação parcial sem gravação, confirmação, lacunas, estados
  desconhecidos, unidades e representação da procedência nos snapshots existentes.
- `tests/contratos_http.rs`: página completa/fragmento, rascunho preservado, redirecionamento
  HTTP/htmx, proteção de escrita e rota inválida.

Os testes anteriores continuam cobrindo revisões imutáveis, reprodução, persistência, backup,
validação, motor e apresentação. As consultas apenas mudaram de pasta; o cache `.sqlx/`,
o esquema, as migrações e as cargas de referência foram mantidos.

Validação local em 21/10/2026: 46 testes Rust, build, clippy com todos os alvos/features,
rustfmt, limite de 100 caracteres e 28 consultas SQLx contra banco limpo passaram.
Os fluxos completos de Chromium, Firefox e WebKit passaram, com inspeção visual das capturas
em desktop e mobile. O ensaio isolado de 100 execuções após aquecimento atingiu as metas,
com p95 de 12,6 ms na verificação (20 ingredientes) e 19,5 ms na formulação (10 ingredientes),
no ambiente local em debug. A execução remota do CI depende da publicação dos commits.

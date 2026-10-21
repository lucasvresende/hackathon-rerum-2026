# FeedYourCow

Sistema especialista local para verificar e formular dietas de vacas adultas em lactação.
Oferece cadastros de lotes e alimentos, cálculos explicáveis, comparação de cenários e histórico.
A interface é em português do Brasil e funciona sem internet após a instalação.

O perfil `E2020_NRC1989_RUMEN_MVP_v1` avalia nutrientes e restrições específicas; não implementa
integralmente NASEM/BR-LEITE. Atendimento químico não significa adequação nutricional integral:
dimensões sem dados suficientes, como fibra física e micronutrientes, permanecem identificadas.
O perfil aceita PV de 300 a 800 kg. As tabelas de referência cobrem apenas 400–700 kg para
exigências e 400–600 kg para consumo; nas extensões, o sistema usa extrapolação linear e exibe
um aviso de menor precisão. O consumo-alvo é opcional e, quando informado, tem precedência.

## Índice

- [Executar](#executar)
- [Usar a aplicação](#usar-a-aplicação)
- [Banco de dados e recuperação](#banco-de-dados-e-recuperação)
- [Distribuição de produção](#distribuição-de-produção)
- [Desenvolvimento e arquitetura](#desenvolvimento-e-arquitetura)
- [Testes e qualidade](#testes-e-qualidade)
- [Documentação](#documentação)

## Executar

Requer Rust/Cargo estáveis e compilador C/linker para as dependências. Na raiz do repositório:

```sh
cargo build --locked
cargo run -- preparar
cargo run -- servir
```

Abra [http://127.0.0.1:3000](http://127.0.0.1:3000) e encerre com Ctrl+C.
`preparar` aplica migrações e carrega o conhecimento, seis lotes de um animal e quinze alimentos
sintéticos, incluindo duas misturas. Na Visão geral, use **Abrir cenário de exemplo** para calcular
sem cadastrar ou preencher dados previamente. Os exemplos seguem o anexo A de `REQUISITOS.md`.
`demonstrar` acrescenta as fixtures históricas A/B, aviso de extrapolação e bloqueio para regressão
e inspeção.
Esses dados não representam análises ou cotações reais.

A compilação usa metadados SQLx versionados em `.sqlx/`, sem exigir um banco operacional.
O aplicativo não precisa de Node, Python ou solver externo para funcionar.

### Configuração

Exporte as variáveis no ambiente antes de executar os comandos:

| Variável | Padrão | Finalidade |
| --- | --- | --- |
| `FEEDYOURCOW_BANCO` | `dados/aplicacao.sqlite3` | Caminho do banco SQLite. |
| `FEEDYOURCOW_ENDERECO` | `127.0.0.1:3000` | Endereço e porta do servidor. |
| `RUST_LOG` | `info` | Nível de logs. |

O arquivo `.env.example` é uma referência e não é carregado automaticamente.
O servidor aceita apenas loopback; o MVP não inclui autenticação ou acesso compartilhado.

## Usar a aplicação

1. Cadastre ou revise um lote, informando seus estados fisiológicos.
2. Consulte o catálogo ou cadastre uma composição com fonte e condições de uso.
3. Crie um cenário com lote, datas, consumo, ingredientes, preços, estoques e premissas.
4. Informe quantidades para verificar a dieta ou solicite uma formulação de menor custo.
5. Consulte balanços, restrições, fornecimento, custo, fontes e explicações do resultado.
6. Use o histórico para reproduzir, duplicar, simular, comparar e imprimir cenários.

Os formulários apresentam todos os campos, sem modo de preenchimento simplificado.
As seções expansíveis continuam disponíveis para organizar grupos de informações.
Use **Preencher com exemplo** em um formulário ou **Criar cópia** em um cadastro existente.
Exemplos permitem edição e remoção como os demais registros. **Restaurar exemplos** oferece cópia
independente ou restauração do registro inicial com nova revisão, mediante confirmação explícita.
O original versionado e o histórico são preservados. Reiniciar ou repetir `preparar` não desfaz
alterações nem recria exclusões; a inicialização dos registros e seu marcador são atômicos.
Valores nutricionais, preços e condições sintéticas são hipóteses de demonstração. A procedência
acompanha o cadastro e o parecer; a ausência de laudo continua desconhecida. Para suplementos
sintéticos, o máximo de inclusão explícito do cenário pode ser um teto demonstrativo, identificado
no parecer; não representa rótulo comercial. Produtos reais continuam exigindo limite de rótulo.
Cadastros abrem em nova guia; ao retornar, o cenário atualiza as opções e preserva os campos.
“Atualizar cadastros” oferece a atualização manual.

Acesse **Ajuda** no menu para consultar as instruções. **Ajuda nesta página** abre a seção
correspondente em uma nova guia, mantendo a tarefa atual aberta.
Os indicadores **?** explicam conceitos na Visão geral, no cenário, no catálogo/cadastro de
alimentos e nos resultados. Passe o mouse, use o foco do teclado ou toque/clique para ler;
Escape ou clique fora fecha a dica. As cartas correspondentes da Ajuda trazem mais detalhes
e exemplos de unidades, consumo, dados e interpretação do parecer.
Em **Perfil e fontes**, consulte os coeficientes, normas e referências utilizados.
Os fluxos essenciais também estão disponíveis sem JavaScript.

## Banco de dados e recuperação

O banco ativo fica fora do Git. Lotes e composições têm revisões; cada execução preserva
entradas, versões, fontes e traço em uma transação. Alterações posteriores nos cadastros
não modificam o histórico.

- `migrations/` contém mudanças de estrutura aplicadas pelo SQLx.
- `dados_referencia/` contém cargas versionadas de conhecimento e catálogo.
- `queries/` contém consultas parametrizadas em arquivos SQL, verificadas por macros SQLx.

`preparar` pode ser repetido para aplicar migrações e cargas pendentes.
Cargas têm versão e hash; reaplicações idênticas são aceitas e colisões são rejeitadas.
Para alterar conhecimento já distribuído, crie uma nova carga em vez de editar a anterior.
Uma carga administrativa local também pode ser aplicada pela CLI:

```sh
cargo run -- carregar dados_referencia/001_conhecimento.sql
```

### Backup e restauração

```sh
cargo run -- backup /tmp/feedyourcow-backup
cargo run -- restaurar /tmp/feedyourcow-backup /tmp/feedyourcow-restaurado
```

Use diretórios novos. O backup usa `VACUUM INTO`, inclui documentos locais e gera manifesto
SHA-256. A restauração verifica hashes, integridade e vínculos do SQLite antes de concluir.
Para usar o banco restaurado, configure `FEEDYOURCOW_BANCO` para o arquivo no novo diretório.

## Distribuição de produção

O build requer, além das ferramentas de compilação, `sh`, `tar`, `gzip` e `sha256sum`:

```sh
sh scripts/build_producao.sh
```

O script gera `dist/feedyourcow-<alvo>.tar.gz` e seu arquivo `.sha256`, com build
`--release --locked --no-default-features` e SQLx offline. O autoreload fica desativado.
O pacote inclui executável, inicializador, recursos estáticos, migrações, cargas e documentação;
bancos, backups e ferramentas de desenvolvimento ficam de fora.

A distribuição é nativa para o sistema e a arquitetura da máquina de build. Para Linux,
a máquina de destino precisa ter bibliotecas compatíveis, incluindo glibc quando aplicável.
Não é necessário instalar Rust/Cargo para executar o pacote.

### Instalar

Exemplo para Linux x86-64; ajuste o nome ao alvo gerado:

```sh
cd dist
sha256sum -c feedyourcow-x86_64-unknown-linux-gnu.tar.gz.sha256
mkdir instalacao
tar -xzf feedyourcow-x86_64-unknown-linux-gnu.tar.gz -C instalacao
./instalacao/feedyourcow/iniciar.sh preparar
./instalacao/feedyourcow/iniciar.sh servir
```

Acesse [http://127.0.0.1:3000](http://127.0.0.1:3000). `iniciar.sh` resolve os caminhos em relação
à instalação, mesmo chamado de outro diretório, e aceita todos os comandos administrativos.
O banco padrão fica em `feedyourcow/dados/aplicacao.sqlite3`. Para mantê-lo em outro local,
exporte um caminho absoluto antes de preparar e iniciar:

```sh
export FEEDYOURCOW_BANCO="$HOME/feedyourcow-dados/aplicacao.sqlite3"
```

### Atualizar

Encerre o servidor, faça backup com a versão anterior e extraia a nova versão em outra pasta.
Mantenha o caminho absoluto do banco, execute `iniciar.sh preparar` e depois `iniciar.sh servir`.
Preserve o backup e o pacote anterior. Para voltar a uma versão anterior, restaure seu backup
em um destino novo; não abra um banco já migrado com o executável antigo.

## Desenvolvimento e arquitetura

O monólito usa Rust, Axum com tracing, Maud, SQLite e SQLx. A interface combina renderização
no servidor, htmx v4 e hx-live locais, com CSS plano e variáveis no início do arquivo.
As versões reproduzíveis estão nos lockfiles. Código `unsafe` próprio é proibido.

| Diretório | Responsabilidade |
| --- | --- |
| `src/dominio/` | Unidades, validações, cálculos, inferência e otimização. |
| `src/aplicacao/` | Serviços de cadastros, cenários, execução, histórico e comparação. |
| `src/persistencia/` | Banco, cargas, revisões, histórico e recuperação. |
| `src/web/handlers/` | Rotas por recurso e tratamento HTTP. |
| `src/web/entradas/` | Conversão e validação de formulários HTTP. |
| `src/web/templates/` | Layouts, páginas, formulários e componentes Maud com `Render`. |
| `static/` | CSS, JavaScript e dependências locais com licenças. |
| `tests/` | Testes Rust e testes de navegador em `javascript/e2e/`. |
| `scripts/` | Qualidade, conferência SQLx, distribuição e teste do pacote. |
| `fontes/`, `hackathon/` | Fontes, pesquisa, requisitos do evento e planos. |

As requisições passam pelos handlers e pela conversão das entradas; os serviços coordenam
o domínio e a persistência. Templates recebem dados preparados, sem consultar o banco ou
executar regras de negócio. O servidor mantém validações, decisões e proteção de escrita
por POST, token CSRF e verificação de origem. Recursos estáticos são servidos localmente.

O motor distingue dados desconhecidos, violações e bloqueios, com conclusão por dimensão.
A formulação usa `microlp` para cinco programas lineares, um por degrau de fibra,
e revalida as candidatas. O cálculo roda em processo filho com cancelamento efetivo;
um timeout produz resultado inconclusivo. A alegação de ótimo depende da resolução de todos
os subproblemas. Quantidades arredondadas para fornecimento passam por nova validação.
Preços usam representação decimal; grandezas físicas usam `f64` e as tolerâncias dos requisitos.

Os arquivos Rust começam com docstrings de módulo que descrevem sua responsabilidade. Rotinas
com regras de cálculo, inferência, otimização, conversão de entradas e recuperação documentam
seus comportamentos e limites diretamente junto às funções.

Para recarga automática, habilite o recurso `desenvolvimento`:

```sh
cargo run --features desenvolvimento -- servir
```

Com `bacon` instalado, use `bacon` para acompanhar verificações, `bacon testar` para testes
e `bacon servir` para executar com recarga automática.

## Testes e qualidade

Na raiz do repositório:

```sh
cargo build --locked
cargo test --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo fmt --check
python3 scripts/qualidade.py
python3 scripts/verificar_sqlx.py
RUST_LOG=warn cargo run --quiet -- ensaiar
```

Os testes Rust cobrem cálculos, regras, otimização, persistência, contratos HTTP e recuperação.
`tests/exemplos.rs` verifica catálogo completo, receitas ponderadas, primeiro cálculo, inicialização
atômica, preservação de alterações/exclusões, restauração explícita e backup. As fixtures A/B são
selecionadas por identidade, independentemente do catálogo incluído no primeiro uso.
`qualidade.py` exige até 100 caracteres por linha e avisa sobre módulos acima de 300 linhas.
`verificar_sqlx.py` recria o banco de compilação e compara as consultas com o cache `.sqlx/`.
`ensaiar` usa banco isolado, cinco aquecimentos e 100 execuções por operação: a meta de p95
é até 1 segundo para verificar 20 ingredientes e 5 segundos para formular com 10 ingredientes.

### Navegadores

```sh
npm ci --ignore-scripts
npx playwright install --with-deps chromium firefox webkit
npm run test:e2e
```

Node/Playwright são ferramentas de teste. `npm test` também executa a suíte completa.
Os testes criam banco e servidor temporários, cobrem Chromium, Firefox e WebKit e produzem
capturas em `output/playwright/` para inspeção visual. Incluem formulários, erros, fragmentos
htmx, histórico, impressão, acessibilidade, layout móvel e uso sem JavaScript.
Consulte [testes JavaScript](tests/javascript/README.md) para contratos e filtros de diagnóstico.

### Pacote de produção

Com Python 3.12+ e o pacote já gerado:

```sh
python3 scripts/testar_producao.py dist/feedyourcow-x86_64-unknown-linux-gnu.tar.gz
```

Ajuste o alvo do arquivo. O teste verifica checksum, conteúdo, preparação, demonstração,
backup/restauração, motor e HTTP em instalação temporária. Para testar a interface com o
binário otimizado no checkout:

```sh
FEEDYOURCOW_BINARIO="$PWD/target/x86_64-unknown-linux-gnu/release/feed-your-cow" npm test
```

O GitHub Actions executa build, testes, clippy, formatação, qualidade textual, SQLx e desempenho
no job Rust. Um segundo job reutiliza o binário para os testes nos três navegadores e preserva
as capturas. O build do pacote de produção é executado localmente.

## Documentação

- [Requisitos](REQUISITOS.md) (versão documental 1.5): especificação funcional vigente,
  escopo, regras, limitações e casos de aceitação CT-01 a CT-35. Os requisitos definem o
  comportamento esperado; a implementação está em `src/` e a cobertura automatizada em `tests/`.
  Os testes são a evidência de conformidade. Mudanças de comportamento, regras, escopo ou aceite
  devem atualizar este documento e o README na mesma alteração.
- [Anexo nutricional](hackathon/TEMA_2_DIETAS_LITERATURA_E_MOTOR.md): tabelas, fórmulas e pesquisa.
- [Fontes preservadas](fontes/README.md) e [modelos de preenchimento](fontes/MODELOS.md):
  procedência, unidades, conversões e limitações dos dados.
- [Recursos visuais](static/README.md) e [bibliotecas locais](static/vendor/README.md):
  origens, versões e licenças.
- [Instruções de contribuição](AGENTS.md): stack, documentação, qualidade e commits.

O desenvolvimento teve auxílio de IA, com verificação por compilador e testes.
O cálculo operacional é determinístico e não utiliza modelos de linguagem.

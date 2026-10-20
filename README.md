# FeedYourCow

Sistema especialista local para verificar e formular dietas de vacas adultas em lactação.
Interface em português do Brasil com tema verde-esmeralda, cálculos explicáveis e histórico.

## Executar

Requer Rust estável. A compilação usa consultas SQLx verificadas e metadados offline versionados.
Nenhum runtime Node/Python ou solver externo é necessário para usar o aplicativo.

```sh
cargo build --locked
cargo run -- demonstrar
cargo run -- servir
```

Abra <http://127.0.0.1:3000>. `demonstrar` prepara o banco, carrega quatro composições sintéticas,
cria um lote e calcula as dietas A/B. Dados sintéticos não são análises nem cotações reais.
Para instalar sem exemplos, use `cargo run -- preparar` no lugar de `demonstrar`.

Configuração por variáveis de ambiente: `FEEDYOURCOW_BANCO` (padrão `dados/aplicacao.sqlite3`),
`FEEDYOURCOW_ENDERECO` (padrão `127.0.0.1:3000`) e `RUST_LOG`. Somente loopback é aceito.
O arquivo `.env.example` serve como referência; exporte as variáveis no ambiente de execução.

## Fluxo de uso

1. Cadastre/revise um lote e confirme os estados fisiológicos conhecidos.
2. Consulte alimentos ou cadastre uma análise com sua fonte e condições de uso.
3. Crie cenário: selecione lote, datas, consumo, ingredientes, preços, estoques e premissas.
4. Verifique quantidades ou solicite formulação de menor custo.
5. Leia as dimensões, balanços, fornecimento, fontes e explicações.
6. Consulte o histórico, duplique para simular, compare e imprima pelo navegador.

Atendimento químico não significa adequação nutricional integral. Fibra física, micronutrientes
ou contaminantes sem dados suficientes permanecem identificados. O perfil histórico é
`E2020_NRC1989_RUMEN_MVP_v1`, sem implementação integral de NASEM/BR-LEITE.

## Desenvolvimento e testes

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
bacon
bacon servir
```

O recurso `desenvolvimento` ativa tower-livereload; a execução normal não inclui autoreload.
O motor usa Rust tipado, sem código `unsafe` próprio. Bibliotecas existentes são priorizadas.
As consultas normais usam `query_file!`; SQL administrativo é restrito a cargas locais por CLI.

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

## Cargas e recuperação

```sh
cargo run -- carregar dados_referencia/001_conhecimento.sql
cargo run -- backup /tmp/feedyourcow-backup
cargo run -- restaurar /tmp/feedyourcow-backup /tmp/feedyourcow-restaurado
```

Cargas têm versão/hash e são idempotentes; colisões de versão são rejeitadas.
Backup usa `VACUUM INTO`, preserva documentos locais e gera manifesto de integridade.
Restauração exige diretório novo e verifica hashes, integridade e vínculos do SQLite.

## Referências

`REQUISITOS.md` define o comportamento; o anexo específico em `hackathon/` contém tabelas,
fórmulas e procedência. A implementação foi desenvolvida com auxílio de IA, revisada por
compilador e testes. O cálculo operacional é determinístico e não usa modelos de linguagem.

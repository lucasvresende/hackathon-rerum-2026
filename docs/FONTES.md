# Fontes, hipóteses e dependências

Esta página registra **o papel de cada referência**. Os valores carregados no programa ficam em
`dados_referencia/*.sql`, com identificadores, edições e metadados preservados. A consulta de uma
fonte não transforma uma média tabelada em medição do alimento de uma fazenda. As URLs abaixo
servem para auditoria e atualização; o cálculo não faz download durante o uso.

## Base nutricional do perfil

| ID | Referência | Uso e limite |
| --- | --- | --- |
| `C01` | [Embrapa 2020][embrapa20] | Mantença, leite e consumo no perfil. |
| Contexto | [NASEM 2021][nasem] | Modelo integral não implementado. |
| Contexto | [NRC 2001][nrc] | História dos sistemas de exigências. |
| Contexto | [BR-LEITE 2024][brleite] | Alternativa não ativada como perfil. |
| `C16` | Fibra e amido em `001_conhecimento.sql` | Cinco linhas de FDN. |
| `MVP` | Política em `001_conhecimento.sql` | FDN ≥30%, amido ≤25%, EE ≤6%. |

[embrapa20]: https://www.infoteca.cnptia.embrapa.br/infoteca/handle/doc/1123902
[nasem]: https://doi.org/10.17226/25806
[nrc]: https://doi.org/10.17226/9825
[brleite]: https://doi.org/10.26626/9786556682075.2024.B001

As tabelas numéricas exatas do perfil podem ser vistas em **Perfil nutricional** na aplicação
ou no SQL versionado. A [explicação das fórmulas](MOTOR.md) define bases, interpolação e
extrapolação. O nome `E2020_NRC1989_RUMEN_MVP_v1` identifica uma combinação histórica
específica; não atribui ao NRC, à NASEM ou à Embrapa as políticas criadas para o produto.

## Composição e catálogo de exemplos

`dados_referencia/005_exemplos.sql` instala seis lotes e quinze alimentos sintéticos. O cadastro
de cada exemplo aponta para a fonte `EXEMPLOS-V1` e preserva contexto, campos e hipóteses. Entre
as referências consultadas para aproximar valores e categorias estão:

- [Tabelas nordestinas de composição, Embrapa 2014][embrapa14].
- [Feedipedia, silagem de milho com 30–35% de MS](https://www.feedipedia.org/node/12871) e
  [polpa cítrica seca](https://www.feedipedia.org/node/680).
- [Universidade do Missouri, comparação de alimentos][missouri].
- [Embrapa, raças e cruzamentos leiteiros][racas].

[embrapa14]: https://www.infoteca.cnptia.embrapa.br/infoteca/handle/doc/1007523
[missouri]: http://agebb.missouri.edu/dairy/byprod/energygain.php
[racas]: https://www.infoteca.cnptia.embrapa.br/infoteca/handle/doc/737102

As fontes têm amostras, métodos, populações e bases diferentes. Valores de NDT, amido e outros
campos dos exemplos podem conter hipóteses adicionais e arredondamentos. As misturas `ex-a14` e
`ex-a15` são calculadas pela contribuição de MS de cada componente, não são produtos comerciais
nem dietas completas. Condições fisiológicas, autorização de produtos fictícios, preços e
estoques dos exemplos são **hipóteses de simulação**. Laudos não são inventados; dimensão sem
laudo permanece desconhecida.

A carga `004_modelos.sql` conserva modelos parciais de preenchimento, com unidades e lacunas
explícitas. Silagem de milho e milho em grão não recebem NDT presumido; farelo de soja não recebe
amido deduzido de carboidratos não fibrosos. O modelo de lote Jersey tem dados reais parciais de
um estudo específico e não deve ser tratado como média da raça ou indicação de dieta. Os
modelos são ponto de partida editável, não validação automática. As origens registradas incluem
[Feedipedia (milho)](https://www.feedipedia.org/node/11664),
[Embrapa 2014](https://www.infoteca.cnptia.embrapa.br/infoteca/handle/doc/1007523) e
[estudo com Jersey](https://doi.org/10.1590/S1516-35982005000600027).

## Normas e contaminantes

A carga `001_conhecimento.sql` contém entradas normativas com IDs, jurisdição, datas,
condições e fontes. A aplicação testa vigência na data de aplicação do cenário. A entrada
`LEG-ANIMAL` trata a restrição brasileira de ingredientes de origem animal para ruminantes;
`N03` e `N07` contextualizam catálogo e transições; `N11` identifica a referência **estrangeira**
para AFB1. Consultar a norma primária é necessário antes de atualizar ou ampliar regras:

- [MAPA, legislação de alimentação animal][mapa].
- [Diretiva 2002/32/CE sobre substâncias indesejáveis][diretiva].
- [Regulamento UE 2023/1708][regulamento].

[mapa]: https://www.gov.br/agricultura/pt-br/assuntos/insumos-agropecuarios
[diretiva]: https://eur-lex.europa.eu/legal-content/EN/ALL/?uri=CELEX%3A02002L0032-20191128
[regulamento]: https://eur-lex.europa.eu/legal-content/EN/ALL/?uri=CELEX%3A32023R1708

A referência UE para contaminantes é opcional e rotulada como estrangeira. O produto não carrega
um teto brasileiro universal de micotoxinas para a dieta. Um limite de AFM1 no leite não é
convertido em limite de AFB1 na ração. Ausência de laudo, identidade incompleta ou base de
umidade incompatível geram pendência. A regra técnica não constitui parecer jurídico ou
sanitário sobre um produto real.

## Bibliotecas e documentação técnica

A implementação se apoia nas documentações oficiais das versões declaradas no projeto:

| Recurso | Documentação e uso |
| --- | --- |
| [Rust](https://doc.rust-lang.org/book/) | Linguagem. |
| [Tokio](https://docs.rs/tokio/latest/tokio/) | Runtime assíncrono. |
| [Axum 0.8](https://docs.rs/axum/0.8.9/axum/) | Rotas e respostas HTTP. |
| [Maud 0.27](https://docs.rs/maud/0.27.0/maud/) | Templates `Render` e `Markup`. |
| [SQLx 0.9](https://docs.rs/sqlx/0.9.0/sqlx/) | Macros `query_file!`, SQLite e migrações. |
| [microlp 0.6](https://docs.rs/microlp/0.6.0/microlp/) | Formulação linear. |
| [htmx 4 e hx-live](https://four.htmx.org/docs) | Atualização de fragmentos e reatividade. |
| [Playwright](https://playwright.dev/docs/intro) | Testes de navegador e capturas. |

As cópias de `htmx.min.js` e `hx-live.min.js` em `static/vendor/` vieram do pacote
`htmx.org@4.0.0` e são distribuídas localmente, sob a licença preservada em
[licencas/htmx.txt](licencas/htmx.txt). Os SVGs são locais; o desenho da vaca deriva de
EmojiOne/JoyPixels e mantém o aviso original em [licencas/cow.txt](licencas/cow.txt).
[SVG Repo](https://www.svgrepo.com/) foi usado para os ícones; confira a licença de cada ativo
antes de redistribuir versões alteradas. Os ícones e scripts não carregam recursos externos em
tempo de execução.

## Atualizar uma referência

1. Verifique a edição, o método, a unidade, a população e a vigência na fonte primária.
2. Registre o que foi transcrito e o que é hipótese do produto; não preencha lacunas por analogia.
3. Crie nova carga versionada sem reescrever silenciosamente uma versão já instalada.
4. Atualize [requisitos](REQUISITOS.md), [motor](MOTOR.md) e testes do cálculo afetado.
5. Revise a explicação exibida em **Perfil nutricional** e em pareceres preservados.

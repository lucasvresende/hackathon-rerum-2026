# Arquitetura e decisões do FeedYourCow

## Separação de responsabilidades

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

## Conhecimento e banco

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

## Otimização e precisão

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

## Operação e segurança

Somente loopback; sem autenticação nem acesso compartilhado no MVP. Escritas exigem POST, token
CSRF e verificação da origem quando enviada. Texto é escapado pelo Maud; corpos têm limite.
Não há interpretador de regras arbitrárias, CDN, APIs ou modelos de linguagem em execução.

Backups usam `VACUUM INTO`, documentos locais e manifesto SHA-256. A restauração exige destino
novo e confere hashes, integridade e referências do banco antes do uso.

## Fontes técnicas consultadas

- [Axum 0.8.9](https://docs.rs/axum/0.8.9/axum/).
- [Maud 0.27.0](https://docs.rs/maud/0.27.0/maud/).
- [SQLx 0.9.0](https://docs.rs/sqlx/0.9.0/sqlx/).
- [microlp 0.6.0](https://docs.rs/microlp/0.6.0/microlp/).
- [htmx v4](https://four.htmx.org/) e
  [hx-live](https://four.htmx.org/extensions/hx-live).

A IA auxiliou leitura, implementação e testes. Dados sintéticos têm identificação explícita.
Nenhuma conclusão operacional é delegada à IA. Limitações científicas seguem `REQUISITOS.md`.

## Ajuda contextual

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

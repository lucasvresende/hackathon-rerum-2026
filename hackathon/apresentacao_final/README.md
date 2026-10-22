# Materiais da apresentação final

PDFs preparados em 22/10/2026, em A4, com capa, sumário navegável e páginas numeradas.

- [Documentação completa](DOCUMENTACAO_COMPLETA.pdf): 31 páginas em dez capítulos numerados,
  com índice navegável de 59 tópicos e suas páginas. O texto foi reformulado em uma narrativa
  contínua, com requisitos funcionais, regras de negócio e requisitos não funcionais em tabelas.
  Inclui cinco diagramas SVG, imagens de uso e avisos de licença, sem mapa ou relatórios de auditoria.
  A [fonte editorial](DOCUMENTACAO_COMPLETA.md) preserva o conteúdo editável desta edição.
- [Literatura e motor do Tema 2](TEMA_2_DIETAS_LITERATURA_E_MOTOR.pdf): 31 páginas com o
  conteúdo de `hackathon/TEMA_2_DIETAS_LITERATURA_E_MOTOR.md`. Preserva a pesquisa e proposta
  documental original de 20/10/2026; o contrato implementado está na documentação completa.
- [Roteiro da demonstração](ROTEIRO_DEMO.md).

Os Markdown originais continuam sendo as fontes editáveis. Os links entre documentos incluídos
no PDF principal levam às respectivas seções; referências externas conservam seus endereços.
Links para arquivos do repositório não incorporados exigem acesso ao checkout original.

Verificação: geração no Chromium, imagens incorporadas carregadas, extração de texto com Poppler
e inspeção visual das páginas renderizadas, incluindo tabelas, fórmulas e blocos de código.

## Diagramas vetoriais

Os seis blocos Mermaid foram renderizados em SVG e incorporados aos dois PDFs como gráficos
vetoriais. As cópias ficam em `diagramas/`: cinco da documentação e uma do relatório do Tema 2.
Os fluxos horizontais receberam disposição vertical para leitura em A4; nós, textos e conexões
foram preservados. Os blocos Mermaid originais permanecem nos Markdown de origem.

Verificação adicional: todos os SVGs carregaram no Chromium, os PDFs não contêm mais os blocos
`flowchart` e a paginação foi inspecionada após a substituição.

A edição da documentação completa reorganiza o conteúdo do README e dos guias de `docs/` por
assunto, elimina introduções e atalhos repetidos e usa referências internas aos capítulos.
O índice foi calculado por extração de texto do PDF e conferido após estabilizar a paginação.
Os resultados de testes citados são registros da revisão técnica de 22/10/2026, sem nova execução
das suítes da aplicação durante esta revisão editorial.

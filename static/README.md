# Recursos visuais locais

`cow.svg`: desenho Cow, coleção EmojiOne, autor EmojiOne / JoyPixels, obtido no
[SVG Repo](https://www.svgrepo.com/svg/401350/cow). Consulta em 20/10/2026.
O arquivo foi revisado para conter somente geometria SVG; dimensões e formatação ajustadas.
Não há scripts, eventos ou referências externas.

`icones/` contém imagens locais de 21 × 21 px para as abas, obtidas no SVG Repo: House Chimney
para Visão geral, Holstein para Lotes de animais, Food para Alimentos, Add Circle para Nova dieta,
Magnifying Glass for Search para Histórico e análise, Notebook para Perfil nutricional e Help para
Ajuda. As origens são
[Holstein](https://www.svgrepo.com/svg/482249/illustration-of-holstein),
[Food](https://www.svgrepo.com/svg/494047/food) e
[lupa](https://www.svgrepo.com/svg/479646/magnifying-glass-9). Os demais foram baixados em SVG Repo
com os nomes `house-chimney-blank`, `add-circle`, `notebook-1` e `help`. Holstein e Food indicam
CC0; consulte a licença indicada pelo SVG Repo para cada arquivo antes de redistribuir a aplicação.
Os arquivos são servidos localmente, sem chamadas externas durante o uso.

A página do SVG Repo indica MIT, mas a licença da versão histórica EmojiOne 2.2.7 distingue
código e arte: PNG/SVG são CC BY 4.0. A atribuição ao autor e o vínculo para a licença
original são preservados aqui e na página “Perfil nutricional”. Não tratar a licença do
código como a da arte.

- [Origem e licença EmojiOne 2.2.7][origem].
- [Creative Commons Attribution 4.0 International][licenca].
- `LICENCA-cow.txt` preserva o aviso de licença da distribuição original.

[origem]: https://github.com/joypixels/emojione/blob/v2.2.7/LICENSE.md
[licenca]: https://creativecommons.org/licenses/by/4.0/

Nos formulários, `app.js` cancela `htmx:before:swap` para respostas HTTP de erro (status >=400).
Isso preserva o rascunho em falhas de autorização ou do servidor, permitindo nova tentativa.
Os contratos correspondentes estão em `tests/javascript/e2e/falhas.mjs`.

A validação cancela o foco/popup nativo do evento `invalid`, mantém a mensagem inline em português
e abre as seções necessárias antes de focar o primeiro campo inválido, após dois frames.
Isso evita a tentativa de focar um controle ainda `inert`, reproduzida no WebKit pelo teste
`condicional-laudos`; a restrição nativa continua impedindo o envio do formulário inválido.

A sinalização entre guias usa `BroadcastChannel` e `localStorage`, transportando somente um
identificador de atualização. Entregas repetidas do mesmo sinal são ignoradas, evitando POST
repetido. A atualização manual continua disponível quando a sinalização não estiver acessível.

`cenarios.js` coordena os rascunhos de dieta: confere os valores atuais antes da substituição,
descarta respostas obsoletas e repete a atualização com as entradas atuais. Um envio solicitado
durante a espera é retomado. A sincronização usa eventos nativos de htmx 4 e `hx-sync`, sem
substituir `fetch` nem modificar as bibliotecas vendorizadas. `settle:0` nas trocas do cenário
evita copiar atributos antigos de controles focados durante o período de acomodação do htmx.
O mesmo arquivo filtra ingredientes sem desmarcá-los, atualiza o contador e abre os grupos
indicados pelos links do resumo de erros. Sem JavaScript, confirmação e pesquisa usam POST
com CSRF e preservação do rascunho no servidor.

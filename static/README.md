# Recursos visuais locais

`cow.svg`: desenho Cow, coleção EmojiOne, autor EmojiOne / JoyPixels, obtido no
[SVG Repo](https://www.svgrepo.com/svg/401350/cow). Consulta em 20/10/2026.
O arquivo foi revisado para conter somente geometria SVG; dimensões e formatação ajustadas.
Não há scripts, eventos ou referências externas.

A página do SVG Repo indica MIT, mas a licença da versão histórica EmojiOne 2.2.7 distingue
código e arte: PNG/SVG são CC BY 4.0. A atribuição ao autor e o vínculo para a licença original
são preservados aqui e na página “Perfil e fontes”. Não tratar a licença do código como a da arte.

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

# Fontes dos templates editáveis

Pesquisa consultada em 20/10/2026; a carga `dados_referencia/004_modelos.sql` preserva valores,
unidades, número de observações, contexto e hashes. Aprovação nesta conversa: carregar valores
verificados e manter lacunas explícitas. Modelos podem ser carregados e editados antes de salvar.
Não confirmam estados de segurança nem representam medições da fazenda.

| Modelo | Fonte | Campos nutricionais ausentes |
| --- | --- | --- |
| Silagem de milho 30–35% MS | [Feedipedia 12871][s] | NDT |
| Farelo de soja | [Embrapa 2014][e], p. 44–45 | Amido |
| Milho em grão, América Central e do Sul | [Feedipedia 11664][m] | NDT |
| Jersey, estudo RS 2005 | [Duarte et al.][j], Material e Métodos | Gordura e DEL |

MS é % MN; nutrientes são % MS. Ca/P da Feedipedia estão em g/kg MS e são divididos por 10
para exibição em % MS. O motor converte percentuais em frações. Na silagem, FDN é média
estimada por equação; não tratar como medição. NDT não é inferido de energia ou digestibilidade.

A Embrapa reúne análises de diferentes espécies, idades e métodos; p. 15 recomenda cautela
com valores energéticos. Para soja, PB tem 88 observações, NDT tem 15. Amido não é CNF.
PDF conferido: páginas impressas 44–45, páginas 45–46 do arquivo original.

O modelo Jersey usa apenas raça, peso e produção real inicial de oito vacas do estudo.
Peso 389 kg fica fora do perfil MVP; não arredondá-lo para 400 kg para permitir recomendação.
Não é uma média populacional da raça nem instrução para repetir a dieta experimental.
Novilhas Holandesas/Jersey de [Penn State][p] e produção acumulada de [Girolando][g] foram
examinadas e excluídas como padrões para vacas adultas: população ou grandeza incompatível.
Gordura, dias em lactação, número de animais e condições fisiológicas permanecem informados.

Dados sintéticos A/B continuam separados em `002_demonstracao.sql`.
O aplicativo não consulta a internet para aplicar templates ou executar cálculos.

[s]: https://www.feedipedia.org/node/12871
[m]: https://www.feedipedia.org/node/11664
[e]: https://www.infoteca.cnptia.embrapa.br/infoteca/handle/doc/1007523
[j]: https://doi.org/10.1590/S1516-35982005000600027
[p]: https://extension.psu.edu/monitoring-dairy-heifer-growth
[g]: https://www.infoteca.cnptia.embrapa.br/infoteca/handle/doc/594820

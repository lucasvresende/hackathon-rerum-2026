-- versao: modelos-pesquisados-v1

INSERT INTO fontes
(id,titulo,edicao,localizador,natureza,conteudo,hash) VALUES (
    'TPL-FEED-SILAGEM',
    'Feedipedia: Maize silage, dry matter 30-35%',
    'Tabela 12871; 25/08/2016',
    'https://www.feedipedia.org/node/12871',
    'TECNICA',
    'Consulta em 20/10/2026. Médias: MS 32.5 % MN; PB 6.9, FDN 44.3, amido '
    || '29.1 e EE 2.8 % MS. Ca 1.9 e P 1.7 g/kg MS. FDN: média obtida por '
    || 'equação. NDT ausente. Número de observações: MS 5642, PB 5438, FDN 26, '
    || 'amido 519, EE 27, Ca 4939, P 4972.',
    '7f4869ba358a98888fba2a886989f901c28e8fab3c5d528209a7091a80eddc20'
);

INSERT INTO fontes
(id,titulo,edicao,localizador,natureza,conteudo,hash) VALUES (
    'TPL-EMBRAPA-SOJA',
    'Embrapa: Tabelas nordestinas de composição de alimentos',
    '1ª edição, 2014, p. 44–45; notas p. 15',
    'https://www.infoteca.cnptia.embrapa.br/infoteca/handle/doc/1007523',
    'TECNICA',
    'Consulta em 20/10/2026. Soja farelo: MS 88.46 % MN; NDT 79.42, PB 48.84,'
    || ' Ca 0.47, P 0.52, FDN 15.34, EE 2.16 % MS. Amido ausente. Observações: '
    || 'MS 82, PB 88, NDT 15, FDN 61, EE 75, Ca 23, P 19. A publicação alerta '
    || 'para diversidade de espécies, idades e métodos nos valores energéticos.',
    '0d7331709324c698818f2c289af4e64d29aef81df3571603ad993e36eedeed20'
);

INSERT INTO fontes
(id,titulo,edicao,localizador,natureza,conteudo,hash) VALUES (
    'TPL-FEED-MILHO',
    'Feedipedia: Maize grain, Central and South America',
    'Tabela 11664',
    'https://www.feedipedia.org/node/11664',
    'TECNICA',
    'Consulta em 20/10/2026. Médias: MS 86.7 % MN; PB 9.2, FDN 13.3, amido '
    || '71.8, EE 4.4 % MS. Ca 0.4 e P 3.3 g/kg MS. NDT ausente. Grão; não '
    || 'inferir composição de produto processado. Observações: MS 67, PB 72, FDN'
    || ' 12, amido 51, EE 24, Ca 11, P 13.',
    'aefda20d80ed43140f6163c517966e39e6c0a50ee98bbec528c99b923a3b766f'
);

INSERT INTO fontes
(id,titulo,edicao,localizador,natureza,conteudo,hash) VALUES (
    'TPL-JERSEY-2005',
    'Duarte et al.: fontes de gordura e vacas Jersey',
    'RBZ 34(6), 2005; métodos',
    'https://doi.org/10.1590/S1516-35982005000600027',
    'TECNICA',
    'Consulta em 20/10/2026. Oito vacas Jersey, 2ª e 3ª lactação, Embrapa '
    || 'Clima Temperado, RS. Peso inicial 389 ± 27 kg; produção real diária 21 ±'
    || ' 3 kg. Estudo no terço inicial de lactação. Não é média populacional da '
    || 'raça. Peso 389 kg está fora do perfil MVP (400–700 kg).',
    'dab27f7fa3a55b0bdb7e23ec4873e9db263ef3019cea65cd6afab3fd97eb4c9c'
);

INSERT INTO modelos_preenchimento
(id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES (
    'silagem-milho-v1',
    '1',
    'Silagem de milho · 30–35% MS',
    'alimento',
    'Médias de silagem de milho na classe de 30–35% MS. FDN estimada por '
    || 'equação.',
    'NDT desconhecido. Não é análise do produto. Confirme processamento e '
    || 'procedência.',
    'TPL-FEED-SILAGEM'
);

INSERT INTO valores_modelo VALUES (
    'silagem-milho-v1',
    'nome',
    'Silagem de milho (30–35% MS)',
    'texto',
    'Silagem de milho (30–35% MS)',
    'texto',
    'METADADO',
    'TPL-FEED-SILAGEM'
);

INSERT INTO valores_modelo VALUES (
    'silagem-milho-v1',
    'classe',
    'FORRAGEM',
    'texto',
    'FORRAGEM',
    'texto',
    'METADADO',
    'TPL-FEED-SILAGEM'
);

INSERT INTO valores_modelo VALUES (
    'silagem-milho-v1',
    'ms',
    '32.5',
    '% MN',
    '32.5',
    '% MN',
    'MEDIA_TABELADA',
    'TPL-FEED-SILAGEM'
);

INSERT INTO valores_modelo VALUES (
    'silagem-milho-v1',
    'pb',
    '6.9',
    '% MS',
    '6.9',
    '% MS',
    'MEDIA_TABELADA',
    'TPL-FEED-SILAGEM'
);

INSERT INTO valores_modelo VALUES (
    'silagem-milho-v1',
    'ca',
    '0.19',
    '% MS',
    '1.90',
    'g/kg MS',
    'MEDIA_TABELADA',
    'TPL-FEED-SILAGEM'
);

INSERT INTO valores_modelo VALUES (
    'silagem-milho-v1',
    'p',
    '0.17',
    '% MS',
    '1.70',
    'g/kg MS',
    'MEDIA_TABELADA',
    'TPL-FEED-SILAGEM'
);

INSERT INTO valores_modelo VALUES (
    'silagem-milho-v1',
    'fdn',
    '44.3',
    '% MS',
    '44.3',
    '% MS',
    'ESTIMATIVA_TABELADA',
    'TPL-FEED-SILAGEM'
);

INSERT INTO valores_modelo VALUES (
    'silagem-milho-v1',
    'amido',
    '29.1',
    '% MS',
    '29.1',
    '% MS',
    'MEDIA_TABELADA',
    'TPL-FEED-SILAGEM'
);

INSERT INTO valores_modelo VALUES (
    'silagem-milho-v1',
    'ee',
    '2.8',
    '% MS',
    '2.8',
    '% MS',
    'MEDIA_TABELADA',
    'TPL-FEED-SILAGEM'
);

INSERT INTO valores_modelo VALUES (
    'silagem-milho-v1',
    'natureza',
    'MEDIA_TABELADA',
    'texto',
    'MEDIA_TABELADA',
    'texto',
    'METADADO',
    'TPL-FEED-SILAGEM'
);

INSERT INTO valores_modelo VALUES (
    'silagem-milho-v1',
    'fonte_id',
    'TPL-FEED-SILAGEM',
    'texto',
    'TPL-FEED-SILAGEM',
    'texto',
    'METADADO',
    'TPL-FEED-SILAGEM'
);

INSERT INTO valores_modelo VALUES (
    'silagem-milho-v1',
    'origem',
    'Referência bibliográfica; procedência do produto a confirmar',
    'texto',
    'Referência bibliográfica; procedência do produto a confirmar',
    'texto',
    'METADADO',
    'TPL-FEED-SILAGEM'
);

INSERT INTO valores_modelo VALUES (
    'silagem-milho-v1',
    'metodo',
    'Tabela 12871; 25/08/2016',
    'texto',
    'Tabela 12871; 25/08/2016',
    'texto',
    'METADADO',
    'TPL-FEED-SILAGEM'
);

INSERT INTO modelos_preenchimento
(id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES (
    'soja-farelo-v1',
    '1',
    'Farelo de soja · Embrapa 2014',
    'alimento',
    'Médias compiladas no Nordeste, p. 44–45. NDT agrega métodos e populações'
    || ' distintos.',
    'Amido desconhecido. Revise a compatibilidade do NDT e os dados reais do '
    || 'produto.',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO valores_modelo VALUES (
    'soja-farelo-v1',
    'nome',
    'Farelo de soja',
    'texto',
    'Farelo de soja',
    'texto',
    'METADADO',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO valores_modelo VALUES (
    'soja-farelo-v1',
    'classe',
    'CONCENTRADO',
    'texto',
    'CONCENTRADO',
    'texto',
    'METADADO',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO valores_modelo VALUES (
    'soja-farelo-v1',
    'ms',
    '88.46',
    '% MN',
    '88.46',
    '% MN',
    'MEDIA_TABELADA',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO valores_modelo VALUES (
    'soja-farelo-v1',
    'ndt',
    '79.42',
    '% MS',
    '79.42',
    '% MS',
    'MEDIA_TABELADA',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO valores_modelo VALUES (
    'soja-farelo-v1',
    'pb',
    '48.84',
    '% MS',
    '48.84',
    '% MS',
    'MEDIA_TABELADA',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO valores_modelo VALUES (
    'soja-farelo-v1',
    'ca',
    '0.47',
    '% MS',
    '0.47',
    '% MS',
    'MEDIA_TABELADA',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO valores_modelo VALUES (
    'soja-farelo-v1',
    'p',
    '0.52',
    '% MS',
    '0.52',
    '% MS',
    'MEDIA_TABELADA',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO valores_modelo VALUES (
    'soja-farelo-v1',
    'fdn',
    '15.34',
    '% MS',
    '15.34',
    '% MS',
    'MEDIA_TABELADA',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO valores_modelo VALUES (
    'soja-farelo-v1',
    'ee',
    '2.16',
    '% MS',
    '2.16',
    '% MS',
    'MEDIA_TABELADA',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO valores_modelo VALUES (
    'soja-farelo-v1',
    'natureza',
    'MEDIA_TABELADA',
    'texto',
    'MEDIA_TABELADA',
    'texto',
    'METADADO',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO valores_modelo VALUES (
    'soja-farelo-v1',
    'fonte_id',
    'TPL-EMBRAPA-SOJA',
    'texto',
    'TPL-EMBRAPA-SOJA',
    'texto',
    'METADADO',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO valores_modelo VALUES (
    'soja-farelo-v1',
    'origem',
    'Referência bibliográfica; procedência do produto a confirmar',
    'texto',
    'Referência bibliográfica; procedência do produto a confirmar',
    'texto',
    'METADADO',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO valores_modelo VALUES (
    'soja-farelo-v1',
    'metodo',
    '1ª edição, 2014, p. 44–45; notas p. 15',
    'texto',
    '1ª edição, 2014, p. 44–45; notas p. 15',
    'texto',
    'METADADO',
    'TPL-EMBRAPA-SOJA'
);

INSERT INTO modelos_preenchimento
(id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES (
    'milho-grao-v1',
    '1',
    'Milho em grão · América Central e do Sul',
    'alimento',
    'Médias de milho em grão da América Central e do Sul; confirmar o '
    || 'processamento utilizado.',
    'NDT desconhecido. Não corresponde automaticamente a milho moído, '
    || 'floculado ou úmido.',
    'TPL-FEED-MILHO'
);

INSERT INTO valores_modelo VALUES (
    'milho-grao-v1',
    'nome',
    'Milho em grão',
    'texto',
    'Milho em grão',
    'texto',
    'METADADO',
    'TPL-FEED-MILHO'
);

INSERT INTO valores_modelo VALUES (
    'milho-grao-v1',
    'classe',
    'CONCENTRADO',
    'texto',
    'CONCENTRADO',
    'texto',
    'METADADO',
    'TPL-FEED-MILHO'
);

INSERT INTO valores_modelo VALUES (
    'milho-grao-v1',
    'ms',
    '86.7',
    '% MN',
    '86.7',
    '% MN',
    'MEDIA_TABELADA',
    'TPL-FEED-MILHO'
);

INSERT INTO valores_modelo VALUES (
    'milho-grao-v1',
    'pb',
    '9.2',
    '% MS',
    '9.2',
    '% MS',
    'MEDIA_TABELADA',
    'TPL-FEED-MILHO'
);

INSERT INTO valores_modelo VALUES (
    'milho-grao-v1',
    'ca',
    '0.04',
    '% MS',
    '0.40',
    'g/kg MS',
    'MEDIA_TABELADA',
    'TPL-FEED-MILHO'
);

INSERT INTO valores_modelo VALUES (
    'milho-grao-v1',
    'p',
    '0.33',
    '% MS',
    '3.30',
    'g/kg MS',
    'MEDIA_TABELADA',
    'TPL-FEED-MILHO'
);

INSERT INTO valores_modelo VALUES (
    'milho-grao-v1',
    'fdn',
    '13.3',
    '% MS',
    '13.3',
    '% MS',
    'MEDIA_TABELADA',
    'TPL-FEED-MILHO'
);

INSERT INTO valores_modelo VALUES (
    'milho-grao-v1',
    'amido',
    '71.8',
    '% MS',
    '71.8',
    '% MS',
    'MEDIA_TABELADA',
    'TPL-FEED-MILHO'
);

INSERT INTO valores_modelo VALUES (
    'milho-grao-v1',
    'ee',
    '4.4',
    '% MS',
    '4.4',
    '% MS',
    'MEDIA_TABELADA',
    'TPL-FEED-MILHO'
);

INSERT INTO valores_modelo VALUES (
    'milho-grao-v1',
    'natureza',
    'MEDIA_TABELADA',
    'texto',
    'MEDIA_TABELADA',
    'texto',
    'METADADO',
    'TPL-FEED-MILHO'
);

INSERT INTO valores_modelo VALUES (
    'milho-grao-v1',
    'fonte_id',
    'TPL-FEED-MILHO',
    'texto',
    'TPL-FEED-MILHO',
    'texto',
    'METADADO',
    'TPL-FEED-MILHO'
);

INSERT INTO valores_modelo VALUES (
    'milho-grao-v1',
    'origem',
    'Referência bibliográfica; procedência do produto a confirmar',
    'texto',
    'Referência bibliográfica; procedência do produto a confirmar',
    'texto',
    'METADADO',
    'TPL-FEED-MILHO'
);

INSERT INTO valores_modelo VALUES (
    'milho-grao-v1',
    'metodo',
    'Tabela 11664',
    'texto',
    'Tabela 11664',
    'texto',
    'METADADO',
    'TPL-FEED-MILHO'
);

INSERT INTO modelos_preenchimento
(id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES (
    'jersey-estudo-v1',
    '1',
    'Jersey · estudo RS, 2005 (parcial)',
    'lote',
    'Oito vacas Jersey em 2ª/3ª lactação, início da lactação. Peso 389 ± 27 '
    || 'kg; leite 21 ± 3 kg/dia.',
    'Não é padrão da raça. Peso sugerido está fora do perfil. Informe '
    || 'gordura, DEL e dados do lote.',
    'TPL-JERSEY-2005'
);

INSERT INTO valores_modelo VALUES (
    'jersey-estudo-v1',
    'raca',
    'Jersey',
    'texto',
    'Jersey',
    'texto',
    'METADADO',
    'TPL-JERSEY-2005'
);

INSERT INTO valores_modelo VALUES (
    'jersey-estudo-v1',
    'peso',
    '389',
    'kg',
    '389',
    'kg',
    'MEDIA_TABELADA',
    'TPL-JERSEY-2005'
);

INSERT INTO valores_modelo VALUES (
    'jersey-estudo-v1',
    'producao',
    '21',
    'kg/animal/dia',
    '21',
    'kg/animal/dia',
    'MEDIA_TABELADA',
    'TPL-JERSEY-2005'
);

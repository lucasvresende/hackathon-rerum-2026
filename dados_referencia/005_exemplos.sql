-- versao: exemplos-v1

INSERT INTO fontes (id,titulo,edicao,localizador,natureza,conteudo,hash) VALUES ( 'EXEMPLOS-V1',
'Catálogo sintético de exemplos', 'v1 · 21/10/2026', '/conhecimento', 'SINTETICA',
'Embrapa 2014, tabelas nordestinas, doc/1007523; Embrapa 2009, raças, '
|| 'doc/737102; Feedipedia, tabelas 12871 e 680; Missouri, '
|| 'dairy/byprod/energygain.php; Procreare, racas-de-bovinos-leiteiro. '
|| 'Consulta 21/10/2026. Exemplo sintético para experimentação; valores '
|| 'aproximados e condições assumidas. Referências e hipóteses: '
|| 'REQUISITOS.md, anexo A, versão 1.4. Não representa medição real ou '
|| 'comprovação de produto.', '438c72654f52d4469ba1355fd92a44162b61d204d884756c631d116c473967b5' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-l01',
'1', 'Holandesa — exemplo', 'lote',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Todas as condições são hipóteses; ver anexo A.1.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-l01', 'lote',
'{"procedencia":null,"id":"ex-l01","revisao_id":"","nome":"Holandesa — '
|| 'exemplo","animais":1,"raca":"Holandesa","idade_meses":48,"estado_produ'
|| 'tivo":"LACTACAO","destinacao":"Produção de leite","peso":600,"producao'
|| '":28,"gordura":3.6,"dias_lactacao":120,"adulta":"SIM","gestante":"NAO"'
|| ',"crescimento":"NAO","peso_estavel":"SIM","confinada":"SIM","correcao_'
|| 'ambiente":"NAO","correcao_atividade":"NAO","agua":"SIM","ordem_parto":'
|| '2,"dispersao_peso":null,"condicao_corporal":3,"observacoes":"Exemplo '
|| 'sintético para experimentação; valores aproximados e condições '
|| 'assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5."}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'nome', 'Holandesa — exemplo', 'texto', 'Holandesa — exemplo', 'texto', 'SINTETICA', 'EXEMPLOS-V1'
);

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'animais', '1', 'texto', '1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'raca', 'Holandesa', 'texto', 'Holandesa', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'idade_meses', '48', 'texto', '48', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'estado_produtivo', 'LACTACAO', 'texto', 'LACTACAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'destinacao', 'Produção de leite', 'texto', 'Produção de leite', 'texto', 'SINTETICA', 'EXEMPLOS-V1'
);

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'peso', '600', 'kg', '600', 'kg', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'producao', '28', 'kg/animal/dia', '28', 'kg/animal/dia', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'gordura', '3.6', '%', '3.6', '%', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'dias_lactacao', '120', 'texto', '120', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'adulta', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'gestante', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'crescimento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'peso_estavel', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'confinada', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'correcao_ambiente', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'correcao_atividade', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'agua', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'ordem_parto', '2', 'texto', '2', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'condicao_corporal', '3', 'texto', '3', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l01',
'observacoes', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5.', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-l02',
'1', 'Jersey — exemplo', 'lote',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Todas as condições são hipóteses; ver anexo A.1.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-l02', 'lote',
'{"procedencia":null,"id":"ex-l02","revisao_id":"","nome":"Jersey — exe'
|| 'mplo","animais":1,"raca":"Jersey","idade_meses":48,"estado_produtivo":'
|| '"LACTACAO","destinacao":"Produção de leite","peso":420,"producao":18,"'
|| 'gordura":4.8,"dias_lactacao":120,"adulta":"SIM","gestante":"NAO","cres'
|| 'cimento":"NAO","peso_estavel":"SIM","confinada":"SIM","correcao_ambien'
|| 'te":"NAO","correcao_atividade":"NAO","agua":"SIM","ordem_parto":2,"dis'
|| 'persao_peso":null,"condicao_corporal":3,"observacoes":"Exemplo '
|| 'sintético para experimentação; valores aproximados e condições '
|| 'assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5."}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'nome', 'Jersey — exemplo', 'texto', 'Jersey — exemplo', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'animais', '1', 'texto', '1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'raca', 'Jersey', 'texto', 'Jersey', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'idade_meses', '48', 'texto', '48', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'estado_produtivo', 'LACTACAO', 'texto', 'LACTACAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'destinacao', 'Produção de leite', 'texto', 'Produção de leite', 'texto', 'SINTETICA', 'EXEMPLOS-V1'
);

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'peso', '420', 'kg', '420', 'kg', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'producao', '18', 'kg/animal/dia', '18', 'kg/animal/dia', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'gordura', '4.8', '%', '4.8', '%', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'dias_lactacao', '120', 'texto', '120', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'adulta', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'gestante', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'crescimento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'peso_estavel', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'confinada', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'correcao_ambiente', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'correcao_atividade', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'agua', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'ordem_parto', '2', 'texto', '2', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'condicao_corporal', '3', 'texto', '3', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l02',
'observacoes', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5.', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-l03',
'1', 'Girolando 5/8 Holandês × 3/8 Gir — exemplo', 'lote',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Todas as condições são hipóteses; ver anexo A.1.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-l03', 'lote',
'{"procedencia":null,"id":"ex-l03","revisao_id":"","nome":"Girolando '
|| '5/8 Holandês × 3/8 Gir — exemplo","animais":1,"raca":"Girolando 5/8 '
|| 'Holandês × 3/8 Gir","idade_meses":54,"estado_produtivo":"LACTACAO","de'
|| 'stinacao":"Produção de leite","peso":550,"producao":22,"gordura":3.8,"'
|| 'dias_lactacao":120,"adulta":"SIM","gestante":"NAO","crescimento":"NAO"'
|| ',"peso_estavel":"SIM","confinada":"SIM","correcao_ambiente":"NAO","cor'
|| 'recao_atividade":"NAO","agua":"SIM","ordem_parto":2,"dispersao_peso":n'
|| 'ull,"condicao_corporal":3,"observacoes":"Exemplo sintético para '
|| 'experimentação; valores aproximados e condições assumidas. Referências'
|| ' e hipóteses: REQUISITOS.md, anexo A, versão 1.4. Não representa '
|| 'medição real ou comprovação de produto. Condição corporal na escala 1 ' || 'a 5."}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'nome', 'Girolando 5/8 Holandês × 3/8 Gir — exemplo', 'texto',
'Girolando 5/8 Holandês × 3/8 Gir — exemplo', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'animais', '1', 'texto', '1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'raca', 'Girolando 5/8 Holandês × 3/8 Gir', 'texto', 'Girolando 5/8 Holandês × 3/8 Gir', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'idade_meses', '54', 'texto', '54', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'estado_produtivo', 'LACTACAO', 'texto', 'LACTACAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'destinacao', 'Produção de leite', 'texto', 'Produção de leite', 'texto', 'SINTETICA', 'EXEMPLOS-V1'
);

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'peso', '550', 'kg', '550', 'kg', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'producao', '22', 'kg/animal/dia', '22', 'kg/animal/dia', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'gordura', '3.8', '%', '3.8', '%', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'dias_lactacao', '120', 'texto', '120', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'adulta', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'gestante', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'crescimento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'peso_estavel', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'confinada', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'correcao_ambiente', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'correcao_atividade', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'agua', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'ordem_parto', '2', 'texto', '2', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'condicao_corporal', '3', 'texto', '3', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l03',
'observacoes', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5.', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-l04',
'1', 'Mestiça F1 1/2 Holandês × 1/2 Gir — exemplo', 'lote',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Todas as condições são hipóteses; ver anexo A.1.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-l04', 'lote',
'{"procedencia":null,"id":"ex-l04","revisao_id":"","nome":"Mestiça F1 '
|| '1/2 Holandês × 1/2 Gir — exemplo","animais":1,"raca":"Mestiça F1 1/2 '
|| 'Holandês × 1/2 Gir","idade_meses":54,"estado_produtivo":"LACTACAO","de'
|| 'stinacao":"Produção de leite","peso":500,"producao":18,"gordura":4,"di'
|| 'as_lactacao":120,"adulta":"SIM","gestante":"NAO","crescimento":"NAO","'
|| 'peso_estavel":"SIM","confinada":"SIM","correcao_ambiente":"NAO","corre'
|| 'cao_atividade":"NAO","agua":"SIM","ordem_parto":2,"dispersao_peso":nul'
|| 'l,"condicao_corporal":3,"observacoes":"Exemplo sintético para '
|| 'experimentação; valores aproximados e condições assumidas. Referências'
|| ' e hipóteses: REQUISITOS.md, anexo A, versão 1.4. Não representa '
|| 'medição real ou comprovação de produto. Condição corporal na escala 1 ' || 'a 5."}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'nome', 'Mestiça F1 1/2 Holandês × 1/2 Gir — exemplo', 'texto',
'Mestiça F1 1/2 Holandês × 1/2 Gir — exemplo', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'animais', '1', 'texto', '1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'raca', 'Mestiça F1 1/2 Holandês × 1/2 Gir', 'texto', 'Mestiça F1 1/2 Holandês × 1/2 Gir', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'idade_meses', '54', 'texto', '54', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'estado_produtivo', 'LACTACAO', 'texto', 'LACTACAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'destinacao', 'Produção de leite', 'texto', 'Produção de leite', 'texto', 'SINTETICA', 'EXEMPLOS-V1'
);

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'peso', '500', 'kg', '500', 'kg', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'producao', '18', 'kg/animal/dia', '18', 'kg/animal/dia', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'gordura', '4', '%', '4', '%', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'dias_lactacao', '120', 'texto', '120', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'adulta', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'gestante', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'crescimento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'peso_estavel', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'confinada', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'correcao_ambiente', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'correcao_atividade', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'agua', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'ordem_parto', '2', 'texto', '2', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'condicao_corporal', '3', 'texto', '3', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l04',
'observacoes', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5.', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-l05',
'1', 'Gir Leiteiro — exemplo', 'lote',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Todas as condições são hipóteses; ver anexo A.1.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-l05', 'lote',
'{"procedencia":null,"id":"ex-l05","revisao_id":"","nome":"Gir Leiteiro'
|| ' — exemplo","animais":1,"raca":"Gir Leiteiro","idade_meses":60,"estado'
|| '_produtivo":"LACTACAO","destinacao":"Produção de leite","peso":450,"pr'
|| 'oducao":12,"gordura":4.5,"dias_lactacao":120,"adulta":"SIM","gestante"'
|| ':"NAO","crescimento":"NAO","peso_estavel":"SIM","confinada":"SIM","cor'
|| 'recao_ambiente":"NAO","correcao_atividade":"NAO","agua":"SIM","ordem_p'
|| 'arto":2,"dispersao_peso":null,"condicao_corporal":3,"observacoes":"Exe'
|| 'mplo sintético para experimentação; valores aproximados e condições '
|| 'assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5."}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'nome', 'Gir Leiteiro — exemplo', 'texto', 'Gir Leiteiro — exemplo', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'animais', '1', 'texto', '1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'raca', 'Gir Leiteiro', 'texto', 'Gir Leiteiro', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'idade_meses', '60', 'texto', '60', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'estado_produtivo', 'LACTACAO', 'texto', 'LACTACAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'destinacao', 'Produção de leite', 'texto', 'Produção de leite', 'texto', 'SINTETICA', 'EXEMPLOS-V1'
);

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'peso', '450', 'kg', '450', 'kg', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'producao', '12', 'kg/animal/dia', '12', 'kg/animal/dia', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'gordura', '4.5', '%', '4.5', '%', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'dias_lactacao', '120', 'texto', '120', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'adulta', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'gestante', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'crescimento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'peso_estavel', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'confinada', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'correcao_ambiente', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'correcao_atividade', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'agua', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'ordem_parto', '2', 'texto', '2', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'condicao_corporal', '3', 'texto', '3', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l05',
'observacoes', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5.', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-l06',
'1', 'Pardo-Suíça leiteira — exemplo', 'lote',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Todas as condições são hipóteses; ver anexo A.1.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-l06', 'lote',
'{"procedencia":null,"id":"ex-l06","revisao_id":"","nome":"Pardo-Suíça '
|| 'leiteira — exemplo","animais":1,"raca":"Pardo-Suíça leiteira","idade_m'
|| 'eses":54,"estado_produtivo":"LACTACAO","destinacao":"Produção de leite'
|| '","peso":600,"producao":24,"gordura":4,"dias_lactacao":120,"adulta":"S'
|| 'IM","gestante":"NAO","crescimento":"NAO","peso_estavel":"SIM","confina'
|| 'da":"SIM","correcao_ambiente":"NAO","correcao_atividade":"NAO","agua":'
|| '"SIM","ordem_parto":2,"dispersao_peso":null,"condicao_corporal":3,"obs'
|| 'ervacoes":"Exemplo sintético para experimentação; valores aproximados '
|| 'e condições assumidas. Referências e hipóteses: REQUISITOS.md, anexo '
|| 'A, versão 1.4. Não representa medição real ou comprovação de produto. '
|| 'Condição corporal na escala 1 a 5."}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'nome', 'Pardo-Suíça leiteira — exemplo', 'texto', 'Pardo-Suíça leiteira — exemplo', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'animais', '1', 'texto', '1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'raca', 'Pardo-Suíça leiteira', 'texto', 'Pardo-Suíça leiteira', 'texto', 'SINTETICA', 'EXEMPLOS-V1'
);

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'idade_meses', '54', 'texto', '54', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'estado_produtivo', 'LACTACAO', 'texto', 'LACTACAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'destinacao', 'Produção de leite', 'texto', 'Produção de leite', 'texto', 'SINTETICA', 'EXEMPLOS-V1'
);

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'peso', '600', 'kg', '600', 'kg', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'producao', '24', 'kg/animal/dia', '24', 'kg/animal/dia', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'gordura', '4', '%', '4', '%', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'dias_lactacao', '120', 'texto', '120', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'adulta', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'gestante', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'crescimento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'peso_estavel', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'confinada', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'correcao_ambiente', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'correcao_atividade', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'agua', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'ordem_parto', '2', 'texto', '2', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'condicao_corporal', '3', 'texto', '3', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-l06',
'observacoes', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto. Condição '
|| 'corporal na escala 1 a 5.', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a01',
'1', 'Silagem de milho', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Feedipedia 12871; NDT 65% assumido; FDN estimada por equação. '
|| 'Condições de uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a01', 'alimento',
'{"procedencia":null,"id":"ex-a01","composicao_id":"","nome":"Silagem '
|| 'de milho","classe":"FORRAGEM","ms":0.325,"nutrientes":[0.65,0.069,0.00'
|| '19,0.0017000000000000001,0.44299999999999995,0.29100000000000004,0.027'
|| '999999999999997],"fonte_id":"EXEMPLOS-V1","natureza":"SINTETICA","orig'
|| 'em":"Catálogo de exemplos do '
|| 'sistema","processamento":"Ensilado","metodo":"Feedipedia 12871; NDT '
|| '65% assumido; FDN estimada por equação.","data":"2026-10-21","origem_a'
|| 'nimal":"NAO","excecao_animal":"NAO","nnp_adicionado":"NAO","medicament'
|| 'o":"NAO","autorizado":"SIM","evidencia":"Exemplo sintético para '
|| 'experimentação; valores aproximados e condições assumidas. Referências'
|| ' e hipóteses: REQUISITOS.md, anexo A, versão 1.4. Não representa '
|| 'medição real ou comprovação de produto.","fornecedor":"Demonstração","'
|| 'lote_fisico":"EXEMPLO-ex-a01","rotulo_max_ms":null,"pendf":null,"metod'
|| 'o_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'nome', 'Silagem de milho', 'texto', 'Silagem de milho', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'classe', 'FORRAGEM', 'texto', 'FORRAGEM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'ms', '32.5', '% MN', '32.5', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'processamento', 'Ensilado', 'texto', 'Ensilado', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'metodo', 'Feedipedia 12871; NDT 65% assumido; FDN estimada por equação.', 'texto',
'Feedipedia 12871; NDT 65% assumido; FDN estimada por equação.', 'texto', 'SINTETICA', 'EXEMPLOS-V1'
);

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'lote_fisico', 'EXEMPLO-ex-a01', 'texto', 'EXEMPLO-ex-a01', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'ndt', '65.0', '% MS', '65.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'pb', '6.9', '% MS', '6.9', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'ca', '0.19', '% MS', '0.19', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'p', '0.17', '% MS', '0.17', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'fdn', '44.3', '% MS', '44.3', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'amido', '29.100000000000005', '% MS', '29.100000000000005', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a01',
'ee', '2.8', '% MS', '2.8', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a02',
'1', 'Silagem de sorgo', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Embrapa 2014, silagem de sorgo; valores arredondados; amido assumido. '
|| 'Condições de uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a02', 'alimento',
'{"procedencia":null,"id":"ex-a02","composicao_id":"","nome":"Silagem '
|| 'de sorgo","classe":"FORRAGEM","ms":0.314,"nutrientes":[0.551,0.062,0.0'
|| '028000000000000004,0.0022,0.6709999999999999,0.15,0.024],"fonte_id":"E'
|| 'XEMPLOS-V1","natureza":"SINTETICA","origem":"Catálogo de exemplos do '
|| 'sistema","processamento":"Ensilado","metodo":"Embrapa 2014, silagem de'
|| ' sorgo; valores arredondados; amido assumido.","data":"2026-10-21","or'
|| 'igem_animal":"NAO","excecao_animal":"NAO","nnp_adicionado":"NAO","medi'
|| 'camento":"NAO","autorizado":"SIM","evidencia":"Exemplo sintético para '
|| 'experimentação; valores aproximados e condições assumidas. Referências'
|| ' e hipóteses: REQUISITOS.md, anexo A, versão 1.4. Não representa '
|| 'medição real ou comprovação de produto.","fornecedor":"Demonstração","'
|| 'lote_fisico":"EXEMPLO-ex-a02","rotulo_max_ms":null,"pendf":null,"metod'
|| 'o_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'nome', 'Silagem de sorgo', 'texto', 'Silagem de sorgo', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'classe', 'FORRAGEM', 'texto', 'FORRAGEM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'ms', '31.4', '% MN', '31.4', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'processamento', 'Ensilado', 'texto', 'Ensilado', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'metodo', 'Embrapa 2014, silagem de sorgo; valores arredondados; amido assumido.', 'texto',
'Embrapa 2014, silagem de sorgo; valores arredondados; amido assumido.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'lote_fisico', 'EXEMPLO-ex-a02', 'texto', 'EXEMPLO-ex-a02', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'ndt', '55.1', '% MS', '55.1', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'pb', '6.2', '% MS', '6.2', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'ca', '0.28', '% MS', '0.28', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'p', '0.22', '% MS', '0.22', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'fdn', '67.1', '% MS', '67.1', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'amido', '15.0', '% MS', '15.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a02',
'ee', '2.4', '% MS', '2.4', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a03',
'1', 'Feno de tifton', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Embrapa 2014, feno de tifton; valores arredondados; amido assumido. '
|| 'Condições de uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a03', 'alimento',
'{"procedencia":null,"id":"ex-a03","composicao_id":"","nome":"Feno de t'
|| 'ifton","classe":"FORRAGEM","ms":0.9059999999999999,"nutrientes":[0.529'
|| ',0.081,0.0049,0.0014000000000000002,0.7909999999999999,0.02,0.016],"fo'
|| 'nte_id":"EXEMPLOS-V1","natureza":"SINTETICA","origem":"Catálogo de '
|| 'exemplos do sistema","processamento":"Fenado","metodo":"Embrapa 2014, '
|| 'feno de tifton; valores arredondados; amido assumido.","data":"2026-10'
|| '-21","origem_animal":"NAO","excecao_animal":"NAO","nnp_adicionado":"NA'
|| 'O","medicamento":"NAO","autorizado":"SIM","evidencia":"Exemplo '
|| 'sintético para experimentação; valores aproximados e condições '
|| 'assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.","forneced'
|| 'or":"Demonstração","lote_fisico":"EXEMPLO-ex-a03","rotulo_max_ms":null'
|| ',"pendf":null,"metodo_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'nome', 'Feno de tifton', 'texto', 'Feno de tifton', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'classe', 'FORRAGEM', 'texto', 'FORRAGEM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'ms', '90.6', '% MN', '90.6', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'processamento', 'Fenado', 'texto', 'Fenado', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'metodo', 'Embrapa 2014, feno de tifton; valores arredondados; amido assumido.', 'texto',
'Embrapa 2014, feno de tifton; valores arredondados; amido assumido.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'lote_fisico', 'EXEMPLO-ex-a03', 'texto', 'EXEMPLO-ex-a03', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'ndt', '52.900000000000006', '% MS', '52.900000000000006', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'pb', '8.1', '% MS', '8.1', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'ca', '0.49', '% MS', '0.49', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'p', '0.14', '% MS', '0.14', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'fdn', '79.1', '% MS', '79.1', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'amido', '2.0', '% MS', '2.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a03',
'ee', '1.6', '% MS', '1.6', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a04',
'1', 'Capim-elefante picado', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Perfil inteiramente aproximado, inspirado em Embrapa 2014. Condições '
|| 'de uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a04', 'alimento',
'{"procedencia":null,"id":"ex-a04","composicao_id":"","nome":"Capim-ele'
|| 'fante picado","classe":"FORRAGEM","ms":0.2,"nutrientes":[0.55,0.1,0.00'
|| '4,0.0025,0.65,0.02,0.02],"fonte_id":"EXEMPLOS-V1","natureza":"SINTETIC'
|| 'A","origem":"Catálogo de exemplos do '
|| 'sistema","processamento":"Picado","metodo":"Perfil inteiramente '
|| 'aproximado, inspirado em Embrapa 2014.","data":"2026-10-21","origem_an'
|| 'imal":"NAO","excecao_animal":"NAO","nnp_adicionado":"NAO","medicamento'
|| '":"NAO","autorizado":"SIM","evidencia":"Exemplo sintético para '
|| 'experimentação; valores aproximados e condições assumidas. Referências'
|| ' e hipóteses: REQUISITOS.md, anexo A, versão 1.4. Não representa '
|| 'medição real ou comprovação de produto.","fornecedor":"Demonstração","'
|| 'lote_fisico":"EXEMPLO-ex-a04","rotulo_max_ms":null,"pendf":null,"metod'
|| 'o_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'nome', 'Capim-elefante picado', 'texto', 'Capim-elefante picado', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'classe', 'FORRAGEM', 'texto', 'FORRAGEM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'ms', '20.0', '% MN', '20.0', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'processamento', 'Picado', 'texto', 'Picado', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'metodo', 'Perfil inteiramente aproximado, inspirado em Embrapa 2014.', 'texto',
'Perfil inteiramente aproximado, inspirado em Embrapa 2014.', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'lote_fisico', 'EXEMPLO-ex-a04', 'texto', 'EXEMPLO-ex-a04', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'ndt', '55.00000000000001', '% MS', '55.00000000000001', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'pb', '10.0', '% MS', '10.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'ca', '0.4', '% MS', '0.4', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'p', '0.25', '% MS', '0.25', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'fdn', '65.0', '% MS', '65.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'amido', '2.0', '% MS', '2.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a04',
'ee', '2.0', '% MS', '2.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a05',
'1', 'Cana-de-açúcar picada, sem ureia', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Perfil inteiramente aproximado, inspirado em Embrapa 2014; sem ureia '
|| 'adicionada. Condições de uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a05', 'alimento',
'{"procedencia":null,"id":"ex-a05","composicao_id":"","nome":"Cana-de-a'
|| 'çúcar picada, sem ureia","classe":"FORRAGEM","ms":0.28,"nutrientes":[0'
|| '.6,0.03,0.0025,0.0008,0.5,0.01,0.015],"fonte_id":"EXEMPLOS-V1","nature'
|| 'za":"SINTETICA","origem":"Catálogo de exemplos do '
|| 'sistema","processamento":"Picado","metodo":"Perfil inteiramente '
|| 'aproximado, inspirado em Embrapa 2014; sem ureia adicionada.","data":"'
|| '2026-10-21","origem_animal":"NAO","excecao_animal":"NAO","nnp_adiciona'
|| 'do":"NAO","medicamento":"NAO","autorizado":"SIM","evidencia":"Exemplo '
|| 'sintético para experimentação; valores aproximados e condições '
|| 'assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.","forneced'
|| 'or":"Demonstração","lote_fisico":"EXEMPLO-ex-a05","rotulo_max_ms":null'
|| ',"pendf":null,"metodo_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'nome', 'Cana-de-açúcar picada, sem ureia', 'texto', 'Cana-de-açúcar picada, sem ureia', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'classe', 'FORRAGEM', 'texto', 'FORRAGEM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'ms', '28.000000000000004', '% MN', '28.000000000000004', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'processamento', 'Picado', 'texto', 'Picado', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'metodo', 'Perfil inteiramente aproximado, inspirado em Embrapa 2014; sem ureia ' || 'adicionada.',
'texto', 'Perfil inteiramente aproximado, inspirado em Embrapa 2014; sem ureia ' || 'adicionada.',
'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'lote_fisico', 'EXEMPLO-ex-a05', 'texto', 'EXEMPLO-ex-a05', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'ndt', '60.0', '% MS', '60.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'pb', '3.0', '% MS', '3.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'ca', '0.25', '% MS', '0.25', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'p', '0.08', '% MS', '0.08', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'fdn', '50.0', '% MS', '50.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'amido', '1.0', '% MS', '1.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a05',
'ee', '1.5', '% MS', '1.5', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a06',
'1', 'Milho moído', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Aproximação dos dados sintéticos do projeto e tabela Missouri. '
|| 'Condições de uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a06', 'alimento',
'{"procedencia":null,"id":"ex-a06","composicao_id":"","nome":"Milho moí'
|| 'do","classe":"CONCENTRADO","ms":0.88,"nutrientes":[0.86,0.09,0.0003,0.'
|| '003,0.12,0.7,0.04],"fonte_id":"EXEMPLOS-V1","natureza":"SINTETICA","or'
|| 'igem":"Catálogo de exemplos do '
|| 'sistema","processamento":"Moído","metodo":"Aproximação dos dados '
|| 'sintéticos do projeto e tabela Missouri.","data":"2026-10-21","origem_'
|| 'animal":"NAO","excecao_animal":"NAO","nnp_adicionado":"NAO","medicamen'
|| 'to":"NAO","autorizado":"SIM","evidencia":"Exemplo sintético para '
|| 'experimentação; valores aproximados e condições assumidas. Referências'
|| ' e hipóteses: REQUISITOS.md, anexo A, versão 1.4. Não representa '
|| 'medição real ou comprovação de produto.","fornecedor":"Demonstração","'
|| 'lote_fisico":"EXEMPLO-ex-a06","rotulo_max_ms":null,"pendf":null,"metod'
|| 'o_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'nome', 'Milho moído', 'texto', 'Milho moído', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'classe', 'CONCENTRADO', 'texto', 'CONCENTRADO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'ms', '88.0', '% MN', '88.0', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'processamento', 'Moído', 'texto', 'Moído', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'metodo', 'Aproximação dos dados sintéticos do projeto e tabela Missouri.', 'texto',
'Aproximação dos dados sintéticos do projeto e tabela Missouri.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'lote_fisico', 'EXEMPLO-ex-a06', 'texto', 'EXEMPLO-ex-a06', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'ndt', '86.0', '% MS', '86.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'pb', '9.0', '% MS', '9.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'ca', '0.03', '% MS', '0.03', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'p', '0.3', '% MS', '0.3', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'fdn', '12.0', '% MS', '12.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'amido', '70.0', '% MS', '70.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a06',
'ee', '4.0', '% MS', '4.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a07',
'1', 'Farelo de soja', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Embrapa 2014, farelo de soja; valores arredondados; amido assumido. '
|| 'Condições de uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a07', 'alimento',
'{"procedencia":null,"id":"ex-a07","composicao_id":"","nome":"Farelo de'
|| ' soja","classe":"CONCENTRADO","ms":0.885,"nutrientes":[0.794,0.488,0.0'
|| '04699999999999999,0.0052,0.153,0.02,0.022000000000000002],"fonte_id":"'
|| 'EXEMPLOS-V1","natureza":"SINTETICA","origem":"Catálogo de exemplos do '
|| 'sistema","processamento":"Farelo","metodo":"Embrapa 2014, farelo de '
|| 'soja; valores arredondados; amido assumido.","data":"2026-10-21","orig'
|| 'em_animal":"NAO","excecao_animal":"NAO","nnp_adicionado":"NAO","medica'
|| 'mento":"NAO","autorizado":"SIM","evidencia":"Exemplo sintético para '
|| 'experimentação; valores aproximados e condições assumidas. Referências'
|| ' e hipóteses: REQUISITOS.md, anexo A, versão 1.4. Não representa '
|| 'medição real ou comprovação de produto.","fornecedor":"Demonstração","'
|| 'lote_fisico":"EXEMPLO-ex-a07","rotulo_max_ms":null,"pendf":null,"metod'
|| 'o_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'nome', 'Farelo de soja', 'texto', 'Farelo de soja', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'classe', 'CONCENTRADO', 'texto', 'CONCENTRADO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'ms', '88.5', '% MN', '88.5', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'processamento', 'Farelo', 'texto', 'Farelo', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'metodo', 'Embrapa 2014, farelo de soja; valores arredondados; amido assumido.', 'texto',
'Embrapa 2014, farelo de soja; valores arredondados; amido assumido.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'lote_fisico', 'EXEMPLO-ex-a07', 'texto', 'EXEMPLO-ex-a07', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'ndt', '79.4', '% MS', '79.4', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'pb', '48.8', '% MS', '48.8', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'ca', '0.4699999999999999', '% MS', '0.4699999999999999', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'p', '0.52', '% MS', '0.52', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'fdn', '15.299999999999999', '% MS', '15.299999999999999', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'amido', '2.0', '% MS', '2.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a07',
'ee', '2.2', '% MS', '2.2', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a08',
'1', 'Farelo de trigo', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Embrapa 2014, farelo de trigo; valores arredondados; amido assumido. '
|| 'Condições de uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a08', 'alimento',
'{"procedencia":null,"id":"ex-a08","composicao_id":"","nome":"Farelo de'
|| ' trigo","classe":"CONCENTRADO","ms":0.88,"nutrientes":[0.7040000000000'
|| '001,0.165,0.002,0.0086,0.409,0.22,0.031],"fonte_id":"EXEMPLOS-V1","nat'
|| 'ureza":"SINTETICA","origem":"Catálogo de exemplos do '
|| 'sistema","processamento":"Farelo","metodo":"Embrapa 2014, farelo de '
|| 'trigo; valores arredondados; amido assumido.","data":"2026-10-21","ori'
|| 'gem_animal":"NAO","excecao_animal":"NAO","nnp_adicionado":"NAO","medic'
|| 'amento":"NAO","autorizado":"SIM","evidencia":"Exemplo sintético para '
|| 'experimentação; valores aproximados e condições assumidas. Referências'
|| ' e hipóteses: REQUISITOS.md, anexo A, versão 1.4. Não representa '
|| 'medição real ou comprovação de produto.","fornecedor":"Demonstração","'
|| 'lote_fisico":"EXEMPLO-ex-a08","rotulo_max_ms":null,"pendf":null,"metod'
|| 'o_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'nome', 'Farelo de trigo', 'texto', 'Farelo de trigo', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'classe', 'CONCENTRADO', 'texto', 'CONCENTRADO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'ms', '88.0', '% MN', '88.0', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'processamento', 'Farelo', 'texto', 'Farelo', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'metodo', 'Embrapa 2014, farelo de trigo; valores arredondados; amido assumido.', 'texto',
'Embrapa 2014, farelo de trigo; valores arredondados; amido assumido.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'lote_fisico', 'EXEMPLO-ex-a08', 'texto', 'EXEMPLO-ex-a08', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'ndt', '70.4', '% MS', '70.4', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'pb', '16.5', '% MS', '16.5', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'ca', '0.2', '% MS', '0.2', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'p', '0.86', '% MS', '0.86', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'fdn', '40.9', '% MS', '40.9', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'amido', '22.0', '% MS', '22.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a08',
'ee', '3.1', '% MS', '3.1', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a09',
'1', 'Polpa cítrica seca', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Feedipedia 680; NDT assumido; amido aproximado do resultado '
|| 'enzimático. Condições de uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a09', 'alimento',
'{"procedencia":null,"id":"ex-a09","composicao_id":"","nome":"Polpa '
|| 'cítrica seca","classe":"CONCENTRADO","ms":0.903,"nutrientes":[0.78,0.0'
|| '7,0.017,0.001,0.21600000000000003,0.001,0.025],"fonte_id":"EXEMPLOS-V1'
|| '","natureza":"SINTETICA","origem":"Catálogo de exemplos do '
|| 'sistema","processamento":"Seco","metodo":"Feedipedia 680; NDT '
|| 'assumido; amido aproximado do resultado enzimático.","data":"2026-10-2'
|| '1","origem_animal":"NAO","excecao_animal":"NAO","nnp_adicionado":"NAO"'
|| ',"medicamento":"NAO","autorizado":"SIM","evidencia":"Exemplo sintético'
|| ' para experimentação; valores aproximados e condições assumidas. '
|| 'Referências e hipóteses: REQUISITOS.md, anexo A, versão 1.4. Não '
|| 'representa medição real ou comprovação de produto.","fornecedor":"Demo'
|| 'nstração","lote_fisico":"EXEMPLO-ex-a09","rotulo_max_ms":null,"pendf":'
|| 'null,"metodo_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'nome', 'Polpa cítrica seca', 'texto', 'Polpa cítrica seca', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'classe', 'CONCENTRADO', 'texto', 'CONCENTRADO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'ms', '90.3', '% MN', '90.3', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'processamento', 'Seco', 'texto', 'Seco', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'metodo', 'Feedipedia 680; NDT assumido; amido aproximado do resultado ' || 'enzimático.', 'texto',
'Feedipedia 680; NDT assumido; amido aproximado do resultado ' || 'enzimático.', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'lote_fisico', 'EXEMPLO-ex-a09', 'texto', 'EXEMPLO-ex-a09', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'ndt', '78.0', '% MS', '78.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'pb', '7.000000000000001', '% MS', '7.000000000000001', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'ca', '1.7000000000000002', '% MS', '1.7000000000000002', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'p', '0.1', '% MS', '0.1', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'fdn', '21.6', '% MS', '21.6', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'amido', '0.1', '% MS', '0.1', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a09',
'ee', '2.5', '% MS', '2.5', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a10',
'1', 'Casca de soja', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Aproximação Missouri; NDT, FDN, amido e EE assumidos. Condições de uso'
|| ' assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a10', 'alimento',
'{"procedencia":null,"id":"ex-a10","composicao_id":"","nome":"Casca de '
|| 'soja","classe":"CONCENTRADO","ms":0.9,"nutrientes":[0.74,0.12,0.006,0.'
|| '0018,0.65,0.03,0.02],"fonte_id":"EXEMPLOS-V1","natureza":"SINTETICA","'
|| 'origem":"Catálogo de exemplos do '
|| 'sistema","processamento":"Seco","metodo":"Aproximação Missouri; NDT, '
|| 'FDN, amido e EE assumidos.","data":"2026-10-21","origem_animal":"NAO",'
|| '"excecao_animal":"NAO","nnp_adicionado":"NAO","medicamento":"NAO","aut'
|| 'orizado":"SIM","evidencia":"Exemplo sintético para experimentação; '
|| 'valores aproximados e condições assumidas. Referências e hipóteses: '
|| 'REQUISITOS.md, anexo A, versão 1.4. Não representa medição real ou '
|| 'comprovação de produto.","fornecedor":"Demonstração","lote_fisico":"EX'
|| 'EMPLO-ex-a10","rotulo_max_ms":null,"pendf":null,"metodo_fisico":"","co' || 'ntaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'nome', 'Casca de soja', 'texto', 'Casca de soja', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'classe', 'CONCENTRADO', 'texto', 'CONCENTRADO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'ms', '90.0', '% MN', '90.0', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'processamento', 'Seco', 'texto', 'Seco', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'metodo', 'Aproximação Missouri; NDT, FDN, amido e EE assumidos.', 'texto',
'Aproximação Missouri; NDT, FDN, amido e EE assumidos.', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'lote_fisico', 'EXEMPLO-ex-a10', 'texto', 'EXEMPLO-ex-a10', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'ndt', '74.0', '% MS', '74.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'pb', '12.0', '% MS', '12.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'ca', '0.6', '% MS', '0.6', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'p', '0.18', '% MS', '0.18', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'fdn', '65.0', '% MS', '65.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'amido', '3.0', '% MS', '3.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a10',
'ee', '2.0', '% MS', '2.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a11',
'1', 'Caroço de algodão', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Embrapa 2014 arredondada; Ca/P aproximados Missouri; NDT e amido '
|| 'assumidos. Condições de uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a11', 'alimento',
'{"procedencia":null,"id":"ex-a11","composicao_id":"","nome":"Caroço de'
|| ' algodão","classe":"CONCENTRADO","ms":0.897,"nutrientes":[0.9,0.223999'
|| '99999999998,0.0017000000000000001,0.006,0.441,0.02,0.198],"fonte_id":"'
|| 'EXEMPLOS-V1","natureza":"SINTETICA","origem":"Catálogo de exemplos do '
|| 'sistema","processamento":"Integral","metodo":"Embrapa 2014 '
|| 'arredondada; Ca/P aproximados Missouri; NDT e amido assumidos.","data"'
|| ':"2026-10-21","origem_animal":"NAO","excecao_animal":"NAO","nnp_adicio'
|| 'nado":"NAO","medicamento":"NAO","autorizado":"SIM","evidencia":"Exempl'
|| 'o sintético para experimentação; valores aproximados e condições '
|| 'assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.","forneced'
|| 'or":"Demonstração","lote_fisico":"EXEMPLO-ex-a11","rotulo_max_ms":null'
|| ',"pendf":null,"metodo_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'nome', 'Caroço de algodão', 'texto', 'Caroço de algodão', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'classe', 'CONCENTRADO', 'texto', 'CONCENTRADO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'ms', '89.7', '% MN', '89.7', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'processamento', 'Integral', 'texto', 'Integral', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'metodo', 'Embrapa 2014 arredondada; Ca/P aproximados Missouri; NDT e amido ' || 'assumidos.',
'texto', 'Embrapa 2014 arredondada; Ca/P aproximados Missouri; NDT e amido ' || 'assumidos.',
'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'lote_fisico', 'EXEMPLO-ex-a11', 'texto', 'EXEMPLO-ex-a11', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'ndt', '90.0', '% MS', '90.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'pb', '22.4', '% MS', '22.4', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'ca', '0.17', '% MS', '0.17', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'p', '0.6', '% MS', '0.6', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'fdn', '44.1', '% MS', '44.1', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'amido', '2.0', '% MS', '2.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a11',
'ee', '19.8', '% MS', '19.8', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a12',
'1', 'Mistura mineral — exemplo', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Composição sintética do projeto; zeros são hipóteses simplificadoras. '
|| 'Condições de uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a12', 'alimento',
'{"procedencia":null,"id":"ex-a12","composicao_id":"","nome":"Mistura '
|| 'mineral — exemplo","classe":"SUPLEMENTO","ms":0.95,"nutrientes":[0.0,0'
|| '.0,0.2,0.08,0.0,0.0,0.0],"fonte_id":"EXEMPLOS-V1","natureza":"SINTETIC'
|| 'A","origem":"Catálogo de exemplos do '
|| 'sistema","processamento":"Misturado","metodo":"Composição sintética do'
|| ' projeto; zeros são hipóteses simplificadoras.","data":"2026-10-21","o'
|| 'rigem_animal":"NAO","excecao_animal":"NAO","nnp_adicionado":"NAO","med'
|| 'icamento":"NAO","autorizado":"SIM","evidencia":"Exemplo sintético para'
|| ' experimentação; valores aproximados e condições assumidas. '
|| 'Referências e hipóteses: REQUISITOS.md, anexo A, versão 1.4. Não '
|| 'representa medição real ou comprovação de produto.","fornecedor":"Demo'
|| 'nstração","lote_fisico":"EXEMPLO-ex-a12","rotulo_max_ms":null,"pendf":'
|| 'null,"metodo_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'nome', 'Mistura mineral — exemplo', 'texto', 'Mistura mineral — exemplo', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'classe', 'SUPLEMENTO', 'texto', 'SUPLEMENTO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'ms', '95.0', '% MN', '95.0', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'processamento', 'Misturado', 'texto', 'Misturado', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'metodo', 'Composição sintética do projeto; zeros são hipóteses simplificadoras.', 'texto',
'Composição sintética do projeto; zeros são hipóteses simplificadoras.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'lote_fisico', 'EXEMPLO-ex-a12', 'texto', 'EXEMPLO-ex-a12', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'ndt', '0.0', '% MS', '0.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'pb', '0.0', '% MS', '0.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'ca', '20.0', '% MS', '20.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'p', '8.0', '% MS', '8.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'fdn', '0.0', '% MS', '0.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'amido', '0.0', '% MS', '0.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a12',
'ee', '0.0', '% MS', '0.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a13',
'1', 'Calcário calcítico — exemplo', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'MS/Ca Missouri; demais zeros são hipóteses simplificadoras. Condições '
|| 'de uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a13', 'alimento',
'{"procedencia":null,"id":"ex-a13","composicao_id":"","nome":"Calcário '
|| 'calcítico — exemplo","classe":"SUPLEMENTO","ms":0.98,"nutrientes":[0.0'
|| ',0.0,0.36,0.0,0.0,0.0,0.0],"fonte_id":"EXEMPLOS-V1","natureza":"SINTET'
|| 'ICA","origem":"Catálogo de exemplos do '
|| 'sistema","processamento":"Moído","metodo":"MS/Ca Missouri; demais '
|| 'zeros são hipóteses simplificadoras.","data":"2026-10-21","origem_anim'
|| 'al":"NAO","excecao_animal":"NAO","nnp_adicionado":"NAO","medicamento":'
|| '"NAO","autorizado":"SIM","evidencia":"Exemplo sintético para '
|| 'experimentação; valores aproximados e condições assumidas. Referências'
|| ' e hipóteses: REQUISITOS.md, anexo A, versão 1.4. Não representa '
|| 'medição real ou comprovação de produto.","fornecedor":"Demonstração","'
|| 'lote_fisico":"EXEMPLO-ex-a13","rotulo_max_ms":null,"pendf":null,"metod'
|| 'o_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'nome', 'Calcário calcítico — exemplo', 'texto', 'Calcário calcítico — exemplo', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'classe', 'SUPLEMENTO', 'texto', 'SUPLEMENTO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'ms', '98.0', '% MN', '98.0', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'processamento', 'Moído', 'texto', 'Moído', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'metodo', 'MS/Ca Missouri; demais zeros são hipóteses simplificadoras.', 'texto',
'MS/Ca Missouri; demais zeros são hipóteses simplificadoras.', 'texto', 'SINTETICA', 'EXEMPLOS-V1'
);

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'lote_fisico', 'EXEMPLO-ex-a13', 'texto', 'EXEMPLO-ex-a13', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'ndt', '0.0', '% MS', '0.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'pb', '0.0', '% MS', '0.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'ca', '36.0', '% MS', '36.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'p', '0.0', '% MS', '0.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'fdn', '0.0', '% MS', '0.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'amido', '0.0', '% MS', '0.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a13',
'ee', '0.0', '% MS', '0.0', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a14',
'1', 'Mistura milho + soja', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Receita em MN: 70% ex-a06, 27% ex-a07, 3% ex-a12. Nutrientes '
|| 'ponderados pela MS; hipótese de aditividade; anexo A.3. Condições de '
|| 'uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a14', 'alimento',
'{"procedencia":null,"id":"ex-a14","composicao_id":"","nome":"Mistura '
|| 'milho + soja","classe":"CONCENTRADO","ms":0.88345,"nutrientes":[0.8144'
|| '052294979908,0.19474514686739489,0.007932384402060105,0.00607905370988'
|| '7373,0.1250544456392552,0.4934959533646499,0.033841077593525386],"font'
|| 'e_id":"EXEMPLOS-V1","natureza":"SINTETICA","origem":"Catálogo de '
|| 'exemplos do sistema","processamento":"Misturado","metodo":"Receita em '
|| 'MN: 70% ex-a06, 27% ex-a07, 3% ex-a12. Nutrientes ponderados pela MS; '
|| 'hipótese de aditividade; anexo A.3.","data":"2026-10-21","origem_anima'
|| 'l":"NAO","excecao_animal":"NAO","nnp_adicionado":"NAO","medicamento":"'
|| 'NAO","autorizado":"SIM","evidencia":"Exemplo sintético para '
|| 'experimentação; valores aproximados e condições assumidas. Referências'
|| ' e hipóteses: REQUISITOS.md, anexo A, versão 1.4. Não representa '
|| 'medição real ou comprovação de produto.","fornecedor":"Demonstração","'
|| 'lote_fisico":"EXEMPLO-ex-a14","rotulo_max_ms":null,"pendf":null,"metod'
|| 'o_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'nome', 'Mistura milho + soja', 'texto', 'Mistura milho + soja', 'texto', 'SINTETICA', 'EXEMPLOS-V1'
);

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'classe', 'CONCENTRADO', 'texto', 'CONCENTRADO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'ms', '88.345', '% MN', '88.345', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'processamento', 'Misturado', 'texto', 'Misturado', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'metodo', 'Receita em MN: 70% ex-a06, 27% ex-a07, 3% ex-a12. Nutrientes '
|| 'ponderados pela MS; hipótese de aditividade; anexo A.3.', 'texto',
'Receita em MN: 70% ex-a06, 27% ex-a07, 3% ex-a12. Nutrientes '
|| 'ponderados pela MS; hipótese de aditividade; anexo A.3.', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'lote_fisico', 'EXEMPLO-ex-a14', 'texto', 'EXEMPLO-ex-a14', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'ndt', '81.44052294979907', '% MS', '81.44052294979907', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'pb', '19.47451468673949', '% MS', '19.47451468673949', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'ca', '0.7932384402060104', '% MS', '0.7932384402060104', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'p', '0.6079053709887373', '% MS', '0.6079053709887373', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'fdn', '12.505444563925519', '% MS', '12.505444563925519', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'amido', '49.34959533646499', '% MS', '49.34959533646499', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a14',
'ee', '3.3841077593525384', '% MS', '3.3841077593525384', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO modelos_preenchimento (id,versao,nome,tipo,contexto,lacunas,fonte_id) VALUES ( 'ex-a15',
'1', 'Mistura com casca de soja', 'alimento',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.',
'Receita em MN: 50% ex-a06, 25% ex-a07, 22% ex-a10, 3% ex-a12. '
|| 'Nutrientes ponderados pela MS; hipótese de aditividade; anexo A.3. '
|| 'Condições de uso assumidas; sem laudos ou rótulo real.', 'EXEMPLOS-V1' );

INSERT INTO exemplos (id,tipo,conteudo) VALUES ( 'ex-a15', 'alimento',
'{"procedencia":null,"id":"ex-a15","composicao_id":"","nome":"Mistura '
|| 'com casca de soja","classe":"CONCENTRADO","ms":0.88775,"nutrientes":[0'
|| '.7891776964235426,0.19299352295128133,0.009078991833286398,0.005752633'
|| '061109546,0.24258096310898336,0.3586201070121093,0.02976907913263869],'
|| '"fonte_id":"EXEMPLOS-V1","natureza":"SINTETICA","origem":"Catálogo de '
|| 'exemplos do sistema","processamento":"Misturado","metodo":"Receita em '
|| 'MN: 50% ex-a06, 25% ex-a07, 22% ex-a10, 3% ex-a12. Nutrientes '
|| 'ponderados pela MS; hipótese de aditividade; anexo A.3.","data":"2026-'
|| '10-21","origem_animal":"NAO","excecao_animal":"NAO","nnp_adicionado":"'
|| 'NAO","medicamento":"NAO","autorizado":"SIM","evidencia":"Exemplo '
|| 'sintético para experimentação; valores aproximados e condições '
|| 'assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.","forneced'
|| 'or":"Demonstração","lote_fisico":"EXEMPLO-ex-a15","rotulo_max_ms":null'
|| ',"pendf":null,"metodo_fisico":"","contaminantes":[]}' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'nome', 'Mistura com casca de soja', 'texto', 'Mistura com casca de soja', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'classe', 'CONCENTRADO', 'texto', 'CONCENTRADO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'ms', '88.775', '% MN', '88.775', '% MN', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'fonte_id', 'EXEMPLOS-V1', 'texto', 'EXEMPLOS-V1', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'natureza', 'SINTETICA', 'texto', 'SINTETICA', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'origem', 'Catálogo de exemplos do sistema', 'texto', 'Catálogo de exemplos do sistema', 'texto',
'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'processamento', 'Misturado', 'texto', 'Misturado', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'metodo', 'Receita em MN: 50% ex-a06, 25% ex-a07, 22% ex-a10, 3% ex-a12. '
|| 'Nutrientes ponderados pela MS; hipótese de aditividade; anexo A.3.', 'texto',
'Receita em MN: 50% ex-a06, 25% ex-a07, 22% ex-a10, 3% ex-a12. '
|| 'Nutrientes ponderados pela MS; hipótese de aditividade; anexo A.3.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'data', '2026-10-21', 'texto', '2026-10-21', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'origem_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'excecao_animal', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'nnp_adicionado', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'medicamento', 'NAO', 'texto', 'NAO', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'autorizado', 'SIM', 'texto', 'SIM', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'evidencia', 'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto',
'Exemplo sintético para experimentação; valores aproximados e condições'
|| ' assumidas. Referências e hipóteses: REQUISITOS.md, anexo A, versão '
|| '1.4. Não representa medição real ou comprovação de produto.', 'texto', 'SINTETICA',
'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'fornecedor', 'Demonstração', 'texto', 'Demonstração', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'lote_fisico', 'EXEMPLO-ex-a15', 'texto', 'EXEMPLO-ex-a15', 'texto', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'ndt', '78.91776964235426', '% MS', '78.91776964235426', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'pb', '19.299352295128134', '% MS', '19.299352295128134', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'ca', '0.9078991833286398', '% MS', '0.9078991833286398', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'p', '0.5752633061109547', '% MS', '0.5752633061109547', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'fdn', '24.258096310898335', '% MS', '24.258096310898335', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'amido', '35.86201070121093', '% MS', '35.86201070121093', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );

INSERT INTO valores_modelo
(modelo_id,campo,valor,unidade,valor_original,unidade_original,natureza,fonte_id) VALUES ( 'ex-a15',
'ee', '2.9769079132638687', '% MS', '2.9769079132638687', '% MS', 'SINTETICA', 'EXEMPLOS-V1' );
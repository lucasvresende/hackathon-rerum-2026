-- versao: contaminantes-v1
INSERT INTO limites_contaminantes
(perfil_id,norma_id,analito,matriz,limite,umidade,unidade) VALUES
('E2020_NRC1989_RUMEN_MVP_v1','UE-AFB1','AFB1','MATERIA_PRIMA',20,0.12,'µg/kg'),
('E2020_NRC1989_RUMEN_MVP_v1','UE-AFB1','AFB1','COMPOSTO_LEITEIRO',5,0.12,'µg/kg'),
('E2020_NRC1989_RUMEN_MVP_v1','UE-AFB1','GOSSIPOL_LIVRE',
'COMPOSTO_LEITEIRO',500000,0.12,'µg/kg'),
('E2020_NRC1989_RUMEN_MVP_v1','UE-AFB1','HCN','COMPOSTO_LEITEIRO',50000,0.12,'µg/kg');

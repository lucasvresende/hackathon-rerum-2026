SELECT norma_id,analito,matriz,limite,umidade,unidade FROM limites_contaminantes
WHERE perfil_id = ? ORDER BY analito, matriz;

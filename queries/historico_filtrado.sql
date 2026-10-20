SELECT e.id,c.id AS cenario_id,c.nome,e.criada_em,e.estado,e.custo,c.entradas,
e.resultado,e.artefatos FROM execucoes e JOIN cenarios c ON c.id = e.cenario_id
WHERE (?1 IS NULL OR json_extract(c.entradas,'$.lote.nome') LIKE ?1)
AND (?2 IS NULL OR e.criada_em >= ?2) AND (?3 IS NULL OR e.criada_em < ?3)
ORDER BY e.criada_em DESC,e.id LIMIT 100 OFFSET ?4;

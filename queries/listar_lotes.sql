SELECT r.id, l.id AS lote_id, l.nome, r.conteudo FROM lotes l
JOIN revisoes_lote r ON r.lote_id = l.id WHERE l.ativo = 1
AND r.numero = (SELECT MAX(v.numero) FROM revisoes_lote v WHERE v.lote_id = l.id)
ORDER BY l.nome, l.id;

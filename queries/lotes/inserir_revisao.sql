INSERT INTO revisoes_lote
(id, lote_id, numero, peso, animais, producao, gordura, dias_lactacao, conteudo)
VALUES (?, ?, (SELECT COALESCE(MAX(numero), 0) + 1 FROM revisoes_lote WHERE lote_id = ?),
?, ?, ?, ?, ?, ?);

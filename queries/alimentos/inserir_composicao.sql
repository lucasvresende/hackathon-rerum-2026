INSERT INTO composicoes
(id, alimento_id, fonte_id, numero, ms, ndt, pb, ca, p, fdn, amido, ee, natureza, conteudo)
VALUES (?, ?, ?, (SELECT COALESCE(MAX(numero), 0) + 1 FROM composicoes WHERE alimento_id = ?),
?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

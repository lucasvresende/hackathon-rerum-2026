SELECT e.id, c.id AS cenario_id, c.nome, e.criada_em, e.estado, e.custo,
c.entradas, e.resultado, e.artefatos FROM execucoes e
JOIN cenarios c ON c.id = e.cenario_id WHERE e.id = ?;

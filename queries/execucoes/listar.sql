SELECT e.id, c.nome, e.criada_em, e.estado, e.custo, c.entradas,
e.resultado, e.artefatos FROM execucoes e JOIN cenarios c ON c.id = e.cenario_id
ORDER BY e.criada_em DESC, e.id LIMIT 100;

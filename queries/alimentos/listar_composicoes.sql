SELECT c.id, a.id AS alimento_id, a.nome, c.conteudo
FROM composicoes c JOIN alimentos a ON a.id = c.alimento_id
WHERE a.ativo = 1 ORDER BY a.nome, c.numero DESC;

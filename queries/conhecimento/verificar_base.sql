SELECT
(SELECT COUNT(*) FROM perfis WHERE ativo = 1) AS "perfis!: i64",
(SELECT COUNT(*) FROM fontes) AS "fontes!: i64";

SELECT m.*, f.titulo AS fonte_titulo, f.localizador AS fonte_url
FROM modelos_preenchimento m JOIN fontes f ON f.id = m.fonte_id
ORDER BY m.tipo, m.nome, m.id;

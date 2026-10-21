CREATE TABLE exemplos (
    id TEXT PRIMARY KEY NOT NULL REFERENCES modelos_preenchimento(id),
    tipo TEXT NOT NULL CHECK(tipo IN ('lote', 'alimento')),
    conteudo TEXT NOT NULL CHECK(json_valid(conteudo))
);
CREATE TRIGGER preservar_exemplo BEFORE UPDATE ON exemplos
BEGIN SELECT RAISE(ABORT, 'Exemplo original imutável: crie uma versão'); END;
CREATE TRIGGER preservar_exemplo_exclusao BEFORE DELETE ON exemplos
BEGIN SELECT RAISE(ABORT, 'Exemplo original preservado'); END;
CREATE TABLE inicializacao_exemplos (
    id INTEGER PRIMARY KEY CHECK(id = 1),
    cenario TEXT NOT NULL CHECK(json_valid(cenario))
);

CREATE TABLE limites_contaminantes (
    perfil_id TEXT NOT NULL REFERENCES perfis(id),
    norma_id TEXT NOT NULL REFERENCES normas(id),
    analito TEXT NOT NULL,
    matriz TEXT NOT NULL CHECK(matriz IN ('MATERIA_PRIMA','COMPOSTO_LEITEIRO')),
    limite REAL NOT NULL CHECK(limite >= 0),
    umidade REAL NOT NULL CHECK(umidade >= 0 AND umidade < 1),
    unidade TEXT NOT NULL CHECK(unidade = 'µg/kg'),
    PRIMARY KEY(perfil_id, norma_id, analito, matriz)
);
CREATE TRIGGER preservar_limite BEFORE UPDATE ON limites_contaminantes
BEGIN SELECT RAISE(ABORT, 'Limite publicado imutável'); END;

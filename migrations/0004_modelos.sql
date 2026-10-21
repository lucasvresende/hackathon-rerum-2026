CREATE TABLE modelos_preenchimento (
    id TEXT PRIMARY KEY NOT NULL, versao TEXT NOT NULL,
    nome TEXT NOT NULL, tipo TEXT NOT NULL CHECK(tipo IN ('lote','alimento')),
    contexto TEXT NOT NULL, lacunas TEXT NOT NULL,
    fonte_id TEXT NOT NULL REFERENCES fontes(id)
);
CREATE TABLE valores_modelo (
    modelo_id TEXT NOT NULL REFERENCES modelos_preenchimento(id),
    campo TEXT NOT NULL, valor TEXT NOT NULL, unidade TEXT NOT NULL,
    valor_original TEXT NOT NULL, unidade_original TEXT NOT NULL,
    natureza TEXT NOT NULL, fonte_id TEXT NOT NULL REFERENCES fontes(id),
    PRIMARY KEY(modelo_id, campo)
);
CREATE TRIGGER preservar_modelo BEFORE UPDATE ON modelos_preenchimento
BEGIN SELECT RAISE(ABORT, 'Modelo imutável: crie uma versão'); END;
CREATE TRIGGER preservar_valor_modelo BEFORE UPDATE ON valores_modelo
BEGIN SELECT RAISE(ABORT, 'Valor de modelo imutável: crie uma versão'); END;
CREATE TRIGGER preservar_modelo_exclusao BEFORE DELETE ON modelos_preenchimento
BEGIN SELECT RAISE(ABORT, 'Modelo preservado para auditoria'); END;
CREATE TRIGGER preservar_valor_exclusao BEFORE DELETE ON valores_modelo
BEGIN SELECT RAISE(ABORT, 'Valor preservado para auditoria'); END;

CREATE TABLE fontes (
    id TEXT PRIMARY KEY NOT NULL, titulo TEXT NOT NULL, edicao TEXT NOT NULL,
    localizador TEXT NOT NULL, natureza TEXT NOT NULL,
    conteudo TEXT NOT NULL, hash TEXT NOT NULL
);
CREATE TABLE perfis (
    id TEXT PRIMARY KEY NOT NULL, versao TEXT NOT NULL,
    fonte_id TEXT NOT NULL REFERENCES fontes(id),
    fdn_min REAL NOT NULL CHECK(fdn_min BETWEEN 0 AND 1),
    amido_max REAL NOT NULL CHECK(amido_max BETWEEN 0 AND 1),
    ee_max REAL NOT NULL CHECK(ee_max BETWEEN 0 AND 1),
    discrepancia REAL NOT NULL CHECK(discrepancia > 0), ativo INTEGER NOT NULL DEFAULT 0
);
CREATE UNIQUE INDEX perfil_ativo ON perfis(ativo) WHERE ativo = 1;
CREATE TABLE coeficientes (
    perfil_id TEXT NOT NULL REFERENCES perfis(id),
    tabela TEXT NOT NULL CHECK(tabela IN ('mantenca','leite','consumo','fibra')),
    eixo REAL NOT NULL, eixo2 REAL NOT NULL DEFAULT 0,
    a REAL NOT NULL CHECK(a >= 0), b REAL NOT NULL CHECK(b >= 0),
    c REAL NOT NULL CHECK(c >= 0), d REAL NOT NULL CHECK(d >= 0),
    unidade TEXT NOT NULL CHECK(unidade IN ('kg/dia','kg/kg leite','% PV','% MS')),
    PRIMARY KEY(perfil_id, tabela, eixo, eixo2)
);
CREATE TABLE cargas (
    versao TEXT PRIMARY KEY NOT NULL, hash TEXT NOT NULL, conteudo TEXT NOT NULL,
    aplicada_em TEXT NOT NULL DEFAULT(strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);
CREATE TABLE normas (
    id TEXT PRIMARY KEY NOT NULL, fonte_id TEXT NOT NULL REFERENCES fontes(id),
    natureza TEXT NOT NULL, jurisdicao TEXT NOT NULL, publicacao TEXT NOT NULL,
    inicio TEXT NOT NULL, fim TEXT, transicao TEXT, condicoes TEXT NOT NULL,
    excecoes TEXT NOT NULL, evidencia TEXT NOT NULL, acao TEXT NOT NULL
);
CREATE TABLE lotes (
    id TEXT PRIMARY KEY NOT NULL, nome TEXT NOT NULL, ativo INTEGER NOT NULL DEFAULT 1
);
CREATE TABLE revisoes_lote (
    id TEXT PRIMARY KEY NOT NULL, lote_id TEXT NOT NULL REFERENCES lotes(id),
    numero INTEGER NOT NULL CHECK(numero > 0), peso REAL NOT NULL CHECK(peso > 0),
    animais INTEGER NOT NULL CHECK(animais > 0), producao REAL NOT NULL CHECK(producao >= 0),
    gordura REAL NOT NULL CHECK(gordura > 0), dias_lactacao INTEGER NOT NULL,
    conteudo TEXT NOT NULL CHECK(json_valid(conteudo)),
    criada_em TEXT NOT NULL DEFAULT(strftime('%Y-%m-%dT%H:%M:%fZ','now')),
    UNIQUE(lote_id, numero)
);
CREATE TABLE alimentos (
    id TEXT PRIMARY KEY NOT NULL, nome TEXT NOT NULL, classe TEXT NOT NULL,
    ativo INTEGER NOT NULL DEFAULT 1
);
CREATE TABLE composicoes (
    id TEXT PRIMARY KEY NOT NULL, alimento_id TEXT NOT NULL REFERENCES alimentos(id),
    fonte_id TEXT NOT NULL REFERENCES fontes(id), numero INTEGER NOT NULL CHECK(numero > 0),
    ms REAL NOT NULL CHECK(ms > 0 AND ms <= 1),
    ndt REAL CHECK(ndt >= 0), pb REAL CHECK(pb BETWEEN 0 AND 1),
    ca REAL CHECK(ca BETWEEN 0 AND 1), p REAL CHECK(p BETWEEN 0 AND 1),
    fdn REAL CHECK(fdn BETWEEN 0 AND 1), amido REAL CHECK(amido BETWEEN 0 AND 1),
    ee REAL CHECK(ee BETWEEN 0 AND 1), natureza TEXT NOT NULL,
    conteudo TEXT NOT NULL CHECK(json_valid(conteudo)), UNIQUE(alimento_id, numero)
);
CREATE TABLE produtos (
    id TEXT PRIMARY KEY NOT NULL, composicao_id TEXT NOT NULL REFERENCES composicoes(id),
    fornecedor TEXT NOT NULL, lote_fisico TEXT NOT NULL, evidencia TEXT NOT NULL,
    conteudo TEXT NOT NULL CHECK(json_valid(conteudo))
);
CREATE TABLE cenarios (
    id TEXT PRIMARY KEY NOT NULL, nome TEXT NOT NULL,
    revisao_lote_id TEXT NOT NULL REFERENCES revisoes_lote(id),
    perfil_id TEXT NOT NULL REFERENCES perfis(id), original_id TEXT REFERENCES cenarios(id),
    horizonte INTEGER NOT NULL CHECK(horizonte > 0), data_aplicacao TEXT NOT NULL,
    entradas TEXT NOT NULL CHECK(json_valid(entradas))
);
CREATE TABLE itens_cenario (
    cenario_id TEXT NOT NULL REFERENCES cenarios(id),
    composicao_id TEXT NOT NULL REFERENCES composicoes(id), quantidade REAL NOT NULL,
    preco TEXT NOT NULL, estoque REAL, perda REAL NOT NULL,
    minimo REAL NOT NULL, maximo REAL, PRIMARY KEY(cenario_id, composicao_id)
);
CREATE TABLE execucoes (
    id TEXT PRIMARY KEY NOT NULL, cenario_id TEXT NOT NULL REFERENCES cenarios(id),
    criada_em TEXT NOT NULL, metodo TEXT NOT NULL, versao_app TEXT NOT NULL,
    estado TEXT NOT NULL, custo REAL, duracao_ms INTEGER NOT NULL,
    resultado TEXT NOT NULL CHECK(json_valid(resultado)),
    artefatos TEXT NOT NULL CHECK(json_valid(artefatos))
);
CREATE TABLE verificacoes (
    execucao_id TEXT NOT NULL REFERENCES execucoes(id), ordem INTEGER NOT NULL,
    regra TEXT NOT NULL, dimensao TEXT NOT NULL, estado TEXT NOT NULL,
    valor REAL, limite REAL, unidade TEXT NOT NULL, fonte TEXT NOT NULL,
    explicacao TEXT NOT NULL, dependencias TEXT NOT NULL,
    PRIMARY KEY(execucao_id, ordem)
);
CREATE INDEX historico_por_data ON execucoes(criada_em DESC);
CREATE TRIGGER preservar_execucao BEFORE UPDATE ON execucoes
BEGIN SELECT RAISE(ABORT, 'Execução imutável'); END;
CREATE TRIGGER preservar_composicao BEFORE UPDATE ON composicoes
BEGIN SELECT RAISE(ABORT, 'Composição imutável: crie uma revisão'); END;
CREATE TRIGGER preservar_revisao BEFORE UPDATE ON revisoes_lote
BEGIN SELECT RAISE(ABORT, 'Revisão imutável'); END;

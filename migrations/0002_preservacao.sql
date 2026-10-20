CREATE TRIGGER preservar_fonte BEFORE UPDATE ON fontes
BEGIN SELECT RAISE(ABORT, 'Fonte publicada imutável'); END;
CREATE TRIGGER preservar_coeficiente BEFORE UPDATE ON coeficientes
BEGIN SELECT RAISE(ABORT, 'Coeficiente publicado imutável'); END;
CREATE TRIGGER preservar_norma BEFORE UPDATE ON normas
BEGIN SELECT RAISE(ABORT, 'Norma publicada imutável'); END;
CREATE TRIGGER preservar_perfil BEFORE UPDATE OF
id,versao,fonte_id,fdn_min,amido_max,ee_max,discrepancia ON perfis
BEGIN SELECT RAISE(ABORT, 'Perfil publicado imutável'); END;
CREATE TRIGGER preservar_cenario BEFORE UPDATE ON cenarios
BEGIN SELECT RAISE(ABORT, 'Cenário executado imutável'); END;
CREATE TRIGGER preservar_trace BEFORE UPDATE ON verificacoes
BEGIN SELECT RAISE(ABORT, 'Traço imutável'); END;
CREATE TRIGGER preservar_itens BEFORE UPDATE ON itens_cenario
BEGIN SELECT RAISE(ABORT, 'Entradas imutáveis'); END;
CREATE TRIGGER impedir_exclusao_execucao BEFORE DELETE ON execucoes
BEGIN SELECT RAISE(ABORT, 'Execução histórica não pode ser excluída'); END;
CREATE TRIGGER impedir_exclusao_trace BEFORE DELETE ON verificacoes
BEGIN SELECT RAISE(ABORT, 'Traço histórico não pode ser excluído'); END;

# Mapa da documentação

O [README principal](../README.md) oferece o início rápido em desenvolvimento e release.
Continue conforme sua tarefa:

1. **Operar:** [uso e telas](USO.md) → [instalação, configuração e backup](OPERACAO.md).
2. **Entender o produto:** [requisitos](REQUISITOS.md) → [motor](MOTOR.md).
3. **Manter o código:** [arquitetura e banco](ARQUITETURA.md) →
   [testes, CI e rastreabilidade](QUALIDADE.md).
4. **Auditar decisões:** [fontes, hipóteses e dependências](FONTES.md) →
   [concepção e assistência por IA](PROCESSO.md).

Cada tema tem um documento principal. Requisitos definem o comportamento; Motor detalha
cálculos; Qualidade mostra onde verificá-los. Números do perfil têm origem na carga SQL
versionada. Capturas em `imagens/` ilustram fluxos de teste e são identificadas como tais.

Ao mudar código, regra, dependência ou operação, atualize a página correspondente no mesmo
commit. Evite reproduzir grandes tabelas de fonte em vários arquivos: a carga versionada é a
referência numérica e a interface **Perfil nutricional** expõe seus valores.

A [auditoria dos fluxos no navegador](AUDITORIA_FLUXOS.md) registra a inspeção de 22/10/2026,
com achados reproduzíveis sobre cadastros, cálculos, comparação e navegação.

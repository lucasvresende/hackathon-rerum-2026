# Plano de implementação do MVP de dietas

## Objetivo e direção

Implementar o sistema definido em
[REQUISITOS.md](../REQUISITOS.md),
partindo da estrutura Rust inicial existente.

O primeiro marco funcional será **verificar uma dieta de ponta a ponta**:
cadastrar lote, selecionar ingredientes, calcular, explicar e consultar o histórico.
Depois, integrar a formulação de menor custo e concluir os demais requisitos P0.

## Etapas e entregas


**1. Fundação técnica**

Estruturar o monólito modular; configurar Axum, tracing, Maud, recursos locais, desenvolvimento e
CI. Avaliar antecipadamente o solver Rust nativo, incluindo cancelamento efetivo.

Conclusão:
Aplicação inicia em localhost; build, testes e verificações básicas passam.

**2. Dados e conhecimento**

Criar migrações SQLite, consultas SQLx verificadas em compilação e cargas versionadas. Introduzir
fontes, perfil, tabelas A/B/C, catálogo e dados sintéticos identificados.

Conclusão:
Banco vazio pode ser preparado; recargas são idempotentes e falhas não ativam dados parcialmente.

**3. Verificação ponta a ponta**

Construir cadastro de lotes e composições, cenário, validação, cálculos e regras. Integrar
formulário, parecer explicável e gravação transacional do histórico.

Conclusão:
Dietas A/B são calculadas pelo motor; violações, pendências e bloqueios aparecem com fundamento.

**4. Formulação de menor custo**

Integrar os cinco subproblemas lineares, restrições nutricionais, disponibilidade, rótulos e
exclusões. Revalidar candidatas e aplicar desempate determinístico.

Conclusão:
Soluções conhecidas são reproduzidas; ótimo, viabilidade, inviabilidade e interrupção são
distinguidos.

**5. Fornecimento e comparação**

Calcular oferta por animal, lote e horizonte; considerar perdas e sobras; arredondar e revalidar.
Permitir duplicação de cenários e comparação econômica.

Conclusão:
Mudanças de preço/MS têm efeito explicado; arredondamentos incompatíveis bloqueiam a recomendação
prática.

**6. Operação e entrega**

Completar reprodução histórica, backup/restauração, impressão e demonstração. Executar validação
visual, testes nos três navegadores e ensaios de desempenho.

Conclusão:
Instalação limpa e uso sem internet funcionam; todos os P0 e testes aplicáveis passam.

Testes, documentação e commits acompanharão cada etapa.

## Arquitetura e contratos centrais

- **Domínio:** tipos para unidades, dinheiro, procedência, dados desconhecidos e estados de
  resultado; cálculos independentes da interface e do banco.
- **Aplicação:** serviços separados para verificar, formular, comparar e reproduzir execuções,
  compartilhando as mesmas regras de validação.
- **Persistência:** revisões de cadastros e versões de conhecimento preservadas; cada execução
  congela entradas, parâmetros, resultados e traço de inferência.
- **Motor:** regras Rust tipadas, encadeamento para frente e explicações com fonte e versão.
  Resultados separados por dimensão, incluindo avaliações ausentes.
- **Web:** componentes Maud, htmx v4/hx-live locais e CSS plano. Validação obrigatória no
  servidor, proteção das operações de escrita e mensagens acessíveis por campo.

## Validação e critérios de entrega

- Transformar **CT-01 a CT-28** em uma matriz de rastreabilidade e testes automatizados.
- Cobrir cálculos, interpolação, bordas, desconhecidos, normas e contaminantes com testes unitários.
- Validar otimização com instâncias de ótimo conhecido, restrições, desempates e interrupções.
- Testar cargas, transações, imutabilidade histórica e restauração com bancos temporários.
- Exercitar os fluxos exigidos em Chromium, WebKit e Firefox, incluindo foco, erros e impressão.
- Manter CI com build, testes, clippy, formatação, limite de 100 caracteres e avisos de arquivos
  extensos. Verificar as metas de desempenho e o cancelamento da formulação em até 10 segundos.

## Premissas e condução

- Manter o perfil `E2020_NRC1989_RUMEN_MVP_v1` e o escopo local para um operador.
- Concluir todos os P0; manter evoluções P1 fora desta implementação.
- Priorizar recursos existentes no stack e dependências Rust nativas, sem blocos `unsafe`.
- Conferir documentação oficial atual e fixar versões ao introduzir bibliotecas.
- Discutir decisões relevantes antes de implementá-las, especialmente alterações de escopo,
  interpretação das fontes e dependências com impacto arquitetural.
- Fazer commits pequenos em português, com testes e documentação correspondentes a cada entrega.

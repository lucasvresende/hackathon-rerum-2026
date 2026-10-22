# Mapa da documentação

Comece pelo [README principal](../README.md) para instalar, usar, desenvolver, testar e gerar o
pacote de produção. Os documentos abaixo aprofundam os assuntos sem substituir o guia de uso.

| Objetivo | Documento |
| --- | --- |
| Operar e distribuir | [README principal](../README.md) |
| Conhecer os fluxos e ver a interface | [Interface ilustrada](INTERFACE.md) |
| Conferir o contrato funcional | [Requisitos](REQUISITOS.md) |
| Entender regras, estados e otimização | [Motor especialista](motor-especialista.md) |
| Consultar fórmulas e fontes | [Anexo nutricional](tema_2_dietas_literatura_e_motor.md) |
| Relacionar código, requisitos e testes | [Entrega e rastreabilidade](ENTREGA.md) |
| Examinar procedência dos cadastros | [Fontes preservadas](../fontes/README.md) |

## Ordem e caráter dos documentos

`REQUISITOS.md` é a especificação funcional vigente. O anexo nutricional contém a base de pesquisa
e as fórmulas que a sustentam. `motor-especialista.md` descreve a implementação do motor em
linguagem de fluxo; o código e os testes prevalecem na verificação do comportamento efetivo.

O [relatório inicial dos três temas](../hackathon/RELATORIO_TEMAS_E_ARQUITETURA.md) e a
[sequência de implementação](../hackathon/SEQUENCIA_IMPLEMENTACAO.md) registram a concepção e o
processo de desenvolvimento. O arquivo
[`hackathon/TEMA_2_DIETAS_LITERATURA_E_MOTOR.md`](../hackathon/TEMA_2_DIETAS_LITERATURA_E_MOTOR.md)
preserva uma versão anterior da pesquisa. Para consulta atual, use o anexo em `docs/`, que
incorpora revisões posteriores. Os documentos históricos podem conter propostas substituídas.

Os [diagnósticos de formulação](../diagnosticos/DIAGNOSTICO_FORMULACAO.md) registram observações
de rodadas específicas, não o estado garantido da versão corrente. Os testes automatizados em
`tests/` e a execução do CI são as evidências atuais de conformidade.

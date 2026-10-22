# Roteiro de demonstração — FeedYourCow

Guia de entradas para preencher durante a apresentação e de comportamentos que cada
etapa demonstra. Os exemplos são sintéticos: não representam animais, análises,
fornecedores, preços ou recomendações reais.

## Preparação antes de apresentar

Na raiz do repositório, prepare e inicie o aplicativo:

```sh
cargo run -- preparar
cargo run -- servir
```

Abra <http://127.0.0.1:3000>. O comando `preparar` instala o catálogo inicial sem
sobrescrever cadastros existentes. Se quiser uma base nova e isolada para ensaiar,
configure `FEEDYOURCOW_BANCO` com um caminho absoluto antes de executar os comandos.
As datas dos cenários são preenchidas pelo aplicativo no dia da execução.

Para preencher também as demonstrações A/B e o histórico sintético antes da sessão,
use `cargo run -- demonstrar`. Em banco com histórico existente, esse comando não
acrescenta as execuções históricas novamente.

## Sequência sugerida

### 1. Apresentar os cadastros de exemplo

Na lista de lotes, abra **Girolando 5/8 Holandês × 3/8 Gir — exemplo**:

| Campo | Valor |
| --- | --- |
| Animais | 1 |
| Raça | Girolando 5/8 Holandês × 3/8 Gir |
| Idade | 54 meses |
| Peso médio | 550 kg |
| Produção | 22 kg/animal/dia |
| Gordura do leite | 3,8% |
| Dias em lactação | 120 |
| Adulta / gestante / em crescimento | Sim / Não / Não |
| Peso estável / confinada | Sim / Sim |
| Correção por ambiente / atividade | Não / Não |
| Água disponível | Sim |

**Demonstra:** cadastro com condições fisiológicas e de manejo explícitas, origem
sintética e valores dentro das faixas de entrada do perfil.

Na lista de alimentos, abra a **Silagem de milho** do catálogo e mostre que a ficha
separa matéria seca de nutrientes na matéria seca. Exemplos da composição cadastrada:

| Campo | Valor de exemplo |
| --- | ---: |
| Matéria seca | 32,5% da matéria natural |
| NDT | 65,0% da MS |
| Proteína bruta | 6,9% da MS |
| FDN | 44,3% da MS |
| Amido | 29,1% da MS |
| Extrato etéreo | 2,8% da MS |

**Demonstra:** composição, fonte, natureza do dado e procedência do modelo de
preenchimento. Esses números são sintéticos/aproximados e não substituem análise
laboratorial.

### 2. Fazer um cadastro manual sem inventar um caso real

Para mostrar a criação de um lote, use a ação de copiar no lote de exemplo e salve
uma cópia com estes valores. A cópia preserva o registro original.

| Campo | Valor para digitar |
| --- | --- |
| Nome | Lote apresentação — sintético |
| Número de animais | 8 |
| Raça | Girolando — hipótese de demonstração |
| Idade | 48 meses |
| Peso médio | 500 kg |
| Produção | 18 kg/animal/dia |
| Gordura do leite | 4,0% |
| Dias em lactação | 150 |
| Adulta / gestante / em crescimento | Sim / Não / Não |
| Peso estável / confinada | Sim / Sim |
| Correção por ambiente / atividade | Não / Não |
| Água disponível | Sim |
| Observações | Exemplo sintético para apresentação; não é rebanho real. |

**Demonstra:** criação por cópia, validação de campos, salvamento de um cadastro
independente e preservação da origem.

Para mostrar revisão de alimento, use **Criar cópia** em Silagem de milho, nomeie-a
**Silagem da apresentação — sintética** e mantenha os valores acima. Troque apenas a
data para a data atual e registre em método/evidência que são valores sintéticos para
demonstração. Salve e abra o cadastro novamente para mostrar a revisão e a procedência.
Para demonstrar revisão, salve uma alteração posterior nesse cadastro copiado; assim,
o exemplo de catálogo original continua preservado. Não apresente esses dados como laudo
nem como composição de um produto comercial.

### 3. Verificar uma dieta informada

Na visão geral, escolha **Abrir cenário de exemplo**. O cenário pronto usa o lote
`ex-l03` (Girolando, 550 kg, 22 kg de leite/dia) e esta dieta de consumo em MS por
animal/dia:

| Ingrediente | Quantidade | Preço ilustrativo |
| --- | ---: | ---: |
| Silagem de milho (`ex-a01`) | 11,0 kg MS | R$ 0,25/kg MN |
| Milho moído (`ex-a06`) | 0,8 kg MS | R$ 1,30/kg MN |
| Farelo de soja (`ex-a07`) | 3,0 kg MS | R$ 2,20/kg MN |
| Mistura mineral (`ex-a12`) | 0,3 kg MS | R$ 4,00/kg MN |
| Polpa cítrica (`ex-a10`) | 2,0 kg MS | R$ 1,10/kg MN |

O cenário também traz horizonte de 7 dias, zero sobras como hipótese explícita e
disponibilidade sem limite como premissa de demonstração. Confira a base das
quantidades antes de calcular.

Clique em **Calcular e salvar cenário**.

**Demonstra:** conversão entre matéria natural e matéria seca, cálculo dos nutrientes,
exigências do lote, custo, avisos e parecer por dimensão. Mostre também as entradas,
fontes, regras e versões preservadas na execução. Um aviso ou estado inconclusivo pode
ser correto quando o dado não existe; ausência de laudo não comprova ausência de
contaminante. Não trate o resultado como recomendação integral.

### 4. Demonstrar a formulação de menor custo

Em **Nova dieta**, selecione o lote da demonstração, escolha **Formular** e selecione
Silagem de milho, Milho moído, Farelo de soja e Mistura mineral. Use horizonte de 7 dias,
zero sobras explicitamente hipotéticas e limite de busca de 10 segundos. Para cada item,
use a composição sintética carregada e preencha:

| Ingrediente | Preço (R$/kg MN) | Estoque | Inclusão mínima | Inclusão máxima |
| --- | ---: | --- | ---: | ---: |
| Silagem de milho | 0,25 | Sem limite (hipótese) | 0% | 100% |
| Milho moído | 1,30 | Sem limite (hipótese) | 0% | 100% |
| Farelo de soja | 2,20 | Sem limite (hipótese) | 0% | 100% |
| Mistura mineral | 4,00 | Sem limite (hipótese) | 0% | 30% |

Use como origem de cada preço: **Cotação sintética para demonstração**. Informe consumo
alvo somente se o formulário exigir; prefira o consumo de referência do perfil para
mostrar a origem calculada. Inicie o cálculo e abra o bloco de busca, restrições,
candidata, custo e fornecimento.

**Demonstra:** busca de custo mínimo no modelo contínuo, restrições nutricionais e de
inclusão, estado da busca e revalidação. O estado deve ser lido na tela: ótimo no modelo
contínuo não significa que a quantidade arredondada seja prática ou que a dieta esteja
aprovada em dimensões não avaliadas.

### 5. Mostrar avaliação prática e resolução de pesagem

Na candidata formulada, defina resolução positiva para os ingredientes usados. Para
uma demonstração simples, use **0,5 kg MN** como resolução em cada item usado e confira
a oferta arredondada e o novo parecer. Se o formulário mantiver a solução sem resolução,
use a ação **Ajustar e verificar quantidades práticas** quando disponível.

**Demonstra:** que a aproximação contínua e a quantidade de fornecimento são avaliadas
separadamente; arredondamento incompatível mantém a candidata contínua no histórico e
recusa a recomendação prática.

### 6. Comparar execuções e consultar o histórico

Faça uma segunda verificação do mesmo lote e da mesma revisão usando a opção
**Demonstração B** ou duplique a primeira execução e altere a quantidade de milho moído
de 0,8 para 1,3 kg MS/animal/dia. Salve ambas e abra **Histórico e análise**.

Compare somente execuções do mesmo lote e revisão. Mostre as premissas, diferenças de
custo e dimensões lado a lado. Abra uma execução antiga para demonstrar as entradas e
fontes preservadas; depois use **Duplicar e simular** para mostrar que uma nova execução
é criada sem editar o parecer original.

**Demonstra:** histórico imutável, comparação condicionada ao mesmo lote/revisão e
reutilização de uma dieta em nova simulação. A diferença de custo depende dos preços e
premissas cadastrados e não prevê mudança de produção de leite.

### 7. Imprimir o parecer

Na página de uma execução, use **Imprimir parecer**. Se a dieta tiver quantidades
práticas validadas, mostre também **Imprimir ficha de fornecimento**; se não tiver,
mostre a recusa operacional.

**Demonstra:** apresentação auditável das entradas, estados, limites, fontes e trilha de
cálculo, além da distinção entre parecer completo e ficha curta de fornecimento.

## Atalhos da instalação

| Ação | Entrada / local | O que demonstra |
| --- | --- | --- |
| Abrir cenário de exemplo | Visão geral | Verificação rápida de uma dieta preenchida. |
| Demonstração A e B | Nova dieta | Duas dietas sintéticas com quantidades diferentes. |
| Criar cópia | Cadastro de lote/alimento | Cadastro independente sem alterar a origem. |
| Salvar revisão | Edição de cadastro | Versão nova mantendo execuções anteriores. |
| Duplicar e simular | Página da execução | Novo cálculo com conhecimento ativo. |
| Histórico e análise | Navegação principal | Consulta, impressão e comparação compatível. |

## Limites para narrar corretamente

- Todos os cadastros e preços desta apresentação são dados sintéticos.
- A formulação busca menor custo dentro do perfil, ingredientes e limites informados.
- Fibra fisicamente efetiva, vitaminas, aminoácidos, água e outras dimensões sem
  perfil completo podem permanecer não avaliadas.
- Ausência de dado fica desconhecida; não deve ser narrada como aprovação.
- Uma execução histórica preserva seu resultado; editar ou duplicar inicia outro caso.
- Ver [requisitos do FeedYourCow](../../docs/REQUISITOS.md), [uso](../../docs/USO.md) e
  [motor](../../docs/MOTOR.md) para o contrato e as limitações completas.

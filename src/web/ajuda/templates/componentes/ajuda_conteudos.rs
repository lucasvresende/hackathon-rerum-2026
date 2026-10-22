//! Conteúdo textual da ajuda organizada por assunto.
pub const ALIMENTOS: &[&str] = &[
    "O sistema inclui quinze alimentos e seis lotes de exemplo. Você pode selecioná-los, \
     editar ou usar Criar cópia. Em Restaurar exemplos, escolha criar outra cópia ou restaurar o \
     registro inicial com nova revisão. Reiniciar não recria exclusões nem desfaz suas alterações. \
     Condições e valores sintéticos são hipóteses, não medições ou comprovações de produtos reais.",
    "Pense no alimento como o produto e na composição como seus dados nutricionais. \
            Duas análises de silagem podem ter teores diferentes de água e nutrientes. \
            Revise a versão adequada ao seu produto; criar uma revisão não significa que \
            uma nova amostra tenha sido analisada. O histórico conserva as versões usadas.",
    "Matéria natural (MN) inclui a água; matéria seca (MS) é a parte sem água. \
            Por exemplo, 20 kg de silagem com 35% de MS contêm 7 kg de MS. Se a proteína \
            bruta for 8% da MS, esses 7 kg contêm 0,56 kg de proteína bruta. Não confunda \
            percentual na matéria seca com percentual no alimento úmido.",
    "A natureza dos valores explica sua origem: análise é um resultado de amostra, \
            média tabelada reúne uma referência publicada e sintético é dado de demonstração. \
            Garantias mínima e máxima indicam limites, não teores exatos. Use os indicadores \
            de interrogação para entender as siglas; dado desconhecido não deve virar zero.",
    "No catálogo, digite parte do nome e use Pesquisar. Confira composição, natureza \
            do dado e a seção Procedência e condições de cada alimento.",
    "Use Cadastrar composição ou Criar revisão ou nova análise. Registre identidade, \
            classe, origem, processamento, fonte, método, data e evidências de uso. \
            Identifique corretamente dados sintéticos, médias e análises.",
    "A matéria seca (MS) é expressa em % da matéria natural (MN); NDT, PB, Ca, P, \
            FDN, amido e EE são expressos em % da MS. Não preencha um nutriente desconhecido \
            como zero. Registre origem animal, NNP, autorização e limite de rótulo do suplemento.",
    "Use Preencher com exemplo, veja a prévia, confirme e edite os valores sintéticos. \
            Edite antes de salvar; a referência original e as alterações são preservadas. \
            Salve a revisão para novos cenários. Análises anteriores continuam disponíveis; \
            preço e estoque locais são informados no cenário, sem alterar a composição científica.",
];

pub const CENARIOS: &[&str] = &[
    "Consumo é o que se estima ou se observa que o animal ingere; oferta é o que \
            vai para o cocho. Por exemplo, para consumir 9 kg de MS com 10% de sobras, \
            é preciso ofertar 10 kg de MS. Essa conta ainda não inclui perdas anteriores \
            ao fornecimento, como as que ocorrem no armazenamento.",
    "Sobras ficam no cocho; perdas ocorrem antes de o alimento chegar a ele. \
            Informe as duas separadamente para calcular oferta, custo e estoque bruto. \
            O horizonte é o período que o estoque precisa cobrir para todo o lote. \
            Campo de estoque vazio significa desconhecido, não disponibilidade infinita.",
    "Na simulação, escolha o que deve permanecer fixo ao mudar a MS do alimento. \
            Se mantiver o consumo de MS, um alimento mais úmido exigirá mais kg de MN. \
            Se mantiver os kg de MN, a quantidade de MS mudará. Essa escolha ajuda a \
            comparar o efeito da umidade sem misturar bases diferentes.",
    "Em Nova dieta, informe nome, lote, tipo de cálculo, horizonte em dias e datas \
            de avaliação e aplicação das regras. Escolha a base das quantidades por ingrediente: \
            consumo em MS, consumo em MN ou oferta em MN, sempre por animal/dia.",
    "Informe consumo-alvo em kg MS/animal/dia e sua origem ou deixe vazio para usar a \
            estimativa da Tabela B, extrapolada linearmente fora de 400–600 kg. \
            Para consumo observado, informe oferta e sobras em MS do lote e período \
            de medição; deixe o alvo vazio. Seleção de partículas limita essa estimativa.",
    "Registre sobras planejadas e origem das sobras e perdas. Zero também é uma \
            hipótese que precisa ser declarada. O preço do leite é opcional para calcular \
            margem sobre alimentação. A referência UE adicional é uma escolha explícita, \
            separada da legislação brasileira.",
    "Selecione uma composição por alimento e abra a carta do ingrediente. Confira MS, \
            preço, unidade comercial, massa de embalagem, estoque bruto em MN, perda, inclusão \
            mínima/máxima e resolução de fornecimento. Sem limite de estoque \
            é uma hipótese explícita; estoque desconhecido não significa infinito.",
    "Use Calcular e salvar cenário. Se houver erros, corrija os campos indicados; \
            os valores válidos permanecem no formulário. O resultado salva as entradas e versões.",
];

pub const RESULTADOS: &[&str] = &[
    "Comece por Resultados por dimensão. Núcleo químico verifica NDT, proteína \
            bruta, cálcio e fósforo; as outras dimensões tratam de outros critérios. \
            ATENDE vale para o grupo avaliado. INCONCLUSIVO indica informação insuficiente; \
            NAO_AVALIADO não equivale a aprovação. Um bloqueio em ingredientes continua \
            relevante mesmo quando os nutrientes atendem ao perfil.",
    "No balanço, compare a quantidade calculada com o requisito. Se o requisito \
            for 1 kg e a dieta fornecer 0,8 kg, o balanço é −0,2 kg e a cobertura é 80%. \
            Um valor acima do mínimo aparece como excesso, mas isso, sozinho, não é \
            um diagnóstico de intoxicação. Consulte também as regras e limitações.",
    "Ótimo no modelo significa menor custo para os dados e restrições utilizados \
            na solução contínua. Ao arredondar para a pesagem real, uma restrição pode \
            deixar de ser atendida. Resolução zero indica aproximação contínua; informe resolução \
            positiva nos ingredientes usados e confira a revalidação prática. \
            Resultado inconclusivo ou interrompido não prova que não exista uma solução.",
    "Leia as dimensões separadamente: integridade, nutrientes, barreiras ruminais, \
            ingredientes e avaliações fora do núcleo. ATENDE é atendimento ao perfil avaliado; \
            não é aprovação nutricional integral. DESCONHECIDO e NAO_AVALIADO exigem atenção.",
    "Confira o balanço nutricional, custos e quantidades em MN/MS por animal, lote e \
            horizonte. Só use o fornecimento prático quando sua revalidação estiver indicada \
            como válida. INVALIDO, FORA_DO_PERFIL e BLOQUEADO impedem recomendação automática.",
    "Abra Cálculos e regras aplicadas para consultar entradas, hipóteses, fontes e \
            caminho de inferência. Use Imprimir parecer para gerar uma versão de impressão \
            ou salvar como PDF pelo navegador.",
    "Use Duplicar e simular nas verificações ou Refazer otimização nas formulações. \
            Ajustar e verificar copia a solução contínua em MS ou a oferta prática em MN. \
            A cópia prática pode exigir correções e não constitui aprovação. Altere \
            preços, MS ou premissas e escolha a base da simulação de MS (MS ou MN fixas). \
            Calcule com outro nome e compare no histórico; o original continua preservado.",
];

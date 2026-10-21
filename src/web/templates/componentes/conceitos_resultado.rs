//! Textos de ajuda para estados e valores do parecer.
use super::dica::Dica;

pub const ESTADOS: Dica = Dica {
    id: "dica-estados",
    conceito: "Estados do parecer",
    texto: "ATENDE e NAO_ATENDE resumem as verificações de uma dimensão. INCONCLUSIVO indica \
        dados ou conclusões insuficientes; NAO_AVALIADO indica ausência de avaliação. \
        Um atendimento em nutrientes não elimina bloqueios ou pendências em outras dimensões.",
};
pub const NUCLEO: Dica = Dica {
    id: "dica-nucleo",
    conceito: "Núcleo químico",
    texto: "Compara o aporte de NDT, proteína bruta, cálcio e fósforo com os requisitos \
        do perfil. Atender a esses quatro critérios não comprova adequação nutricional \
        completa. Leia também as demais dimensões do parecer.",
};
pub const DADOS: Dica = Dica {
    id: "dica-dados",
    conceito: "Dados",
    texto: "Indica se os campos necessários para calcular e interpretar o cenário estão \
        presentes e consistentes, como composição, unidades e quantidades. Uma pendência \
        aqui pode tornar outras conclusões inconclusivas.",
};
pub const ELEGIBILIDADE: Dica = Dica {
    id: "dica-elegibilidade",
    conceito: "Elegibilidade",
    texto: "Confere se o lote está dentro do perfil implementado: vacas adultas em lactação, \
        confinadas e nas faixas previstas de peso, produção, gordura e dias em lactação. \
        Estar elegível não significa que a dieta atende às necessidades.",
};
pub const BARREIRAS: Dica = Dica {
    id: "dica-barreiras",
    conceito: "Barreiras ruminais",
    texto: "Compara FDN total, FDN proveniente de forragens, amido e extrato etéreo com \
        os limites do perfil. NAO ATENDE aponta ao menos um limite excedido; isso sinaliza \
        uma restrição do modelo, não diagnostica um problema clínico.",
};
pub const INGREDIENTES: Dica = Dica {
    id: "dica-ingredientes",
    conceito: "Ingredientes",
    texto: "Verifica se os alimentos da dieta podem ser usados conforme as informações \
        cadastradas, incluindo identidade, autorização e restrições aplicáveis. Uma hipótese \
        de demonstração sobre produto fictício não comprova autorização de um produto real.",
};
pub const CONTAMINANTES: Dica = Dica {
    id: "dica-contaminantes",
    conceito: "Contaminantes",
    texto: "Avalia os contaminantes cadastrados e seus laudos contra os limites aplicáveis. \
        INCONCLUSIVO costuma indicar laudo ou dado insuficiente; ausência de laudo não \
        significa ausência de contaminantes.",
};
pub const INCLUSAO_ESTOQUE: Dica = Dica {
    id: "dica-inclusao-estoque",
    conceito: "Inclusão e estoque",
    texto: "Confere limites mínimos e máximos de inclusão, disponibilidade no estoque e \
        quantidade necessária para o horizonte informado, considerando perdas e sobras. \
        ATENDE vale para as quantidades e premissas deste cenário.",
};
pub const FIBRA_FISICA: Dica = Dica {
    id: "dica-fibra-fisica",
    conceito: "Fibra física e micronutrientes",
    texto: "Reúne avaliações como fibra fisicamente efetiva e micronutrientes, que não são \
        calculados pelo perfil químico atual. NAO AVALIADO significa que faltam método ou \
        dados para emitir parecer; não equivale a atendimento.",
};
pub const BALANCO: Dica = Dica {
    id: "dica-balanco",
    conceito: "Balanço nutricional",
    texto: "Diferença entre a quantidade de nutriente calculada na dieta e o requisito. \
        Negativo indica déficit; positivo indica quantidade acima do mínimo. Cobertura \
        de 100% corresponde ao requisito. Excesso sobre o mínimo não comprova intoxicação.",
};
pub const FORNECIMENTO: Dica = Dica {
    id: "dica-fornecimento",
    conceito: "Fornecimento prático",
    texto: "Quantidades para fornecer após o arredondamento escolhido por ingrediente. \
        Arredondar pode alterar o atendimento das restrições; use a indicação de revalidação. \
        Consumo em MS, oferta em MN e necessidade bruta de estoque são grandezas diferentes.",
};
pub const OTIMO: Dica = Dica {
    id: "dica-otimo",
    conceito: "Ótimo no modelo",
    texto: "Menor custo comprovado dentro das entradas e restrições do modelo contínuo, \
        após resolver os subproblemas necessários. Não garante o menor custo após \
        arredondar. Busca inconclusiva ou interrompida não comprova inviabilidade.",
};

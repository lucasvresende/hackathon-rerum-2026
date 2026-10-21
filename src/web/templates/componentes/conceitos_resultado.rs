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

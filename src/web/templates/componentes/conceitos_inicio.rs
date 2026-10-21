use super::dica::Dica;

pub const LOTES: Dica = Dica {
    id: "dica-lotes",
    conceito: "Lotes cadastrados",
    texto: "Grupos de animais com características semelhantes, como peso e fase de lactação. \
        Cada lote tem seus próprios dados para calcular a dieta. O número conta os lotes, \
        sem somar novamente suas revisões.",
};
pub const ANIMAIS: Dica = Dica {
    id: "dica-animais",
    conceito: "Animais cadastrados",
    texto: "Soma do número de animais informado na revisão atual de cada lote. Por exemplo, \
        dois lotes com 20 e 30 vacas somam 50 animais. Não é um cadastro individual de cada vaca.",
};
pub const COMPOSICOES: Dica = Dica {
    id: "dica-composicoes",
    conceito: "Composições disponíveis",
    texto: "Registros dos teores de matéria seca e nutrientes de um alimento. Um mesmo \
        alimento pode ter várias análises ou revisões; cada composição disponível conta aqui. \
        Preço e estoque são informados separadamente no cenário.",
};
pub const CENARIOS: Dica = Dica {
    id: "dica-cenarios",
    conceito: "Cenários",
    texto: "Conjuntos de dados usados para avaliar uma dieta: lote, alimentos, quantidades, \
        preços, disponibilidade e hipóteses. Ao calcular e salvar, esses dados e o resultado \
        ficam no histórico. Duplicar um cenário permite testar mudanças sem alterar o original.",
};
pub const PERFIL: Dica = Dica {
    id: "dica-perfil",
    conceito: "Perfil ativo",
    texto: "Conjunto de referências e regras usado para avaliar as dietas. Define quais \
        animais podem ser avaliados e quais nutrientes e limites são considerados. Atender \
        ao perfil não garante adequação nutricional completa; consulte também as limitações.",
};

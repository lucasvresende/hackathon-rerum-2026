use super::dica::Dica;

pub const HORIZONTE: Dica = Dica {
    id: "dica-horizonte",
    conceito: "Horizonte",
    texto: "Número de dias cobertos pelo cenário. Por exemplo, 30 dias permitem calcular \
        quanto alimento o lote precisará nesse período e conferir se o estoque é suficiente. \
        O cálculo não retira quantidades do estoque cadastrado.",
};
pub const CONSUMO: Dica = Dica {
    id: "dica-consumo",
    conceito: "Consumo-alvo",
    texto: "Quantidade de matéria seca que se pretende que cada animal consuma por dia. \
        Na formulação, esse total fica fixo. Quando permitido pelo perfil, o campo vazio \
        usa uma estimativa; acima de 600 kg de peso vivo, é necessário informar o alvo.",
};
pub const BASE: Dica = Dica {
    id: "dica-base",
    conceito: "Consumo e oferta",
    texto: "Consumo é a quantidade ingerida; oferta é a quantidade colocada à disposição, \
        antes das sobras. Escolha também se a quantidade está em matéria seca (MS) ou \
        matéria natural (MN). Ofertar 20 kg não comprova que o animal consumiu 20 kg.",
};
pub const SOBRAS: Dica = Dica {
    id: "dica-sobras",
    conceito: "Sobras planejadas",
    texto: "Parte da matéria seca ofertada que se espera que fique no cocho. Com 10% \
        de sobras, é preciso ofertar mais para alcançar o consumo pretendido. Informe \
        a origem dessa estimativa; zero também deve ser uma hipótese explícita.",
};
pub const PERDA: Dica = Dica {
    id: "dica-perda",
    conceito: "Perda anterior ao fornecimento",
    texto: "Parte do alimento perdida antes de chegar ao cocho, por exemplo no armazenamento \
        ou transporte. Ela aumenta a necessidade de estoque bruto e o custo. É diferente \
        da sobra, que permanece no cocho depois do fornecimento.",
};
pub const ESTOQUE: Dica = Dica {
    id: "dica-estoque",
    conceito: "Estoque bruto",
    texto: "Quantidade disponível do alimento em matéria natural, antes das perdas. \
        Deve cobrir todos os animais durante o horizonte do cenário. Campo vazio significa \
        desconhecido; disponibilidade sem limite precisa ser declarada explicitamente.",
};
pub const INCLUSAO: Dica = Dica {
    id: "dica-inclusao",
    conceito: "Limites de inclusão",
    texto: "Menor e maior quantidade permitida de um ingrediente em kg de matéria seca \
        por animal e por dia. Um mínimo positivo obriga sua presença. Esses limites \
        não substituem as demais restrições do perfil ou do rótulo.",
};
pub const RESOLUCAO: Dica = Dica {
    id: "dica-resolucao",
    conceito: "Resolução de oferta",
    texto: "Passo usado para arredondar a quantidade prática em matéria natural. Por exemplo, \
        0,5 kg permite incrementos de meio quilo. A dieta arredondada precisa passar \
        novamente pelas verificações; zero mantém a quantidade contínua.",
};
pub const SIMULACAO: Dica = Dica {
    id: "dica-simulacao",
    conceito: "Base da simulação de MS",
    texto: "Define o que fica fixo ao mudar a matéria seca do alimento em uma simulação. \
        Com MS fixa, muda a quantidade natural necessária. Com MN fixa, fica a mesma \
        quantidade natural, mas muda a matéria seca consumida.",
};

pub fn campo(nome: &str) -> Option<&'static Dica> {
    match nome.split('-').next().unwrap_or(nome) {
        "horizonte" => Some(&HORIZONTE),
        "consumo_alvo" => Some(&CONSUMO),
        "base" => Some(&BASE),
        "sobras" => Some(&SOBRAS),
        "perda" => Some(&PERDA),
        "estoque" => Some(&ESTOQUE),
        "min" | "max" => Some(&INCLUSAO),
        "resolucao" => Some(&RESOLUCAO),
        "base_comparacao" => Some(&SIMULACAO),
        _ => None,
    }
}

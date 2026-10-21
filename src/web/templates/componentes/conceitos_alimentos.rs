use super::dica::Dica;

pub const COMPOSICAO: Dica = Dica {
    id: "dica-composicao",
    conceito: "Composição do alimento",
    texto: "Registro dos teores de matéria seca e nutrientes de um alimento. O mesmo alimento \
        pode ter várias composições, vindas de análises ou revisões diferentes. Escolha \
        a que corresponde ao produto utilizado; preço e estoque ficam no cenário.",
};
pub const REVISAO: Dica = Dica {
    id: "dica-revisao",
    conceito: "Análises e revisões",
    texto: "Uma análise registra os dados de uma amostra; uma revisão cria outra versão \
        do cadastro. A versão anterior continua disponível e os cálculos antigos preservam \
        a composição que utilizaram. Uma revisão não comprova que houve nova análise.",
};
pub const NATUREZA: Dica = Dica {
    id: "dica-natureza",
    conceito: "Natureza dos valores",
    texto: "Indica como os valores foram obtidos: análise de uma amostra, média publicada, \
        dado sintético ou garantia mínima/máxima. Garantia é um limite, não uma medição exata. \
        Dados sintéticos servem para demonstração.",
};
pub const PROCEDENCIA: Dica = Dica {
    id: "dica-procedencia",
    conceito: "Procedência",
    texto: "Origem dos dados e do alimento: fonte, método, data, processamento e condições \
        de uso. Essas informações ajudam a decidir se a composição corresponde ao seu \
        produto. Ter uma fonte não comprova, por si só, autorização de uso.",
};
pub const MS: Dica = Dica {
    id: "dica-ms",
    conceito: "Matéria seca (MS) e natural (MN)",
    texto: "Matéria natural é o alimento com sua água. Matéria seca é o que resta sem a água. \
        Por exemplo, 20 kg de alimento com 35% de MS contêm 7 kg de matéria seca. Os teores \
        de nutrientes em % MS se referem a essa parte seca.",
};
pub const NDT: Dica = Dica {
    id: "dica-ndt",
    conceito: "NDT",
    texto: "Nutrientes digestíveis totais: referência usada pelo perfil para avaliar \
        o aporte energético. O teor é informado sobre a matéria seca do alimento. \
        Um valor ausente continua desconhecido, em vez de ser considerado zero.",
};
pub const PB: Dica = Dica {
    id: "dica-pb",
    conceito: "Proteína bruta (PB)",
    texto: "Medida de proteína bruta usada na avaliação química do perfil, expressa em \
        percentual da matéria seca. Ela não descreve sozinha o aproveitamento da proteína \
        nem comprova adequação de todos os aminoácidos.",
};
pub const CA: Dica = Dica {
    id: "dica-ca",
    conceito: "Cálcio (Ca)",
    texto: "Teor de cálcio expresso em percentual da matéria seca. O cálculo soma \
        a quantidade fornecida pelos ingredientes e compara com o requisito do perfil.",
};
pub const P: Dica = Dica {
    id: "dica-p",
    conceito: "Fósforo (P)",
    texto: "Teor de fósforo expresso em percentual da matéria seca. O cálculo soma \
        a quantidade fornecida pelos ingredientes e compara com o requisito do perfil.",
};
pub const FDN: Dica = Dica {
    id: "dica-fdn",
    conceito: "Fibra em detergente neutro (FDN)",
    texto: "Medida de fibra usada nas restrições químicas da dieta, em percentual da matéria \
        seca. O perfil distingue FDN total e FDN vinda de forragem. Esse teor sozinho \
        não informa o tamanho das partículas nem comprova adequação da fibra física.",
};
pub const AMIDO: Dica = Dica {
    id: "dica-amido",
    conceito: "Amido",
    texto: "Teor de amido em percentual da matéria seca. O perfil limita sua participação \
        na dieta em conjunto com as regras de fibra. O limite avaliado não é um diagnóstico \
        clínico nem uma previsão da resposta do animal.",
};
pub const EE: Dica = Dica {
    id: "dica-ee",
    conceito: "Extrato etéreo (EE)",
    texto: "Medida usada para representar a fração de gordura na avaliação do perfil, \
        em percentual da matéria seca. O total da dieta é comparado com o limite \
        implementado; dado ausente permanece desconhecido.",
};

pub fn campo(nome: &str) -> Option<&'static Dica> {
    match nome.split('-').next().unwrap_or(nome) {
        "ms" => Some(&MS),
        "ndt" => Some(&NDT),
        "pb" => Some(&PB),
        "ca" => Some(&CA),
        "p" => Some(&P),
        "fdn" => Some(&FDN),
        "amido" => Some(&AMIDO),
        "ee" => Some(&EE),
        "natureza" => Some(&NATUREZA),
        "fonte_id" => Some(&PROCEDENCIA),
        _ => None,
    }
}

pub const NUTRIENTES: [&Dica; 7] = [&NDT, &PB, &CA, &P, &FDN, &AMIDO, &EE];

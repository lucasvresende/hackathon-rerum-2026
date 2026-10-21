type CampoNumerico = (&'static str, &'static str, &'static str, bool);

pub fn numericos(lote: bool) -> Vec<CampoNumerico> {
    if lote {
        vec![
            ("animais", "Número de animais", "Inteiro positivo", true),
            ("peso", "Peso médio (kg)", "Cobertura: 400 a 700 kg", true),
            (
                "producao",
                "Leite (kg/animal/dia)",
                "Produção real, não litros",
                true,
            ),
            (
                "gordura",
                "Gordura do leite (%)",
                "Pontos percentuais: por exemplo, 4",
                true,
            ),
            (
                "dias_lactacao",
                "Dias em lactação",
                "Cobertura: 60 a 250",
                true,
            ),
            (
                "idade_meses",
                "Idade (meses)",
                "Vazio significa desconhecida",
                false,
            ),
            ("ordem_parto", "Ordem de parto", "Opcional", false),
            (
                "dispersao_peso",
                "Dispersão de peso (kg)",
                "Opcional",
                false,
            ),
            (
                "condicao_corporal",
                "Condição corporal",
                "Registre a escala em observações",
                false,
            ),
        ]
    } else {
        vec![
            ("ms", "Matéria seca (% MN)", "Maior que 0 e até 100%", true),
            (
                "ndt",
                "NDT (% MS)",
                "Não negativo; definição do perfil",
                false,
            ),
            (
                "pb",
                "Proteína bruta (% MS)",
                "Vazio significa desconhecido",
                false,
            ),
            ("ca", "Cálcio (% MS)", "Vazio significa desconhecido", false),
            ("p", "Fósforo (% MS)", "Vazio significa desconhecido", false),
            ("fdn", "FDN (% MS)", "Vazio significa desconhecido", false),
            (
                "amido",
                "Amido (% MS)",
                "Vazio significa desconhecido",
                false,
            ),
            (
                "ee",
                "Extrato etéreo (% MS)",
                "Vazio significa desconhecido",
                false,
            ),
            (
                "rotulo_max_ms",
                "Máximo do rótulo (kg MS/animal/dia)",
                "Obrigatório para recomendar suplemento",
                false,
            ),
        ]
    }
}

pub fn estados(lote: bool) -> Vec<(&'static str, &'static str)> {
    if lote {
        vec![
            ("adulta", "Classe adulta"),
            ("gestante", "Gestante"),
            ("crescimento", "Em crescimento"),
            ("peso_estavel", "Peso estável"),
            ("confinada", "Confinada com dieta total quantificável"),
            ("correcao_ambiente", "Precisa de correção por ambiente"),
            ("correcao_atividade", "Precisa de correção por atividade"),
            ("agua", "Acesso à água"),
        ]
    } else {
        vec![
            ("origem_animal", "Origem animal"),
            ("excecao_animal", "Exceção legal documentada"),
            ("nnp_adicionado", "NNP intencionalmente adicionado"),
            ("medicamento", "Contém medicamento"),
            ("autorizado", "Autorização / hipótese do exemplo"),
        ]
    }
}

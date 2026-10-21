//! Normalização de laudos e avaliação dos limites aplicáveis a contaminantes.
use super::{Cenario, Comparacao, Conhecimento, Estado, Laudo, Parecer};

/// Mantém o analito e a matriz; converte somente unidades e bases de umidade compatíveis.
pub fn base_ms(laudo: &Laudo) -> Option<f64> {
    if !laudo.valor.is_finite()
        || !laudo.ms.is_finite()
        || laudo.valor < 0.0
        || laudo.ms <= 0.0
        || laudo.ms > 1.0
    {
        return None;
    }
    let valor = match laudo.qualificador.as_str() {
        "EXATO" => laudo.valor,
        "<LOQ" => laudo.loq.filter(|v| v.is_finite() && *v >= 0.0)?,
        _ => return None,
    };
    match laudo.unidade.as_str() {
        "µg/kg MN" => Some(valor / laudo.ms),
        "µg/kg MS" => Some(valor),
        "µg/kg 12%" => Some(valor / 0.88),
        "mg/kg MN" => Some(1000.0 * valor / laudo.ms),
        "mg/kg MS" => Some(1000.0 * valor),
        _ => None,
    }
}

/// Compara laudos compatíveis aos limites vigentes e registra pendências separadamente.
pub fn avaliar(c: &Cenario, base: &Conhecimento, p: &mut Parecer) {
    let norma = base.normas.iter().find(|n| n.id == "UE-AFB1");
    if !c.referencia_ue || !norma.is_some_and(|n| n.vigente(&c.data_aplicacao)) {
        p.registrar(
            "CON-02",
            "Contaminantes",
            Estado::Desconhecido,
            "Não há limite brasileiro universal de micotoxinas carregado para esta dieta. \
             Laudo ausente não é zero; referência UE não ativada ou fora da vigência."
                .into(),
        );
        return;
    }
    if base.limites_contaminantes.is_empty() {
        p.registrar(
            "CON-02",
            "Contaminantes",
            Estado::Desconhecido,
            "Carga de limites estrangeiros ausente.".into(),
        );
        return;
    }
    for limite in &base.limites_contaminantes {
        let mut mistura = Some(0.0);
        let mut tem_loq = false;
        for item in &c.itens {
            let x = super::calculos::consumo_item(item, c.sobras);
            if x == 0.0 {
                continue;
            }
            let laudo = laudo(&item.alimento, &limite.analito);
            let valor_ms = laudo.and_then(base_ms);
            let abaixo_loq = laudo.is_some_and(|l| l.qualificador == "<LOQ");
            tem_loq |= abaixo_loq;
            mistura = mistura.zip(valor_ms).map(|(s, v)| s + x * v);
            if limite.matriz == "MATERIA_PRIMA" {
                let valor = valor_ms
                    .map(|v| v * (1.0 - limite.umidade))
                    .filter(|v| !abaixo_loq || *v <= limite.limite);
                p.comparar(Comparacao {
                    regra: "CON-01",
                    dimensao: "Contaminantes",
                    valor,
                    limite: limite.limite,
                    maximo: true,
                    sanitario: true,
                    unidade: "µg/kg base de umidade declarada",
                    texto: &format!(
                        "{}: {} na matéria-prima; referência UE N11. \
                        <LOQ usa limite superior; se insuficiente, resultado desconhecido.",
                        item.alimento.nome, limite.analito
                    ),
                });
            }
        }
        if limite.matriz != "COMPOSTO_LEITEIRO" {
            continue;
        }
        let valor = mistura
            .filter(|_| p.consumo_ms > 0.0)
            .map(|v| (1.0 - limite.umidade) * v / p.consumo_ms)
            .filter(|v| !tem_loq || *v <= limite.limite);
        p.comparar(Comparacao {
            regra: "CON-01",
            dimensao: "Contaminantes",
            valor,
            limite: limite.limite,
            maximo: true,
            sanitario: true,
            unidade: "µg/kg base de umidade declarada",
            texto: &format!(
                "{}: composto leiteiro, referência UE N11 ativada, umidade {}%. \
                Analito/matriz/laudo incompatível mantém pendência; diluição não autoriza
                ingrediente.",
                limite.analito,
                100.0 * limite.umidade
            ),
        });
    }
}

pub fn laudo<'a>(alimento: &'a super::Alimento, analito: &str) -> Option<&'a Laudo> {
    alimento.contaminantes.iter().find(|l| {
        l.analito == analito
            && l.matriz == "MATERIA_PRIMA"
            && !l.metodo.is_empty()
            && !l.documento.is_empty()
    })
}

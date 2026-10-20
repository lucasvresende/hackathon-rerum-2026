use super::{Cenario, Comparacao, Conhecimento, Estado, Laudo, Parecer};

/// Mantém o analito e a matriz; converte somente unidades e bases de umidade compatíveis.
pub fn base_ms(laudo: &Laudo) -> Option<f64> {
    if !laudo.valor.is_finite() || laudo.valor < 0.0 || laudo.ms <= 0.0 || laudo.ms > 1.0 {
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
    for (analito, teto_mistura) in [
        ("AFB1", 5.0),
        ("GOSSIPOL_LIVRE", 500_000.0),
        ("HCN", 50_000.0),
    ] {
        let mut mistura = Some(0.0);
        for item in &c.itens {
            let x = super::calculos::consumo_item(item, c.sobras);
            if x == 0.0 {
                continue;
            }
            let laudo = item.alimento.contaminantes.iter().find(|l| {
                l.analito == analito
                    && l.matriz == "MATERIA_PRIMA"
                    && !l.metodo.is_empty()
                    && !l.documento.is_empty()
            });
            let valor_ms = laudo.and_then(base_ms);
            mistura = mistura.zip(valor_ms).map(|(s, v)| s + x * v);
            if analito == "AFB1" {
                p.comparar(Comparacao {
                    regra: "CON-01",
                    dimensao: "Contaminantes",
                    valor: valor_ms.map(|v| v * 0.88),
                    limite: 20.0,
                    maximo: true,
                    sanitario: true,
                    unidade: "µg/kg base 12%",
                    texto: &format!(
                        "{}: AFB1 na matéria-prima; referência UE N11, \
                        não legislação brasileira. <LOQ usa limite superior conservador.",
                        item.alimento.nome
                    ),
                });
            }
        }
        p.comparar(Comparacao {
            regra: "CON-01", dimensao: "Contaminantes",
            valor: mistura.filter(|_| p.consumo_ms > 0.0).map(|v| 0.88 * v / p.consumo_ms),
            limite: teto_mistura, maximo: true, sanitario: true, unidade: "µg/kg base 12%",
            texto: &format!("{analito}: alimento completo, referência UE N11 ativada. \
                Analito/matriz/laudo incompatível mantém pendência. Diluição não autoriza ingrediente."),
        });
    }
}

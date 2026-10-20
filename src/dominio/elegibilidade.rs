use super::{Cenario, Conhecido, Conhecimento, Estado, Parecer};

pub fn avaliar(c: &Cenario, base: &Conhecimento, p: &mut Parecer) -> bool {
    let l = &c.lote;
    let fora = !(400.0..=700.0).contains(&l.peso)
        || !(3.0..=5.0).contains(&l.gordura)
        || !(5.0..=30.0).contains(&p.lcg4)
        || !(60..=250).contains(&l.dias_lactacao)
        || l.adulta == Conhecido::Nao
        || l.gestante == Conhecido::Sim
        || l.crescimento == Conhecido::Sim
        || l.peso_estavel == Conhecido::Nao
        || l.confinada == Conhecido::Nao
        || l.correcao_ambiente == Conhecido::Sim
        || l.correcao_atividade == Conhecido::Sim;
    let pendente = [
        l.adulta,
        l.gestante,
        l.crescimento,
        l.peso_estavel,
        l.confinada,
    ]
    .contains(&Conhecido::Desconhecido);
    let estado = if fora {
        Estado::Viola
    } else if pendente {
        Estado::Desconhecido
    } else {
        Estado::Atende
    };
    p.registrar(
        "ESC-01",
        "Elegibilidade",
        estado,
        "Adultas em lactação, confinadas, peso estável, sem gestação/crescimento; \
         PV 400–700 kg, gordura 3–5%, LCG4 5–30 kg/dia, DEL 60–250."
            .into(),
    );
    if fora {
        p.condicao = "FORA_DO_PERFIL".into();
    } else if pendente {
        p.condicao = "INCONCLUSIVO".into();
    }
    if l.correcao_ambiente == Conhecido::Desconhecido
        || l.correcao_atividade == Conhecido::Desconhecido
    {
        p.avisos
            .push("Hipótese: ausência de correção ambiental/atividade; não foi avaliada.".into());
    }
    p.consumo_referencia = super::calculos::consumo_referencia(base, l.peso, p.lcg4);
    if l.peso > 600.0 && c.consumo_alvo.is_none() {
        p.registrar(
            "ESC-02",
            "Elegibilidade",
            Estado::Desconhecido,
            "Acima de 600 kg é obrigatório informar consumo-alvo e origem; sem extrapolação."
                .into(),
        );
        if !fora {
            p.condicao = "INCONCLUSIVO".into();
        }
        return false;
    }
    !fora && !pendente
}

pub fn ingredientes(c: &Cenario, base: &Conhecimento, p: &mut Parecer) {
    for item in &c.itens {
        if item.quantidade == 0.0 && !c.formular && item.minimo_ms == 0.0 {
            continue;
        }
        let a = &item.alimento;
        let animal = a.origem_animal == Conhecido::Sim;
        let norma = base.normas.iter().find(|n| n.id == "LEG-ANIMAL");
        let norma_vigente = norma.is_some_and(|n| n.vigente(&c.data_aplicacao));
        let vedado = animal && a.excecao_animal == Conhecido::Nao && norma_vigente;
        let desconhecido = a.autorizado == Conhecido::Desconhecido
            || a.origem_animal == Conhecido::Desconhecido
            || a.evidencia.trim().is_empty()
            || (animal && (!norma_vigente || a.excecao_animal == Conhecido::Desconhecido));
        let estado = if vedado || a.autorizado == Conhecido::Nao {
            Estado::Viola
        } else if desconhecido {
            Estado::Desconhecido
        } else {
            Estado::Atende
        };
        p.registrar(
            if desconhecido { "LEG-02" } else { "LEG-01" },
            "Ingredientes",
            estado,
            format!(
                "{}: identidade, autorização, origem, exceções e vigência; evidência: {}.",
                a.nome, a.evidencia
            ),
        );
        let excluido =
            animal || a.nnp_adicionado == Conhecido::Sim || a.medicamento == Conhecido::Sim;
        let incerto = [a.nnp_adicionado, a.medicamento].contains(&Conhecido::Desconhecido);
        p.registrar(
            "MVP-01",
            "Ingredientes",
            if excluido {
                Estado::Viola
            } else if incerto {
                Estado::Desconhecido
            } else {
                Estado::Atende
            },
            format!(
                "{}: exclusão por POLITICA_MVP de NNP adicionado, medicamentos e \
                subprodutos animais; exceção legal não afasta essa política.",
                a.nome
            ),
        );
    }
}

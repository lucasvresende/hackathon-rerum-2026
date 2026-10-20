use super::{Cenario, Conhecido, Conhecimento};
use microlp::{ComparisonOp, Problem, Variable};

pub fn contaminantes(
    problema: &mut Problem,
    variaveis: &[Variable],
    c: &Cenario,
    base: &Conhecimento,
    consumo: f64,
) -> Result<(), String> {
    if !c.referencia_ue {
        return Ok(());
    }
    if base.limites_contaminantes.is_empty() {
        return Err("Carga dos limites de contaminantes ausente.".into());
    }
    for limite in &base.limites_contaminantes {
        if !base
            .normas
            .iter()
            .any(|n| n.id == limite.norma_id && n.vigente(&c.data_aplicacao))
        {
            return Err("Referência de contaminantes fora da vigência.".into());
        }
        let mut termos = Vec::new();
        for (item, variavel) in c.itens.iter().zip(variaveis) {
            let a = &item.alimento;
            if a.origem_animal != Conhecido::Nao
                || a.nnp_adicionado != Conhecido::Nao
                || a.medicamento != Conhecido::Nao
                || a.autorizado != Conhecido::Sim
                || item.maximo_ms == Some(0.0)
            {
                continue;
            }
            let laudo = super::contaminantes::laudo(a, &limite.analito)
                .ok_or_else(|| format!("{}: falta laudo de {}.", a.nome, limite.analito))?;
            let valor_ms = super::contaminantes::base_ms(laudo)
                .ok_or("Base de umidade ou laudo incompatível.")?;
            if limite.matriz == "MATERIA_PRIMA" {
                if valor_ms * (1.0 - limite.umidade) > limite.limite {
                    if laudo.qualificador == "<LOQ" {
                        return Err("LOQ não permite concluir atendimento do ingrediente.".into());
                    }
                    problema.add_constraint([(*variavel, 1.0)], ComparisonOp::Le, 0.0);
                }
            } else {
                termos.push((*variavel, valor_ms));
            }
        }
        if limite.matriz == "COMPOSTO_LEITEIRO" {
            problema.add_constraint(
                termos,
                ComparisonOp::Le,
                consumo * limite.limite / (1.0 - limite.umidade),
            );
        }
    }
    Ok(())
}

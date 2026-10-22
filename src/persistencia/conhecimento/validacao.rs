//! Verificação da completude e consistência da base de conhecimento carregada.
use crate::Resultado;
use crate::dominio::Conhecimento;

pub fn validar(c: &Conhecimento) -> Resultado<()> {
    for (tabela, eixos, unidade) in [
        (
            "mantenca",
            vec![400., 450., 500., 550., 600., 650., 700.],
            "kg/dia",
        ),
        ("leite", vec![3., 3.5, 4., 4.5, 5.], "kg/kg leite"),
        ("consumo", vec![5., 10., 15., 20., 25., 30.], "% PV"),
        ("fibra", vec![15., 16., 17., 18., 19.], "% MS"),
    ] {
        let linhas: Vec<_> = c
            .coeficientes
            .iter()
            .filter(|v| v.tabela == tabela)
            .collect();
        let quantidade = if tabela == "consumo" {
            eixos.len() * 3
        } else {
            eixos.len()
        };
        if linhas.len() != quantidade
            || linhas.iter().any(|v| {
                v.unidade != unidade
                    || !eixos.contains(&v.eixo)
                    || v.valores.iter().any(|x| !x.is_finite() || *x < 0.0)
            })
        {
            return Err(
                format!("Tabela {tabela} incompleta, eixos ou unidades incompatíveis.").into(),
            );
        }
        if tabela == "consumo"
            && eixos.iter().any(|eixo| {
                [400., 500., 600.]
                    .iter()
                    .any(|peso| !linhas.iter().any(|r| r.eixo == *eixo && r.eixo2 == *peso))
            })
        {
            return Err("Tabela de consumo não cobre todos os nós necessários.".into());
        }
    }
    if c.fontes
        .iter()
        .any(|f| super::conhecimento::hash(&f.conteudo) != f.hash)
    {
        return Err("Fonte com conteúdo/hash incompatível.".into());
    }
    if c.versao.is_empty() || c.perfil_id.is_empty() || c.discrepancia <= 0.0 {
        return Err("Perfil sem versão ou política operacional válida.".into());
    }
    for n in &c.normas {
        if chrono::NaiveDate::parse_from_str(&n.inicio, "%Y-%m-%d").is_err()
            || n.fim.as_ref().is_some_and(|f| f <= &n.inicio)
            || n.transicao
                .as_ref()
                .is_some_and(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").is_err())
            || !c.fontes.iter().any(|f| f.id == n.fonte_id)
        {
            return Err(format!("Norma {} tem fonte ou vigência incompatível.", n.id).into());
        }
    }
    Ok(())
}

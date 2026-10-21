use sqlx::SqliteConnection;

use crate::Resultado;
use crate::dominio::templates::{ModeloPreenchimento, ValorModelo, unidade_campo};

pub async fn ler(conexao: &mut SqliteConnection) -> Resultado<Vec<ModeloPreenchimento>> {
    let valores = sqlx::query_file!("queries/modelos_preenchimento/listar_valores.sql")
        .fetch_all(&mut *conexao)
        .await?;
    Ok(
        sqlx::query_file!("queries/modelos_preenchimento/listar.sql")
            .fetch_all(&mut *conexao)
            .await?
            .into_iter()
            .map(|m| ModeloPreenchimento {
                valores: valores
                    .iter()
                    .filter(|v| v.modelo_id == m.id)
                    .map(|v| ValorModelo {
                        campo: v.campo.clone(),
                        valor: v.valor.clone(),
                        unidade: v.unidade.clone(),
                        valor_original: v.valor_original.clone(),
                        unidade_original: v.unidade_original.clone(),
                        natureza: v.natureza.clone(),
                        fonte_id: v.fonte_id.clone(),
                    })
                    .collect(),
                id: m.id,
                nome: m.nome,
                versao: m.versao,
                tipo: m.tipo,
                contexto: m.contexto,
                lacunas: m.lacunas,
                fonte_id: m.fonte_id,
                fonte_titulo: m.fonte_titulo,
                fonte_url: m.fonte_url,
            })
            .collect(),
    )
}

pub async fn validar(conexao: &mut SqliteConnection) -> Resultado<()> {
    for modelo in ler(conexao).await? {
        if modelo.valores.is_empty() || modelo.versao.is_empty() || modelo.contexto.is_empty() {
            return Err("Modelo sem versão, contexto ou valores.".into());
        }
        for v in &modelo.valores {
            if unidade_campo(&modelo.tipo, &v.campo) != Some(v.unidade.as_str())
                || !["MEDIA_TABELADA", "ESTIMATIVA_TABELADA", "METADADO"]
                    .contains(&v.natureza.as_str())
            {
                return Err(format!("Campo, unidade ou natureza inválida: {}", v.campo).into());
            }
            if (v.campo == "classe"
                && !["FORRAGEM", "CONCENTRADO", "SUPLEMENTO"].contains(&v.valor.as_str()))
                || (v.campo == "natureza" && v.valor != "MEDIA_TABELADA")
                || (v.campo == "fonte_id" && v.valor != modelo.fonte_id)
            {
                return Err("Metadado incompatível no template.".into());
            }
            if v.unidade != "texto" {
                let numero = v
                    .valor
                    .parse::<f64>()
                    .map_err(|_| "Valor numérico inválido")?;
                let original = v
                    .valor_original
                    .parse::<f64>()
                    .map_err(|_| "Valor original inválido")?;
                let convertido = match (v.unidade_original.as_str(), v.unidade.as_str()) {
                    ("g/kg MS", "% MS") => original / 10.0,
                    (a, b) if a == b => original,
                    _ => return Err("Conversão de unidade não autorizada no modelo.".into()),
                };
                if !numero.is_finite()
                    || !original.is_finite()
                    || numero < 0.0
                    || (numero - convertido).abs() > 1e-9
                    || ((v.campo == "ms" || v.campo == "peso") && numero <= 0.0)
                    || (v.unidade.starts_with('%') && v.campo != "ndt" && numero > 100.0)
                {
                    return Err("Valor ou conversão incompatível no modelo.".into());
                }
            }
        }
    }
    Ok(())
}

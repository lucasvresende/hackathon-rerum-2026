use axum::Form;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};

use crate::dominio::templates::{CampoPreenchido, ProcedenciaModelo, equivalentes};
use crate::dominio::validacao::{Problema, problema};
use crate::web::entradas::{Campos, texto};
use crate::web::rotas::{ErroWeb, EstadoApp, autorizado};

pub async fn procedencia(
    e: &EstadoApp,
    campos: &Campos,
    usados: &Campos,
    tipo: &str,
) -> Result<Option<ProcedenciaModelo>, Problema> {
    let id = texto(campos, "modelo_aplicado");
    if id.is_empty() {
        return Ok(None);
    }
    let modelos = crate::persistencia::templates::ler(
        &mut *e
            .pool
            .acquire()
            .await
            .map_err(|_| problema("modelo", "Não foi possível consultar os templates."))?,
    )
    .await
    .map_err(|_| problema("modelo", "Não foi possível consultar os templates."))?;
    let modelo = modelos
        .into_iter()
        .find(|m| m.id == id && m.tipo == tipo)
        .ok_or_else(|| problema("modelo", "Template inexistente ou incompatível."))?;
    let mut campos: Vec<_> = modelo
        .valores
        .iter()
        .map(|v| {
            let utilizado = texto(usados, &v.campo).to_owned();
            CampoPreenchido {
                campo: v.campo.clone(),
                alterado: !equivalentes(&utilizado, &v.valor),
                utilizado,
            }
        })
        .collect();
    let adicionais = if tipo == "alimento" {
        vec!["ms", "ndt", "pb", "ca", "p", "fdn", "amido", "ee"]
    } else {
        vec!["raca", "peso", "producao", "gordura"]
    };
    for campo in adicionais {
        let utilizado = texto(usados, campo);
        if !utilizado.is_empty() && !modelo.valores.iter().any(|v| v.campo == campo) {
            campos.push(CampoPreenchido {
                campo: campo.into(),
                utilizado: utilizado.into(),
                alterado: true,
            });
        }
    }
    Ok(Some(ProcedenciaModelo { modelo, campos }))
}

/// Carrega somente a lista fechada de campos da versão preservada no servidor.
/// Não salva cadastro e não aceita valores de referência enviados pelo navegador.
pub async fn carregar(
    State(e): State<EstadoApp>,
    Path(tipo): Path<String>,
    Query(q): Query<Campos>,
    headers: HeaderMap,
    Form(mut campos): Form<Campos>,
) -> Result<Response, ErroWeb> {
    if !autorizado(&e, &headers, &campos) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }
    if !["lote", "alimento"].contains(&tipo.as_str()) {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }
    let modelos = crate::persistencia::templates::ler(
        &mut *e.pool.acquire().await.map_err(|e| ErroWeb(e.into()))?,
    )
    .await?;
    let modelo = modelos
        .iter()
        .find(|m| m.id == texto(&campos, "modelo") && m.tipo == tipo);
    let mut erros = vec![];
    if let Some(modelo) = modelo {
        campos.insert("previa_modelo".into(), "on".into());
        if texto(&campos, "acao_modelo") == "aplicar" || texto(&q, "acao") == "aplicar" {
            if texto(&campos, "confirmar_modelo") != "on" {
                erros.push(problema(
                    "confirmar_modelo",
                    "Confirme a substituição antes de carregar.",
                ));
            } else {
                // Limpa apenas campos de composição/população: lacuna nunca herda valor
                // antigo.
                let limpar = if tipo == "alimento" {
                    vec!["ms", "ndt", "pb", "ca", "p", "fdn", "amido", "ee"]
                } else {
                    vec!["peso", "producao", "gordura"]
                };
                let estados = if tipo == "alimento" {
                    vec![
                        "origem_animal",
                        "excecao_animal",
                        "nnp_adicionado",
                        "medicamento",
                        "autorizado",
                    ]
                } else {
                    vec![
                        "adulta",
                        "gestante",
                        "crescimento",
                        "peso_estavel",
                        "confinada",
                        "correcao_ambiente",
                        "correcao_atividade",
                        "agua",
                    ]
                };
                for campo in estados {
                    campos.insert(campo.into(), "DESCONHECIDO".into());
                }
                for campo in limpar {
                    campos.insert(campo.into(), String::new());
                }
                for v in &modelo.valores {
                    campos.insert(v.campo.clone(), v.valor.clone());
                }
                campos.insert("modelo_aplicado".into(), modelo.id.clone());
                campos.remove("previa_modelo");
                campos.remove("confirmar_modelo");
            }
        }
    } else {
        erros.push(problema(
            "modelo",
            "Escolha um template para consultar a prévia.",
        ));
    }
    Ok(crate::web::apresentacao::cadastro(
        &tipo,
        &campos,
        &erros,
        &e,
        headers.contains_key("hx-request"),
    )
    .await?
    .into_response())
}

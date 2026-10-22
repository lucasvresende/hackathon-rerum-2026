//! Comparação de cenários salvos e diferenças econômicas entre execuções.
use crate::{
    Resultado,
    dominio::unidades::proximos,
    persistencia::historico::{self, Execucao},
};
use sqlx::SqlitePool;

pub struct Economia {
    pub absoluta: f64,
    pub percentual: Option<f64>,
    pub alternativa_validada: bool,
}

pub struct Comparacao {
    pub referencia: Execucao,
    pub alternativa: Execucao,
    pub equivalentes: bool,
    pub economia: Option<Economia>,
}
impl Comparacao {
    pub fn entre(a: Execucao, b: Execucao) -> Self {
        let equivalentes = a.cenario.perfil_id == b.cenario.perfil_id
            && proximos(a.cenario.lote.producao, b.cenario.lote.producao)
            && proximos(a.parecer.consumo_ms, b.parecer.consumo_ms)
            && a.cenario.horizonte == b.cenario.horizonte
            && a.cenario.referencia_ue == b.cenario.referencia_ue;
        let economia = a
            .parecer
            .custo_resultado()
            .zip(b.parecer.custo_resultado())
            .map(|(ca, cb)| Economia {
                absoluta: ca - cb,
                percentual: (ca > 0.0).then(|| 100.0 * (ca - cb) / ca),
                alternativa_validada: b.parecer.pratico_valido,
            });
        Self {
            referencia: a,
            alternativa: b,
            equivalentes,
            economia,
        }
    }
}

pub async fn comparar(pool: &SqlitePool, a: &str, b: &str) -> Resultado<Comparacao> {
    let a = historico::obter(pool, a).await?;
    let b = historico::obter(pool, b).await?;
    Ok(Comparacao::entre(a, b))
}

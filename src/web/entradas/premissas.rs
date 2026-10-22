//! Atualiza apenas textos gerados pelo sistema; comentários próprios permanecem intactos.
use crate::dominio::Cenario;

pub(super) fn atualizar(c: &mut Cenario) {
    if [
        "Estimativa do perfil; não observado",
        "Referência Tabela B; consumo estimado.",
    ]
    .contains(&c.origem_consumo.as_str())
    {
        c.origem_consumo = c.tipo_consumo().into();
    }
    let mudou_zero = c.sobras != 0.0 || c.itens.iter().any(|i| i.perda != 0.0);
    if mudou_zero
        && [
            "Hipótese: sobras e perdas zero; não medidas",
            "Hipótese explícita: zero sobras; não há medição.",
        ]
        .contains(&c.origem_sobras.as_str())
    {
        c.origem_sobras =
            "Hipótese: taxas informadas pelo operador; sobras e perdas não medidas".into();
    }
}

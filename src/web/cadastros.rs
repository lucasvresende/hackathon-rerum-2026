//! Compatibilidade; novos handlers estão organizados por recurso em `handlers`.
pub use super::handlers::alimentos::{
    novo as novo_alimento, revisar as revisar_alimento, salvar as salvar_alimento,
};
pub use super::handlers::lotes::{editar as editar_lote, novo as novo_lote, salvar as salvar_lote};

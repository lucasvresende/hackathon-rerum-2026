//! Adaptadores HTTP e apresentação de cadastros.
pub mod handlers;
pub mod rotas;
pub use crate::web::alimentos::handlers::{
    novo as novo_alimento, revisar as revisar_alimento, salvar as salvar_alimento,
};
pub use crate::web::lotes::handlers::{
    editar as editar_lote, novo as novo_lote, salvar as salvar_lote,
};
pub mod templates;

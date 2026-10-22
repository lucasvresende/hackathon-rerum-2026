//! Adaptadores HTTP e apresentação de cenarios.
pub mod handlers;
pub mod rotas;
pub use crate::web::execucoes::handlers::{duplicar, resultado};
pub use handlers::{atualizar, executar, novo};
pub mod templates;

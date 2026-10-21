//! Adaptadores HTTP e apresentação; regras e orquestração ficam em `aplicacao`.
mod apresentacao;
mod compatibilidade;
pub mod entradas;
pub mod erro;
pub mod estado;
pub mod handlers;
pub mod resposta;
pub mod rotas;
pub mod seguranca;
pub mod templates;

pub use compatibilidade::{
    cadastro_salvo, cadastros, cenarios, comparacao, modelos_preenchimento, paginas,
};
pub use entradas as formularios;
pub use templates::componentes;

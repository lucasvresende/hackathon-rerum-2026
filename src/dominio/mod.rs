//! Regras de domínio, modelos, validações e motor de inferência e otimização.
#[path = "cenario/modelos.rs"]
pub mod modelos;
pub mod motor;
pub mod parecer;
pub mod unidades;
#[path = "cenario/validacao.rs"]
pub mod validacao;

pub use modelos::*;
pub use motor::{calculos, contaminantes, elegibilidade, inferencia, otimizacao};
pub use parecer::*;
#[path = "conhecimento/templates.rs"]
pub mod templates;
#[path = "cenario/validacao_itens.rs"]
mod validacao_itens;

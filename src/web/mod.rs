//! Adaptadores HTTP e apresentação; regras e orquestração ficam em `aplicacao`.
pub mod ajuda;
pub mod alimentos;
mod apresentacao;
pub mod cadastros;
pub mod cenarios;
pub mod comparacao;
mod compatibilidade;
pub mod comum;
pub mod conhecimento;
pub mod entradas;
#[path = "comum/erro.rs"]
pub mod erro;
#[path = "comum/estado.rs"]
pub mod estado;
pub mod execucoes;
pub mod exemplos;
#[path = "comum/formatacao.rs"]
pub mod formatacao;
pub mod handlers;
pub mod historico;
pub mod inicio;
pub mod lotes;
pub mod modelos_preenchimento;
#[path = "comum/resposta.rs"]
pub mod resposta;
pub mod rotas;
#[path = "comum/seguranca.rs"]
pub mod seguranca;
pub mod templates;

pub use compatibilidade::{cadastro_salvo, paginas};
pub use entradas as formularios;
pub use templates::componentes;

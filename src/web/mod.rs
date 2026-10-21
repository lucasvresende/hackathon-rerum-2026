pub mod cadastro_salvo;
pub mod cadastros;
pub mod cenarios;
pub mod comparacao;
pub mod entradas;
pub mod modelos_preenchimento;
pub mod paginas;
pub mod rotas;
pub mod seguranca;
pub mod templates;

// Mantém os caminhos públicos anteriores durante a migração dos consumidores.
pub use entradas as formularios;
pub use templates::componentes;

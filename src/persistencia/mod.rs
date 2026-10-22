//! Acesso ao SQLite, consultas, validação do esquema e recuperação de dados.
#[path = "banco/banco.rs"]
pub mod banco;
#[path = "cadastros/cadastros.rs"]
pub mod cadastros;
#[path = "conhecimento/conhecimento.rs"]
pub mod conhecimento;
#[path = "demonstracao/exemplos.rs"]
pub mod exemplos;
#[path = "execucoes/historico.rs"]
pub mod historico;
#[path = "banco/recuperacao.rs"]
pub mod recuperacao;
#[path = "conhecimento/templates.rs"]
pub mod templates;
#[path = "conhecimento/validacao.rs"]
mod validacao;

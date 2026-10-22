//! Formulários Maud e campos compartilhados para cadastros e cenários.
#[path = "../../cadastros/templates/formularios/cadastro.rs"]
mod cadastro;
pub use cadastro::DadosCadastro;
#[path = "../../alimentos/templates/formularios/alimento.rs"]
pub mod alimento;
#[path = "../../cadastros/templates/formularios/campos_cadastro.rs"]
mod campos_cadastro;
#[path = "../../cenarios/templates/formularios/cenario.rs"]
pub mod cenario;
#[path = "../../cenarios/templates/formularios/ingredientes.rs"]
mod ingredientes;
#[path = "../../lotes/templates/formularios/lote.rs"]
pub mod lote;

#[path = "../../cenarios/templates/formularios/configuracoes_cenario.rs"]
mod configuracoes_cenario;

#[path = "../../cenarios/templates/formularios/exemplo_cenario.rs"]
mod exemplo_cenario;
#[path = "../../cenarios/templates/formularios/selecao_ingredientes.rs"]
mod selecao_ingredientes;

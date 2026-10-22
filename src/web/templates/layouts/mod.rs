//! Estruturas externas das páginas e documentos HTML.
#[path = "../../comum/templates/layouts/cadastro.rs"]
pub mod cadastro;
#[path = "../../comum/templates/layouts/documento.rs"]
mod documento;
#[path = "../../comum/templates/layouts/principal.rs"]
pub mod principal;

pub use cadastro::Cadastro;
pub use principal::Principal;

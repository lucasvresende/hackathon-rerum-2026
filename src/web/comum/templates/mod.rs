//! Templates Maud da funcionalidade.
pub mod componentes {
    pub use crate::web::templates::componentes::{
        cabecalho, campo, dica, estrutura, expansivel, resumo_erros,
    };
}
pub mod paginas {
    pub use crate::web::templates::paginas::erro;
}
pub mod layouts {
    pub use crate::web::templates::layouts::{cadastro, principal};
}

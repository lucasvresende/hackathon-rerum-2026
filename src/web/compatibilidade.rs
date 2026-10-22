//! Reexportações dos pontos de entrada anteriores à organização por recurso.

pub mod cadastro_salvo {
    pub use crate::web::handlers::cadastros::pagina;
}

pub mod paginas {
    pub use crate::web::handlers::{
        ajuda::ajuda,
        alimentos::listar as alimentos,
        conhecimento::conhecimento,
        historico::historico,
        inicio::{inicio, nao_encontrado},
        lotes::listar as lotes,
    };
}

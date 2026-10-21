//! Reexportações dos pontos de entrada anteriores à organização por recurso.

pub mod cadastro_salvo {
    pub use crate::web::handlers::cadastros::pagina;
}

pub mod cadastros {
    pub use crate::web::handlers::alimentos::{
        novo as novo_alimento, revisar as revisar_alimento, salvar as salvar_alimento,
    };
    pub use crate::web::handlers::lotes::{
        editar as editar_lote, novo as novo_lote, salvar as salvar_lote,
    };
}

pub mod cenarios {
    pub use crate::web::handlers::cenarios::{atualizar, executar, novo};
    pub use crate::web::handlers::execucoes::{duplicar, reproduzir, resultado};
}

pub mod comparacao {
    pub use crate::web::handlers::comparacao::comparar;
}

pub mod modelos_preenchimento {
    pub use crate::web::handlers::modelos_preenchimento::carregar;
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

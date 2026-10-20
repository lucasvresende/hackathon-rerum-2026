#![forbid(unsafe_code)]

pub mod aplicacao;
pub mod dominio;
pub mod persistencia;
pub mod web;

pub type Erro = Box<dyn std::error::Error + Send + Sync>;
pub type Resultado<T> = Result<T, Erro>;

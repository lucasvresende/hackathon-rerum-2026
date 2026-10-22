//! Componentes Maud reutilizáveis, textos de ajuda e conceitos da interface.
#[path = "../../ajuda/templates/componentes/ajuda.rs"]
pub mod ajuda;
#[path = "../../ajuda/templates/componentes/ajuda_conteudos.rs"]
mod ajuda_conteudos;
#[path = "../../comum/templates/componentes/cabecalho.rs"]
pub mod cabecalho;
#[path = "../../comum/templates/componentes/campo.rs"]
pub mod campo;
#[path = "../../alimentos/templates/componentes/conceitos_alimentos.rs"]
pub mod conceitos_alimentos;
#[path = "../../cenarios/templates/componentes/conceitos_cenario.rs"]
pub mod conceitos_cenario;
#[path = "../../inicio/templates/componentes/conceitos_inicio.rs"]
pub mod conceitos_inicio;
#[path = "../../execucoes/templates/componentes/conceitos_resultado.rs"]
pub mod conceitos_resultado;
#[path = "../../comum/templates/componentes/dica.rs"]
pub mod dica;
#[path = "../../execucoes/templates/componentes/entradas.rs"]
pub mod entradas;
#[path = "../../comum/templates/componentes/estrutura.rs"]
pub mod estrutura;
#[path = "../../comum/templates/componentes/expansivel.rs"]
pub mod expansivel;
#[path = "../../inicio/templates/componentes/inicio.rs"]
pub mod inicio;
#[path = "../../modelos_preenchimento/templates/componentes/modelos_preenchimento.rs"]
pub mod modelos_preenchimento;
#[path = "../../execucoes/templates/componentes/resultado.rs"]
pub mod resultado;
#[path = "../../comum/templates/componentes/resumo_erros.rs"]
pub mod resumo_erros;

#[path = "../../exemplos/templates/componentes/exemplos.rs"]
pub mod exemplos;

#[path = "../../execucoes/templates/componentes/consumo_execucao.rs"]
mod consumo_execucao;
#[path = "../../execucoes/templates/componentes/resultado_numeros.rs"]
mod resultado_numeros;
#[path = "../../execucoes/templates/componentes/resultado_pratico.rs"]
mod resultado_pratico;

#[path = "../../execucoes/templates/componentes/resultado_fornecimento.rs"]
mod resultado_fornecimento;
#[path = "../../execucoes/templates/componentes/situacao_execucao.rs"]
mod situacao_execucao;

#[path = "../../comparacao/templates/componentes/comparacao_item.rs"]
pub mod comparacao_item;

#[path = "../../execucoes/templates/componentes/resultado_barreiras.rs"]
mod resultado_barreiras;

#[path = "../../execucoes/templates/componentes/ficha_fornecimento.rs"]
mod ficha_fornecimento;

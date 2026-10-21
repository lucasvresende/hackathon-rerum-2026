//! Componente de apresentação para ajuda, mantendo a renderização desta responsabilidade isolada.
use maud::{Markup, Render, html};

use super::cabecalho::Cabecalho;

pub struct Ajuda;

struct Carta {
    id: &'static str,
    titulo: &'static str,
    passos: &'static [&'static str],
    destino: &'static str,
    acao: &'static str,
}

const CARTAS: &[Carta] = &[
    Carta {
        id: "inicio",
        titulo: "Visão geral e primeiros passos",
        passos: &[
            "A Visão geral é o ponto de partida para trabalhar com as dietas. Os números no \
            topo resumem lotes, animais e composições cadastrados. Passe o mouse sobre o ponto de \
            interrogação ou toque nele para entender cada conceito. Pelo teclado, use Tab \
            para chegar ao indicador e Escape para fechar a explicação.",
            "Lote é um grupo de animais que será avaliado com os mesmos dados, como peso \
            médio e produção de leite. Separe grupos com características diferentes. \
            Lotes cadastrados conta os grupos atuais, sem duplicar suas revisões. \
            Animais cadastrados soma os animais desses lotes: 20 vacas em um lote e 30 \
            em outro resultam em 2 lotes e 50 animais.",
            "Composição é o registro do que um alimento contém, incluindo matéria seca e \
            nutrientes. Duas análises de silagem podem ter valores diferentes e contar como \
            duas composições do mesmo alimento. Ao montar a dieta, escolha a análise que \
            corresponde ao alimento disponível. Preço e estoque serão informados no cenário.",
            "Cenário reúne tudo o que será usado em um cálculo: lote, alimentos, quantidades, \
            preços, disponibilidade, datas e hipóteses. Por exemplo, você pode duplicar um \
            cenário e mudar o preço do milho para comparar custos, preservando o original.",
            "Execução é o cálculo salvo de um cenário, com suas entradas e seu resultado. \
            Ela pode apontar atendimento, violações, pendências ou bloqueios; estar no \
            histórico não significa que a dieta foi aprovada. A tabela Últimos cenários mostra \
            os cinco mais recentes; use Ver histórico para consultar os demais e abrir \
            os pareceres.",
            "Perfil ativo é o conjunto de referências e regras que orienta a avaliação. \
            Ele define a população atendida e os nutrientes e limites considerados. \
            O identificador informa qual versão está em uso. O perfil atual se destina a \
            vacas adultas em lactação e confinamento, dentro dos limites apresentados em \
            Perfil nutricional. Atender ao núcleo avaliado não garante uma dieta completa: \
            confira as limitações e as dimensões que ficaram sem avaliação.",
            "Para começar, cadastre um lote e confira as composições dos alimentos. \
            Verificar dieta avalia quantidades que você informa; Formular dieta busca \
            quantidades de menor custo dentro das condições do cenário. Cadastros abrem \
            em nova guia e permitem carregar modelos editáveis. Se usar a demonstração, \
            lembre que seus dados são sintéticos e precisam ser substituídos para uso real.",
        ],
        destino: "/",
        acao: "Abrir visão geral",
    },
    Carta {
        id: "lotes",
        titulo: "Cadastrar e revisar lotes",
        passos: &[
            concat!(
                "Em Lotes de animais, escolha Cadastrar lote. Informe identificação, número de ",
                "animais, peso médio em kg, produção em kg de leite/animal/dia, gordura em % e ",
                "dias em lactação."
            ),
            "Preencha os estados fisiológicos e de manejo. Informe explicitamente se as vacas \
            são adultas, gestantes, em crescimento, com peso estável e confinadas. Desconhecido \
            não significa não. Separe animais com características diferentes em outros lotes.",
            "Use um lote de exemplo já cadastrado ou Preencher com exemplo; edite os valores. \
            Use Salvar revisão. Para corrigir um lote, volte à lista e escolha Revisar. \
            A alteração gera nova revisão; as entradas das execuções antigas ficam preservadas.",
        ],
        destino: "/lotes",
        acao: "Abrir lotes",
    },
    Carta {
        id: "alimentos",
        titulo: "Pesquisar alimentos e cadastrar composições",
        passos: super::ajuda_conteudos::ALIMENTOS,
        destino: "/alimentos",
        acao: "Abrir catálogo",
    },
    Carta {
        id: "cenarios",
        titulo: "Montar um cenário: consumo, preços e disponibilidade",
        passos: super::ajuda_conteudos::CENARIOS,
        destino: "/cenarios/novo",
        acao: "Montar cenário",
    },
    Carta {
        id: "verificar",
        titulo: "Verificar uma dieta informada",
        passos: &[
            "Escolha Verificar dieta e preencha as premissas do cenário. Selecione até 20 \
            ingredientes e informe suas quantidades na base escolhida, por animal/dia.",
            "Confira se os valores representam consumo ou oferta. Para oferta, mantenha \
            explícitas as hipóteses de sobras e ingestão; oferta não é consumo medido.",
            "Calcule e confira balanços de nutrientes, barreiras ruminais, restrições, custo \
            e quantidades práticas. A verificação avalia a dieta informada; ela não ajusta \
            automaticamente as quantidades para corrigir violações.",
        ],
        destino: "/cenarios/novo?modo=verificar",
        acao: "Verificar dieta",
    },
    Carta {
        id: "formular",
        titulo: "Formular uma dieta de menor custo",
        passos: &[
            "Escolha Formular dieta, confira o consumo-alvo e selecione até 10 ingredientes. \
            Informe preços, disponibilidade, limites de inclusão e resolução de fornecimento.",
            "Calcule para buscar quantidades de menor custo dentro do perfil e das restrições \
            informadas. O motor resolve cinco subproblemas de fibra e revalida as candidatas; \
            ele não relaxa limites automaticamente.",
            "Confira o estado da formulação e de cada subproblema. Ótimo no modelo se limita \
            às entradas e restrições registradas. Busca inconclusiva ou interrompida não prova \
            inviabilidade. Confira também a revalidação após arredondar as quantidades práticas.",
        ],
        destino: "/cenarios/novo?modo=formular",
        acao: "Formular dieta",
    },
    Carta {
        id: "resultados",
        titulo: "Ler o parecer, imprimir, reproduzir e simular",
        passos: super::ajuda_conteudos::RESULTADOS,
        destino: "/historico",
        acao: "Escolher um parecer no histórico",
    },
    Carta {
        id: "historico",
        titulo: "Consultar e filtrar o histórico",
        passos: &[
            concat!(
                "Em Histórico e análise, filtre pelo nome do lote e pela data da execução no ",
                "horário de São Paulo. Use Filtrar histórico e navegue pelas páginas de até 100 ",
                "resultados."
            ),
            "Escolha Ver parecer para reabrir as entradas e conclusões congeladas da execução. \
            Revisões posteriores de lotes, alimentos ou fontes não alteram esse registro.",
            "Para comparar, selecione os cenários de referência e alternativo disponíveis na \
            página e use Comparar premissas e custos.",
        ],
        destino: "/historico",
        acao: "Abrir histórico",
    },
    Carta {
        id: "comparacao",
        titulo: "Comparar premissas e custos de cenários",
        passos: &[
            "Em Histórico e análise, selecione duas execuções e use Comparar premissas e custos. \
            A primeira é a referência e a segunda é a alternativa.",
            "Confira perfil, produção, consumo, sobras, horizonte, base de comparação e \
            premissas por ingrediente antes de interpretar a diferença de custo por animal/dia.",
            "Uma diferença positiva (referência menos alternativa) indica custo menor na \
            alternativa. Mudanças de perfil, produção, consumo, horizonte ou critérios podem \
            impedir equivalência. Menor custo sem fornecimento validado não aprova substituição.",
            "Use Abrir referência ou Abrir alternativa para consultar os pareceres completos \
            e suas restrições.",
        ],
        destino: "/historico",
        acao: "Selecionar execuções para comparar",
    },
    Carta {
        id: "conhecimento",
        titulo: "Consultar o perfil nutricional, regras e fontes",
        passos: &[
            "Em Perfil nutricional, confira o identificador, a versão, a população e os limites \
            antes de usar os cálculos. Um perfil reúne referências, coeficientes e regras que \
            definem quais animais e condições são cobertos e como os resultados são calculados. \
            O perfil atual contempla vacas adultas em lactação, confinadas e estáveis, sem \
            gestação nem crescimento, dentro das faixas apresentadas.",
            "Use os pontos de interrogação na tabela para entender tabela, eixos, coeficientes \
            A–D e unidades. Expanda as fontes de referência para ver edição, localização, \
            conteúdo e hash. Em Normas, vigência e condições, confira datas, transições, \
            exceções, evidências e ações aplicáveis.",
            "Distinga legislação brasileira, referência estrangeira, referência técnica e \
            política do MVP. O perfil histórico NRC 1989 não implementa integralmente NASEM \
            2021 nem BR-LEITE 2024; fibra física e outras dimensões podem ficar não avaliadas.",
        ],
        destino: "/conhecimento",
        acao: "Abrir perfil nutricional",
    },
];

impl Render for Carta {
    fn render(&self) -> Markup {
        let botao = format!("{}-botao", self.id);
        let conteudo = format!("{}-conteudo", self.id);
        html! {
            article.card.help-card.help-open id=(self.id) data-aberta="true" data-pronta="false"
                hx-live:class="{ 'help-open': data.aberta, 'help-ready': data.pronta }"
                hx-on:load="data.aberta = location.hash === '#' + this.id;
                    await nextFrame(); await nextFrame(); data.pronta = true" {
                h2 {
                    button.help-toggle id=(botao) type="button" hidden
                        hx-live:hidden="false" aria-expanded="true"
                        hx-live:aria-expanded="data.aberta" aria-controls=(conteudo)
                        hx-on:click="data.aberta = !data.aberta" {
                        (self.titulo)
                        span.help-indicator aria-hidden="true"
                            hx-live:text="data.aberta ? '−' : '+'" { "−" }
                    }
                    span hx-live:hidden="true" { (self.titulo) }
                }
                div.help-content id=(conteudo) hx-live:inert="!data.aberta" {
                    div.help-content-inner {
                        div.help-body {
                            ol { @for passo in self.passos { li { (passo) } } }
                            a.text-link href=(self.destino) { (self.acao) " →" }
                        }
                    }
                }
            }
        }
    }
}

impl Render for Ajuda {
    fn render(&self) -> Markup {
        html! {
            (Cabecalho {
                titulo: "Ajuda",
                texto: "Abra uma carta para consultar as instruções de cada tarefa.",
                ajuda: "inicio"
            })
            nav.card.help-index aria-label="Seções da ajuda" {
                @for carta in CARTAS {
                    a href=(format!("#{}", carta.id))
                        hx-on:click=(format!("q('#{}').data.aberta = true", carta.id)) {
                        (carta.titulo)
                    }
                }
            }
            @for carta in CARTAS { (carta) }
        }
    }
}

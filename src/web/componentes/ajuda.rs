use maud::{Markup, Render, html};

use super::estrutura::cabecalho;

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
            "Use o menu para acessar os cadastros, os cálculos, o histórico e as fontes. \
            Na visão geral, confira os totais cadastrados e abra as execuções recentes.",
            "Comece cadastrando um lote e conferindo as composições dos alimentos. Depois use \
            Verificar dieta para avaliar quantidades conhecidas ou Formular dieta para buscar \
            quantidades de menor custo.",
            "Os exemplos de demonstração são sintéticos. Confira os dados antes de criar um \
            cenário de uso. O botão Ajuda nesta página abre a seção relacionada à tarefa.",
        ],
        destino: "/",
        acao: "Abrir visão geral",
    },
    Carta {
        id: "lotes",
        titulo: "Cadastrar e revisar lotes",
        passos: &[
            "Em Lotes, escolha Cadastrar lote. Informe identificação, número de animais, peso \
            médio em kg, produção em kg de leite/animal/dia, gordura em % e dias em lactação.",
            "Preencha os estados fisiológicos e de manejo. Informe explicitamente se as vacas \
            são adultas, gestantes, em crescimento, com peso estável e confinadas. Desconhecido \
            não significa não. Separe animais com características diferentes em outros lotes.",
            "Use Salvar revisão. Para corrigir um lote, volte à lista e escolha Revisar. \
            A alteração gera nova revisão; as entradas das execuções antigas ficam preservadas.",
        ],
        destino: "/lotes",
        acao: "Abrir lotes",
    },
    Carta {
        id: "alimentos",
        titulo: "Pesquisar alimentos e cadastrar composições",
        passos: &[
            "No catálogo, digite parte do nome e use Pesquisar. Confira composição, natureza \
            do dado e a seção Procedência e condições de cada alimento.",
            "Use Cadastrar composição ou Criar revisão ou nova análise. Registre identidade, \
            classe, origem, processamento, fonte, método, data e evidências de uso. \
            Identifique corretamente dados sintéticos, médias e análises.",
            "A matéria seca (MS) é expressa em % da matéria natural (MN); NDT, PB, Ca, P, \
            FDN, amido e EE são expressos em % da MS. Não preencha um nutriente desconhecido \
            como zero. Registre origem animal, NNP, autorização e limite de rótulo do suplemento.",
            "Salve a revisão para novos cenários. Análises anteriores continuam disponíveis; \
            preço e estoque locais são informados no cenário, sem alterar a composição científica.",
        ],
        destino: "/alimentos",
        acao: "Abrir catálogo",
    },
    Carta {
        id: "cenarios",
        titulo: "Montar um cenário: consumo, preços e disponibilidade",
        passos: &[
            "Em Novo cenário, informe nome, lote, tipo de cálculo, horizonte em dias e datas \
            de avaliação e aplicação das regras. Escolha a base das quantidades por ingrediente: \
            consumo em MS, consumo em MN ou oferta em MN, sempre por animal/dia.",
            "Informe consumo-alvo em kg MS/animal/dia e sua origem ou deixe vazio para usar a \
            Tabela B, quando aplicável. Acima de 600 kg de peso vivo, o alvo explícito é \
            obrigatório. Para consumo observado, informe oferta e sobras em MS do lote e período \
            de medição; deixe o alvo vazio. Seleção de partículas limita essa estimativa.",
            "Registre sobras planejadas e origem das sobras e perdas. Zero também é uma \
            hipótese que precisa ser declarada. O preço do leite é opcional para calcular \
            margem sobre alimentação. A referência UE adicional é uma escolha explícita, \
            separada da legislação brasileira.",
            "Selecione uma composição por alimento e abra a carta do ingrediente. Confira MS, \
            preço, unidade comercial, massa de embalagem, estoque bruto em MN, perda, inclusão \
            mínima/máxima e resolução de fornecimento. Sem limite de estoque \
            é uma hipótese explícita; estoque desconhecido não significa infinito.",
            "Use Calcular e salvar cenário. Se houver erros, corrija os campos indicados; \
            os valores válidos permanecem no formulário. O resultado salva as entradas e versões.",
        ],
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
        passos: &[
            "Leia as dimensões separadamente: integridade, nutrientes, barreiras ruminais, \
            ingredientes e avaliações fora do núcleo. ATENDE é atendimento ao perfil avaliado; \
            não é aprovação nutricional integral. DESCONHECIDO e NAO_AVALIADO exigem atenção.",
            "Confira o balanço nutricional, custos e quantidades em MN/MS por animal, lote e \
            horizonte. Só use o fornecimento prático quando sua revalidação estiver indicada \
            como válida. INVALIDO, FORA_DO_PERFIL e BLOQUEADO impedem recomendação automática.",
            "Abra Cálculos e regras aplicadas para consultar entradas, hipóteses, fontes e \
            caminho de inferência. Use Imprimir parecer para gerar uma versão de impressão \
            ou salvar como PDF pelo navegador.",
            "Use Reproduzir versões originais para executar novamente com as entradas e \
            artefatos preservados. Use Duplicar e simular para criar outro cenário: altere \
            preços, MS ou premissas e escolha a base da simulação de MS (MS ou MN fixas). \
            Calcule com outro nome e compare no histórico; o original continua preservado.",
        ],
        destino: "/historico",
        acao: "Escolher um parecer no histórico",
    },
    Carta {
        id: "historico",
        titulo: "Consultar e filtrar o histórico",
        passos: &[
            "Em Histórico, filtre pelo nome do lote e pela data da execução no horário de \
            São Paulo. Use Filtrar histórico e navegue pelas páginas de até 100 resultados.",
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
            "No Histórico, selecione duas execuções e use Comparar premissas e custos. \
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
        titulo: "Consultar perfil nutricional, regras e fontes",
        passos: &[
            "Em Perfil e fontes, confira o identificador do perfil, sua população e limites \
            antes de usar os cálculos. O núcleo contempla vacas adultas em lactação, confinadas, \
            estáveis, sem gestação nem crescimento, dentro das faixas apresentadas.",
            "Consulte os coeficientes e expanda as fontes para ver edição, localização, \
            conteúdo e hash. Em Normas, vigência e condições, confira datas, transições, \
            exceções, evidências e ações aplicáveis.",
            "Distinga legislação brasileira, referência estrangeira, referência técnica e \
            política do MVP. O perfil histórico NRC 1989 não implementa integralmente NASEM \
            2021 nem BR-LEITE 2024; fibra física e outras dimensões podem ficar não avaliadas.",
        ],
        destino: "/conhecimento",
        acao: "Abrir perfil e fontes",
    },
];

impl Render for Carta {
    fn render(&self) -> Markup {
        let botao = format!("{}-botao", self.id);
        let conteudo = format!("{}-conteudo", self.id);
        html! {
            article.card.help-card id=(self.id) data-aberta="true"
                hx-on:load="data.aberta = location.hash === '#' + this.id" {
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
                div.help-content id=(conteudo) hx-live:hidden="!data.aberta" {
                    ol { @for passo in self.passos { li { (passo) } } }
                    a.text-link href=(self.destino) { (self.acao) " →" }
                }
            }
        }
    }
}

impl Render for Ajuda {
    fn render(&self) -> Markup {
        html! {
            (cabecalho("Ajuda", "Abra uma carta para consultar as instruções de cada tarefa."))
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

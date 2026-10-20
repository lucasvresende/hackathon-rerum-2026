## Instruções

1. Commits contínuos sempre que uma nova funcionalidade ou alteração for realizada, ainda que dentro de uma mesma sessão de execução
2. Commit messages curtas, descritivas e fáceis de identificar. ex.: "docs: atualiza a documentação de <substituir>", "feat: adiciona <substituir>", etc.
3. Repositório em português do Brasil
4. Sempre atualizar a documentação quando houver mudanças no repositório
5. Antes de implementar uma funcionalidade, verifique se já existem recursos no tech stack que permitem uma resolução mais elegante
6. Evitar dependências não rust-native
7. Novas funcionalidades devem sempre estar cobertas por um conjunto de testes para testá-las individualmente ou em conjunto com o restante da aplicação
8. Tenha como referência a documentação mais recente dos recursos e bibliotecas utilizados, incluindo sintaxe, boas práticas e exemplos.

## Tech Stack

1. Server-side rendering em **Rust**
2. Web framework: [Axum](https://github.com/tokio-rs/axum) com tracing
3. Templating engine: [Maud](https://github.com/lambda-fairy/maud), com structs implementando o trait _Maud::Render_ e gerando _Maud::Markup_. Utilizar componentes e evitar muitos templates em um mesmo arquivo para funcionalidades diferentes
4. Reatividade e AJAX: [htmx v4](https://four.htmx.org/) para requisições e substituição de trechos de html da página. Extensão [hx-live](https://four.htmx.org/extensions/hx-live) para reatividade client-side e sincronização de comportamentos com o htmx. Mantidos em cópia local minificada do javascript
5. Fase de desenvolvimento: autoreload com ferramentas como [tower-livereload](https://github.com/leotaku/tower-livereload) e [bacon](https://github.com/canop/bacon)
6. Banco de dados: Arquivo **SQLite** local
7. Gestão do banco de dados: [SQLx](https://docs.rs/sqlx/latest/sqlx/), utilizando obrigatoriamente macros para validação em tempo de compilação e o sistema próprio de migração. Os queries devem estar em arquivos .sql separados.
8. Estilo: Um arquivo **css plano**, com variáveis definidas na parte inicial.
9. Outras bibliotecas/crates e recursos podem ser adicionados caso necessário, priorizando recursos bem mantidos e sem excesso de dependências

## Formatação e Documentação

1. Priorizar a legibilidade dos arquivos
2. Comprimento máximo da linha de 100 caracteres, inclusive para os templates maud. Enforce com script e CI
3. Evitar arquivos/módulos excessivamente longos. Refatorar para dividir funcionalidades entre arquivos separados. Incluir script para warnings
4. Incluir documentation strings para funcionalidades não intuitivas
5. Incluir documentação completa em markdown, incluindo informações sobre como rodar, estrutura e fluxo da aplicação, organização do repositório, banco de dados e testes

## Setup de Continuous Integration:

1. GitHub actions
2. Testar build
3. Rodar testes
4. Rodar clippy
5. Teste de regras de formatação
6. Iterar até que funcione

## Instruções

1. Commits contínuos sempre que uma nova funcionalidade ou alteração for realizada, ainda que dentro de uma mesma sessão de execução
2. Commit messages curtas, descritivas e fáceis de identificar. ex.: "docs: atualiza a documentação de <substituir>", "feat: adiciona <substituir>"
3. Repositório em português
4. Sempre atualizar a documentação quando houver mudanças

## Tech Stack

1. Server-side rendering em **Rust**
2. Web framework: **Axum**
3. Templating engine: **Maud**, com structs implementando o trait _Maud::Render_ e gerando _Maud::Markup_
4. Reatividade e AJAX: **htmx v4**
5. Fase de desenvolvimento: autoreload com ferramentas como **tower-livereload**
6. Banco de dados: **SQLite** local
7. Gestão do banco de dados: **SQLx**, utilizando obrigatoriamente macros para validação em tempo de compilação e o sistema próprio de migração
8. Outras bibliotecas/crates e recursos podem ser adicionados

## Continuous Integration:

1. GitHub actions
2. Testar build
3. Rodar testes
4. Rodar clippy
5. Teste de regras de formatação
6. Iterar até que funcione

# Instruções para Agentes AI - RustMoon

Este é um projeto educacional em Rust focado em demonstrar conceitos fundamentais da linguagem. Aqui estão as diretrizes principais para trabalhar neste código:

## Estrutura do Projeto

O projeto está organizado em módulos temáticos, cada um focando em um conceito específico do Rust:

- `mut01-04/`: Exemplos de mutabilidade e imutabilidade
- `condicionais/`: Demonstrações de estruturas de controle de fluxo

## Padrões do Projeto

### Mutabilidade
- Por padrão, todas as variáveis são imutáveis
- Use `let mut` explicitamente quando mutabilidade for necessária
- Exemplo em `mut02/src/main.rs`:
```rust
let x:i32 = 5;        // imutável
let mut y:i32 = 5;    // mutável
```

### Constantes
- Devem sempre ter tipo explícito
- São declaradas com `const` e em SNAKE_CASE maiúsculo
- Exemplo em `mut04/src/main.rs`:
```rust
const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60;
```

### Convenções de Código
- Usar tipos explícitos para melhor legibilidade
- Comentários explicativos para conceitos importantes
- Demonstrar diferentes formas de alcançar o mesmo resultado

## Fluxo de Desenvolvimento

### Comandos Principais
```bash
cargo new <nome-do-módulo>   # Criar novo módulo
cargo build                  # Compilar o projeto
cargo run                    # Executar o programa
```

### Debug e Warnings
- O Rust emite warnings úteis sobre:
  - Variáveis não utilizadas
  - Atribuições sem uso
  - Reassociações que podem causar confusão

## Dicas para Agentes AI

1. Ao adicionar novos exemplos, seguir o padrão de numeração existente (mut01, mut02, etc.)
2. Incluir comentários explicativos detalhando os conceitos demonstrados
3. Manter consistência com o estilo de código existente
4. Priorizar clareza e valor educacional sobre otimização

## Exemplos de Referência

Para entender o estilo do projeto, consulte:
- Mutabilidade básica: `mut01/src/main.rs`
- Reatribuição de variáveis: `mut03/src/main.rs`
- Constantes e escopos: `mut04/src/main.rs`
- Controle de fluxo: `condicionais/cond01/src/main.rs`
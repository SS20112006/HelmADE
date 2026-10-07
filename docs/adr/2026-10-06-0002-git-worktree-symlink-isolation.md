# Git Worktree e Symlink de Dependências para Isolamento Concorrente de Grelhas

* **Status**: accepted
* **Deciders**: Principal Architect (Orchestrator), Simão Sousa
* **Date**: 2026-10-06

## Context and Problem Statement

O HelmADE opera múltiplos agentes de IA a trabalhar concorrentemente em grelha (até 16 grelhas em simultâneo). Se múltiplos agentes alterarem os mesmos ficheiros no mesmo diretório de trabalho ao mesmo tempo, ocorrerá corrupção de código, conflitos de escrita destrutivos e builds inconsistentes. Além disso, duplicar diretórios pesados de dependências (`node_modules/`, `.venv/`, `target/`) por cada grelha causaria desperdício massivo de disco (gigabytes desnecessários) e lentidão extrema ao iniciar uma nova grelha.

## Decision Drivers

* **Invariante de Concorrência Segura**: Eliminar qualquer possibilidade de múltiplos agentes alterarem a mesma árvore de ficheiros em simultâneo sem isolamento.
* **Economia de Disco & Tempo de Arranque Instantâneo**: Subir novas grelhas em milissegundos (< 200ms) sem copiar gigabytes de dependências.
* **Facilidade de Fusão**: Permitir rever diffs limpos e aplicar fusão segura (`Squash & Merge` ou `Rebase Merge`) na branch de destino após a conclusão da tarefa do agente.
* **Proibição de `git stash`**: Manter conformidade estrita com as regras universais de multi-agente.

## Considered Options

* **Opção A — Git Worktrees + Symlinks Automáticos de Pastas Pesadas**: Cada grelha tem o seu próprio worktree isolado (`.helm/worktrees/<id>` apontando para a branch `helm/swarm-<id>`). As pastas pesadas e ignoradas no Git (`node_modules/`, `.venv/`, `target/`) são ligadas simbolicamente (`symlink`) a partir do diretório raiz.
* **Opção B — Clonagem Completa do Repositório (`git clone`)**: Criação de cópias completas do projeto para cada grelha. Inviável: consome gigabytes por agente e exige `npm install` repetido.
* **Opção C — Branches Normais no Mesmo Diretório com `git checkout`**: Impossível para execução concorrente, pois mudar de branch afeta todos os agentes no mesmo diretório.
* **Opção D — Containers Docker Locais**: Cria isolamento forte, mas quebra o invariante de custo e performance (overhead de memória alto, lento em macOS, inviabiliza acesso rápido a ferramentas do host).

## Decision Outcome

Chosen option: **"Opção A — Git Worktrees + Symlinks Automáticos"**, porque fornece isolamento nativo de sistema de ficheiros ao nível do Git, permitindo commits atómicos isolados sem interferência entre grelhas, enquanto os symlinks para pastas de dependências garantem arranque a frio instantâneo com zero desperdício de espaço em disco.

### Positive Consequences

* Cada agente trabalha no seu próprio ramo e pasta isolada sem risco de sobreposição de ficheiros ou conflitos em tempo de execução.
* A criação de um worktree com symlinks leva menos de 200ms.
* Zero consumo adicional de disco para pacotes e compilados reutilizáveis.
* Conexão nativa com ferramentas Git padrão do programador; o utilizador pode inspecionar qualquer worktree a qualquer momento.

### Negative Consequences

* Em sistemas operativos que limitem a criação de symlinks sem privilégios (ex.: Windows Developer Mode desativado), requer atenção especial (no macOS é 100% nativo e sem restrições).
* Ferramentas que não sigam symlinks por padrão exigem flags de configuração adequadas.

## Sub-Agent Delegation Plan

1. **`fullstack-engineer`**: Implementar o gestor de ciclo de vida de worktrees em `src-tauri/src/git/worktree.rs` e symlinks em `src-tauri/src/git/symlink.rs`.
2. **`database-specialist`**: Modelar a tabela `worktrees` no SQLite local para rastreio de grelhas ativas e caminhos.
3. **`frontend-design-specialist`**: Criar a interface de revisão visual de diffs e fusão segura em `src/views/MergeReviewView.tsx`.
4. **`qa-security-auditor`**: Validar segurança de paths para prevenir path traversal na criação de worktrees e symlinks.
5. **`devops-release`**: `feat(git): implement zero-corruption git worktree lifecycle manager`.

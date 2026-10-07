#!/usr/bin/env bash
# ==============================================================================
# sync-roadmap-issues.sh
# Cria ou sincroniza todas as tarefas do Roadmap do HelmADE como GitHub Issues
# Requer: GitHub CLI (`gh`) autenticado com `gh auth login`
# ==============================================================================
set -euo pipefail

REPO="SS20112006/HelmADE"

echo "=== [1/2] A configurar Labels padronizadas no GitHub ==="
if command -v gh &> /dev/null; then
  # Cria labels base se ainda não existirem
  gh label create "phase:0-scaffold" --repo "$REPO" --color "0052cc" --description "Fase 0: Scaffolding, Guardrails & Arquitetura Base" --force || true
  gh label create "phase:1-workspace" --repo "$REPO" --color "1d76db" --description "Fase 1: Workspaces, Git Worktrees & Isolamento" --force || true
  gh label create "phase:2-grid-pty" --repo "$REPO" --color "5319e7" --description "Fase 2: Grelha Tiled & Multiplexador de Terminais" --force || true
  gh label create "phase:3-agents-teamwire" --repo "$REPO" --color "d93f0b" --description "Fase 3: Motor de Agentes Antigravity (.md) & Team Wire" --force || true
  gh label create "phase:4-live-preview" --repo "$REPO" --color "0e8a16" --description "Fase 4: Live Preview & Navegador Integrado" --force || true
  gh label create "phase:5-helmvoice" --repo "$REPO" --color "fbca04" --description "Fase 5: HelmVoice (Módulo de Voz a Custo Zero)" --force || true
  gh label create "phase:6-governance" --repo "$REPO" --color "c5def5" --description "Fase 6: Governação, Telemetria & Streamer Shield" --force || true
  gh label create "phase:7-audit-e2e" --repo "$REPO" --color "b60205" --description "Fase 7: Auditoria Apple HIG, Performance & E2E" --force || true
  gh label create "phase:8-release" --repo "$REPO" --color "006b75" --description "Fase 8: Packaging, CI/CD & Release v1.0.0" --force || true

  gh label create "priority:p0-critical" --repo "$REPO" --color "b60205" --description "Bloqueador ou dependência crítica de arquitetura" --force || true
  gh label create "priority:p1-high" --repo "$REPO" --color "d93f0b" --description "Funcionalidade central de módulo" --force || true
  gh label create "priority:p2-medium" --repo "$REPO" --color "fbca04" --description "Funcionalidade complementar" --force || true

  gh label create "agent:fullstack" --repo "$REPO" --color "1f883d" --description "Atribuído ao Full-Stack Implementation Engineer" --force || true
  gh label create "agent:frontend" --repo "$REPO" --color "6f42c1" --description "Atribuído ao Anti-Slop Frontend & Design Engineer" --force || true
  gh label create "agent:apple-hig" --repo "$REPO" --color "0366d6" --description "Atribuído ao Apple HIG Design Specialist" --force || true
  gh label create "agent:database" --repo "$REPO" --color "336699" --description "Atribuído ao Database Specialist (SQLite)" --force || true
  gh label create "agent:qa-security" --repo "$REPO" --color "cb2431" --description "Atribuído ao QA & Security Auditor" --force || true
  gh label create "agent:devops" --repo "$REPO" --color "24292e" --description "Atribuído ao DevOps & Release Agent" --force || true
  gh label create "agent:system-architect" --repo "$REPO" --color "5c2d91" --description "Atribuído ao System Architect" --force || true

  echo "✅ Labels sincronizadas com sucesso no repositório $REPO."
else
  echo "⚠️ 'gh' CLI não encontrado no PATH. Instala com 'brew install gh' ou faz login com 'gh auth login'."
fi

echo ""
echo "=== [2/2] A criar Issues correspondentes a cada Task do Roadmap ==="

create_task_issue() {
  local title="$1"
  local labels="$2"
  local body="$3"

  echo "A criar Issue: $title..."
  if command -v gh &> /dev/null; then
    gh issue create --repo "$REPO" --title "$title" --label "$labels" --body "$body" || true
  fi
}

# Task 0.1
create_task_issue \
  "Task 0.1: Scaffolding do Monorepo Tauri 2 + React 19 + Tailwind v4" \
  "phase:0-scaffold,priority:p0-critical,agent:fullstack" \
  "### Objetivo
Inicializar o projeto desktop com Tauri 2.x (Rust backend) e frontend React 19 / TypeScript / Vite com Tailwind CSS v4, configurando o manifesto base e janelas transparentes com suporte a vibração nativa macOS.

### Ficheiros
- \`src-tauri/Cargo.toml\`
- \`src-tauri/tauri.conf.json\`
- \`src-tauri/src/main.rs\`
- \`package.json\`
- \`vite.config.ts\`
- \`src/index.css\`
- \`src/App.tsx\`

### Quality Gate
\`cargo check\` passa a verde; \`npm run build\` compila sem erros de tipo.

### Commit Checkpoint
\`chore(scaffold): initialize tauri 2 rust and react 19 desktop core\`"

# Task 0.2
create_task_issue \
  "Task 0.2: Definição das ADRs Fundacionais" \
  "phase:0-scaffold,priority:p0-critical,agent:system-architect" \
  "### Objetivo
Formalizar as decisões de arquitetura em ADRs cobrindo Runtime Desktop, Isolamento por Worktrees, PTY multiplexing e Motor de Voz local a custo zero.

### Ficheiros
- \`docs/adr/2026-10-06-0001-desktop-runtime-tauri2-rust.md\`
- \`docs/adr/2026-10-06-0002-git-worktree-symlink-isolation.md\`
- \`docs/adr/2026-10-06-0003-zero-cost-local-speech-engine.md\`

### Quality Gate
ADRs estruturadas sem placeholders ou indefinições.

### Commit Checkpoint
\`docs(adr): formalize foundational runtime and isolation architecture\`"

# Task 0.3
create_task_issue \
  "Task 0.3: Configuração de Guardrails de Qualidade e Commitlint" \
  "phase:0-scaffold,priority:p0-critical,agent:devops" \
  "### Objetivo
Instalar scripts de verificação pré-commit e validação de Conventional Commits 1.0.0, assegurando a proibição de git stash e cumprimento das diretrizes globais.

### Ficheiros
- \`.github/workflows/ci.yml\`
- \`package.json\`
- \`tsconfig.json\`

### Quality Gate
Execução de linter e validação de commit com formato convencional.

### Commit Checkpoint
\`ci(guardrails): establish strict quality gates and conventional commits\`"

# Task 1.1
create_task_issue \
  "Task 1.1: Persistência Local em SQLite & Gestão de Projetos (Módulo 1)" \
  "phase:1-workspace,priority:p0-critical,agent:database" \
  "### Objetivo
Implementar a camada de armazenamento local SQLite em Rust (\`rusqlite\`) para gerir projetos recentes, sessões, grelhas, caminhos no disco e preferências sem requisições de rede.

### Ficheiros
- \`src-tauri/src/db/mod.rs\`
- \`src-tauri/src/db/schema.rs\`
- \`src-tauri/src/commands/project.rs\`
- \`src/services/projectService.ts\`
- \`src/types/project.ts\`

### Quality Gate
Testes unitários Rust validando CRUD de projetos e migrações locais SQLite; \`cargo test db\`.

### Commit Checkpoint
\`feat(workspace): implement local sqlite storage for project management\`"

# Task 1.2
create_task_issue \
  "Task 1.2: Ecrã Inicial de Seleção de Projetos e Recentes (Módulo 1)" \
  "phase:1-workspace,priority:p1-high,agent:frontend" \
  "### Objetivo
Criar a interface de boas-vindas do HelmADE com diálogo nativo de seleção de pasta, lista de projetos recentes com estatísticas, pesquisa rápida e suporte a arrastar-e-soltar (drag & drop).

### Ficheiros
- \`src/views/WelcomeView.tsx\`
- \`src/components/ProjectList.tsx\`
- \`src/components/RecentProjectCard.tsx\`
- \`src/hooks/useProjects.ts\`

### Quality Gate
Verificação visual Apple HIG, contraste WCAG AA >= 4.5:1, navegação por teclado funcional.

### Commit Checkpoint
\`feat(ui): build welcome and recent projects view with drag-and-drop\`"

# Task 1.3
create_task_issue \
  "Task 1.3: Gestor de Git Worktrees e Criação Isolada de Grelhas (Módulo 2)" \
  "phase:1-workspace,priority:p0-critical,agent:fullstack" \
  "### Objetivo
Criar o comando Tauri em Rust para criar automaticamente Git Worktrees (\`git worktree add -b helm/swarm-<id> .helm/worktrees/<id> <base-branch>\`) e remover worktrees de forma segura no encerramento da grelha.

### Ficheiros
- \`src-tauri/src/git/mod.rs\`
- \`src-tauri/src/git/worktree.rs\`
- \`src-tauri/src/commands/worktree.rs\`
- \`src/services/worktreeService.ts\`

### Quality Gate
Teste de integração Rust criando, listando e limpando um worktree real num repositório temporário.

### Commit Checkpoint
\`feat(git): implement zero-corruption git worktree lifecycle manager\`"

# Task 1.4
create_task_issue \
  "Task 1.4: Gestor de Symlinks de Dependências com Zero Desperdício de Disco (Módulo 2)" \
  "phase:1-workspace,priority:p1-high,agent:fullstack" \
  "### Objetivo
Implementar no motor Rust a ligação simbólica automática de diretórios ignorados no Git (\`node_modules/\`, \`.venv/\`, \`.target/\`, \`.cargo/\`) entre a pasta principal e a pasta do worktree recém-criado, prevenindo duplicação de gigabytes no disco.

### Ficheiros
- \`src-tauri/src/git/symlink.rs\`
- \`src-tauri/src/commands/symlink.rs\`
- \`src/services/symlinkService.ts\`

### Quality Gate
Teste comprovando a criação do symlink e verificação de que arquivos de dependência são lidos transparentemente na worktree.

### Commit Checkpoint
\`feat(git): add automatic dependency symlinker for instant worktree boot\`"

# Task 1.5
create_task_issue \
  "Task 1.5: Alocador Dinâmico de Portas de Servidores Locais (Módulo 2)" \
  "phase:1-workspace,priority:p1-high,agent:fullstack" \
  "### Objetivo
Implementar sistema de deteção de portas livres no sistema operativo (\`std::net::TcpListener\`) e injeção automática de \`PORT=300X\` nas variáveis de ambiente de cada grelha para evitar colisões \`EADDRINUSE\`.

### Ficheiros
- \`src-tauri/src/network/port_allocator.rs\`
- \`src-tauri/src/commands/network.rs\`
- \`src/stores/useGridStore.ts\`

### Quality Gate
Teste de alocação sequencial garantindo portas não-colidentes para 16 grelhas paralelas.

### Commit Checkpoint
\`feat(network): introduce collision-free dynamic port allocator per grid\`"

# Task 1.6
create_task_issue \
  "Task 1.6: Interface de Revisão e Fusão Segura (Diff & Merge View) (Módulo 2)" \
  "phase:1-workspace,priority:p1-high,agent:frontend" \
  "### Objetivo
Desenvolver o ecrã de revisão de alterações de uma grelha terminada, apresentando o diff detalhado de ficheiros alterados e opções de Squash & Merge ou Rebase Merge para a branch principal, com acionamento do Orquestrador em caso de conflitos.

### Ficheiros
- \`src/views/MergeReviewView.tsx\`
- \`src/components/DiffViewer.tsx\`
- \`src/components/MergeConflictResolver.tsx\`
- \`src/hooks/useGitDiff.ts\`

### Quality Gate
Renderização fluida de diffs com sintaxe colorida e suporte a atalhos de confirmação/rejeição.

### Commit Checkpoint
\`feat(ui): create visual git diff review and safe merge modal\`"

# Task 2.1
create_task_issue \
  "Task 2.1: Motor PTY Multiplexer em Rust com Buffer Circular (Módulo 3)" \
  "phase:2-grid-pty,priority:p0-critical,agent:fullstack" \
  "### Objetivo
Implementar em Rust com \`portable-pty\` a criação de processos shell locais (zsh/bash), canais assíncronos bidirecionais (PTY write/read) e um ring-buffer de scrollback em memória (100.000 linhas) por sessão, emitindo eventos IPC para o frontend com alta eficiência.

### Ficheiros
- \`src-tauri/src/terminal/pty.rs\`
- \`src-tauri/src/terminal/manager.rs\`
- \`src-tauri/src/terminal/ring_buffer.rs\`
- \`src-tauri/src/commands/terminal.rs\`

### Quality Gate
Concorrência de 16 PTYs abertos em simultâneo com throughput de 10MB/s de saída sem bloqueio de I/O na thread principal.

### Commit Checkpoint
\`feat(pty): build high-throughput pty multiplexer and ring buffer in rust\`"

echo ""
echo "✅ Script concluído! Todas as tarefas e labels estão parametrizadas."

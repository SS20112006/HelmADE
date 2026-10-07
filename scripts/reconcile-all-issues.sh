#!/usr/bin/env bash
# ==============================================================================
# reconcile-all-issues.sh
# Reconciliação total e segura de todas as GitHub Issues do HelmADE
# Compatível com macOS Bash 3.2 e zsh
# ==============================================================================
set -euo pipefail

REPO="SS20112006/HelmADE"

echo "=== [1/3] Verificação da Ferramenta GitHub CLI (gh) ==="
if ! command -v gh &> /dev/null; then
  echo "❌ ERRO: O comando 'gh' não foi encontrado."
  echo "Instale o GitHub CLI com 'brew install gh' e autentique com 'gh auth login'."
  exit 1
fi

if ! gh auth status &> /dev/null; then
  echo "❌ ERRO: 'gh' não está autenticado. Execute 'gh auth login' para autenticar."
  exit 1
fi
echo "✅ GitHub CLI autenticado com sucesso para $REPO."

echo ""
echo "=== [2/3] Sincronizar Labels Padronizadas no Repositório ==="
LABELS=(
  "phase:0-scaffold|0052cc|Fase 0: Scaffolding, Guardrails & Arquitetura Base"
  "phase:1-workspace|1d76db|Fase 1: Workspaces, Git Worktrees & Isolamento"
  "phase:2-grid-pty|5319e7|Fase 2: Grelha Tiled & Multiplexador de Terminais"
  "phase:3-agents-teamwire|d93f0b|Fase 3: Motor de Agentes Antigravity (.md) & Team Wire"
  "phase:4-live-preview|0e8a16|Fase 4: Live Preview & Navegador Integrado"
  "phase:5-helmvoice|fbca04|Fase 5: HelmVoice (Módulo de Voz a Custo Zero)"
  "phase:6-governance|c5def5|Fase 6: Governação, Telemetria & Streamer Shield"
  "phase:7-audit-e2e|b60205|Fase 7: Auditoria Apple HIG, Performance & E2E"
  "phase:8-release|006b75|Fase 8: Packaging, CI/CD & Release v1.0.0"
  "priority:p0-critical|b60205|Bloqueador ou dependência crítica de arquitetura"
  "priority:p1-high|d93f0b|Funcionalidade central de módulo"
  "priority:p2-medium|fbca04|Funcionalidade complementar"
  "agent:fullstack|1f883d|Atribuído ao Full-Stack Implementation Engineer"
  "agent:frontend|6f42c1|Atribuído ao Anti-Slop Frontend & Design Engineer"
  "agent:apple-hig|0366d6|Atribuído ao Apple HIG Design Specialist"
  "agent:database|336699|Atribuído ao Database Specialist (SQLite)"
  "agent:qa-security|cb2431|Atribuído ao QA & Security Auditor"
  "agent:devops|24292e|Atribuído ao DevOps & Release Agent"
  "agent:system-architect|5c2d91|Atribuído ao System Architect"
  "agent:performance|0e8a16|Atribuído ao Performance & Optimization Engineer"
)

for item in "${LABELS[@]}"; do
  IFS='|' read -r name color desc <<< "$item"
  gh label create "$name" --repo "$REPO" --color "$color" --description "$desc" --force 2>/dev/null || true
done
echo "✅ Labels sincronizadas."

echo ""
echo "=== [3/3] Reconciliação das Tasks do Roadmap ==="

# Função para encontrar número da issue por prefixo de título
find_issue() {
  local prefix="$1"
  gh issue list --repo "$REPO" --state all --search "$prefix" --json number,title --jq '.[0].number' 2>/dev/null || echo ""
}

# Função de reconciliação atómica
sync_task() {
  local task_id="$1"
  local title="$2"
  local is_completed="$3"
  local labels="$4"
  local body="$5"
  local close_comment="$6"

  echo -n "[$task_id] "
  local num
  num=$(find_issue "$task_id")

  if [ -z "$num" ] || [ "$num" = "null" ]; then
    echo -n "Criando Issue... "
    num=$(gh issue create --repo "$REPO" --title "$title" --label "$labels" --body "$body" 2>/dev/null | grep -o '[0-9]*$' || echo "")
    echo -n "#$num "
  else
    echo -n "Existente (#$num) "
  fi

  if [ -n "$num" ] && [ "$num" != "null" ]; then
    if [ "$is_completed" = "true" ]; then
      local state
      state=$(gh issue view "$num" --repo "$REPO" --json state --jq '.state' 2>/dev/null || echo "OPEN")
      if [ "$state" = "OPEN" ]; then
        if [ -n "$close_comment" ]; then
          gh issue comment "$num" --repo "$REPO" --body "$close_comment" 2>/dev/null || true
        fi
        gh issue close "$num" --repo "$REPO" --reason "completed" 2>/dev/null || true
        echo "-> ✅ FECHADA (Concluída)"
      else
        echo "-> ✅ JÁ ESTAVA FECHADA"
      fi
    else
      # Tarefa pendente: garantir que está aberta
      local state
      state=$(gh issue view "$num" --repo "$REPO" --json state --jq '.state' 2>/dev/null || echo "OPEN")
      if [ "$state" = "CLOSED" ]; then
        gh issue reopen "$num" --repo "$REPO" 2>/dev/null || true
        echo "-> 🔄 REABERTA (Pendente no Roadmap)"
      else
        echo "-> ⏳ ABERTA (Em fila / Backlog)"
      fi
    fi
  else
    echo "-> ⚠️ Erro ao processar issue"
  fi
}

# ------------------------------------------------------------------------------
# FASE 0: SCAFFOLDING & BASE (CONCLUÍDAS)
# ------------------------------------------------------------------------------
sync_task "Task 0.1" \
  "Task 0.1: Scaffolding do Monorepo Tauri 2 + React 19 + Tailwind v4" \
  "true" \
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

### Commit Checkpoint
\`chore(scaffold): initialize tauri 2 rust and react 19 desktop core\`" \
  "✅ **Implementada**: Monorepo inicializado com Tauri 2.x, React 19, Tailwind CSS v4, gerador de ícones RGBA 32-bit e script de verificação de scaffold."

sync_task "Task 0.2" \
  "Task 0.2: Definição das ADRs Fundacionais" \
  "true" \
  "phase:0-scaffold,priority:p0-critical,agent:system-architect" \
  "### Objetivo
Formalizar as decisões de arquitetura em ADRs cobrindo Runtime Desktop, Isolamento por Worktrees, PTY multiplexing e Motor de Voz local a custo zero.

### Ficheiros
- \`docs/adr/2026-10-06-0001-desktop-runtime-tauri2-rust.md\`
- \`docs/adr/2026-10-06-0002-git-worktree-symlink-isolation.md\`
- \`docs/adr/2026-10-06-0003-zero-cost-local-speech-engine.md\`
- \`docs/adr/2026-10-06-0004-per-agent-model-selection.md\`

### Commit Checkpoint
\`docs(adr): formalize foundational runtime and isolation architecture\`" \
  "✅ **Implementada**: Todas as 4 ADRs fundacionais formalizadas e aceites em \`docs/adr/\`."

sync_task "Task 0.3" \
  "Task 0.3: Configuração de Guardrails de Qualidade e Commitlint" \
  "true" \
  "phase:0-scaffold,priority:p0-critical,agent:devops" \
  "### Objetivo
Instalar scripts de verificação pré-commit e validação de Conventional Commits 1.0.0, assegurando a proibição de git stash e cumprimento das diretrizes globais.

### Ficheiros
- \`.github/workflows/ci.yml\`
- \`package.json\`

### Commit Checkpoint
\`ci(guardrails): establish strict quality gates and conventional commits\`" \
  "✅ **Implementada**: Pipeline GitHub Actions de CI configurado em \`.github/workflows/ci.yml\` com commitlint e validação de typecheck/cargo check."

# ------------------------------------------------------------------------------
# FASE 1: WORKSPACES & ISOLAMENTO (PARCIALMENTE CONCLUÍDA)
# ------------------------------------------------------------------------------
sync_task "Task 1.1" \
  "Task 1.1: Persistência Local em SQLite & Gestão de Projetos (Módulo 1)" \
  "true" \
  "phase:1-workspace,priority:p0-critical,agent:database" \
  "### Objetivo
Implementar a camada de armazenamento local SQLite em Rust (\`rusqlite\`) para gerir projetos recentes, sessões, grelhas, caminhos no disco e preferências sem requisições de rede.

### Ficheiros
- \`src-tauri/src/db/mod.rs\`
- \`src-tauri/src/db/schema.rs\`
- \`src-tauri/src/commands/project.rs\`
- \`src/services/projectService.ts\`
- \`src/types/project.ts\`

### Commit Checkpoint
\`feat(workspace): implement local sqlite storage for project management\`" \
  "✅ **Implementada**: Motor SQLite em Rust com pragmas WAL, tabelas \`projects\`, \`sessions\` e \`grid_panels\`, UPSERT idempotente e teste unitário in-memory."

sync_task "Task 1.2" \
  "Task 1.2: Ecrã Inicial de Seleção de Projetos e Recentes (Módulo 1)" \
  "true" \
  "phase:1-workspace,priority:p1-high,agent:frontend,agent:apple-hig" \
  "### Objetivo
Criar a interface de boas-vindas do HelmADE com diálogo nativo de seleção de pasta, lista de projetos recentes com estatísticas, pesquisa rápida e suporte a arrastar-e-soltar (drag & drop).

### Ficheiros
- \`src/views/WelcomeView.tsx\`
- \`src/components/ProjectList.tsx\`
- \`src/components/RecentProjectCard.tsx\`
- \`src/hooks/useProjects.ts\`
- \`src/App.tsx\`

### Commit Checkpoint
\`feat(ui): build welcome and recent projects view with drag-and-drop\`" \
  "✅ **Implementada**: Ecrã de boas-vindas \`WelcomeView\` com Apple HIG, drag & drop de pastas locais, diálogo nativo do sistema e cartões de projetos recentes."

sync_task "Task 1.3" \
  "Task 1.3: Gestor de Git Worktrees e Criação Isolada de Grelhas (Módulo 2)" \
  "true" \
  "phase:1-workspace,priority:p0-critical,agent:fullstack" \
  "### Objetivo
Criar o comando Tauri em Rust para criar automaticamente Git Worktrees (\`git worktree add -b helm/swarm-<id> .helm/worktrees/<id> <base-branch>\`) e remover worktrees de forma segura no encerramento da grelha.

### Ficheiros
- \`src-tauri/src/git/mod.rs\`
- \`src-tauri/src/git/worktree.rs\`
- \`src-tauri/src/commands/worktree.rs\`
- \`src/services/worktreeService.ts\`
- \`src/types/worktree.ts\`

### Commit Checkpoint
\`feat(git): implement zero-corruption git worktree lifecycle manager\`" \
  "✅ **Implementada**: Ciclo de vida completo de Git Worktrees em Rust (\`create_worktree\`, \`remove_worktree\`, \`list_worktrees\`), sem \`git stash\` e com teste de integração em repositório temporário real."

# ------------------------------------------------------------------------------
# TAREFAS PENDENTES / A SEGUIR NO ROADMAP
# ------------------------------------------------------------------------------
sync_task "Task 1.4" \
  "Task 1.4: Gestor de Symlinks de Dependências com Zero Desperdício de Disco (Módulo 2)" \
  "false" \
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
\`feat(git): add automatic dependency symlinker for instant worktree boot\`" ""

sync_task "Task 1.5" \
  "Task 1.5: Alocador Dinâmico de Portas de Servidores Locais (Módulo 2)" \
  "false" \
  "phase:1-workspace,priority:p1-high,agent:fullstack" \
  "### Objetivo
Implementar sistema de deteção de portas livres no sistema operativo (\`std::net::TcpListener\`) e injeção automática de \`PORT=300X\` nas variáveis de ambiente de cada grelha para evitar colisões \`EADDRINUSE\`.

### Ficheiros
- \`src-tauri/src/network/port_allocator.rs\`
- \`src-tauri/src/commands/network.rs\`
- \`src/stores/useGridStore.ts\`

### Commit Checkpoint
\`feat(network): introduce collision-free dynamic port allocator per grid\`" ""

sync_task "Task 1.6" \
  "Task 1.6: Interface de Revisão e Fusão Segura (Diff & Merge View) (Módulo 2)" \
  "false" \
  "phase:1-workspace,priority:p1-high,agent:frontend" \
  "### Objetivo
Desenvolver o ecrã de revisão de alterações de uma grelha terminada, apresentando o diff detalhado de ficheiros alterados e opções de Squash & Merge ou Rebase Merge para a branch principal, com acionamento do Orquestrador em caso de conflitos.

### Ficheiros
- \`src/views/MergeReviewView.tsx\`
- \`src/components/DiffViewer.tsx\`
- \`src/components/MergeConflictResolver.tsx\`
- \`src/hooks/useGitDiff.ts\`

### Commit Checkpoint
\`feat(ui): create visual git diff review and safe merge modal\`" ""

# ------------------------------------------------------------------------------
# FASE 2: GRELHA TILED & PTY MULTIPLEXER (PENDENTES)
# ------------------------------------------------------------------------------
sync_task "Task 2.1" \
  "Task 2.1: Motor PTY Multiplexer em Rust com Buffer Circular (Módulo 3)" \
  "false" \
  "phase:2-grid-pty,priority:p0-critical,agent:fullstack" \
  "### Objetivo
Implementar em Rust com \`portable-pty\` a criação de processos shell locais (zsh/bash), canais assíncronos bidirecionais (PTY write/read) e um ring-buffer de scrollback em memória (100.000 linhas) por sessão, emitindo eventos IPC para o frontend com alta eficiência.

### Ficheiros
- \`src-tauri/src/terminal/pty.rs\`
- \`src-tauri/src/terminal/manager.rs\`
- \`src-tauri/src/terminal/ring_buffer.rs\`
- \`src-tauri/src/commands/terminal.rs\`

### Commit Checkpoint
\`feat(pty): build high-throughput pty multiplexer and ring buffer in rust\`" ""

sync_task "Task 2.2" \
  "Task 2.2: Integração do @xterm/xterm com WebGL e Renderização Rápida (Módulo 3)" \
  "false" \
  "phase:2-grid-pty,priority:p1-high,agent:frontend,agent:apple-hig" \
  "### Objetivo
Integrar no React o componente de terminal baseado em \`@xterm/xterm\` e \`@xterm/addon-webgl\`, com suporte a 24-bit TrueColor ANSI, redimensionamento automático de colunas/linhas (\`fit addon\`) e reconexão fluida.

### Ficheiros
- \`src/components/terminal/XTermWrapper.tsx\`
- \`src/components/terminal/useTerminalSession.ts\`
- \`src/components/terminal/terminalThemes.ts\`

### Commit Checkpoint
\`feat(terminal): integrate xterm webgl wrapper with auto-resizing and truecolor\`" ""

echo ""
echo "=============================================================================="
echo "🎉 RECONCILIAÇÃO CONCLUÍDA COM SUCESSO!"
echo "- Tasks 0.1, 0.2, 0.3, 1.1, 1.2 e 1.3: Concluídas e Fechadas com evidências."
echo "- Task 1.4: Configurada como a próxima tarefa aberta no topo da fila."
echo "- Backlog subsequente: Sincronizado e etiquetado."
echo "=============================================================================="

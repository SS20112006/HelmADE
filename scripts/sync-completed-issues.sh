#!/usr/bin/env bash
# ==============================================================================
# sync-completed-issues.sh
# Cria as issues caso não existam e fecha as tarefas já implementadas
# ==============================================================================
set -euo pipefail

cd "$(dirname "$0")/.."

echo "=== [1/2] Sincronizar criação de Issues se necessário ==="
bash scripts/sync-roadmap-issues.sh || true

echo ""
echo "=== [2/2] Atualizar estado das Tasks concluídas ==="

bash scripts/update-issue-status.sh "Task 0.1" "closed" "✅ Concluído: Scaffolding inicial Tauri 2 + React 19 + Tailwind v4." || true
bash scripts/update-issue-status.sh "Task 0.2" "closed" "✅ Concluído: ADRs fundacionais 0001, 0002, 0003 e 0004 criadas em docs/adr/." || true
bash scripts/update-issue-status.sh "Task 0.3" "closed" "✅ Concluído: Pipelines GitHub Actions CI configurados com Conventional Commits e Typecheck." || true
bash scripts/update-issue-status.sh "Task 1.1" "closed" "✅ Concluído: Persistência local SQLite (rusqlite), CRUD de projetos e testes unitários." || true
bash scripts/update-issue-status.sh "Task 1.2" "closed" "✅ Concluído: Ecrã inicial WelcomeView com Apple HIG, Drag & Drop e lista de projetos recentes." || true
bash scripts/update-issue-status.sh "Task 1.3" "closed" "✅ Concluído: Gestor de ciclo de vida de Git Worktrees nativo em Rust com isolamento por enxame." || true
bash scripts/update-issue-status.sh "Task 1.4" "closed" "✅ Concluído: Ligação simbólica automática de dependências (node_modules, .venv, target) para worktrees." || true
bash scripts/update-issue-status.sh "Task 1.5" "closed" "✅ Concluído: Alocador dinâmico de portas de rede TCP em Rust prevenindo colisões EADDRINUSE." || true
bash scripts/update-issue-status.sh "Task 1.6" "closed" "✅ Concluído: Interface de revisão de Git Diff, visualizador com Apple HIG e modal de fusão segura (Squash & Rebase)." || true

echo ""
echo "🎉 Sincronização e atualização de Issues no GitHub concluída!"

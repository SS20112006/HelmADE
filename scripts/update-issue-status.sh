#!/usr/bin/env bash
# ==============================================================================
# update-issue-status.sh
# Atualiza e fecha issues no GitHub à medida que as tarefas são concluídas
# Uso: bash scripts/update-issue-status.sh "<task-id>" "<status>" "<comment>"
# Exemplo: bash scripts/update-issue-status.sh "Task 1.1" "closed" "Implementado via commit XYZ"
# ==============================================================================
set -euo pipefail

REPO="SS20112006/HelmADE"

if ! command -v gh &> /dev/null; then
  echo "⚠️ 'gh' CLI não encontrado. Instale com 'brew install gh' e autentique com 'gh auth login'."
  exit 0
fi

TASK_PREFIX="${1:-}"
STATUS="${2:-closed}" # closed ou open
COMMENT="${3:-Tarefa concluída com sucesso e verificada pelos quality gates.}"

if [ -z "$TASK_PREFIX" ]; then
  echo "Uso: bash scripts/update-issue-status.sh <Task-Prefix, ex: 'Task 1.1'> [closed|comment] [comentário]"
  exit 1
fi

echo "A procurar Issue correspondente a '$TASK_PREFIX' no repositório $REPO..."
ISSUE_NUM=$(gh issue list --repo "$REPO" --search "$TASK_PREFIX" --json number --jq '.[0].number' 2>/dev/null || true)

if [ -n "$ISSUE_NUM" ] && [ "$ISSUE_NUM" != "null" ]; then
  echo "Encontrada Issue #$ISSUE_NUM para '$TASK_PREFIX'."
  
  if [ -n "$COMMENT" ]; then
    gh issue comment "$ISSUE_NUM" --repo "$REPO" --body "$COMMENT" || true
  fi

  if [ "$STATUS" = "closed" ]; then
    gh issue close "$ISSUE_NUM" --repo "$REPO" --reason "completed" || true
    echo "✅ Issue #$ISSUE_NUM fechada com sucesso!"
  fi
else
  echo "ℹ️ Nenhuma Issue encontrada para '$TASK_PREFIX'. Execute 'bash scripts/sync-roadmap-issues.sh' para sincronizar todas as issues primeiro."
fi

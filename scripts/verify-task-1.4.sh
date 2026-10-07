#!/usr/bin/env bash
# HelmADE Task 1.4 — Dependency Symlinks Lifecycle Verification
set -euo pipefail

cd "$(dirname "$0")/.."

# Auto-carregar toolchain Rust caso esteja no caminho padrão ~/.cargo/bin
if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck source=/dev/null
  source "$HOME/.cargo/env"
elif [ -d "$HOME/.cargo/bin" ]; then
  export PATH="$HOME/.cargo/bin:$PATH"
fi

echo "=== [1/3] Frontend: Typecheck ==="
npm run typecheck

echo ""
echo "=== [2/3] Backend: Rust Cargo Check ==="
python3 scripts/generate-dev-icons.py
cd src-tauri
cargo check --verbose

echo ""
echo "=== [3/3] Backend: Rust Unit & Integration Tests (Symlinks + Worktrees) ==="
cargo test git::symlink::tests -- --nocapture
cargo test git::worktree::tests -- --nocapture
cd ..

echo ""
echo "✅ TASK 1.4 VERIFIED SUCCESSFULLY"
echo "Commit sugerido:"
echo "  git add src-tauri/ src/ scripts/ docs/ .superpowers/"
echo "  git commit -m \"feat(git): add automatic dependency symlinker for instant worktree boot\""

#!/usr/bin/env bash
# HelmADE Task 1.6 — Visual Git Diff Review & Safe Merge Verification
set -euo pipefail

cd "$(dirname "$0")/.."

# Auto-carregar toolchain Rust caso esteja no caminho padrão ~/.cargo/bin
if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck source=/dev/null
  source "$HOME/.cargo/env"
elif [ -d "$HOME/.cargo/bin" ]; then
  export PATH="$HOME/.cargo/bin:$PATH"
fi

echo "=== [1/4] Frontend: Typecheck ==="
npm run typecheck

echo ""
echo "=== [2/4] Frontend: Production Build ==="
npm run build

echo ""
echo "=== [3/4] Backend: Rust Cargo Check ==="
python3 scripts/generate-dev-icons.py
cd src-tauri
cargo check --verbose

echo ""
echo "=== [4/4] Backend: Rust Unit & Integration Tests (Diff & Safe Merge) ==="
cargo test git::diff::tests -- --nocapture
cd ..

echo ""
echo "✅ TASK 1.6 VERIFIED SUCCESSFULLY"
echo "Commit sugerido:"
echo "  git add src-tauri/ src/ scripts/ docs/ .superpowers/"
echo "  git commit -m \"feat(ui): create visual git diff review and safe merge modal\""

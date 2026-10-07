#!/usr/bin/env bash
# HelmADE Task 1.1 — Local SQLite & Project Management Verification
set -euo pipefail

cd "$(dirname "$0")/.."

# Auto-carregar toolchain Rust caso esteja no caminho padrão ~/.cargo/bin
if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck source=/dev/null
  source "$HOME/.cargo/env"
elif [ -d "$HOME/.cargo/bin" ]; then
  export PATH="$HOME/.cargo/bin:$PATH"
fi

if ! command -v cargo &>/dev/null; then
  echo ""
  echo "❌ ERRO: O comando 'cargo' não foi encontrado no seu PATH."
  echo "Para instalar o toolchain Rust oficial (necessário para o backend Tauri):"
  echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
  echo "Depois de instalar ou se já estiver instalado, recarregue a shell com:"
  echo "  source \"\$HOME/.cargo/env\""
  exit 1
fi

# Gerar ícones RGBA 32-bit de desenvolvimento válidos usando Python 3 (nativo macOS)
python3 scripts/generate-dev-icons.py

echo ""
echo "=== [1/3] Frontend: Typecheck ==="
npm run typecheck

echo ""
echo "=== [2/3] Backend: Rust Cargo Check ==="
cd src-tauri
cargo check --verbose

echo ""
echo "=== [3/3] Backend: Rust Unit Tests (SQLite DB CRUD) ==="
cargo test db::tests -- --nocapture
cd ..

echo ""
echo "✅ TASK 1.1 VERIFIED SUCCESSFULLY"
echo "Commit sugerido:"
echo "  git add src-tauri/ Cargo.toml src/ package.json .github/ scripts/ docs/ .superpowers/"
echo "  git commit -m \"feat(workspace): implement local sqlite storage for project management\""

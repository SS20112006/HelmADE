#!/usr/bin/env bash
# HelmADE Task 0.1 — Quality Gate Script
# Corre: bash scripts/verify-scaffold.sh
set -euo pipefail

cd "$(dirname "$0")/.."

echo "=== [1/3] npm install ==="
npm install

echo ""
echo "=== [2/3] Frontend: typecheck + build ==="
npm run typecheck
npm run build

echo ""
echo "=== Verificar tokens CSS no bundle ==="
if grep -r 'sf-primary' dist/assets/*.css 2>/dev/null; then
  echo "✅ Design tokens Apple HIG presentes no bundle"
else
  echo "❌ FALHA: tokens sf-primary não encontrados no CSS de produção"
  exit 1
fi

echo ""
echo "=== [3/3] Rust: cargo check ==="
cd src-tauri
cargo check
cd ..

echo ""
echo "✅ TODOS OS QUALITY GATES PASSARAM"
echo "Podes fazer o commit:"
echo "  git add src-tauri/ src/ index.html package.json vite.config.ts tsconfig*.json .gitignore Cargo.toml scripts/"
echo "  git commit -m \"chore(scaffold): initialize tauri 2 rust and react 19 desktop core\""

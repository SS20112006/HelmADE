# 🧠 Project Brain: HelmADE
**Living Memory, Architecture Invariants & Operating Ledger**

---

## 1. Identidade & Filosofia
* **Nome do Projeto**: HelmADE (The Zero-Cost Agent Development Environment & Autonomous Swarm Cockpit)
* **Propósito**: Cockpit nativo de comando para desenvolvimento com enxames (*swarms*) de agentes IA em paralelo. O utilizador atua como Arquiteto e Diretor de Produto guiando agentes por texto e voz.
* **Stack Principal**: Tauri 2.x (Rust) + React 19 + TypeScript + Vite + Tailwind CSS v4 + `@xterm/xterm` + SQLite local (`rusqlite`).

---

## 2. Invariantes Sagrados do Projeto
1. **Custo Absoluto ZERO (0.00€)**:
   - 100% da orquestração, bases de dados, terminais, isolamento e reconhecimento de voz funcionam localmente.
   - Nenhuma dependência de serviços cloud pagos ou subscrições proprietárias.
2. **Orquestração Baseada em `.md` (Antigravity)**:
   - Personas e competências carregadas diretamente dos ficheiros `.md` (`~/.gemini/config/agents/` ou diretório do projeto).
   - Suporte ao papel de Orquestrador Líder e agentes especialistas.
3. **Concorrência Segura sem Corrupção de Código**:
   - Cada grelha opera estritamente no seu próprio **Git Worktree** (`helm/swarm-*`).
   - Symlinks automáticos de pastas pesadas de dependências (`node_modules/`, `.venv/`).
   - Proibição absoluta de editar a mesma pasta em simultâneo por múltiplos agentes sem Worktree.
4. **Design & Tipografia Apple HIG**:
   - Família San Francisco obrigatória: **SF Pro Text** (`<= 19pt`) com tracking ótico vs **SF Pro Display** (`>= 20pt`).
   - Contraste WCAG 2.1 AA mínimo (4.5:1 texto padrão, 3:1 texto grande).
   - Touch targets de pelo menos 44x44px. Materiais nativos de transparência macOS (`backdrop-blur`).
5. **Economia de Código (Ponytail) & Verificação Rigorosa (Superpowers)**:
   - YAGNI, standard library antes de bibliotecas, diffs cirúrgicos mínimos.
   - The Iron Law of Verification: zero asserções de conclusão sem evidências frescas de execução.
   - Proibição estrita de `git stash` em fluxos multi-agente.

---

## 3. Matriz de Agentes & Papéis do Sistema

| Agente | Foco Operacional | Ferramentas & Skills Primárias |
| :--- | :--- | :--- |
| `system-architect` | Modelação C4/Mermaid, arquitetura de sandboxes, contratos OpenAPI e MADR | `architectural-decision-record`, `api-design-openapi` |
| `fullstack-engineer` | Implementação de lógica Rust/React, PTY multiplexer, IPC, SQLite | `ponytail`, `test-driven-development`, `error-handling` |
| `frontend-design-specialist` | UI de alta densidade, Tailwind v4, layouts flexíveis, componentes | `design-taste-frontend`, `design-system-catalog` |
| `apple-hig-specialist` | Conformidade com macOS HIG, SF Pro typography, WCAG AA, haptics | `apple-hig-auditor`, `a11y-auditor` |
| `database-specialist` | Schemas SQLite locais, índices, queries otimizadas, lock timeouts | `schema-and-db-migration` |
| `qa-security-auditor` | AST checks, tipagem estrita, Playwright E2E, Streamer Shield masking | `strict-type-verifier`, `production-readiness-verifier` |
| `performance-engineer` | Profiling de CPU/RAM em terminais concorrentes, ring buffers | `performance-profiling-benchmark`, `latency-critical-systems` |
| `complexity-auditor` | Eliminação de código morto, auditoria de dependências, Ponytail audit | `ponytail-review`, `ponytail-audit`, `ponytail-debt` |
| `devops-release` | Commits atómicos Conventional Commits 1.0.0, worktrees, packaging | `semantic-git-workflow`, `ci-cd-workflow-automation` |
| `silent-failure-hunter` | Caça a erros engolidos, bare catch blocks, floating promises | `error-handling`, `safety-guard` |
| `build-error-resolver` | Diffs cirúrgicos mínimos para corrigir falhas de build/compilação | `strict-type-verifier` |
| `sre-observability-engineer` | Telemetria de tokens, métricas USE/RED, exportação de logs | `latency-critical-systems` |

---

## 4. Livro de Decisões Arquiteturais (ADRs)
* `docs/adr/2026-10-06-0001-desktop-runtime-tauri2-rust.md`: Adoção do Tauri 2.x com Rust para suporte nativo macOS, gestão de PTYs a custo zero e pegada de memória mínima (<50MB).
* `docs/adr/2026-10-06-0002-git-worktree-symlink-isolation.md`: Isolamento de grelhas por Git Worktree + symlink inteligente de dependências para prevenir duplicação de disco.
* `docs/adr/2026-10-06-0003-zero-cost-local-speech-engine.md`: Uso do Apple Speech.framework nativo do macOS via FFI/Objective-C para transcrição offline a custo 0.00€.
* `docs/adr/2026-10-06-0004-per-agent-model-selection.md`: Seleção dinâmica de modelo de IA por agente com perfis cognitivos (Deep Thinking, Research, Fast/Economic e Local 0.00€ via Ollama).

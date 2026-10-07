# 🗺️ HelmADE: Master Implementation Roadmap
> **The Zero-Cost Agent Development Environment (ADE) & Autonomous Swarm Cockpit**
> Versão 1.0 — Guia de Execução de Extremo-a-Extremo

---

## 🎯 Sumário Executivo & Princípios de Orquestração

Este documento define o plano integral de implementação do **HelmADE**, cobrindo da fundação até à versão 1.0 pronta para produção. Cada tarefa foi desenhada para ser executada de forma atómica e verificável através de subagentes especializados, aplicando as skills de maior afinidade, com salvaguarda contínua no Git através de Conventional Commits 1.0.0.

### 🛡️ Os Três Invariantes do Projeto
1. **Custo Absoluto ZERO (0.00€)**: Sem cloud paga, sem subscrições, 100% de execução local.
2. **Orquestração Baseada em `.md`**: Agentes definidos por personas Markdown (Antigravity).
3. **Concorrência Segura**: Isolamento total de enxames através de Git Worktrees e symlinks.

---

## 🏗️ Stack Tecnológica de Alta Performance

* **Core Desktop / Backend**: **Tauri 2.x** (Rust)
  - Pegada de memória ultra-reduzida (<50MB vs >500MB do Electron).
  - Gestão nativa de PTYs com `portable-pty`.
  - Acesso direto a APIs macOS (`Speech.framework` via FFI para custo 0.00€).
  - Base de dados local embutida com `rusqlite` (SQLite).
* **Frontend Cockpit**: **React 19** + **TypeScript** + **Vite**
  - Estilização: **Tailwind CSS v4** + Design Tokens Apple HIG (`backdrop-blur`, semantic materials).
  - Emulador de Terminais: `@xterm/xterm` + `@xterm/addon-fit` + `@xterm/addon-webgl`.
  - Estado & Reatividade: **Zustand** (leve e sem boilerplate) + **Lucide React**.
* **Testes & Qualidade**: Vitest, Playwright E2E, Cargo Test, AST Linters.

---

## 🧭 Fases de Implementação & Tarefas Detalhadas

---

### FASE 0: Scaffolding, Guardrails & Arquitetura Base

#### Task 0.1: Scaffolding do Monorepo Tauri 2 + React 19 + Tailwind v4
* **Objetivo**: Inicializar o projeto desktop com Tauri 2.x (Rust backend) e frontend React 19 / TypeScript / Vite com Tailwind CSS v4, configurando o manifesto base e janelas transparentes com suporte a vibração nativa macOS.
* **Ficheiros**: `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `src-tauri/src/main.rs`, `package.json`, `vite.config.ts`, `tailwind.config.js`, `src/index.css`, `src/App.tsx`.
* **Melhores Agentes**:
  - *Primário*: `fullstack-engineer`
  - *Secundário/Review*: `build-error-resolver`
* **Melhores Skills**: `vite-patterns`, `react-patterns`, `rust-patterns`, `project-guardrail-scaffolder`.
* **Quality Gate**: `cargo check` passa a verde; `npm run build` compila sem erros de tipo.
* **Commit Checkpoint**: `chore(scaffold): initialize tauri 2 rust and react 19 desktop core`

#### Task 0.2: Definição das ADRs Fundacionais
* **Objetivo**: Formalizar as decisões de arquitetura em ADRs cobrindo Runtime Desktop, Isolamento por Worktrees, PTY multiplexing e Motor de Voz local a custo zero.
* **Ficheiros**: `docs/adr/2026-10-06-0001-desktop-runtime-tauri2-rust.md`, `docs/adr/2026-10-06-0002-git-worktree-symlink-isolation.md`, `docs/adr/2026-10-06-0003-zero-cost-local-speech-engine.md`.
* **Melhores Agentes**:
  - *Primário*: `system-architect`
  - *Secundário/Review*: `complexity-auditor`
* **Melhores Skills**: `architectural-decision-record`, `living-docs-governance`, `ponytail`.
* **Quality Gate**: ADRs estruturadas sem placeholders ou indefinições.
* **Commit Checkpoint**: `docs(adr): formalize foundational runtime and isolation architecture`

#### Task 0.3: Configuração de Guardrails de Qualidade e Commitlint
* **Objetivo**: Instalar scripts de verificação pré-commit e validação de Conventional Commits 1.0.0, assegurando a proibição de `git stash` e cumprimento das diretrizes globais.
* **Ficheiros**: `.github/workflows/ci.yml`, `package.json`, `tsconfig.json`.
* **Melhores Agentes**:
  - *Primário*: `devops-release`
  - *Secundário/Review*: `qa-security-auditor`
* **Melhores Skills**: `semantic-git-workflow`, `ci-cd-workflow-automation`, `project-guardrail-scaffolder`.
* **Quality Gate**: Execução de linter e validação de commit com formato convencional.
* **Commit Checkpoint**: `ci(guardrails): establish strict quality gates and conventional commits`

---

### FASE 1: Módulo 1 & 2 — Motor de Workspaces, Git Worktrees & Isolamento

#### Task 1.1: Persistência Local em SQLite & Gestão de Projetos (Módulo 1)
* **Objetivo**: Implementar a camada de armazenamento local SQLite em Rust (`rusqlite`) para gerir projetos recentes, sessões, grelhas, caminhos no disco e preferências sem requisições de rede.
* **Ficheiros**: `src-tauri/src/db/mod.rs`, `src-tauri/src/db/schema.rs`, `src-tauri/src/commands/project.rs`, `src/services/projectService.ts`, `src/types/project.ts`.
* **Melhores Agentes**:
  - *Primário*: `database-specialist`
  - *Secundário/Review*: `fullstack-engineer`
* **Melhores Skills**: `schema-and-db-migration`, `rust-testing`, `error-handling`.
* **Quality Gate**: Testes unitários Rust validando CRUD de projetos e migrações locais SQLite; `cargo test db`.
* **Commit Checkpoint**: `feat(workspace): implement local sqlite storage for project management`

#### Task 1.2: Ecrã Inicial de Seleção de Projetos e Recentes (Módulo 1)
* **Objetivo**: Criar a interface de boas-vindas do HelmADE com diálogo nativo de seleção de pasta, lista de projetos recentes com estatísticas, pesquisa rápida e suporte a arrastar-e-soltar (*drag & drop*).
* **Ficheiros**: `src/views/WelcomeView.tsx`, `src/components/ProjectList.tsx`, `src/components/RecentProjectCard.tsx`, `src/hooks/useProjects.ts`.
* **Melhores Agentes**:
  - *Primário*: `frontend-design-specialist`
  - *Secundário/Review*: `apple-hig-specialist`
* **Melhores Skills**: `design-taste-frontend`, `apple-hig-auditor`, `minimalist-ui`, `high-end-visual-design`.
* **Quality Gate**: Verificação visual Apple HIG, contraste WCAG AA >= 4.5:1, navegação por teclado funcional.
* **Commit Checkpoint**: `feat(ui): build welcome and recent projects view with drag-and-drop`

#### Task 1.3: Gestor de Git Worktrees e Criação Isolada de Grelhas (Módulo 2)
* **Objetivo**: Criar o comando Tauri em Rust para criar automaticamente Git Worktrees (`git worktree add -b helm/swarm-<id> .helm/worktrees/<id> <base-branch>`) e remover worktrees de forma segura no encerramento da grelha.
* **Ficheiros**: `src-tauri/src/git/mod.rs`, `src-tauri/src/git/worktree.rs`, `src-tauri/src/commands/worktree.rs`, `src/services/worktreeService.ts`.
* **Melhores Agentes**:
  - *Primário*: `fullstack-engineer`
  - *Secundário/Review*: `silent-failure-hunter`
* **Melhores Skills**: `rust-patterns`, `using-git-worktrees`, `error-handling`, `safety-guard`.
* **Quality Gate**: Teste de integração Rust criando, listando e limpando um worktree real num repositório temporário.
* **Commit Checkpoint**: `feat(git): implement zero-corruption git worktree lifecycle manager`

#### Task 1.4: Gestor de Symlinks de Dependências com Zero Desperdício de Disco (Módulo 2)
* **Objetivo**: Implementar no motor Rust a ligação simbólica automática de diretórios ignorados no Git (`node_modules/`, `.venv/`, `.target/`, `.cargo/`) entre a pasta principal e a pasta do worktree recém-criado, prevenindo duplicação de gigabytes no disco.
* **Ficheiros**: `src-tauri/src/git/symlink.rs`, `src-tauri/src/commands/symlink.rs`, `src/services/symlinkService.ts`.
* **Melhores Agentes**:
  - *Primário*: `fullstack-engineer`
  - *Secundário/Review*: `complexity-auditor`
* **Melhores Skills**: `ponytail`, `rust-testing`, `error-handling`.
* **Quality Gate**: Teste comprovando a criação do symlink e verificação de que arquivos de dependência são lidos transparentemente na worktree.
* **Commit Checkpoint**: `feat(git): add automatic dependency symlinker for instant worktree boot`

#### Task 1.5: Alocador Dinâmico de Portas de Servidores Locais (Módulo 2)
* **Objetivo**: Implementar sistema de deteção de portas livres no sistema operativo (`std::net::TcpListener`) e injeção automática de `PORT=300X` nas variáveis de ambiente de cada grelha para evitar colisões `EADDRINUSE`.
* **Ficheiros**: `src-tauri/src/network/port_allocator.rs`, `src-tauri/src/commands/network.rs`, `src/stores/useGridStore.ts`.
* **Melhores Agentes**:
  - *Primário*: `fullstack-engineer`
  - *Secundário/Review*: `performance-engineer`
* **Melhores Skills**: `rust-patterns`, `latency-critical-systems`, `ponytail`.
* **Quality Gate**: Teste de alocação sequencial garantindo portas não-colidentes para 16 grelhas paralelas.
* **Commit Checkpoint**: `feat(network): introduce collision-free dynamic port allocator per grid`

#### Task 1.6: Interface de Revisão e Fusão Segura (Diff & Merge View) (Módulo 2)
* **Objetivo**: Desenvolver o ecrã de revisão de alterações de uma grelha terminada, apresentando o diff detalhado de ficheiros alterados e opções de `Squash & Merge` ou `Rebase Merge` para a branch principal, com acionamento do Orquestrador em caso de conflitos.
* **Ficheiros**: `src/views/MergeReviewView.tsx`, `src/components/DiffViewer.tsx`, `src/components/MergeConflictResolver.tsx`, `src/hooks/useGitDiff.ts`.
* **Melhores Agentes**:
  - *Primário*: `frontend-design-specialist`
  - *Secundário/Review*: `devops-release`
* **Melhores Skills**: `design-taste-frontend`, `finishing-a-development-branch`, `apple-hig-auditor`.
* **Quality Gate**: Renderização fluida de diffs com sintaxe colorida e suporte a atalhos de confirmação/rejeição.
* **Commit Checkpoint**: `feat(ui): create visual git diff review and safe merge modal`

---

### FASE 2: Módulo 3 — Grelha Tiled & Multiplexador de Terminais

#### Task 2.1: Motor PTY Multiplexer em Rust com Buffer Circular (Módulo 3)
* **Objetivo**: Implementar em Rust com `portable-pty` a criação de processos shell locais (zsh/bash), canais assíncronos bidirecionais (PTY write/read) e um ring-buffer de scrollback em memória (100.000 linhas) por sessão, emitindo eventos IPC para o frontend com alta eficiência.
* **Ficheiros**: `src-tauri/src/terminal/pty.rs`, `src-tauri/src/terminal/manager.rs`, `src-tauri/src/terminal/ring_buffer.rs`, `src-tauri/src/commands/terminal.rs`.
* **Melhores Agentes**:
  - *Primário*: `fullstack-engineer`
  - *Secundário/Review*: `performance-engineer`
* **Melhores Skills**: `rust-patterns`, `latency-critical-systems`, `content-hash-cache-pattern`.
* **Quality Gate**: Concorrência de 16 PTYs abertos em simultâneo com throughput de 10MB/s de saída sem bloqueio de I/O na thread principal.
* **Commit Checkpoint**: `feat(pty): build high-throughput pty multiplexer and ring buffer in rust`

#### Task 2.2: Integração do `@xterm/xterm` com WebGL e Renderização Rápida (Módulo 3)
* **Objetivo**: Integrar no React o componente de terminal baseado em `@xterm/xterm` e `@xterm/addon-webgl`, com suporte a 24-bit TrueColor ANSI, redimensionamento automático de colunas/linhas (`fit addon`) e reconexão fluida.
* **Ficheiros**: `src/components/terminal/XTermWrapper.tsx`, `src/components/terminal/useTerminalSession.ts`, `src/components/terminal/terminalThemes.ts`.
* **Melhores Agentes**:
  - *Primário*: `frontend-design-specialist`
  - *Secundário/Review*: `apple-hig-specialist`
* **Melhores Skills**: `react-patterns`, `react-performance`, `design-taste-frontend`.
* **Quality Gate**: Renderização instantânea de sequências de escape complexas (`htop`, `vim`, spinners ANSI) a 60fps.
* **Commit Checkpoint**: `feat(terminal): integrate xterm webgl wrapper with auto-resizing and truecolor`

#### Task 2.3: Layouts de Grelha Tiled (1x1 a 4x4) e Zoom/Maximizar em 1 Toque (Módulo 3)
* **Objetivo**: Desenvolver o motor de layout em CSS Grid responsivo com alternância dinâmica de 1 a 16 painéis (`1x1`, `1x2`, `2x2`, `2x3`, `3x3`, `4x4`), suporte a zoom de painel (`Cmd+Enter` ou duplo clique no cabeçalho) e preservação do estado de foco.
* **Ficheiros**: `src/components/grid/TiledTerminalGrid.tsx`, `src/components/grid/GridLayoutControls.tsx`, `src/stores/useGridLayoutStore.ts`.
* **Melhores Agentes**:
  - *Primário*: `frontend-design-specialist`
  - *Secundário/Review*: `apple-hig-specialist`
* **Melhores Skills**: `design-taste-frontend`, `industrial-brutalist-ui`, `apple-hig-auditor`.
* **Quality Gate**: Transição de layout instantânea sem recarregar nem reiniciar o processo PTY dos terminais.
* **Commit Checkpoint**: `feat(ui): implement tiled terminal grid layouts from 1x1 to 4x4 with 1-click zoom`

#### Task 2.4: Cabeçalhos Semânticos, Badges de Cor e Indicadores de Estado (Módulo 3)
* **Objetivo**: Construir a barra superior de cada painel com título editável, ícone do papel do agente, distintivo de cor semântica (Dourado, Azul, Verde, Roxo, Vermelho) e indicador de estado reativo (Ativo, Em Espera, Concluído, Erro).
* **Ficheiros**: `src/components/terminal/TerminalHeader.tsx`, `src/components/terminal/TerminalStatusBadge.tsx`, `src/types/terminal.ts`.
* **Melhores Agentes**:
  - *Primário*: `apple-hig-specialist`
  - *Secundário/Review*: `frontend-design-specialist`
* **Melhores Skills**: `apple-hig-auditor`, `design-taste-frontend`, `minimalist-ui`.
* **Quality Gate**: Cumprimento rigoroso da tipografia SF Pro Text (<=19pt) e alinhamento ergonómico de 44x44px.
* **Commit Checkpoint**: `feat(ui): add semantic terminal panel headers and reactive activity badges`

---

### FASE 3: Módulo 4 & 5 — Motor de Agentes Antigravity (`.md`), Team Wire & Protocolo A2A

#### Task 3.1: Descoberta e Leitura de Personas `.md` no Sistema (Módulo 4)
* **Objetivo**: Desenvolver o motor de scanner em Rust para indexar todos os ficheiros de agentes em `~/.gemini/config/agents/` e na pasta `.helm/agents/` do projeto, extraindo metadados YAML (nome, descrição, ferramentas, skills).
* **Ficheiros**: `src-tauri/src/agents/scanner.rs`, `src-tauri/src/agents/parser.rs`, `src-tauri/src/commands/agents.rs`, `src/services/agentRegistry.ts`, `src/types/agent.ts`.
* **Melhores Agentes**:
  - *Primário*: `fullstack-engineer`
  - *Secundário/Review*: `silent-failure-hunter`
* **Melhores Skills**: `rust-patterns`, `living-docs-governance`, `error-handling`.
* **Quality Gate**: Carregamento de todas as personas sem falhas; teste unitário Rust cobrindo frontmatter YAML inválido.
* **Commit Checkpoint**: `feat(agents): build local md persona scanner and yaml metadata parser`

#### Task 3.2: Modelos de Enxame com 1 Clique (*Swarm Presets*) & Injeção de Contexto (Módulo 4)
* **Objetivo**: Criar catálogo de presets de enxame (ex.: "Full-Stack Swarm 6 Painéis", "Security Audit Swarm 3 Painéis", "Refactor Swarm 2 Painéis"), com injeção automática de variáveis de ambiente, system prompt `.md` e cwd isolado no arranque de cada terminal.
* **Ficheiros**: `src/config/swarmPresets.ts`, `src/components/swarm/SwarmPresetSelector.tsx`, `src/services/swarmLauncher.ts`, `src/stores/useSwarmStore.ts`.
* **Melhores Agentes**:
  - *Primário*: `frontend-design-specialist`
  - *Secundário/Review*: `system-architect`
* **Melhores Skills**: `design-taste-frontend`, `background-agent-orchestration`, `ponytail`.
* **Quality Gate**: Lançamento de um preset de 6 painéis em menos de 800ms com terminais devidamente parametrizados.
* **Commit Checkpoint**: `feat(swarm): implement 1-click swarm preset catalog and transparent context injection`

#### Task 3.3: Protocolo de Troca de Mensagens Inter-Agentes A2A & Espelhamento (Módulo 5)
* **Objetivo**: Implementar o protocolo estruturado de comunicação inter-agentes em Rust/IPC, onde tarefas do Orquestrador e respostas dos especialistas são transmitidas com identificadores e espelhadas nos terminais (`➔ [NOME-DO-AGENTE-B]` / `🠔 [NOME-DO-AGENTE-A]`).
* **Ficheiros**: `src-tauri/src/agents/router.rs`, `src-tauri/src/agents/protocol.rs`, `src-tauri/src/commands/message.rs`, `src/services/a2aProtocol.ts`.
* **Melhores Agentes**:
  - *Primário*: `fullstack-engineer`
  - *Secundário/Review*: `system-architect`
* **Melhores Skills**: `rust-patterns`, `omni-agents-a2a`, `error-handling`.
* **Quality Gate**: Teste de roundtrip de mensagens garantindo ordem estrita e espelhamento sem duplicação de texto.
* **Commit Checkpoint**: `feat(a2a): implement inter-agent message router and terminal visual mirroring`

#### Task 3.4: Painel Lateral "Team Wire" (Feed de Chat dos IAs) (Módulo 5)
* **Objetivo**: Construir a barra lateral do Team Wire exibindo o fluxo cronológico de mensagens entre agentes, avatares estilizados, timestamps, rendering de Markdown limpo e atalho para focar/destacar o terminal do agente ao clicar na mensagem.
* **Ficheiros**: `src/components/teamwire/TeamWireSidebar.tsx`, `src/components/teamwire/MessageBubble.tsx`, `src/components/teamwire/AgentAvatar.tsx`, `src/hooks/useTeamWire.ts`.
* **Melhores Agentes**:
  - *Primário*: `frontend-design-specialist`
  - *Secundário/Review*: `apple-hig-specialist`
* **Melhores Skills**: `design-taste-frontend`, `apple-hig-auditor`, `high-end-visual-design`.
* **Quality Gate**: Scroll suave automático para a mensagem mais recente, realce visual animado do terminal correspondente.
* **Commit Checkpoint**: `feat(ui): build team wire realtime agent communication feed sidebar`

#### Task 3.5: Caixa de Prompt Primário do Orquestrador & Modo Broadcast (Módulo 5)
* **Objetivo**: Integrar no rodapé do Team Wire a caixa de comando principal para o utilizador instruir o Orquestrador diretamente, com seletor de modo `Transmissão (Broadcast)` para enviar sinais ou instruções globais a todos os agentes da grelha em simultâneo.
* **Ficheiros**: `src/components/teamwire/PromptInput.tsx`, `src/components/teamwire/BroadcastToggle.tsx`, `src/stores/useTeamWireStore.ts`.
* **Melhores Agentes**:
  - *Primário*: `frontend-design-specialist`
  - *Secundário/Review*: `apple-hig-specialist`
* **Melhores Skills**: `design-taste-frontend`, `apple-hig-auditor`, `ponytail`.
* **Quality Gate**: Submissão rápida por `Enter`, suporte a `Shift+Enter` para quebra de linha, feedback de envio claro.
* **Commit Checkpoint**: `feat(ui): add orchestrator prompt input box with broadcast mode toggle`

#### Task 3.6: Seletor de Modelo de IA por Agente & Perfis Cognitivos (Módulo 4)
* **Objetivo**: Implementar o sistema de atribuição dinâmica de modelos de IA por agente, permitindo selecionar modelos especializados (Raciocínio Profundo/Extended Thinking para o Orquestrador e QA; Pesquisa e Contexto Amplo para documentação/specs; Modelos Rápidos/Económicos para frontend/refactors; e Modelos Locais via Ollama/LM Studio a Custo Zero). Inclui componente dropdown no cabeçalho de cada painel de terminal, modal de configuração de equipa nos Presets, persistência local no SQLite e injeção transparente das flags/variáveis de ambiente (`--model <id>`) no arranque da sessão PTY.
* **Ficheiros**: `src/components/terminal/ModelSelectorDropdown.tsx`, `src/components/swarm/AgentModelConfigModal.tsx`, `src/config/modelCatalog.ts`, `src-tauri/src/db/schema.rs`, `src-tauri/src/commands/agents.rs`, `src/stores/useAgentConfigStore.ts`.
* **Melhores Agentes**:
  - *Primário*: `frontend-design-specialist`
  - *Secundário/Review*: `fullstack-engineer`
  - *Consultoria Arquitetural*: `system-architect`
* **Melhores Skills**: `omni-models`, `design-taste-frontend`, `apple-hig-auditor`, `rust-patterns`, `schema-and-db-migration`.
* **Quality Gate**: A seleção de modelo persiste localmente e injeta a flag correta na shell de arranque do agente; modelos locais a custo 0.00€ são suportados e validados; interface ergonómica com badges de perfil cognitivo no cabeçalho do painel.
* **Commit Checkpoint**: `feat(agents): implement per-agent ai model selector and cognitive profile routing`

---

### FASE 4: Módulo 6 — Live Preview & Navegador Integrado

#### Task 4.1: Painel de Webview Dockada & Deteção Automática de Portas (Módulo 6)
* **Objetivo**: Implementar o painel de navegador embutido usando a webview nativa do Tauri, acompanhado por um sniffer em Rust que inspeciona saídas de PTY e deteta URLs locais iniciadas (ex.: `http://localhost:300X`), apontando a webview automaticamente sem digitação manual.
* **Ficheiros**: `src-tauri/src/browser/sniffer.rs`, `src-tauri/src/commands/browser.rs`, `src/components/preview/LiveBrowserDock.tsx`, `src/hooks/useAutoPortDetection.ts`.
* **Melhores Agentes**:
  - *Primário*: `fullstack-engineer`
  - *Secundário/Review*: `performance-engineer`
* **Melhores Skills**: `rust-patterns`, `react-patterns`, `error-handling`.
* **Quality Gate**: Deteção de portas comuns (`Vite`, `Next.js`, `Webpack`, `Astro`) em menos de 200ms após log de arranque.
* **Commit Checkpoint**: `feat(preview): create docked webview panel and automatic dev server port sniffer`

#### Task 4.2: Controlos de Navegação, Alternador de Viewport e Consola de Erros (Módulo 6)
* **Objetivo**: Construir a barra de ferramentas do Live Browser com botões de navegação (Retroceder, Avançar, Recarregar), seletor de modo de ecrã (Desktop, Tablet, Mobile) e gaveta com histórico de erros da consola para facilitar cópia direta para o Orquestrador.
* **Ficheiros**: `src/components/preview/BrowserToolbar.tsx`, `src/components/preview/ViewportSelector.tsx`, `src/components/preview/ConsoleDrawer.tsx`.
* **Melhores Agentes**:
  - *Primário*: `frontend-design-specialist`
  - *Secundário/Review*: `apple-hig-specialist`
* **Melhores Skills**: `design-taste-frontend`, `apple-hig-auditor`, `minimalist-ui`.
* **Quality Gate**: Redimensionamento fluído da moldura emulada sem distorção e botão de cópia de logs com 1 clique.
* **Commit Checkpoint**: `feat(ui): add responsive viewport switcher and embedded browser console drawer`

---

### FASE 5: Módulo 7 — HelmVoice: Módulo de Voz a Custo Zero (0.00€)

#### Task 5.1: Bindings Nativos de Reconhecimento de Fala macOS em Rust (Módulo 7)
* **Objetivo**: Implementar em Rust via Objective-C runtime bindings (`objc2`) a integração com o `SFSpeechRecognizer` nativo da Apple (ou alternativa whisper.cpp local compilada para Apple Silicon Neural Engine), garantindo transcrição 100% offline e sem custos de API externa.
* **Ficheiros**: `src-tauri/src/voice/macos_speech.rs`, `src-tauri/src/voice/engine.rs`, `src-tauri/src/commands/voice.rs`, `src-tauri/build.rs`.
* **Melhores Agentes**:
  - *Primário*: `fullstack-engineer`
  - *Secundário/Review*: `performance-engineer`
* **Melhores Skills**: `rust-patterns`, `latency-critical-systems`, `error-handling`.
* **Quality Gate**: Transcrição local de áudio funcional no macOS sem requisições de rede externas (`assert offline`).
* **Commit Checkpoint**: `feat(voice): implement zero-cost local speech recognition via native macos apis`

#### Task 5.2: Gestor de Atalho Global Push-to-Talk & Dicionário de Código (Módulo 7)
* **Objetivo**: Configurar atalho de teclado global do sistema (ex.: `Cmd+Shift+V` ou `Caps Lock`) para ativar escuta enquanto premido, e aplicar regras heurísticas de pós-processamento para termos de programação (`camelCase`, comandos git, rotas, tipos).
* **Ficheiros**: `src-tauri/src/voice/hotkey.rs`, `src-tauri/src/voice/dictionary.rs`, `src/services/voiceService.ts`, `src/components/voice/VoiceOverlay.tsx`.
* **Melhores Agentes**:
  - *Primário*: `fullstack-engineer`
  - *Secundário/Review*: `frontend-design-specialist`
* **Melhores Skills**: `apple-hig-auditor`, `react-patterns`, `ponytail`.
* **Quality Gate**: Latência inferior a 300ms entre soltar a tecla e o texto surgir no campo de entrada ativo.
* **Commit Checkpoint**: `feat(voice): add global push-to-talk hotkey and technical coding vocabulary tuner`

---

### FASE 6: Módulo 8, 9 & 10 — Governação, Telemetria & Streamer Shield

#### Task 6.1: Standby de Grelhas em Segundo Plano & Alertas de Bloqueio `y/N` (Módulo 8)
* **Objetivo**: Implementar mitigação de CPU suspendendo temporariamente processos de grelhas em segundo plano ociosas e criar detetor de bloqueio nos PTYs que analisa padrões de espera por confirmação humana (`[y/N]`, `Are you sure?`), acionando alertas visuais pulsantes no cabeçalho.
* **Ficheiros**: `src-tauri/src/terminal/block_detector.rs`, `src-tauri/src/terminal/throttler.rs`, `src/components/grid/BlockAlertIndicator.tsx`, `src/stores/useGridStore.ts`.
* **Melhores Agentes**:
  - *Primário*: `fullstack-engineer`
  - *Secundário/Review*: `silent-failure-hunter`
* **Melhores Skills**: `rust-patterns`, `latency-critical-systems`, `design-taste-frontend`.
* **Quality Gate**: Alerta visual acionado em menos de 100ms quando um comando pede confirmação de stdin.
* **Commit Checkpoint**: `feat(governance): add background grid cpu throttler and interactive prompt block detector`

#### Task 6.2: Dashboard de Telemetria de Tokens & Custos Reais (Módulo 9)
* **Objetivo**: Construir o painel analítico com registo de tokens de entrada e saída por agente/sessão, estimativa em tempo real de despesas em dólares ($) e euros (€) com base em tabelas de preços de modelos, e métricas de tarefas resolvidas.
* **Ficheiros**: `src/views/UsageDashboardView.tsx`, `src/components/telemetry/CostMetricCard.tsx`, `src/components/telemetry/TokenUsageChart.tsx`, `src/services/pricingTable.ts`.
* **Melhores Agentes**:
  - *Primário*: `frontend-design-specialist`
  - *Secundário/Review*: `sre-observability-engineer`
* **Melhores Skills**: `design-taste-frontend`, `industrial-brutalist-ui`, `apple-hig-auditor`.
* **Quality Gate**: Filtros reativos por data, grelha e agente sem re-renderizações lentas; dados conferem com SQLite.
* **Commit Checkpoint**: `feat(telemetry): build token usage and financial cost estimation analytics dashboard`

#### Task 6.3: Streamer Shield (Mascaramento de Segredos & Modo Apresentação) (Módulo 10)
* **Objetivo**: Implementar o motor de mascaramento em tempo real nos streams de terminal e Team Wire que substitui chaves de API (`sk-...`, `ghp_...`), senhas e ficheiros `.env` por `[SECRET_MASKED]`, com botão de alternância para ocultar caminhos de ficheiros privados em transmissões em direto.
* **Ficheiros**: `src-tauri/src/security/redactor.rs`, `src/components/security/StreamerShieldToggle.tsx`, `src/stores/useSecurityStore.ts`.
* **Melhores Agentes**:
  - *Primário*: `qa-security-auditor`
  - *Secundário/Review*: `fullstack-engineer`
* **Melhores Skills**: `security-review`, `strict-type-verifier`, `rust-testing`.
* **Quality Gate**: Testes com 50 padrões de segredos conhecidos confirmando zero vazamentos nos logs do terminal.
* **Commit Checkpoint**: `feat(security): build streamer shield with realtime secret masking and private path hiding`

---

### FASE 7: Auditoria Apple HIG, Performance, Segurança & Verificação E2E

#### Task 7.1: Auditoria Apple HIG & Refinamento de Acessibilidade
* **Objetivo**: Auditar toda a aplicação para garantir estrita conformidade com Apple HIG: escala ótica SF Pro Text (`<=19pt`) vs SF Pro Display (`>=20pt`), contraste de cores WCAG AA (>= 4.5:1), materiais nativos macOS (`NSVisualEffectView` vibrancy) e alvos táteis de 44x44px.
* **Ficheiros**: `src/index.css`, `src/components/**/*.tsx`, `src/styles/higTokens.ts`.
* **Melhores Agentes**:
  - *Primário*: `apple-hig-specialist`
  - *Secundário/Review*: `qa-security-auditor`
* **Melhores Skills**: `apple-hig-auditor`, `a11y-auditor`, `design-taste-frontend`.
* **Quality Gate**: Relatório de auditoria sem qualquer violação crítica; contraste verificado em modo escuro.
* **Commit Checkpoint**: `style(hig): enforce strict apple human interface guidelines and wcag aa contrast`

#### Task 7.2: Auditoria Ponytail de Economia de Código e Limpeza de Código Morto
* **Objetivo**: Executar varredura em todo o código Rust e TypeScript para eliminar abstrações desnecessárias, interfaces com implementação única, dependências redundantes e comentários obsoletos.
* **Ficheiros**: Repositório completo (`src/`, `src-tauri/`).
* **Melhores Agentes**:
  - *Primário*: `complexity-auditor`
  - *Secundário/Review*: `fullstack-engineer`
* **Melhores Skills**: `ponytail-review`, `ponytail-audit`, `ponytail-debt`.
* **Quality Gate**: Redução líquida de linhas; ledger de débitos Ponytail atualizado.
* **Commit Checkpoint**: `refactor(economy): prune speculative abstractions and dead code via ponytail audit`

#### Task 7.3: Caça a Falhas Silenciosas e Resiliência de Erros
* **Objetivo**: Efetuar auditoria forense à procura de blocos `catch` vazios, promessas órfãs sem tratamento de erro, fallbacks perigosos ou perda de contexto de stack trace nos canais IPC e PTY.
* **Ficheiros**: Repositório completo.
* **Melhores Agentes**:
  - *Primário*: `silent-failure-hunter`
  - *Secundário/Review*: `qa-security-auditor`
* **Melhores Skills**: `error-handling`, `safety-guard`, `strict-type-verifier`.
* **Quality Gate**: Zero blocos de erro engolidos sem tipagem ou registo apropriado.
* **Commit Checkpoint**: `fix(resilience): eliminate swallowed errors and dangling promises across ipc channels`

#### Task 7.4: Suite de Testes E2E com Playwright & Verificação Final
* **Objetivo**: Implementar suite completa de testes automatizados E2E validando os fluxos críticos: criação de projeto, abertura de grelha com worktree isolado, execução de comandos nos terminais, comunicação no Team Wire e fusão de diffs.
* **Ficheiros**: `tests/e2e/workspace.spec.ts`, `tests/e2e/grid.spec.ts`, `tests/e2e/teamwire.spec.ts`, `playwright.config.ts`.
* **Melhores Agentes**:
  - *Primário*: `qa-security-auditor`
  - *Secundário/Review*: `fullstack-engineer`
* **Melhores Skills**: `e2e-playwright-testing`, `verification-before-completion`, `strict-type-verifier`.
* **Quality Gate**: Todos os testes E2E executam e passam a 100% verde com relatórios detalhados.
* **Commit Checkpoint**: `test(e2e): implement comprehensive playwright end-to-end test suite`

---

### FASE 8: Packaging, CI/CD & Finalização da Release v1.0.0

#### Task 8.1: Configuração do Empacotamento macOS (DMG & Assinatura)
* **Objetivo**: Configurar o processo de build para gerar o instalador nativo macOS (`.dmg` e `.app`) para arquiteturas Apple Silicon (aarch64) e Intel (x86_64), com ícones de alta resolução e metadados de sistema.
* **Ficheiros**: `src-tauri/tauri.conf.json`, `src-tauri/icons/`, `scripts/build-dmg.sh`.
* **Melhores Agentes**:
  - *Primário*: `devops-release`
  - *Secundário/Review*: `build-error-resolver`
* **Melhores Skills**: `ci-cd-workflow-automation`, `semantic-git-workflow`.
* **Quality Gate**: Binário compilado com sucesso e executável de forma autónoma no macOS.
* **Commit Checkpoint**: `chore(release): configure macos dmg bundler and application icons`

#### Task 8.2: Workflow de CI/CD para GitHub Releases
* **Objetivo**: Criar workflow no GitHub Actions para automatizar validação contínua (lint, typecheck, testes Rust, testes Vitest, Playwright E2E) e publicação automática de releases quando uma tag `vX.Y.Z` for criada.
* **Ficheiros**: `.github/workflows/release.yml`.
* **Melhores Agentes**:
  - *Primário*: `devops-release`
  - *Secundário/Review*: `qa-security-auditor`
* **Melhores Skills**: `ci-cd-workflow-automation`, `finishing-a-development-branch`.
* **Quality Gate**: Validação sintática do workflow e teste dry-run de CI.
* **Commit Checkpoint**: `ci(actions): create multi-architecture automated release workflow`

#### Task 8.3: Manual de Utilização, Documentação e Tag v1.0.0
* **Objetivo**: Atualizar o `README.md`, documentar atalhos de teclado, guia de início rápido e criar a tag oficial de release da versão 1.0.0.
* **Ficheiros**: `README.md`, `CHANGELOG.md`, `docs/USER_GUIDE.md`.
* **Melhores Agentes**:
  - *Primário*: `devops-release`
  - *Secundário/Review*: `system-architect`
* **Melhores Skills**: `semantic-git-workflow`, `living-docs-governance`, `finishing-a-development-branch`.
* **Quality Gate**: Documentação completa e sem links quebrados.
* **Commit Checkpoint**: `docs(release): publish user guide and prepare version 1.0.0 release notes`

---

## 📊 Matriz de Resumo: Agentes e Skills por Fase

| Fase | Âmbito Principal | Agente Líder | Agente Reviewer | Skills em Destaque |
| :--- | :--- | :--- | :--- | :--- |
| **0** | Scaffolding & Setup | `fullstack-engineer` | `system-architect` | `vite-patterns`, `rust-patterns`, `project-guardrail-scaffolder` |
| **1** | Workspaces & Worktrees | `fullstack-engineer` | `database-specialist` | `using-git-worktrees`, `schema-and-db-migration`, `error-handling` |
| **2** | Tiled Grid & PTY Multiplexer | `fullstack-engineer` | `frontend-design-specialist`| `latency-critical-systems`, `react-performance`, `apple-hig-auditor` |
| **3** | Agentes `.md`, Team Wire & Model Routing | `frontend-design-specialist` | `fullstack-engineer` | `omni-models`, `omni-agents-a2a`, `design-taste-frontend`, `living-docs-governance` |
| **4** | Live Preview Browser | `fullstack-engineer` | `frontend-design-specialist`| `react-patterns`, `design-taste-frontend`, `error-handling` |
| **5** | HelmVoice (0.00€) | `fullstack-engineer` | `performance-engineer` | `rust-patterns`, `latency-critical-systems`, `apple-hig-auditor` |
| **6** | Governação & Telemetria | `frontend-design-specialist` | `qa-security-auditor` | `security-review`, `industrial-brutalist-ui`, `strict-type-verifier` |
| **7** | Auditorias HIG & E2E | `apple-hig-specialist` | `qa-security-auditor` | `apple-hig-auditor`, `ponytail-audit`, `e2e-playwright-testing` |
| **8** | Packaging & Release | `devops-release` | `build-error-resolver` | `ci-cd-workflow-automation`, `semantic-git-workflow` |

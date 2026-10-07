# Desktop Runtime: Tauri 2.x com Rust para Aplicação Nativa macOS

* **Status**: accepted
* **Deciders**: Principal Architect (Orchestrator), Simão Sousa
* **Date**: 2026-10-06

## Context and Problem Statement

O HelmADE necessita de um runtime desktop nativo para macOS capaz de gerir múltiplos processos PTY concorrentes (até 16 em simultâneo), aceder a APIs nativas do sistema operativo (Speech.framework, NSVisualEffectView) e manter uma pegada de memória mínima. O invariante de custo ZERO proibe qualquer subscrição ou infraestrutura cloud paga. O framework escolhido determina a linguagem do backend, o mecanismo IPC, a capacidade de vibrancy nativa, e a pegada de memória em produção.

## Decision Drivers

* **Custo Absoluto ZERO (0.00€)**: Sem licenças, sem subscrições de plataforma, 100% open-source e local.
* **Pegada de memória ultra-reduzida**: Necessidade de correr 16 PTYs concorrentes sem degradar o sistema do utilizador.
* **Acesso nativo a APIs macOS**: `Speech.framework` (transcrição offline), `NSVisualEffectView` (vibrancy), `NSWorkspace` (operações de ficheiros) via FFI/Objective-C.
* **Performance I/O de PTY**: Throughput de 10 MB/s por sessão de terminal sem bloquear a thread principal.
* **Frontend React existente**: A equipa já domina React 19 + TypeScript; o backend deve expor IPC simples.
* **Segurança de memória**: Gestão de processos PTY requer garantias contra double-free e race conditions.

## Considered Options

* **Opção A — Tauri 2.x (Rust)**: Framework desktop com backend Rust, frontend Web (React/Vite), IPC via `invoke()`, webview nativa do sistema (WKWebView no macOS), pegada < 50 MB.
* **Opção B — Electron (Node.js)**: Framework maduro com Chromium embutido, pegada > 500 MB, Node.js no backend, não tem acesso directo a APIs Objective-C sem addons nativos complexos.
* **Opção C — Swift + SwiftUI nativo**: Performance máxima e acesso total às APIs Apple, mas frontend proprietário — exige migrar de React para SwiftUI, duplicando o esforço.
* **Opção D — Neutralino.js**: Leve, mas IPC limitado, sem acesso a FFI Rust, comunidade pequena e maturidade insuficiente para PTY multiplexing.

## Decision Outcome

Chosen option: **"Opção A — Tauri 2.x com Rust"**, porque combina pegada de memória mínima (< 50 MB vs > 500 MB do Electron), acesso a FFI nativa macOS via `objc2`, segurança de memória garantida pelo compilador Rust, e permite reutilizar o frontend React 19 existente sem alterações. O modelo IPC baseado em `invoke()` e Tauri Events mapeia directamente ao padrão de PTY read/write assincronos com `portable-pty`.

### Positive Consequences

* Pegada de memória em runtime < 50 MB (vs > 500 MB Electron), preservando recursos para os 16 PTYs concorrentes.
* Rust garante ausência de data races nos canais assincronos de PTY sem GC overhead.
* `objc2` permite FFI directa com `SFSpeechRecognizer` e `NSVisualEffectView` para vibrancy a custo zero.
* `tauri-plugin-dialog` expõe diálogos nativos do sistema sem código adicional.
* Sistema de capabilities granular (Task 0.1) garante que cada plugin tem apenas as permissões mínimas necessárias.
* WKWebView macOS nativa rende o frontend React com hardware acceleration sem bundle Chromium.

### Negative Consequences

* Rust tem curva de aprendizagem mais íngreme que Node.js para operações PTY complexas.
* Cada alteração ao backend Rust requer recompilação (`cargo build`); ciclo de dev é mais lento que hot-reload.
* FFI Objective-C (`objc2`) exige código `unsafe` e conhecimento da ABI Apple; testabilidade reduzida.
* Tauri 2.x é mais recente que 1.x — alguns plugins da comunidade ainda em desenvolvimento.

## Sub-Agent Delegation Plan

1. **`fullstack-engineer`**: Implementar `portable-pty` PTY manager, ring buffer, e IPC handlers em `src-tauri/src/terminal/`.
2. **`fullstack-engineer`**: Implementar FFI Objective-C para `SFSpeechRecognizer` em `src-tauri/src/voice/`.
3. **`performance-engineer`**: Benchmarking de throughput PTY (alvo: 10 MB/s por canal, 16 canais concorrentes).
4. **`qa-security-auditor`**: Auditar código `unsafe` nos blocos FFI e validar que capabilities Tauri aplicam o princípio do mínimo privilégio.
5. **`devops-release`**: `feat(pty): build high-throughput pty multiplexer and ring buffer in rust`.

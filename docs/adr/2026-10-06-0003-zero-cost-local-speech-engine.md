# Motor de Voz Local a Custo Zero (0.00€) via Apple Speech.framework

* **Status**: accepted
* **Deciders**: Principal Architect (Orchestrator), Simão Sousa
* **Date**: 2026-10-06

## Context and Problem Statement

O utilizador do HelmADE comanda o enxame de agentes e o Orquestrador através de instruções em linguagem natural, tanto por texto como por voz (*Push-to-Talk*). O Invariante Sagrado número 1 do HelmADE exige **Custo Absoluto ZERO (0.00€)**, proibindo dependências de APIs de Speech-to-Text proprietárias pagas na nuvem (como OpenAI Whisper API, Google Cloud Speech ou ElevenLabs). Além disso, a latência de transcrição para comandos técnicos deve ser inferior a 300ms para permitir uma experiência de fluxo fluida.

## Decision Drivers

* **Custo Absoluto ZERO (0.00€)**: Sem custos de tokens de voz por minuto ou chaves de API externas pagas.
* **Privacidade Total e Execução Offline**: O áudio do utilizador nunca sai da máquina local; funcionamento em voos ou ambientes sem rede.
* **Latência Ultra-Baixa**: Transcrição quase em tempo real (< 300ms após libertação da tecla Push-to-Talk).
* **Eficiência Energética**: Aproveitamento dos aceleradores de hardware locais (Apple Silicon Neural Engine / Mac local).

## Considered Options

* **Opção A — Apple Speech.framework nativo do macOS via FFI/Objective-C (`objc2` em Rust)**: Utilização da API de reconhecimento de fala integrada do macOS (`SFSpeechRecognizer`). Suporta português e inglês offline nos modelos nativos da Apple, sem custos adicionais.
* **Opção B — Whisper Local embutido via `whisper.rs` / `whisper.cpp`**: Biblioteca C++ compilada com suporte Metal/NEON. Boa precisão, mas adiciona peso de modelo (140MB a 1.5GB) ao binário e consome mais memória/GPU durante a inferência.
* **Opção C — Whisper API em Cloud (OpenAI / Groq)**: Transcrição rápida, mas viola frontalmente o Invariante de Custo Zero (0.00€) e exige ligação à Internet.
* **Opção D — Web Speech API do navegador**: Limitada na WKWebView do macOS, instável em Tauri e sujeita a restrições de permissão e suporte de codecs offline.

## Decision Outcome

Chosen option: **"Opção A — Apple Speech.framework nativo do macOS via FFI/Objective-C"** como motor primário no macOS, com extensão opcional para `whisper.cpp` em plataformas não-Apple. Esta abordagem garante custo zero, zero download de modelos pesados adicionais, latência mínima e conformidade com o ecossistema Apple.

### Positive Consequences

* Custo 0.00€ absoluto para o utilizador.
* Tamanho do executável do HelmADE mantém-se reduzido (< 30MB) por não ter de incluir pesos de rede neuronal.
* Modelos de fala nativos do macOS são atualizados e otimizados pelo próprio sistema operativo.
* Transcrição imediata de atalho global Push-to-Talk (`Cmd+Shift+V` ou `Caps Lock`).

### Negative Consequences

* Dependência da disponibilidade da API Speech no macOS (requer permissão de microfone nas Capabilities de segurança da aplicação).
* Exige pós-processamento heurístico para terminologia de programação (dicionário técnico para termos como `camelCase`, `tsconfig`, `git rebase`).

## Sub-Agent Delegation Plan

1. **`fullstack-engineer`**: Implementar bindings nativos em Rust (`src-tauri/src/voice/macos_speech.rs`) e FFI de áudio.
2. **`performance-engineer`**: Garantir latência < 300ms entre libertação do hotkey e preenchimento da prompt.
3. **`frontend-design-specialist`**: Criar overlay visual e feedback de escuta sonora/háptica de acordo com Apple HIG.
4. **`apple-hig-specialist`**: Auditar acessibilidade e permissões de microfone de acordo com as diretrizes da Apple.
5. **`devops-release`**: `feat(voice): implement zero-cost local speech recognition via native macos apis`.

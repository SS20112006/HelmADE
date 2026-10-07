# Seleção Dinâmica de Modelo de IA por Agente e Perfis Cognitivos

* **Status**: accepted
* **Deciders**: Principal Architect (Orchestrator), Simão Sousa
* **Date**: 2026-10-06

## Context and Problem Statement

Num enxame de agentes com diferentes responsabilidades (Orquestrador, Especialistas Full-Stack, Design Frontend, Auditor de Segurança, Engenheiro de Performance, DevOps), atribuir cegamente o mesmo modelo a todos os agentes gera ineficiência de custos ou lentidão desnecessária. Tarefas de arquitetura e QA exigem modelos com raciocínio profundo (*Extended Thinking* / *Reasoning*), enquanto tarefas de frontend ou formatação beneficiam de modelos rápidos e económicos. Além disso, o suporte a modelos locais a custo zero (via Ollama ou LM Studio) é um requisito essencial para cumprir o Invariante de Custo Zero (0.00€).

## Decision Drivers

* **Otimização de Custos e Performance**: Modelos potentes para tarefas de alto raciocínio; modelos rápidos e económicos para tarefas mecânicas ou repetitivas.
* **Custo Zero (0.00€) via Modelos Locais**: Suporte de primeira classe para servidores de inferência locais (Ollama, LM Studio, vLLM).
* **Flexibilidade por Terminal**: Cada grelha do enxame deve poder correr com um modelo independente configurado na UI ou nos presets de enxame.
* **Persistência Transparente**: As escolhas de modelo devem persistir no SQLite local e ser injetadas de forma limpa na inicialização da sessão PTY do agente.

## Considered Options

* **Opção A — Mapeamento por Perfis Cognitivos e Flags CLI Transparentes**: O HelmADE categoriza modelos em 4 perfis cognitivos (`Deep Thinking`, `Fast / Economic`, `Research / Wide Context`, `Local Zero-Cost`) e injeta as variáveis de ambiente ou flags CLI correspondentes (`--model <id>`) no arranque do PTY de cada agente.
* **Opção B — Modelo Global Único para todo o Cockpit**: Todos os agentes partilham o mesmo modelo configurado globalmente. Rejeitado: ou torna o enxame proibitivamente lento/caro, ou enfraquece a capacidade de raciocínio do Orquestrador.
* **Opção C — Proxy Centralizado Proprietário**: Exige servidor intermediário na cloud. Rejeitado: viola o Invariante de Custo Zero e privacidade local.

## Decision Outcome

Chosen option: **"Opção A — Mapeamento por Perfis Cognitivos e Flags Transparentes"**, permitindo aos utilizadores e presets escolher modelos adequados para cada papel através de dropdowns visuais no cabeçalho do terminal, com suporte total a instâncias locais Ollama e APIs de fornecedores externos.

### Positive Consequences

* Roteamento económico inteligente: tarefas mecânicas não consomem cotas de modelos de raciocínio profundo caros.
* Suporte nativo a Ollama/LM Studio permite operação 100% gratuita (0.00€).
* Interface limpa com distintivos de perfil no cabeçalho de cada terminal.
* Persistência de configuração de equipa nos Presets de Enxame com 1 clique.

### Negative Consequences

* Exige manutenção de um catálogo de modelos suportados e respetivos identificadores de CLI.
* Utilizadores com modelos locais requerem recursos suficientes de RAM/VRAM na sua máquina.

## Sub-Agent Delegation Plan

1. **`frontend-design-specialist`**: Desenvolver `src/components/terminal/ModelSelectorDropdown.tsx` e `AgentModelConfigModal.tsx`.
2. **`database-specialist`**: Adicionar campos de persistência de modelo na tabela `grid_agents` em `src-tauri/src/db/schema.rs`.
3. **`fullstack-engineer`**: Garantir injeção das flags de modelo na criação do processo PTY em `src-tauri/src/terminal/pty.rs`.
4. **`system-architect`**: Manter alinhamento das definições de modelos nos ficheiros Markdown de persona.
5. **`devops-release`**: `feat(agents): implement per-agent ai model selector and cognitive profile routing`.

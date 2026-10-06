# ⚓ HelmADE
> **The Zero-Cost Agent Development Environment (ADE) & Autonomous Swarm Cockpit**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](#)
[![Platform: macOS](https://img.shields.io/badge/Platform-macOS-black.svg)](#)
[![Zero-Cost](https://img.shields.io/badge/Cost-0.00%E2%82%AC-success.svg)](#)
[![Agent Protocol](https://img.shields.io/badge/Agents-Antigravity%20.md-orange.svg)](#)

---

## 🧭 Visão do Projeto

O **HelmADE** é uma aplicação desktop nativa concebida para atuar como o **Cockpit de Comando** para desenvolvimento de software através de **enxames (*swarms*) de agentes de inteligência artificial autónomos**.

Inspirado na visão de *vibe-coding* e orquestração de terminais da BridgeMind, o HelmADE foi desenhado desde o primeiro dia com três princípios invioláveis:
1. **Custo Absoluto ZERO (0.00€ para construir e 0.00€ para usar):** 100% local, sem infraestruturas cloud pagas e sem mensalidades de plataformas proprietárias.
2. **Orquestração Nativa de Agentes Antigravity (`.md`):** Carrega diretamente as personas, regras e competências dos agentes a partir de ficheiros Markdown abertos, com suporte ao papel de **Orquestrador Líder** e agentes especialistas (Frontend, Database, QA, Fullstack).
3. **Múltiplas Grelhas com Isolamento via Git Worktrees:** Permite correr vários enxames em paralelo no mesmo projeto sem nunca gerar conflitos de ficheiros nem corromper o repositório.

---

## 📑 Documentação Central

Toda a especificação funcional detalhada da aplicação encontra-se formalizada no documento de requisitos de produto:

👉 [**Ler o Product Requirements Document (PRD.md)**](./PRD.md)

---

## 🏛️ Os Pilares Funcionais

* **Gestão de Projetos Focada:** 1 Projeto = 1 Diretório Local, sem misturas e com persistência de estado transparente.
* **Grelha Tiled Multiplexer (1 a 16 Painéis):** Suporte de 1 a 16 sessões simultâneas com presets rápidos (`1x1`, `2x2`, `2x3`, `3x3`, `4x4`), zoom de painel e identificação visual por cores e ícones.
* **Team Wire (Chat dos IAs):** Feed em tempo real com todas as mensagens trocadas entre o Orquestrador e os agentes especialistas da grelha.
* **Isolamento sem Falhas:** Criação automática de Git Worktrees e symlinks de dependências (`node_modules`) por grelha.
* **Live Browser Integrado:** Navegador embutido com deteção automática de portas de servidor e recarregamento HMR.
* **HelmVoice (Voz a Custo Zero):** Push-to-talk global com reconhecimento de voz nativo do macOS, vocabulário de código e injeção automática no prompt.
* **Dashboard de Telemetria & Custos:** Monitorização precisa de tokens consumidos, custos estimados em dólares ($) e métricas de velocidade das missões.
* **Streamer Shield:** Mascaramento automático de chaves de API e segredos para programação segura em transmissões ao vivo.

---

## 📜 Licença

Distribuído sob licença aberta MIT. Consulte o ficheiro `LICENSE` para mais detalhes.

import React, { useState, useCallback } from 'react';
import { useProjects } from '../hooks/useProjects';
import { ProjectList } from '../components/ProjectList';
import type { Project } from '../types/project';

interface WelcomeViewProps {
  onOpenProject?: (project: Project) => void;
}

export const WelcomeView: React.FC<WelcomeViewProps> = ({ onOpenProject }) => {
  const {
    projects,
    loading,
    selectFolderAndOpen,
    addProjectByPath,
    removeProject,
  } = useProjects();

  const [isDraggingOver, setIsDraggingOver] = useState(false);

  const handleSelectProject = useCallback(
    (project: Project) => {
      if (onOpenProject) {
        onOpenProject(project);
      } else {
        console.log('Projeto selecionado:', project);
      }
    },
    [onOpenProject]
  );

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDraggingOver(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDraggingOver(false);
  };

  const handleDrop = async (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDraggingOver(false);

    const files = e.dataTransfer.files;
    if (files && files.length > 0) {
      const firstFile = files[0];
      // Em ambientes desktop (Tauri/Electron), o path completo do arquivo ou pasta está disponível
      const fullPath = (firstFile as unknown as { path?: string }).path;
      if (fullPath) {
        const proj = await addProjectByPath(fullPath, firstFile.name);
        handleSelectProject(proj);
      }
    }
  };

  const handleOpenFolder = async () => {
    const proj = await selectFolderAndOpen();
    if (proj) {
      handleSelectProject(proj);
    }
  };

  return (
    <div
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
      className="relative flex flex-col h-screen w-screen overflow-hidden bg-black/40 backdrop-blur-2xl text-white select-none"
    >
      {/* Barra de Título Arrastável (macOS Traffic Lights Area) */}
      <div
        data-tauri-drag-region
        className="h-9 w-full flex items-center justify-between px-4 shrink-0 border-b border-white/[0.06]"
      >
        <div className="flex items-center gap-2 pl-16">
          <span className="text-[11px] font-semibold tracking-wide text-white/40 uppercase">
            HelmADE Cockpit
          </span>
        </div>
        <div className="text-[11px] text-white/30 font-mono">
          v1.0.0 · Local Zero-Cost
        </div>
      </div>

      {/* Conteúdo Principal Dividido em Duas Colunas */}
      <div className="flex-1 flex flex-col md:flex-row overflow-hidden p-6 md:p-8 gap-8 max-w-6xl w-full mx-auto">
        {/* Coluna Esquerda: Hero & Ações Primárias */}
        <div className="flex flex-col justify-between md:w-1/2 space-y-6">
          <div className="space-y-4">
            {/* Logótipo / Cabeçalho Principal */}
            <div className="flex items-center gap-3">
              <div className="flex items-center justify-center w-12 h-12 rounded-[12px] bg-gradient-to-br from-[#ffd60a]/20 to-[#0a84ff]/20 border border-white/15 text-2xl shadow-lg shadow-black/40">
                ⚓
              </div>
              <div>
                <h1 className="text-3xl font-semibold tracking-tight text-white font-sf">
                  HelmADE
                </h1>
                <p className="text-[12px] text-white/50 tracking-normal">
                  Autonomous Swarm Cockpit & Development Environment
                </p>
              </div>
            </div>

            {/* Badges de Invariantes */}
            <div className="flex flex-wrap gap-2 pt-1">
              <span className="px-2.5 py-1 rounded-[6px] text-[11px] font-medium bg-[#30d158]/15 text-[#30d158] border border-[#30d158]/25">
                ● Custo 0.00€
              </span>
              <span className="px-2.5 py-1 rounded-[6px] text-[11px] font-medium bg-[#0a84ff]/15 text-[#0a84ff] border border-[#0a84ff]/25">
                Git Worktrees Isolados
              </span>
              <span className="px-2.5 py-1 rounded-[6px] text-[11px] font-medium bg-[#bf5af2]/15 text-[#bf5af2] border border-[#bf5af2]/25">
                Speech.framework Nativo
              </span>
            </div>

            {/* Texto Descritivo */}
            <p className="text-[13px] text-white/70 leading-relaxed max-w-md">
              Comande enxames paralelos de agentes especializados em terminais
              locais isolados por Git Worktree, com persistência SQLite e zero
              subscrições cloud.
            </p>
          </div>

          {/* Área de Drag & Drop & Botões de Ação */}
          <div className="space-y-3">
            {/* Dropzone com Feedback Visual */}
            <div
              className={`flex flex-col items-center justify-center p-6 rounded-[12px] border-2 border-dashed transition-all duration-200 cursor-pointer ${
                isDraggingOver
                  ? 'border-[#0a84ff] bg-[#0a84ff]/15 scale-[1.01]'
                  : 'border-white/15 bg-white/[0.03] hover:border-white/30 hover:bg-white/[0.06]'
              }`}
              onClick={handleOpenFolder}
            >
              <svg
                className={`w-8 h-8 mb-2 transition-colors ${
                  isDraggingOver ? 'text-[#0a84ff]' : 'text-white/40'
                }`}
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                strokeWidth={1.5}
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                />
              </svg>
              <span className="text-[13px] font-medium text-white/90">
                {isDraggingOver
                  ? 'Solte a pasta para abrir'
                  : 'Arrastar pasta do projeto para aqui'}
              </span>
              <span className="text-[11px] text-white/40 mt-1">
                ou clique para selecionar no Explorador
              </span>
            </div>

            {/* Botões de Ação com Touch Target Apple HIG (mín 44px) */}
            <div className="flex gap-3">
              <button
                type="button"
                onClick={handleOpenFolder}
                className="flex-1 min-h-[44px] flex items-center justify-center gap-2 px-4 rounded-[10px] bg-[#0a84ff] hover:bg-[#0071e3] active:bg-[#0062c4] text-white text-[13px] font-medium transition-all shadow-md shadow-[#0a84ff]/25 focus-visible:ring-2 focus-visible:ring-white"
              >
                <svg
                  className="w-4 h-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  strokeWidth={2}
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
                  />
                </svg>
                Abrir Pasta Local...
              </button>

              <button
                type="button"
                onClick={handleOpenFolder}
                className="min-h-[44px] px-4 flex items-center justify-center gap-2 rounded-[10px] bg-white/[0.08] hover:bg-white/[0.14] active:bg-white/[0.2] border border-white/[0.12] text-white/90 text-[13px] font-medium transition-all focus-visible:ring-2 focus-visible:ring-[#0a84ff]"
              >
                Novo Projeto
              </button>
            </div>
          </div>
        </div>

        {/* Coluna Direita: Projetos Recentes */}
        <div className="flex flex-col md:w-1/2 p-5 rounded-[14px] bg-white/[0.03] border border-white/[0.08] backdrop-blur-md overflow-hidden">
          <div className="flex items-center justify-between mb-3 shrink-0">
            <h2 className="text-[14px] font-semibold text-white/90 tracking-tight">
              Projetos Recentes
            </h2>
            <span className="text-[11px] text-white/40">
              {projects.length} {projects.length === 1 ? 'projeto' : 'projetos'}
            </span>
          </div>

          <ProjectList
            projects={projects}
            loading={loading}
            onSelectProject={handleSelectProject}
            onRemoveProject={(id, e) => {
              e.stopPropagation();
              removeProject(id);
            }}
            onOpenFolderDialog={handleOpenFolder}
          />
        </div>
      </div>
    </div>
  );
};

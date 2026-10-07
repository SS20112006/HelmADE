import { useState } from 'react';
import { WelcomeView } from './views/WelcomeView';
import type { Project } from './types/project';

export default function App() {
  const [activeProject, setActiveProject] = useState<Project | null>(null);

  if (!activeProject) {
    return <WelcomeView onOpenProject={(proj) => setActiveProject(proj)} />;
  }

  // Visualização temporária para o projeto aberto (Módulo 2 / Grelha de Terminais)
  return (
    <div className="flex flex-col h-screen w-screen bg-[#1c1c1e] text-white">
      <div
        data-tauri-drag-region
        className="h-10 border-b border-white/10 flex items-center justify-between px-4 pl-20"
      >
        <div className="flex items-center gap-2">
          <span className="font-semibold text-[13px]">{activeProject.name}</span>
          <span className="text-[11px] text-white/40 font-mono">
            {activeProject.path}
          </span>
        </div>
        <button
          type="button"
          onClick={() => setActiveProject(null)}
          className="text-[12px] px-2.5 py-1 rounded bg-white/10 hover:bg-white/20 text-white/80 transition-colors"
        >
          Voltar ao Início
        </button>
      </div>
      <div className="flex-1 flex items-center justify-center text-white/60 text-[13px]">
        Área de trabalho do projeto ativa (A aguardar Módulo 2: Git Worktrees & Terminais)
      </div>
    </div>
  );
}

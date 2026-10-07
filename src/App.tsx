import { useState } from 'react';
import { WelcomeView } from './views/WelcomeView';
import { MergeReviewView } from './views/MergeReviewView';
import type { Project } from './types/project';

export default function App() {
  const [activeProject, setActiveProject] = useState<Project | null>(null);
  const [reviewingSwarmId, setReviewingSwarmId] = useState<string | null>(null);

  if (!activeProject) {
    return <WelcomeView onOpenProject={(proj) => setActiveProject(proj)} />;
  }

  return (
    <div className="flex flex-col h-screen w-screen bg-[#1c1c1e] text-white select-none">
      <div
        data-tauri-drag-region
        className="h-11 border-b border-white/10 flex items-center justify-between px-4 pl-20 bg-[#161618]"
      >
        <div className="flex items-center gap-2.5">
          <span className="font-semibold text-[13px] text-white/90">{activeProject.name}</span>
          <span className="text-[11px] text-white/40 font-mono truncate max-w-xs">
            {activeProject.path}
          </span>
        </div>

        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={() => setReviewingSwarmId('1')}
            className="text-[12px] px-3 py-1.5 min-h-[36px] rounded-lg bg-blue-600/20 hover:bg-blue-600/30 text-blue-300 border border-blue-500/30 transition-colors flex items-center gap-1.5"
            title="Rever alterações do enxame atual e fundir para a branch principal"
          >
            <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
            </svg>
            Rever & Fundir Worktree
          </button>

          <button
            type="button"
            onClick={() => {
              setActiveProject(null);
              setReviewingSwarmId(null);
            }}
            className="text-[12px] px-3 py-1.5 min-h-[36px] rounded-lg bg-white/10 hover:bg-white/15 text-white/80 transition-colors"
          >
            Voltar ao Início
          </button>
        </div>
      </div>

      <div className="flex-1 flex flex-col items-center justify-center text-white/60 text-[13px] p-6 text-center">
        <div className="w-12 h-12 rounded-xl bg-white/5 border border-white/10 flex items-center justify-center mb-3 text-white/40">
          <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
          </svg>
        </div>
        <p className="font-medium text-white/80">Área de Trabalho Ativa</p>
        <p className="text-white/40 text-[12px] mt-1 max-w-md">
          Módulo 2 (Git Worktrees & Diff/Merge) configurado. Clique em &quot;Rever &amp; Fundir Worktree&quot; no cabeçalho para inspecionar e integrar alterações com segurança.
        </p>
      </div>

      {reviewingSwarmId && (
        <MergeReviewView
          repoPath={activeProject.path}
          swarmId={reviewingSwarmId}
          onClose={() => setReviewingSwarmId(null)}
          onMergeSuccess={(sha) => {
            console.log('Merge concluído com sucesso, SHA:', sha);
          }}
        />
      )}
    </div>
  );
}

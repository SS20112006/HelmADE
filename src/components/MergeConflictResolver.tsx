import React from 'react';

interface MergeConflictResolverProps {
  conflicts: string[];
  errorMessage: string;
  onDelegateToOrchestrator: () => void;
  onAbort: () => void;
}

export const MergeConflictResolver: React.FC<MergeConflictResolverProps> = ({
  conflicts,
  errorMessage,
  onDelegateToOrchestrator,
  onAbort,
}) => {
  return (
    <div className="rounded-xl border border-rose-500/30 bg-rose-950/20 p-5 text-white mb-4 shadow-lg backdrop-blur-md">
      <div className="flex items-start gap-3.5">
        <div className="p-2 rounded-lg bg-rose-500/20 text-rose-400 shrink-0">
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
            />
          </svg>
        </div>

        <div className="flex-1 min-w-0">
          <h3 className="text-[15px] font-semibold text-rose-300">
            Conflito de Fusão Detetado
          </h3>
          <p className="text-[13px] text-white/70 mt-1">
            Não foi possível integrar as alterações automaticamente na branch principal sem colisões de código. A árvore principal foi revertida para o estado limpo e permanece 100% intacta.
          </p>

          {errorMessage && (
            <div className="mt-2 text-[12px] font-mono p-2 rounded bg-black/40 text-rose-300/90 border border-rose-500/20">
              {errorMessage}
            </div>
          )}

          {conflicts.length > 0 && (
            <div className="mt-3">
              <span className="text-[11px] font-semibold text-white/50 uppercase tracking-wider">
                Ficheiros em Conflito ({conflicts.length}):
              </span>
              <ul className="mt-1.5 space-y-1">
                {conflicts.map((file, idx) => (
                  <li
                    key={idx}
                    className="text-[12px] font-mono text-rose-200/90 flex items-center gap-2 bg-rose-950/40 px-2.5 py-1 rounded border border-rose-500/20"
                  >
                    <span className="w-1.5 h-1.5 rounded-full bg-rose-400"></span>
                    <span className="truncate">{file}</span>
                  </li>
                ))}
              </ul>
            </div>
          )}

          <div className="mt-4 flex flex-wrap items-center gap-3">
            <button
              type="button"
              onClick={onDelegateToOrchestrator}
              className="px-3.5 py-2 min-h-[44px] rounded-lg bg-blue-600 hover:bg-blue-500 text-white font-medium text-[13px] shadow transition-colors flex items-center gap-2"
            >
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
              Solicitar Resolução ao Orquestrador
            </button>

            <button
              type="button"
              onClick={onAbort}
              className="px-3.5 py-2 min-h-[44px] rounded-lg bg-white/10 hover:bg-white/15 text-white/80 font-medium text-[13px] transition-colors"
            >
              Cancelar e Manter Worktree
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

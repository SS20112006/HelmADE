import React from 'react';
import type { DiffSummary, DiffFile, FileDiffDetail } from '../types/diff';

interface DiffViewerProps {
  summary: DiffSummary | null;
  selectedFile: DiffFile | null;
  fileDetail: FileDiffDetail | null;
  loading: boolean;
  loadingFile: boolean;
  onSelectFile: (file: DiffFile) => void;
}

export const DiffViewer: React.FC<DiffViewerProps> = ({
  summary,
  selectedFile,
  fileDetail,
  loading,
  loadingFile,
  onSelectFile,
}) => {
  if (loading) {
    return (
      <div className="flex-1 flex items-center justify-center text-white/50 text-[13px] py-16">
        <svg
          className="animate-spin -ml-1 mr-3 h-5 w-5 text-white/60"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
        >
          <circle
            className="opacity-25"
            cx="12"
            cy="12"
            r="10"
            stroke="currentColor"
            strokeWidth="4"
          />
          <path
            className="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          />
        </svg>
        A calcular diferenças Git do worktree...
      </div>
    );
  }

  if (!summary || summary.files.length === 0) {
    return (
      <div className="flex-1 flex flex-col items-center justify-center text-center p-8 text-white/50">
        <div className="w-12 h-12 rounded-full bg-white/5 flex items-center justify-center mb-3">
          <svg className="w-6 h-6 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M5 13l4 4L19 7" />
          </svg>
        </div>
        <p className="text-[14px] font-medium text-white/80">Nenhuma alteração encontrada</p>
        <p className="text-[12px] text-white/40 mt-1 max-w-sm">
          A branch deste worktree ({summary?.branch || 'swarm'}) está sincronizada com a branch alvo ({summary?.base_branch || 'main'}).
        </p>
      </div>
    );
  }

  const renderBadge = (status: string) => {
    switch (status) {
      case 'added':
        return (
          <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-emerald-500/15 text-emerald-400 border border-emerald-500/30">
            A
          </span>
        );
      case 'modified':
        return (
          <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-amber-500/15 text-amber-400 border border-amber-500/30">
            M
          </span>
        );
      case 'deleted':
        return (
          <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-rose-500/15 text-rose-400 border border-rose-500/30">
            D
          </span>
        );
      case 'renamed':
        return (
          <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-purple-500/15 text-purple-400 border border-purple-500/30">
            R
          </span>
        );
      default:
        return (
          <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-blue-500/15 text-blue-400 border border-blue-500/30">
            {status.toUpperCase()}
          </span>
        );
    }
  };

  // Processar linhas do patch com numeração e estilos semânticos
  const renderPatchLines = (patchText: string) => {
    if (!patchText || patchText.trim().length === 0) {
      return (
        <div className="p-8 text-center text-white/40 text-[12px]">
          Ficheiro sem alterações de texto ou conteúdo binário.
        </div>
      );
    }

    const lines = patchText.split('\n');
    let oldLineNum = 0;
    let newLineNum = 0;

    return (
      <div className="font-mono text-[12px] leading-5 divide-y divide-white/5 select-text">
        {lines.map((line, idx) => {
          if (line.startsWith('@@')) {
            // Extrair números de linha do bloco hunk (ex: @@ -1,4 +1,6 @@)
            const match = line.match(/@@ -(\d+)(?:,\d+)? \+(\d+)(?:,\d+)? @@/);
            if (match) {
              oldLineNum = parseInt(match[1], 10);
              newLineNum = parseInt(match[2], 10);
            }
            return (
              <div
                key={idx}
                className="bg-sky-500/10 text-sky-300/80 px-4 py-1 font-semibold flex items-center"
              >
                <span className="w-16 shrink-0 text-white/30 text-[10px]">...</span>
                <span>{line}</span>
              </div>
            );
          }

          if (line.startsWith('+') && !line.startsWith('+++')) {
            const currentNew = newLineNum++;
            return (
              <div
                key={idx}
                className="bg-emerald-950/35 text-emerald-200 px-4 py-0.5 flex items-start border-l-2 border-emerald-500 hover:bg-emerald-950/50"
              >
                <span className="w-8 shrink-0 text-white/20 text-right pr-2 select-none">
                  &nbsp;
                </span>
                <span className="w-8 shrink-0 text-emerald-400/60 text-right pr-3 select-none font-mono text-[11px]">
                  {currentNew}
                </span>
                <span className="whitespace-pre-wrap break-all flex-1">{line}</span>
              </div>
            );
          }

          if (line.startsWith('-') && !line.startsWith('---')) {
            const currentOld = oldLineNum++;
            return (
              <div
                key={idx}
                className="bg-rose-950/35 text-rose-200 px-4 py-0.5 flex items-start border-l-2 border-rose-500 hover:bg-rose-950/50"
              >
                <span className="w-8 shrink-0 text-rose-400/60 text-right pr-2 select-none font-mono text-[11px]">
                  {currentOld}
                </span>
                <span className="w-8 shrink-0 text-white/20 text-right pr-3 select-none">
                  &nbsp;
                </span>
                <span className="whitespace-pre-wrap break-all flex-1">{line}</span>
              </div>
            );
          }

          // Linhas de cabeçalho diff / index / --- / +++
          if (
            line.startsWith('diff --git') ||
            line.startsWith('index ') ||
            line.startsWith('--- ') ||
            line.startsWith('+++ ')
          ) {
            return (
              <div key={idx} className="bg-white/5 text-white/40 px-4 py-0.5 text-[11px]">
                {line}
              </div>
            );
          }

          // Linhas de contexto normais
          const currentOld = oldLineNum ? oldLineNum++ : '';
          const currentNew = newLineNum ? newLineNum++ : '';

          return (
            <div
              key={idx}
              className="text-white/70 px-4 py-0.5 flex items-start hover:bg-white/[0.02]"
            >
              <span className="w-8 shrink-0 text-white/20 text-right pr-2 select-none font-mono text-[11px]">
                {currentOld}
              </span>
              <span className="w-8 shrink-0 text-white/20 text-right pr-3 select-none font-mono text-[11px]">
                {currentNew}
              </span>
              <span className="whitespace-pre-wrap break-all flex-1">{line}</span>
            </div>
          );
        })}
      </div>
    );
  };

  return (
    <div className="flex-1 flex overflow-hidden border border-white/10 rounded-xl bg-[#161618]">
      {/* Barra Lateral: Lista de Ficheiros */}
      <div className="w-72 shrink-0 border-r border-white/10 flex flex-col bg-[#121214]">
        {/* Cabeçalho da Lista */}
        <div className="h-10 px-3.5 border-b border-white/10 flex items-center justify-between text-[11px] font-semibold text-white/60 uppercase tracking-wider">
          <span>Ficheiros Alterados ({summary.files.length})</span>
          <div className="flex items-center gap-1.5 font-mono text-[11px]">
            <span className="text-emerald-400 font-medium">+{summary.total_additions}</span>
            <span className="text-white/30">/</span>
            <span className="text-rose-400 font-medium">-{summary.total_deletions}</span>
          </div>
        </div>

        {/* Lista com Scroll */}
        <div className="flex-1 overflow-y-auto divide-y divide-white/5 py-1">
          {summary.files.map((file) => {
            const isSelected = selectedFile?.path === file.path;
            return (
              <button
                key={file.path}
                type="button"
                onClick={() => onSelectFile(file)}
                className={`w-full text-left px-3 py-2.5 transition-colors flex items-center justify-between gap-2 group ${
                  isSelected
                    ? 'bg-blue-600/20 text-white border-l-2 border-blue-500'
                    : 'text-white/70 hover:bg-white/5 hover:text-white'
                }`}
              >
                <div className="flex items-center gap-2 min-w-0">
                  {renderBadge(file.status)}
                  <span
                    className="text-[12px] font-mono truncate"
                    title={file.path}
                  >
                    {file.path}
                  </span>
                </div>
                <div className="flex items-center gap-1.5 shrink-0 text-[11px] font-mono text-white/40">
                  {file.additions > 0 && (
                    <span className="text-emerald-400/80">+{file.additions}</span>
                  )}
                  {file.deletions > 0 && (
                    <span className="text-rose-400/80">-{file.deletions}</span>
                  )}
                </div>
              </button>
            );
          })}
        </div>
      </div>

      {/* Painel Central: Visualizador do Patch */}
      <div className="flex-1 flex flex-col overflow-hidden bg-[#18181b]">
        {selectedFile ? (
          <>
            {/* Barra de título do ficheiro selecionado */}
            <div className="h-10 px-4 border-b border-white/10 flex items-center justify-between bg-[#141416]">
              <div className="flex items-center gap-2 min-w-0">
                {renderBadge(selectedFile.status)}
                <span className="text-[12px] font-mono font-medium text-white/90 truncate">
                  {selectedFile.path}
                </span>
                {selectedFile.old_path && (
                  <span className="text-[11px] text-white/40 font-mono">
                    (renomeado de {selectedFile.old_path})
                  </span>
                )}
              </div>

              <div className="flex items-center gap-3">
                <div className="flex items-center gap-2 text-[11px] font-mono">
                  <span className="text-emerald-400">+{selectedFile.additions} linhas</span>
                  <span className="text-white/20">|</span>
                  <span className="text-rose-400">-{selectedFile.deletions} linhas</span>
                </div>
              </div>
            </div>

            {/* Conteúdo do Diff com Scroll */}
            <div className="flex-1 overflow-auto bg-[#0d0d0f]">
              {loadingFile ? (
                <div className="p-8 text-center text-white/40 text-[12px]">
                  A carregar patch...
                </div>
              ) : fileDetail ? (
                renderPatchLines(fileDetail.patch)
              ) : (
                <div className="p-8 text-center text-white/40 text-[12px]">
                  Sem conteúdo disponível para este ficheiro.
                </div>
              )}
            </div>
          </>
        ) : (
          <div className="flex-1 flex items-center justify-center text-white/40 text-[12px]">
            Selecione um ficheiro à esquerda para inspecionar as diferenças.
          </div>
        )}
      </div>
    </div>
  );
};

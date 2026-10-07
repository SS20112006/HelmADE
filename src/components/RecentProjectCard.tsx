import React from 'react';
import type { Project } from '../types/project';

interface RecentProjectCardProps {
  project: Project;
  onSelect: (project: Project) => void;
  onRemove: (id: string, e: React.MouseEvent) => void;
}

function formatRelativeTime(timestampSecs: number): string {
  const diffSecs = Math.floor(Date.now() / 1000 - timestampSecs);
  if (diffSecs < 60) return 'agora mesmo';
  const diffMins = Math.floor(diffSecs / 60);
  if (diffMins < 60) return `há ${diffMins} min`;
  const diffHours = Math.floor(diffMins / 60);
  if (diffHours < 24) return `há ${diffHours} h`;
  const diffDays = Math.floor(diffHours / 24);
  if (diffDays === 1) return 'ontem';
  if (diffDays < 30) return `há ${diffDays} dias`;
  return new Date(timestampSecs * 1000).toLocaleDateString('pt-PT');
}

export const RecentProjectCard: React.FC<RecentProjectCardProps> = ({
  project,
  onSelect,
  onRemove,
}) => {
  return (
    <div
      role="button"
      tabIndex={0}
      onClick={() => onSelect(project)}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          onSelect(project);
        }
      }}
      className="group relative flex items-center justify-between p-3.5 rounded-[10px] bg-white/[0.04] hover:bg-white/[0.08] active:bg-white/[0.12] border border-white/[0.08] hover:border-white/[0.16] transition-all duration-150 cursor-pointer outline-none focus-visible:ring-2 focus-visible:ring-[#0a84ff]"
    >
      <div className="flex items-center gap-3 min-w-0">
        {/* Ícone de Pasta macOS */}
        <div className="flex items-center justify-center w-10 h-10 rounded-[8px] bg-[#0a84ff]/15 text-[#0a84ff] border border-[#0a84ff]/25 shrink-0">
          <svg
            className="w-5 h-5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={1.75}
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
            />
          </svg>
        </div>

        {/* Metadados do Projeto */}
        <div className="min-w-0 flex flex-col">
          <span className="text-[13px] font-medium text-white/90 truncate tracking-tight group-hover:text-white">
            {project.name}
          </span>
          <span
            className="text-[11px] text-white/45 truncate tracking-normal font-mono"
            title={project.path}
          >
            {project.path}
          </span>
        </div>
      </div>

      {/* Ações e Data Relativa */}
      <div className="flex items-center gap-2 shrink-0 ml-3">
        <span className="text-[11px] text-white/40 tracking-tight">
          {formatRelativeTime(project.last_opened_at)}
        </span>

        {/* Botão de Remover com Hit-Box Acessível */}
        <button
          type="button"
          aria-label={`Remover ${project.name} dos recentes`}
          onClick={(e) => onRemove(project.id, e)}
          className="opacity-0 group-hover:opacity-100 p-1.5 rounded-[6px] text-white/40 hover:text-[#ff453a] hover:bg-[#ff453a]/15 transition-all duration-150"
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
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>
    </div>
  );
};

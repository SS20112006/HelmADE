import React, { useState, useMemo } from 'react';
import type { Project } from '../types/project';
import { RecentProjectCard } from './RecentProjectCard';

interface ProjectListProps {
  projects: Project[];
  loading: boolean;
  onSelectProject: (project: Project) => void;
  onRemoveProject: (id: string, e: React.MouseEvent) => void;
  onOpenFolderDialog: () => void;
}

export const ProjectList: React.FC<ProjectListProps> = ({
  projects,
  loading,
  onSelectProject,
  onRemoveProject,
  onOpenFolderDialog,
}) => {
  const [searchTerm, setSearchTerm] = useState('');

  const filteredProjects = useMemo(() => {
    if (!searchTerm.trim()) return projects;
    const term = searchTerm.toLowerCase();
    return projects.filter(
      (p) =>
        p.name.toLowerCase().includes(term) ||
        p.path.toLowerCase().includes(term)
    );
  }, [projects, searchTerm]);

  return (
    <div className="flex flex-col h-full w-full">
      {/* Barra de Pesquisa Rápida */}
      <div className="relative mb-3">
        <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-white/40">
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
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
        </div>
        <input
          type="text"
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          placeholder="Pesquisar projetos recentes..."
          className="w-full pl-9 pr-8 py-2 bg-white/[0.05] border border-white/[0.1] rounded-[8px] text-[13px] text-white placeholder-white/35 outline-none focus:border-[#0a84ff] focus:ring-1 focus:ring-[#0a84ff] transition-all"
        />
        {searchTerm && (
          <button
            type="button"
            onClick={() => setSearchTerm('')}
            className="absolute inset-y-0 right-0 pr-2.5 flex items-center text-white/40 hover:text-white/80"
          >
            <svg
              className="w-3.5 h-3.5"
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
        )}
      </div>

      {/* Lista com Scroll Suave */}
      <div className="flex-1 overflow-y-auto space-y-2 pr-1 custom-scrollbar min-h-[180px]">
        {loading ? (
          <div className="flex items-center justify-center h-32 text-white/40 text-[13px]">
            A carregar projetos...
          </div>
        ) : filteredProjects.length > 0 ? (
          filteredProjects.map((project) => (
            <RecentProjectCard
              key={project.id}
              project={project}
              onSelect={onSelectProject}
              onRemove={onRemoveProject}
            />
          ))
        ) : searchTerm ? (
          <div className="flex flex-col items-center justify-center h-32 text-center text-white/40 text-[13px] p-4">
            <span>Nenhum projeto encontrado para "{searchTerm}"</span>
          </div>
        ) : (
          <div className="flex flex-col items-center justify-center h-40 text-center rounded-[10px] border border-dashed border-white/10 p-6">
            <div className="w-10 h-10 rounded-full bg-white/5 flex items-center justify-center text-white/30 mb-2">
              <svg
                className="w-5 h-5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                strokeWidth={1.5}
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                />
              </svg>
            </div>
            <p className="text-[13px] text-white/60 mb-1">
              Ainda não tem projetos abertos
            </p>
            <p className="text-[11px] text-white/35 mb-3">
              Abra uma pasta local do seu computador para começar.
            </p>
            <button
              type="button"
              onClick={onOpenFolderDialog}
              className="px-3.5 py-1.5 rounded-[6px] bg-[#0a84ff] hover:bg-[#0071e3] text-white text-[12px] font-medium transition-colors"
            >
              Abrir Pasta...
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

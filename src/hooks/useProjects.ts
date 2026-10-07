import { useState, useEffect, useCallback } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { projectService } from '../services/projectService';
import type { Project } from '../types/project';

export function useProjects() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchProjects = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const list = await projectService.listRecentProjects(30);
      setProjects(list);
    } catch (err) {
      console.error('Falha ao carregar projetos recentes:', err);
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchProjects();
  }, [fetchProjects]);

  const addProjectByPath = useCallback(
    async (folderPath: string, customName?: string): Promise<Project> => {
      const cleanPath = folderPath.trim();
      const derivedName =
        customName ||
        cleanPath.split('/').filter(Boolean).pop() ||
        'Novo Projeto';
      const id = `proj_${Date.now()}_${Math.random().toString(36).substring(2, 7)}`;

      const created = await projectService.upsertProject({
        id,
        name: derivedName,
        path: cleanPath,
      });

      await fetchProjects();
      return created;
    },
    [fetchProjects]
  );

  const selectFolderAndOpen = useCallback(async (): Promise<Project | null> => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Selecionar Diretório do Projeto',
      });

      if (!selected || typeof selected !== 'string') {
        return null;
      }

      return await addProjectByPath(selected);
    } catch (err) {
      console.error('Erro ao abrir diálogo de seleção:', err);
      setError(err instanceof Error ? err.message : String(err));
      return null;
    }
  }, [addProjectByPath]);

  const removeProject = useCallback(
    async (id: string) => {
      try {
        await projectService.deleteProject(id);
        setProjects((prev) => prev.filter((p) => p.id !== id));
      } catch (err) {
        console.error('Erro ao remover projeto:', err);
        setError(err instanceof Error ? err.message : String(err));
      }
    },
    []
  );

  return {
    projects,
    loading,
    error,
    refreshProjects: fetchProjects,
    selectFolderAndOpen,
    addProjectByPath,
    removeProject,
  };
}

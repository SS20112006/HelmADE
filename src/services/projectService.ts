import { invoke } from '@tauri-apps/api/core';
import type { Project, CreateProjectPayload } from '../types/project';

export const projectService = {
  async upsertProject(payload: CreateProjectPayload): Promise<Project> {
    return invoke<Project>('upsert_project', {
      id: payload.id,
      name: payload.name,
      path: payload.path,
    });
  },

  async listRecentProjects(limit = 20): Promise<Project[]> {
    return invoke<Project[]>('list_recent_projects', { limit });
  },

  async deleteProject(id: string): Promise<boolean> {
    return invoke<boolean>('delete_project', { id });
  },
};

import { invoke } from '@tauri-apps/api/core';
import type { WorktreeInfo, CreateWorktreePayload } from '../types/worktree';

export const worktreeService = {
  async createWorktree(payload: CreateWorktreePayload): Promise<WorktreeInfo> {
    return invoke<WorktreeInfo>('create_worktree', {
      repoPath: payload.repoPath,
      swarmId: payload.swarmId,
      baseBranch: payload.baseBranch,
    });
  },

  async removeWorktree(
    repoPath: string,
    swarmId: string,
    deleteBranch = true
  ): Promise<boolean> {
    return invoke<boolean>('remove_worktree', {
      repoPath,
      swarmId,
      deleteBranch,
    });
  },

  async listWorktrees(repoPath: string): Promise<WorktreeInfo[]> {
    return invoke<WorktreeInfo[]>('list_worktrees', { repoPath });
  },
};

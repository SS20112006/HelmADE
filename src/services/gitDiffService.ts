import { invoke } from '@tauri-apps/api/core';
import type { DiffSummary, FileDiffDetail, MergeResult, MergeStrategy } from '../types/diff';

export const gitDiffService = {
  /**
   * Obtém o resumo de ficheiros alterados no worktree em relação ao base_branch
   */
  async getWorktreeDiff(
    repoPath: string,
    swarmId: string,
    baseBranch?: string
  ): Promise<DiffSummary> {
    return invoke<DiffSummary>('get_worktree_diff', {
      repoPath,
      swarmId,
      baseBranch,
    });
  },

  /**
   * Obtém o patch detalhado (unified diff) de um ficheiro específico
   */
  async getFileDiff(
    repoPath: string,
    swarmId: string,
    filePath: string,
    baseBranch?: string
  ): Promise<FileDiffDetail> {
    return invoke<FileDiffDetail>('get_file_diff', {
      repoPath,
      swarmId,
      filePath,
      baseBranch,
    });
  },

  /**
   * Executa a fusão segura do worktree para a branch principal (squash ou rebase)
   */
  async mergeWorktree(
    repoPath: string,
    swarmId: string,
    strategy: MergeStrategy,
    commitMessage?: string,
    baseBranch?: string
  ): Promise<MergeResult> {
    return invoke<MergeResult>('merge_worktree', {
      repoPath,
      swarmId,
      strategy,
      commitMessage,
      baseBranch,
    });
  },

  /**
   * Aborta uma operação de fusão pendente ou em conflito
   */
  async abortMerge(repoPath: string): Promise<boolean> {
    return invoke<boolean>('abort_merge', { repoPath });
  },
};

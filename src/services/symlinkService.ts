import { invoke } from '@tauri-apps/api/core';
import type { SymlinkEntry } from '../types/symlink';

export const symlinkService = {
  async symlinkDependencies(repoPath: string, swarmId: string): Promise<SymlinkEntry[]> {
    return invoke<SymlinkEntry[]>('symlink_dependencies', {
      repoPath,
      swarmId,
    });
  },
};

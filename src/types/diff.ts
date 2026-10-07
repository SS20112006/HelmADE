export interface DiffFile {
  path: string;
  old_path?: string | null;
  status: 'added' | 'modified' | 'deleted' | 'renamed' | string;
  additions: number;
  deletions: number;
  binary: boolean;
}

export interface DiffSummary {
  swarm_id: string;
  branch: string;
  base_branch: string;
  files: DiffFile[];
  total_files: number;
  total_additions: number;
  total_deletions: number;
}

export interface FileDiffDetail {
  path: string;
  patch: string;
}

export type MergeStrategy = 'squash' | 'rebase';

export interface MergeResult {
  success: boolean;
  strategy: string;
  merged_branch: string;
  target_branch: string;
  commit_sha?: string | null;
  conflicts: string[];
  message: string;
}

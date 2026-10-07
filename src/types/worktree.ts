export interface WorktreeInfo {
  swarm_id: string;
  branch: string;
  path: string;
  is_bare: boolean;
  created_at: number;
}

export interface CreateWorktreePayload {
  repoPath: string;
  swarmId: string;
  baseBranch?: string;
}

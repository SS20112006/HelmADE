import { invoke } from '@tauri-apps/api/core';

export const portService = {
  async allocatePort(swarmId: string): Promise<number> {
    return invoke<number>('allocate_swarm_port', { swarmId });
  },

  async releasePort(swarmId: string): Promise<boolean> {
    return invoke<boolean>('release_swarm_port', { swarmId });
  },

  async getPort(swarmId: string): Promise<number | null> {
    return invoke<number | null>('get_swarm_port', { swarmId });
  },

  async getEnvVars(swarmId: string): Promise<Record<string, string>> {
    return invoke<Record<string, string>>('get_swarm_env_vars', { swarmId });
  },
};

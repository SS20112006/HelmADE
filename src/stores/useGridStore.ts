import { useSyncExternalStore } from 'react';
import { portService } from '../services/portService';

export interface GridPanelState {
  id: string;
  swarmId: string;
  title: string;
  port?: number;
  envVars?: Record<string, string>;
  status: 'idle' | 'running' | 'completed' | 'error';
}

interface GridStoreState {
  panels: GridPanelState[];
}

let state: GridStoreState = {
  panels: [],
};

const listeners = new Set<() => void>();

function emitChange() {
  for (const listener of listeners) {
    listener();
  }
}

export const gridStore = {
  getState() {
    return state;
  },

  subscribe(listener: () => void) {
    listeners.add(listener);
    return () => {
      listeners.delete(listener);
    };
  },

  addPanel(panel: GridPanelState) {
    state = {
      ...state,
      panels: [...state.panels, panel],
    };
    emitChange();
  },

  removePanel(id: string) {
    const panel = state.panels.find((p) => p.id === id);
    if (panel) {
      portService.releasePort(panel.swarmId).catch(() => {});
    }
    state = {
      ...state,
      panels: state.panels.filter((p) => p.id !== id),
    };
    emitChange();
  },

  async allocatePortForPanel(swarmId: string): Promise<number> {
    const port = await portService.allocatePort(swarmId);
    const envVars = await portService.getEnvVars(swarmId);

    state = {
      ...state,
      panels: state.panels.map((p) =>
        p.swarmId === swarmId ? { ...p, port, envVars } : p
      ),
    };
    emitChange();
    return port;
  },

  async releasePortForPanel(swarmId: string): Promise<boolean> {
    const success = await portService.releasePort(swarmId);
    state = {
      ...state,
      panels: state.panels.map((p) =>
        p.swarmId === swarmId ? { ...p, port: undefined, envVars: undefined } : p
      ),
    };
    emitChange();
    return success;
  },
};

export function useGridStore() {
  const storeState = useSyncExternalStore(gridStore.subscribe, gridStore.getState);
  return {
    ...storeState,
    addPanel: gridStore.addPanel,
    removePanel: gridStore.removePanel,
    allocatePortForPanel: gridStore.allocatePortForPanel,
    releasePortForPanel: gridStore.releasePortForPanel,
  };
}

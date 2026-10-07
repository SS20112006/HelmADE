export interface Project {
  id: string;
  name: string;
  path: string;
  created_at: number;
  last_opened_at: number;
  settings_json?: string | null;
}

export interface CreateProjectPayload {
  id: string;
  name: string;
  path: string;
}

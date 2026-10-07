export interface SymlinkEntry {
  name: string;
  source: string;
  target: string;
  success: boolean;
  error?: string | null;
}

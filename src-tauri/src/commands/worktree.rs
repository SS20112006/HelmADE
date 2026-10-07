use crate::git::worktree::{self, WorktreeInfo};

#[tauri::command]
pub fn create_worktree(
    repo_path: String,
    swarm_id: String,
    base_branch: Option<String>,
) -> Result<WorktreeInfo, String> {
    worktree::create_worktree(&repo_path, &swarm_id, base_branch.as_deref())
}

#[tauri::command]
pub fn remove_worktree(
    repo_path: String,
    swarm_id: String,
    delete_branch: Option<bool>,
) -> Result<bool, String> {
    worktree::remove_worktree(&repo_path, &swarm_id, delete_branch.unwrap_or(true))
}

#[tauri::command]
pub fn list_worktrees(repo_path: String) -> Result<Vec<WorktreeInfo>, String> {
    worktree::list_worktrees(&repo_path)
}

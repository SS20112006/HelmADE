use crate::git::diff::{self, DiffSummary, FileDiffDetail, MergeResult};
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

#[tauri::command]
pub fn get_worktree_diff(
    repo_path: String,
    swarm_id: String,
    base_branch: Option<String>,
) -> Result<DiffSummary, String> {
    diff::get_worktree_diff(&repo_path, &swarm_id, base_branch.as_deref())
}

#[tauri::command]
pub fn get_file_diff(
    repo_path: String,
    swarm_id: String,
    file_path: String,
    base_branch: Option<String>,
) -> Result<FileDiffDetail, String> {
    diff::get_file_diff(&repo_path, &swarm_id, &file_path, base_branch.as_deref())
}

#[tauri::command]
pub fn merge_worktree(
    repo_path: String,
    swarm_id: String,
    strategy: String,
    commit_message: Option<String>,
    base_branch: Option<String>,
) -> Result<MergeResult, String> {
    diff::merge_worktree(
        &repo_path,
        &swarm_id,
        &strategy,
        commit_message.as_deref(),
        base_branch.as_deref(),
    )
}

#[tauri::command]
pub fn abort_merge(repo_path: String) -> Result<bool, String> {
    diff::abort_merge(&repo_path)
}

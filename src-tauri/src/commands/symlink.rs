use crate::git::symlink::{self, SymlinkEntry};
use std::path::Path;

fn sanitize_id(id: &str) -> Result<String, String> {
    let clean: String = id
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    if clean.is_empty() {
        return Err("Identificador de swarm inválido ou vazio".to_string());
    }
    Ok(clean)
}

#[tauri::command]
pub fn symlink_dependencies(
    repo_path: String,
    swarm_id: String,
) -> Result<Vec<SymlinkEntry>, String> {
    let clean_id = sanitize_id(&swarm_id)?;
    let repo = Path::new(&repo_path);
    if !repo.exists() {
        return Err(format!("Repositório não encontrado em '{repo_path}'"));
    }

    let worktree_dir = repo.join(".helm").join("worktrees").join(&clean_id);
    if !worktree_dir.exists() {
        return Err(format!(
            "Diretório de worktree não encontrado em '{}'",
            worktree_dir.display()
        ));
    }

    symlink::link_dependencies(repo, &worktree_dir)
}

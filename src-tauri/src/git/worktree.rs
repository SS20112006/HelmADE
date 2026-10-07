use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct WorktreeInfo {
    pub swarm_id: String,
    pub branch: String,
    pub path: String,
    pub is_bare: bool,
    pub created_at: i64,
}

/// Sanitiza identificadores para evitar injeção em caminhos de ficheiros
fn sanitize_id(id: &str) -> Result<String, String> {
    let clean: String = id
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    if clean.is_empty() {
        return Err("Identificador de worktree inválido ou vazio".to_string());
    }
    Ok(clean)
}

/// Cria um novo Git Worktree isolado na pasta .helm/worktrees/<swarm_id>
pub fn create_worktree(
    repo_path: &str,
    swarm_id: &str,
    base_branch: Option<&str>,
) -> Result<WorktreeInfo, String> {
    let clean_id = sanitize_id(swarm_id)?;
    let repo = Path::new(repo_path);
    if !repo.exists() || !repo.join(".git").exists() {
        return Err(format!("O caminho '{repo_path}' não é um repositório Git válido"));
    }

    let branch = format!("helm/swarm-{clean_id}");
    let worktree_dir: PathBuf = repo.join(".helm").join("worktrees").join(&clean_id);

    if worktree_dir.exists() {
        return Err(format!(
            "O diretório de worktree '{}' já existe",
            worktree_dir.display()
        ));
    }

    if let Some(parent) = worktree_dir.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Falha ao criar diretório base de worktrees: {e}"))?;
    }

    let base = base_branch.unwrap_or("HEAD");

    // Executa `git worktree add -b helm/swarm-<id> <path> <base>`
    let output = Command::new("git")
        .current_dir(repo)
        .args([
            "worktree",
            "add",
            "-b",
            &branch,
            worktree_dir.to_str().unwrap(),
            base,
        ])
        .output()
        .map_err(|e| format!("Falha ao executar comando git: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Erro do Git ao criar worktree: {stderr}"));
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    Ok(WorktreeInfo {
        swarm_id: clean_id,
        branch,
        path: worktree_dir.to_string_lossy().to_string(),
        is_bare: false,
        created_at: now,
    })
}

/// Remove um Git Worktree de forma limpa e segura
pub fn remove_worktree(
    repo_path: &str,
    swarm_id: &str,
    delete_branch: bool,
) -> Result<bool, String> {
    let clean_id = sanitize_id(swarm_id)?;
    let repo = Path::new(repo_path);
    let worktree_dir: PathBuf = repo.join(".helm").join("worktrees").join(&clean_id);
    let branch = format!("helm/swarm-{clean_id}");

    if worktree_dir.exists() {
        let output = Command::new("git")
            .current_dir(repo)
            .args(["worktree", "remove", "--force", worktree_dir.to_str().unwrap()])
            .output()
            .map_err(|e| format!("Falha ao executar git worktree remove: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Erro ao remover worktree: {stderr}"));
        }
    }

    if delete_branch {
        let _ = Command::new("git")
            .current_dir(repo)
            .args(["branch", "-D", &branch])
            .output();
    }

    Ok(true)
}

/// Lista todos os Git Worktrees ativos no repositório
pub fn list_worktrees(repo_path: &str) -> Result<Vec<WorktreeInfo>, String> {
    let repo = Path::new(repo_path);
    if !repo.exists() || !repo.join(".git").exists() {
        return Err(format!("O caminho '{repo_path}' não é um repositório Git"));
    }

    let output = Command::new("git")
        .current_dir(repo)
        .args(["worktree", "list", "--porcelain"])
        .output()
        .map_err(|e| format!("Falha ao listar worktrees: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Erro ao obter lista de worktrees: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut list = Vec::new();
    let mut current_path: Option<String> = None;
    let mut current_branch: Option<String> = None;
    let mut is_bare = false;

    for line in stdout.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            current_path = Some(path.trim().to_string());
            current_branch = None;
            is_bare = false;
        } else if let Some(branch_ref) = line.strip_prefix("branch ") {
            let clean_b = branch_ref
                .trim()
                .strip_prefix("refs/heads/")
                .unwrap_or(branch_ref.trim());
            current_branch = Some(clean_b.to_string());
        } else if line.trim() == "bare" {
            is_bare = true;
        } else if line.is_empty() {
            if let Some(p) = current_path.take() {
                let swarm_id = Path::new(&p)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                list.push(WorktreeInfo {
                    swarm_id,
                    branch: current_branch.take().unwrap_or_else(|| "detached".to_string()),
                    path: p,
                    is_bare,
                    created_at: 0,
                });
            }
        }
    }

    if let Some(p) = current_path {
        let swarm_id = Path::new(&p)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        list.push(WorktreeInfo {
            swarm_id,
            branch: current_branch.unwrap_or_else(|| "detached".to_string()),
            path: p,
            is_bare,
            created_at: 0,
        });
    }

    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_worktree_lifecycle_in_temp_repo() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("helmade_test_repo_{timestamp}"));
        fs::create_dir_all(&temp_dir).unwrap();

        // Inicializar repositório Git temporário
        let init_status = Command::new("git")
            .current_dir(&temp_dir)
            .args(["init"])
            .status()
            .expect("falha ao inicializar git");
        assert!(init_status.success());

        // Configurar user local temporário para commits
        let _ = Command::new("git")
            .current_dir(&temp_dir)
            .args(["config", "user.name", "HelmADE Test"])
            .status();
        let _ = Command::new("git")
            .current_dir(&temp_dir)
            .args(["config", "user.email", "test@helmade.dev"])
            .status();

        // Criar ficheiro inicial e commit
        fs::write(temp_dir.join("README.md"), "# HelmADE Test").unwrap();
        let _ = Command::new("git")
            .current_dir(&temp_dir)
            .args(["add", "."])
            .status();
        let commit_status = Command::new("git")
            .current_dir(&temp_dir)
            .args(["commit", "-m", "chore: initial test commit"])
            .status()
            .expect("falha no commit inicial");
        assert!(commit_status.success());

        let swarm_id = "agent-alpha-1";
        let repo_str = temp_dir.to_str().unwrap();

        // 1. Criar Worktree
        let wt = create_worktree(repo_str, swarm_id, None).expect("falha ao criar worktree");
        assert_eq!(wt.swarm_id, swarm_id);
        assert!(Path::new(&wt.path).exists());
        assert!(Path::new(&wt.path).join("README.md").exists());

        // 2. Listar Worktrees
        let list = list_worktrees(repo_str).expect("falha ao listar worktrees");
        assert!(list.iter().any(|item| item.swarm_id == swarm_id));

        // 3. Remover Worktree
        let removed = remove_worktree(repo_str, swarm_id, true).expect("falha ao remover worktree");
        assert!(removed);
        assert!(!Path::new(&wt.path).exists());

        // Limpeza do diretório de teste
        let _ = fs::remove_dir_all(&temp_dir);
    }
}

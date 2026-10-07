use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DiffFile {
    pub path: String,
    pub old_path: Option<String>,
    pub status: String, // "added", "modified", "deleted", "renamed", "untracked"
    pub additions: usize,
    pub deletions: usize,
    pub binary: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DiffSummary {
    pub swarm_id: String,
    pub branch: String,
    pub base_branch: String,
    pub files: Vec<DiffFile>,
    pub total_files: usize,
    pub total_additions: usize,
    pub total_deletions: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FileDiffDetail {
    pub path: String,
    pub patch: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct MergeResult {
    pub success: bool,
    pub strategy: String,
    pub merged_branch: String,
    pub target_branch: String,
    pub commit_sha: Option<String>,
    pub conflicts: Vec<String>,
    pub message: String,
}

fn sanitize_identifier(id: &str) -> Result<String, String> {
    let clean: String = id
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '/' || *c == '.')
        .collect();
    if clean.is_empty() {
        return Err("Identificador inválido ou vazio".to_string());
    }
    Ok(clean)
}

/// Extrai o resumo e a lista de ficheiros alterados no worktree em relação ao base_branch
pub fn get_worktree_diff(
    repo_path: &str,
    swarm_id: &str,
    base_branch: Option<&str>,
) -> Result<DiffSummary, String> {
    let clean_id = sanitize_identifier(swarm_id)?;
    let base = base_branch
        .map(sanitize_identifier)
        .transpose()?
        .unwrap_or_else(|| "main".to_string());

    let repo = Path::new(repo_path);
    if !repo.exists() || !repo.join(".git").exists() {
        return Err(format!("O caminho '{repo_path}' não é um repositório Git válido"));
    }

    let branch = format!("helm/swarm-{clean_id}");
    let worktree_dir: PathBuf = repo.join(".helm").join("worktrees").join(&clean_id);

    // 1. Obter numstat: adições e remoções por ficheiro entre base e a branch da worktree
    let numstat_output = Command::new("git")
        .current_dir(repo)
        .args([
            "diff",
            "--numstat",
            &format!("{base}...{branch}"),
        ])
        .output()
        .map_err(|e| format!("Falha ao executar git diff --numstat: {e}"))?;

    let mut files_map: std::collections::HashMap<String, (usize, usize, bool)> =
        std::collections::HashMap::new();

    if numstat_output.status.success() {
        let stdout = String::from_utf8_lossy(&numstat_output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                let add_str = parts[0].trim();
                let del_str = parts[1].trim();
                let path_str = parts[2].trim().to_string();

                let binary = add_str == "-" && del_str == "-";
                let additions = add_str.parse::<usize>().unwrap_or(0);
                let deletions = del_str.parse::<usize>().unwrap_or(0);

                files_map.insert(path_str, (additions, deletions, binary));
            }
        }
    }

    // 2. Obter name-status: estado dos ficheiros (A, M, D, R, etc.)
    let name_status_output = Command::new("git")
        .current_dir(repo)
        .args([
            "diff",
            "--name-status",
            &format!("{base}...{branch}"),
        ])
        .output()
        .map_err(|e| format!("Falha ao executar git diff --name-status: {e}"))?;

    let mut diff_files = Vec::new();

    if name_status_output.status.success() {
        let stdout = String::from_utf8_lossy(&name_status_output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.is_empty() {
                continue;
            }

            let status_code = parts[0].trim();
            if status_code.starts_with('R') && parts.len() >= 3 {
                let old_p = parts[1].trim().to_string();
                let new_p = parts[2].trim().to_string();
                let (add, del, bin) = files_map
                    .get(&new_p)
                    .copied()
                    .unwrap_or((0, 0, false));

                diff_files.push(DiffFile {
                    path: new_p,
                    old_path: Some(old_p),
                    status: "renamed".to_string(),
                    additions: add,
                    deletions: del,
                    binary: bin,
                });
            } else if parts.len() >= 2 {
                let file_path = parts[1].trim().to_string();
                let status_str = match status_code.chars().next() {
                    Some('A') => "added",
                    Some('M') => "modified",
                    Some('D') => "deleted",
                    _ => "modified",
                };

                let (add, del, bin) = files_map
                    .get(&file_path)
                    .copied()
                    .unwrap_or((0, 0, false));

                diff_files.push(DiffFile {
                    path: file_path,
                    old_path: None,
                    status: status_str.to_string(),
                    additions: add,
                    deletions: del,
                    binary: bin,
                });
            }
        }
    }

    // 3. Se a pasta de worktree existir, verificar alterações não commitadas ou untracked
    if worktree_dir.exists() {
        let status_cmd = Command::new("git")
            .current_dir(&worktree_dir)
            .args(["status", "--porcelain"])
            .output();

        if let Ok(st_out) = status_cmd {
            if st_out.status.success() {
                let stdout = String::from_utf8_lossy(&st_out.stdout);
                for line in stdout.lines() {
                    if line.len() > 3 {
                        let code = &line[0..2];
                        let f_path = line[3..].trim().to_string();

                        if !diff_files.iter().any(|f| f.path == f_path) {
                            let (status, add, del) = if code.contains('?') {
                                // Ficheiro não monitorizado: contar linhas
                                let count = fs::read_to_string(worktree_dir.join(&f_path))
                                    .map(|c| c.lines().count())
                                    .unwrap_or(1);
                                ("added", count, 0)
                            } else if code.contains('D') {
                                ("deleted", 0, 1)
                            } else {
                                ("modified", 1, 0)
                            };

                            diff_files.push(DiffFile {
                                path: f_path,
                                old_path: None,
                                status: status.to_string(),
                                additions: add,
                                deletions: del,
                                binary: false,
                            });
                        }
                    }
                }
            }
        }
    }

    let total_files = diff_files.len();
    let total_additions = diff_files.iter().map(|f| f.additions).sum();
    let total_deletions = diff_files.iter().map(|f| f.deletions).sum();

    Ok(DiffSummary {
        swarm_id: clean_id,
        branch,
        base_branch: base,
        files: diff_files,
        total_files,
        total_additions,
        total_deletions,
    })
}

/// Obtém o patch unificado (unified diff) de um ficheiro específico
pub fn get_file_diff(
    repo_path: &str,
    swarm_id: &str,
    file_path: &str,
    base_branch: Option<&str>,
) -> Result<FileDiffDetail, String> {
    let clean_id = sanitize_identifier(swarm_id)?;
    let base = base_branch
        .map(sanitize_identifier)
        .transpose()?
        .unwrap_or_else(|| "main".to_string());

    let repo = Path::new(repo_path);
    let branch = format!("helm/swarm-{clean_id}");
    let worktree_dir: PathBuf = repo.join(".helm").join("worktrees").join(&clean_id);

    // Tentar obter com git diff entre base e branch
    let output = Command::new("git")
        .current_dir(repo)
        .args([
            "diff",
            "-U3",
            &format!("{base}...{branch}"),
            "--",
            file_path,
        ])
        .output()
        .map_err(|e| format!("Falha ao extrair patch do ficheiro: {e}"))?;

    let mut patch = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        String::new()
    };

    // Se o patch for vazio e o ficheiro existir na worktree (ex: novo untracked file)
    if patch.trim().is_empty() && worktree_dir.exists() {
        let full_path = worktree_dir.join(file_path);
        if full_path.exists() {
            if let Ok(content) = fs::read_to_string(&full_path) {
                let mut fake_patch = format!("--- /dev/null\n+++ b/{file_path}\n@@ -0,0 +1,{} @@\n", content.lines().count());
                for line in content.lines() {
                    fake_patch.push('+');
                    fake_patch.push_str(line);
                    fake_patch.push('\n');
                }
                patch = fake_patch;
            }
        }
    }

    Ok(FileDiffDetail {
        path: file_path.to_string(),
        patch,
    })
}

/// Executa a fusão segura de um worktree para o branch alvo com prevenção de conflitos
pub fn merge_worktree(
    repo_path: &str,
    swarm_id: &str,
    strategy: &str,
    commit_message: Option<&str>,
    base_branch: Option<&str>,
) -> Result<MergeResult, String> {
    let clean_id = sanitize_identifier(swarm_id)?;
    let base = base_branch
        .map(sanitize_identifier)
        .transpose()?
        .unwrap_or_else(|| "main".to_string());

    let repo = Path::new(repo_path);
    if !repo.exists() || !repo.join(".git").exists() {
        return Err(format!("O caminho '{repo_path}' não é um repositório Git"));
    }

    let branch = format!("helm/swarm-{clean_id}");

    // 1. Verificar se o repositório principal tem alterações pendentes não commitadas
    let status_check = Command::new("git")
        .current_dir(repo)
        .args(["status", "--porcelain"])
        .output()
        .map_err(|e| format!("Erro ao verificar estado do repositório: {e}"))?;

    let st_str = String::from_utf8_lossy(&status_check.stdout);
    // Ignorar .helm se estiver no status
    let dirty_lines: Vec<&str> = st_str
        .lines()
        .filter(|l| !l.contains(".helm"))
        .collect();

    if !dirty_lines.is_empty() {
        return Err("O repositório principal contém alterações locais não commitadas. Conclua ou descarte as alterações antes de realizar o merge.".to_string());
    }

    // 2. Garantir que estamos no branch base
    let checkout = Command::new("git")
        .current_dir(repo)
        .args(["checkout", &base])
        .output()
        .map_err(|e| format!("Falha ao trocar para o branch {base}: {e}"))?;

    if !checkout.status.success() {
        let err = String::from_utf8_lossy(&checkout.stderr);
        return Err(format!("Falha ao mudar para o branch base '{base}': {err}"));
    }

    // 3. Executar estratégia selecionada
    match strategy {
        "squash" => {
            let merge_cmd = Command::new("git")
                .current_dir(repo)
                .args(["merge", "--squash", &branch])
                .output()
                .map_err(|e| format!("Falha ao executar squash merge: {e}"))?;

            if !merge_cmd.status.success() {
                // Conflito detetado! Obter ficheiros em conflito e abortar com segurança
                let conflict_cmd = Command::new("git")
                    .current_dir(repo)
                    .args(["diff", "--name-only", "--diff-filter=U"])
                    .output();

                let mut conflicts = Vec::new();
                if let Ok(c_out) = conflict_cmd {
                    let c_str = String::from_utf8_lossy(&c_out.stdout);
                    conflicts = c_str.lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                }

                // Abortar merge imediatamente para restaurar o estado limpo
                let _ = Command::new("git")
                    .current_dir(repo)
                    .args(["reset", "--hard", "HEAD"])
                    .output();

                return Ok(MergeResult {
                    success: false,
                    strategy: "squash".to_string(),
                    merged_branch: branch,
                    target_branch: base,
                    commit_sha: None,
                    conflicts,
                    message: "Conflitos detetados durante o Squash & Merge. A operação foi abortada com segurança.".to_string(),
                });
            }

            // Squash sucedido: criar o commit
            let default_msg = format!("feat(swarm-{clean_id}): integrate changes from worktree");
            let msg = commit_message.unwrap_or(&default_msg);

            let commit_cmd = Command::new("git")
                .current_dir(repo)
                .args(["commit", "-m", msg])
                .output()
                .map_err(|e| format!("Falha ao criar commit de squash: {e}"))?;

            if !commit_cmd.status.success() {
                let err = String::from_utf8_lossy(&commit_cmd.stderr);
                let _ = Command::new("git").current_dir(repo).args(["reset", "--hard", "HEAD"]).output();
                return Err(format!("Falha ao concluir commit de fusão: {err}"));
            }

            let sha_cmd = Command::new("git")
                .current_dir(repo)
                .args(["rev-parse", "HEAD"])
                .output()
                .map_err(|e| format!("Falha ao obter commit SHA: {e}"))?;

            let sha = String::from_utf8_lossy(&sha_cmd.stdout).trim().to_string();

            Ok(MergeResult {
                success: true,
                strategy: "squash".to_string(),
                merged_branch: branch,
                target_branch: base,
                commit_sha: Some(sha),
                conflicts: Vec::new(),
                message: "Squash & Merge concluído com sucesso e integrado na branch principal.".to_string(),
            })
        }
        "rebase" => {
            // Rebase merge: git merge --ff-only branch
            let merge_cmd = Command::new("git")
                .current_dir(repo)
                .args(["merge", "--ff-only", &branch])
                .output()
                .map_err(|e| format!("Falha ao executar rebase / fast-forward merge: {e}"))?;

            if !merge_cmd.status.success() {
                let err = String::from_utf8_lossy(&merge_cmd.stderr);
                return Ok(MergeResult {
                    success: false,
                    strategy: "rebase".to_string(),
                    merged_branch: branch,
                    target_branch: base,
                    commit_sha: None,
                    conflicts: vec![err.to_string()],
                    message: "Não foi possível efetuar Rebase Fast-Forward. Conflitos ou ramificação divergente.".to_string(),
                });
            }

            let sha_cmd = Command::new("git")
                .current_dir(repo)
                .args(["rev-parse", "HEAD"])
                .output()
                .map_err(|e| format!("Falha ao obter commit SHA: {e}"))?;

            let sha = String::from_utf8_lossy(&sha_cmd.stdout).trim().to_string();

            Ok(MergeResult {
                success: true,
                strategy: "rebase".to_string(),
                merged_branch: branch,
                target_branch: base,
                commit_sha: Some(sha),
                conflicts: Vec::new(),
                message: "Rebase Fast-Forward merge concluído com sucesso!".to_string(),
            })
        }
        _ => Err(format!("Estratégia de fusão não suportada: '{strategy}'. Use 'squash' ou 'rebase'.")),
    }
}

/// Aborta qualquer operação de merge pendente no repositório
pub fn abort_merge(repo_path: &str) -> Result<bool, String> {
    let repo = Path::new(repo_path);
    if !repo.exists() || !repo.join(".git").exists() {
        return Err(format!("O caminho '{repo_path}' não é um repositório Git"));
    }

    let _ = Command::new("git").current_dir(repo).args(["merge", "--abort"]).output();
    let _ = Command::new("git").current_dir(repo).args(["reset", "--hard", "HEAD"]).output();
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_diff_and_merge_lifecycle_in_temp_repo() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("helmade_diff_test_{timestamp}"));
        fs::create_dir_all(&temp_dir).unwrap();

        // 1. Inicializar Git repo
        let _ = Command::new("git").current_dir(&temp_dir).args(["init", "-b", "main"]).status();
        let _ = Command::new("git").current_dir(&temp_dir).args(["config", "user.name", "HelmADE Diff Test"]).status();
        let _ = Command::new("git").current_dir(&temp_dir).args(["config", "user.email", "difftest@helmade.dev"]).status();

        // 2. Commit inicial
        fs::write(temp_dir.join("README.md"), "# Original Main\nInitial line\n").unwrap();
        let _ = Command::new("git").current_dir(&temp_dir).args(["add", "."]).status();
        let _ = Command::new("git").current_dir(&temp_dir).args(["commit", "-m", "chore: initial commit"]).status();

        let repo_str = temp_dir.to_str().unwrap();
        let swarm_id = "test-grid-merge";

        // 3. Criar worktree
        let wt = crate::git::worktree::create_worktree(repo_str, swarm_id, Some("main"))
            .expect("falha ao criar worktree");

        // 4. Modificar ficheiro e criar novo ficheiro no worktree
        let wt_path = Path::new(&wt.path);
        fs::write(wt_path.join("README.md"), "# Modified Main\nInitial line\nAdded line in swarm\n").unwrap();
        fs::write(wt_path.join("feature.txt"), "New feature created in swarm grid\n").unwrap();

        // Fazer commit na worktree
        let _ = Command::new("git").current_dir(wt_path).args(["add", "."]).status();
        let _ = Command::new("git").current_dir(wt_path).args(["commit", "-m", "feat: swarm worktree changes"]).status();

        // 5. Obter resumo de diff
        let summary = get_worktree_diff(repo_str, swarm_id, Some("main")).expect("falha ao obter diff");
        assert_eq!(summary.swarm_id, swarm_id);
        assert_eq!(summary.total_files, 2);
        assert!(summary.files.iter().any(|f| f.path == "README.md" && f.status == "modified"));
        assert!(summary.files.iter().any(|f| f.path == "feature.txt" && f.status == "added"));
        assert!(summary.total_additions >= 2);

        // 6. Obter patch de um ficheiro
        let file_detail = get_file_diff(repo_str, swarm_id, "feature.txt", Some("main"))
            .expect("falha ao obter patch");
        assert!(file_detail.patch.contains("+New feature created in swarm grid"));

        // 7. Executar Squash & Merge
        let merge_res = merge_worktree(
            repo_str,
            swarm_id,
            "squash",
            Some("feat(merge): successfully merged swarm grid"),
            Some("main"),
        )
        .expect("falha ao executar merge");

        assert!(merge_res.success);
        assert!(merge_res.commit_sha.is_some());
        assert!(merge_res.conflicts.is_empty());

        // Verificar que os ficheiros agora existem no main
        assert!(temp_dir.join("feature.txt").exists());

        // 8. Limpar worktree
        let _ = crate::git::worktree::remove_worktree(repo_str, swarm_id, true);
        let _ = fs::remove_dir_all(&temp_dir);
    }
}

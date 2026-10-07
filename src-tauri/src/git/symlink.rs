use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Lista padrão de diretórios pesados de dependências e builds para ligar por symlink
pub const DEFAULT_DEPENDENCY_DIRS: &[&str] = &["node_modules", ".venv", "target", ".cargo"];

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SymlinkEntry {
    pub name: String,
    pub source: String,
    pub target: String,
    pub success: bool,
    pub error: Option<String>,
}

/// Cria um link simbólico de diretório de forma compatível com a plataforma
fn create_dir_symlink(source: &Path, target: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(source, target)
    }
    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_dir(source, target)
    }
    #[cfg(not(any(unix, windows)))]
    {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Links simbólicos não suportados nesta plataforma",
        ))
    }
}

/// Cria links simbólicos dos diretórios pesados de dependência da raiz para o worktree
pub fn link_dependencies(repo_root: &Path, worktree_dir: &Path) -> Result<Vec<SymlinkEntry>, String> {
    if !repo_root.exists() {
        return Err(format!("Diretório raiz não existe: {}", repo_root.display()));
    }
    if !worktree_dir.exists() {
        return Err(format!("Diretório de worktree não existe: {}", worktree_dir.display()));
    }

    // Prevenir criação de links circulares se forem o mesmo caminho
    if repo_root == worktree_dir {
        return Err("O diretório raiz e o worktree não podem ser idênticos".to_string());
    }

    let mut results = Vec::new();

    for dir_name in DEFAULT_DEPENDENCY_DIRS {
        let source_path: PathBuf = repo_root.join(dir_name);

        // Se a dependência não existir na raiz (ex: projeto sem .venv), ignora silenciosamente
        if !source_path.exists() {
            continue;
        }

        let target_path: PathBuf = worktree_dir.join(dir_name);

        // Idempotência: verificar se o destino já existe
        if target_path.exists() || target_path.is_symlink() {
            // Se for um link simbólico que aponta para o destino correto, marcar como sucesso
            if let Ok(dest) = fs::read_link(&target_path) {
                if dest == source_path {
                    results.push(SymlinkEntry {
                        name: dir_name.to_string(),
                        source: source_path.to_string_lossy().to_string(),
                        target: target_path.to_string_lossy().to_string(),
                        success: true,
                        error: None,
                    });
                    continue;
                }
            }

            // Caso seja um link quebrado ou antigo, remove antes de recriar
            let _ = fs::remove_file(&target_path);
        }

        match create_dir_symlink(&source_path, &target_path) {
            Ok(()) => {
                results.push(SymlinkEntry {
                    name: dir_name.to_string(),
                    source: source_path.to_string_lossy().to_string(),
                    target: target_path.to_string_lossy().to_string(),
                    success: true,
                    error: None,
                });
            }
            Err(err) => {
                results.push(SymlinkEntry {
                    name: dir_name.to_string(),
                    source: source_path.to_string_lossy().to_string(),
                    target: target_path.to_string_lossy().to_string(),
                    success: false,
                    error: Some(err.to_string()),
                });
            }
        }
    }

    Ok(results)
}

/// Remove os links simbólicos de dependências criados dentro de um worktree
/// Garantindo que NENHUM dado na raiz original seja apagado
pub fn unlink_dependencies(worktree_dir: &Path) -> Result<(), String> {
    if !worktree_dir.exists() {
        return Ok(());
    }

    for dir_name in DEFAULT_DEPENDENCY_DIRS {
        let target_path = worktree_dir.join(dir_name);
        if target_path.is_symlink() {
            let _ = fs::remove_file(&target_path);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_dependency_symlink_lifecycle_in_temp_dir() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let base_temp = std::env::temp_dir().join(format!("helmade_symlink_test_{timestamp}"));
        let repo_root = base_temp.join("repo");
        let worktree_dir = base_temp.join("worktree");

        fs::create_dir_all(&repo_root).unwrap();
        fs::create_dir_all(&worktree_dir).unwrap();

        // 1. Criar dependências simuladas na raiz do repo
        let node_modules_dir = repo_root.join("node_modules");
        let venv_dir = repo_root.join(".venv");
        fs::create_dir_all(&node_modules_dir).unwrap();
        fs::create_dir_all(&venv_dir).unwrap();

        let dummy_package = node_modules_dir.join("test-package.txt");
        fs::write(&dummy_package, "dummy content 123").unwrap();

        let dummy_venv_cfg = venv_dir.join("pyvenv.cfg");
        fs::write(&dummy_venv_cfg, "home = /usr/bin").unwrap();

        // 2. Executar ligação simbólica para o worktree
        let entries = link_dependencies(&repo_root, &worktree_dir).expect("falha ao ligar dependências");
        assert_eq!(entries.len(), 2);
        assert!(entries.iter().all(|e| e.success));

        // 3. Validar leitura transparente na worktree
        let wt_dummy = worktree_dir.join("node_modules").join("test-package.txt");
        assert!(wt_dummy.exists(), "arquivo deve ser legível através do symlink na worktree");
        let content = fs::read_to_string(&wt_dummy).unwrap();
        assert_eq!(content, "dummy content 123");

        let wt_venv = worktree_dir.join(".venv").join("pyvenv.cfg");
        assert!(wt_venv.exists());

        // 4. Testar idempotência (executar novamente sem erro)
        let second_run = link_dependencies(&repo_root, &worktree_dir).expect("segunda execução deve ser idempotente");
        assert_eq!(second_run.len(), 2);
        assert!(second_run.iter().all(|e| e.success));

        // 5. Executar unlink_dependencies
        unlink_dependencies(&worktree_dir).expect("falha ao desligar symlinks");
        assert!(!worktree_dir.join("node_modules").exists());
        assert!(!worktree_dir.join(".venv").exists());

        // 6. Assegurar que os ficheiros originais na raiz PERMANECEM INTACTOS
        assert!(repo_root.join("node_modules").join("test-package.txt").exists());
        assert!(repo_root.join(".venv").join("pyvenv.cfg").exists());

        // Limpeza
        let _ = fs::remove_dir_all(&base_temp);
    }
}

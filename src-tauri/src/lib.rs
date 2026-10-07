pub mod commands;
pub mod db;
pub mod git;
pub mod network;

use commands::network::PortState;
use db::DbState;
use std::fs;
use std::path::PathBuf;

fn resolve_db_path() -> PathBuf {
    let mut dir = dirs_or_home();
    let _ = fs::create_dir_all(&dir);
    dir.push("helmade.db");
    dir
}

fn dirs_or_home() -> PathBuf {
    if let Some(mut home) = std::env::var_os("HOME").map(PathBuf::from) {
        home.push(".helmade");
        home
    } else {
        PathBuf::from(".helmade")
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_path = resolve_db_path();
    let db_state = DbState::new(db_path.to_str().unwrap_or("helmade.db"))
        .expect("falha crítica ao inicializar a base de dados SQLite local");
    let port_state = PortState::default();

    tauri::Builder::default()
        .manage(db_state)
        .manage(port_state)
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // Comandos de Projetos & SQLite
            commands::project::upsert_project,
            commands::project::list_recent_projects,
            commands::project::delete_project,
            // Comandos de Git Worktree & Symlinks
            commands::worktree::create_worktree,
            commands::worktree::remove_worktree,
            commands::worktree::list_worktrees,
            commands::symlink::symlink_dependencies,
            // Comandos de Alocação de Rede & Portas
            commands::network::allocate_swarm_port,
            commands::network::release_swarm_port,
            commands::network::get_swarm_port,
            commands::network::get_swarm_env_vars,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

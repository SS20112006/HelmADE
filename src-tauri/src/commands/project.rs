use crate::db::{DbState, Project};
use tauri::State;

#[tauri::command]
pub fn upsert_project(
    state: State<'_, DbState>,
    id: String,
    name: String,
    path: String,
) -> Result<Project, String> {
    state
        .upsert_project(&id, &name, &path)
        .map_err(|e| format!("Falha ao guardar projeto: {e}"))
}

#[tauri::command]
pub fn list_recent_projects(
    state: State<'_, DbState>,
    limit: Option<usize>,
) -> Result<Vec<Project>, String> {
    state
        .list_recent_projects(limit.unwrap_or(20))
        .map_err(|e| format!("Falha ao listar projetos: {e}"))
}

#[tauri::command]
pub fn delete_project(state: State<'_, DbState>, id: String) -> Result<bool, String> {
    state
        .delete_project(&id)
        .map_err(|e| format!("Falha ao remover projeto: {e}"))
}

use crate::network::port_allocator::PortAllocator;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;

pub struct PortState(pub Mutex<PortAllocator>);

impl Default for PortState {
    fn default() -> Self {
        Self(Mutex::new(PortAllocator::default()))
    }
}

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
pub fn allocate_swarm_port(
    state: State<'_, PortState>,
    swarm_id: String,
) -> Result<u16, String> {
    let clean_id = sanitize_id(&swarm_id)?;
    let mut allocator = state
        .0
        .lock()
        .map_err(|e| format!("Falha ao bloquear estado de portas: {e}"))?;
    allocator.allocate(&clean_id)
}

#[tauri::command]
pub fn release_swarm_port(
    state: State<'_, PortState>,
    swarm_id: String,
) -> Result<bool, String> {
    let clean_id = sanitize_id(&swarm_id)?;
    let mut allocator = state
        .0
        .lock()
        .map_err(|e| format!("Falha ao bloquear estado de portas: {e}"))?;
    Ok(allocator.release(&clean_id).is_some())
}

#[tauri::command]
pub fn get_swarm_port(
    state: State<'_, PortState>,
    swarm_id: String,
) -> Result<Option<u16>, String> {
    let clean_id = sanitize_id(&swarm_id)?;
    let allocator = state
        .0
        .lock()
        .map_err(|e| format!("Falha ao bloquear estado de portas: {e}"))?;
    Ok(allocator.get_port(&clean_id))
}

#[tauri::command]
pub fn get_swarm_env_vars(
    state: State<'_, PortState>,
    swarm_id: String,
) -> Result<HashMap<String, String>, String> {
    let clean_id = sanitize_id(&swarm_id)?;
    let allocator = state
        .0
        .lock()
        .map_err(|e| format!("Falha ao bloquear estado de portas: {e}"))?;
    Ok(allocator.get_env_vars(&clean_id))
}

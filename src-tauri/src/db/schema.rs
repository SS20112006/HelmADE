use rusqlite::{Connection, Result};

pub fn init_database(conn: &Connection) -> Result<()> {
    // Pragmas para performance, integridade e proteção contra deadlocks locais
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA synchronous = NORMAL;
         PRAGMA foreign_keys = ON;
         PRAGMA busy_timeout = 2000;"
    )?;

    // Tabela de Projetos Locais
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL UNIQUE,
            created_at INTEGER NOT NULL,
            last_opened_at INTEGER NOT NULL,
            settings_json TEXT
        );",
        [],
    )?;

    // Tabela de Sessões de Enxame
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            layout TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );",
        [],
    )?;

    // Tabela de Grelhas e Painéis PTY Ativos
    conn.execute(
        "CREATE TABLE IF NOT EXISTS grid_panels (
            id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
            panel_index INTEGER NOT NULL,
            agent_role TEXT NOT NULL,
            model_id TEXT NOT NULL,
            worktree_path TEXT,
            worktree_branch TEXT,
            status TEXT NOT NULL,
            created_at INTEGER NOT NULL
        );",
        [],
    )?;

    // Índices concorrentes para consultas rápidas
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_projects_last_opened ON projects(last_opened_at DESC);",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_sessions_project_id ON sessions(project_id);",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_grid_panels_session_id ON grid_panels(session_id);",
        [],
    )?;

    Ok(())
}

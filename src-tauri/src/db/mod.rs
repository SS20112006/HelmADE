pub mod schema;

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub created_at: i64,
    pub last_opened_at: i64,
    pub settings_json: Option<String>,
}

pub struct DbState {
    pub conn: Mutex<Connection>,
}

impl DbState {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        schema::init_database(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    #[cfg(test)]
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        schema::init_database(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn upsert_project(&self, id: &str, name: &str, path: &str) -> Result<Project> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let conn = self.conn.lock().unwrap();

        conn.execute(
            "INSERT INTO projects (id, name, path, created_at, last_opened_at, settings_json)
             VALUES (?1, ?2, ?3, ?4, ?4, NULL)
             ON CONFLICT(path) DO UPDATE SET
                last_opened_at = ?4,
                name = ?2;",
            params![id, name, path, now],
        )?;

        let mut stmt = conn.prepare(
            "SELECT id, name, path, created_at, last_opened_at, settings_json
             FROM projects WHERE path = ?1;",
        )?;

        stmt.query_row(params![path], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                created_at: row.get(3)?,
                last_opened_at: row.get(4)?,
                settings_json: row.get(5)?,
            })
        })
    }

    pub fn list_recent_projects(&self, limit: usize) -> Result<Vec<Project>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, path, created_at, last_opened_at, settings_json
             FROM projects
             ORDER BY last_opened_at DESC
             LIMIT ?1;",
        )?;

        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                created_at: row.get(3)?,
                last_opened_at: row.get(4)?,
                settings_json: row.get(5)?,
            })
        })?;

        let mut projects = Vec::new();
        for proj in rows {
            projects.push(proj?);
        }
        Ok(projects)
    }

    pub fn delete_project(&self, id: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM projects WHERE id = ?1;", params![id])?;
        Ok(affected > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_crud_in_memory() {
        let db = DbState::new_in_memory().expect("failed to init db");

        let p1 = db
            .upsert_project("p1", "HelmADE Core", "/path/to/helmade")
            .expect("failed to insert");
        assert_eq!(p1.name, "HelmADE Core");

        let list = db.list_recent_projects(10).expect("failed to list");
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "p1");

        // Atualizar via UPSERT
        let p1_updated = db
            .upsert_project("p1_alt", "HelmADE Updated", "/path/to/helmade")
            .expect("failed to update");
        assert_eq!(p1_updated.name, "HelmADE Updated");

        let list_after = db.list_recent_projects(10).expect("failed to list");
        assert_eq!(list_after.len(), 1);

        let deleted = db.delete_project(&p1_updated.id).expect("delete failed");
        assert!(deleted);

        let list_empty = db.list_recent_projects(10).expect("failed to list");
        assert_eq!(list_empty.len(), 0);
    }
}

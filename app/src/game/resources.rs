use bevy::prelude::*;
use rusqlite::Connection;
use std::path::PathBuf;
use tauri::AppHandle;

#[derive(Resource)]
pub struct TauriHandle(pub AppHandle);

#[derive(Resource, Default)]
pub struct GameState {
    pub workers_count: u32,
    pub tasks_count: u32,
}

/// Resource for managing projects
#[derive(Resource)]
pub struct ProjectManager {
    pub db_path: PathBuf,
}

impl ProjectManager {
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    pub fn create_project(&self, name: String, path: String) -> Result<String, String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {}", e))?;

        let id = uuid::Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO projects (id, name, path) VALUES (?1, ?2, ?3)",
            [&id, &name, &path],
        ).map_err(|e| format!("Failed to insert project: {}", e))?;

        Ok(id)
    }

    pub fn load_projects(&self) -> Result<Vec<crate::game::project::Project>, String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, name, path, building_theme, total_missions, completed_missions
             FROM projects ORDER BY created_at DESC"
        ).map_err(|e| format!("Query error: {}", e))?;

        let projects = stmt.query_map([], |row| {
            Ok(crate::game::project::Project {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                building_theme: row.get(3)?,
                total_missions: row.get(4)?,
                completed_missions: row.get(5)?,
            })
        }).map_err(|e| format!("Map error: {}", e))?;

        let mut result = Vec::new();
        for project in projects {
            result.push(project.map_err(|e| format!("Row error: {}", e))?);
        }

        Ok(result)
    }

    pub fn update_mission_count(&self, project_id: &str, completed: u32) -> Result<(), String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {}", e))?;

        conn.execute(
            "UPDATE projects SET completed_missions = ?1, last_updated = CURRENT_TIMESTAMP
             WHERE id = ?2",
            [&completed.to_string(), project_id],
        ).map_err(|e| format!("Update error: {}", e))?;

        Ok(())
    }
}

/// Resource for managing workers
#[derive(Resource)]
pub struct WorkerManager {
    pub db_path: PathBuf,
    pub max_workers: usize,
}

impl WorkerManager {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
            max_workers: 20, // Configurable limit
        }
    }

    pub fn create_worker(&self, name: String, color: (f32, f32, f32)) -> Result<String, String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {}", e))?;

        let id = uuid::Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO workers (id, name, color_r, color_g, color_b, state)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [
                &id,
                &name,
                &color.0.to_string(),
                &color.1.to_string(),
                &color.2.to_string(),
                "idle",
            ],
        ).map_err(|e| format!("Insert error: {}", e))?;

        Ok(id)
    }

    pub fn load_workers(&self) -> Result<Vec<crate::game::worker::Worker>, String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, name, color_r, color_g, color_b, state, current_task_id,
                    total_tasks_completed, total_tokens_used
             FROM workers"
        ).map_err(|e| format!("Query error: {}", e))?;

        let workers = stmt.query_map([], |row| {
            Ok(crate::game::worker::Worker {
                id: row.get(0)?,
                name: row.get(1)?,
                color: Color::srgb(row.get(2)?, row.get(3)?, row.get(4)?),
                state: crate::game::worker::WorkerState::from_str(&row.get::<_, String>(5)?),
                current_task_id: row.get(6)?,
                total_tasks_completed: row.get::<_, i32>(7)? as u32,
                total_tokens_used: row.get::<_, i64>(8)? as u64,
            })
        }).map_err(|e| format!("Map error: {}", e))?;

        let mut result = Vec::new();
        for worker in workers {
            result.push(worker.map_err(|e| format!("Row error: {}", e))?);
        }

        Ok(result)
    }

    pub fn update_worker_state(&self, worker_id: &str, state: &crate::game::worker::WorkerState, task_id: Option<&str>) -> Result<(), String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {}", e))?;

        conn.execute(
            "UPDATE workers SET state = ?1, current_task_id = ?2 WHERE id = ?3",
            [state.as_str(), &task_id.unwrap_or(""), worker_id],
        ).map_err(|e| format!("Update error: {}", e))?;

        Ok(())
    }

    pub fn increment_worker_stats(&self, worker_id: &str, tokens: u64) -> Result<(), String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {}", e))?;

        conn.execute(
            "UPDATE workers
             SET total_tasks_completed = total_tasks_completed + 1,
                 total_tokens_used = total_tokens_used + ?1
             WHERE id = ?2",
            [&tokens.to_string(), worker_id],
        ).map_err(|e| format!("Update error: {}", e))?;

        Ok(())
    }

    pub fn count_workers(&self) -> Result<usize, String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {}", e))?;

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM workers",
            [],
            |row| row.get(0)
        ).map_err(|e| format!("Count error: {}", e))?;

        Ok(count as usize)
    }
}

/// Resource for Claude CLI manager
#[derive(Resource)]
pub struct CliManagerResource {
    pub manager: std::sync::Arc<std::sync::Mutex<crate::game::cli::ClaudeCliManager>>,
}

impl CliManagerResource {
    pub fn new(working_dir: PathBuf) -> Self {
        Self {
            manager: std::sync::Arc::new(std::sync::Mutex::new(crate::game::cli::ClaudeCliManager::new(working_dir))),
        }
    }
}

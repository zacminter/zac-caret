use bevy::prelude::*;
use rusqlite::Connection;
use std::path::PathBuf;
use tauri::AppHandle;

#[derive(Resource)]
#[allow(dead_code)]
pub struct TauriHandle(pub AppHandle);

#[derive(Resource, Default)]
pub struct GameState {
    #[allow(dead_code)]
    pub workers_count: u32,
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn create_project(&self, name: String, path: String) -> Result<String, String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {e}"))?;

        let id = uuid::Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO projects (id, name, path) VALUES (?1, ?2, ?3)",
            [&id, &name, &path],
        ).map_err(|e| format!("Failed to insert project: {e}"))?;

        Ok(id)
    }

    pub fn load_projects(&self) -> Result<Vec<crate::game::project::Project>, String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {e}"))?;

        let mut stmt = conn.prepare(
            "SELECT id, name, path, building_theme, total_missions, completed_missions
             FROM projects ORDER BY created_at DESC"
        ).map_err(|e| format!("Query error: {e}"))?;

        let projects = stmt.query_map([], |row| {
            Ok(crate::game::project::Project {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                building_theme: row.get(3)?,
                total_missions: row.get(4)?,
                completed_missions: row.get(5)?,
            })
        }).map_err(|e| format!("Map error: {e}"))?;

        let mut result = Vec::new();
        for project in projects {
            result.push(project.map_err(|e| format!("Row error: {e}"))?);
        }

        Ok(result)
    }

    pub fn update_mission_count(&self, project_id: &str, completed: u32) -> Result<(), String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {e}"))?;

        conn.execute(
            "UPDATE projects SET completed_missions = ?1, last_updated = CURRENT_TIMESTAMP
             WHERE id = ?2",
            [&completed.to_string(), project_id],
        ).map_err(|e| format!("Update error: {e}"))?;

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
            .map_err(|e| format!("Database error: {e}"))?;

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
        ).map_err(|e| format!("Insert error: {e}"))?;

        Ok(id)
    }

    pub fn load_workers(&self) -> Result<Vec<crate::game::worker::Worker>, String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {e}"))?;

        let mut stmt = conn.prepare(
            "SELECT id, name, color_r, color_g, color_b, state, current_task_id,
                    total_tasks_completed, total_tokens_used
             FROM workers"
        ).map_err(|e| format!("Query error: {e}"))?;

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
        }).map_err(|e| format!("Map error: {e}"))?;

        let mut result = Vec::new();
        for worker in workers {
            result.push(worker.map_err(|e| format!("Row error: {e}"))?);
        }

        Ok(result)
    }

    pub fn update_worker_state(&self, worker_id: &str, state: &crate::game::worker::WorkerState, task_id: Option<&str>) -> Result<(), String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {e}"))?;

        conn.execute(
            "UPDATE workers SET state = ?1, current_task_id = ?2 WHERE id = ?3",
            [state.as_str(), task_id.unwrap_or(""), worker_id],
        ).map_err(|e| format!("Update error: {e}"))?;

        Ok(())
    }

    pub fn increment_worker_stats(&self, worker_id: &str, tokens: u64) -> Result<(), String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {e}"))?;

        conn.execute(
            "UPDATE workers
             SET total_tasks_completed = total_tasks_completed + 1,
                 total_tokens_used = total_tokens_used + ?1
             WHERE id = ?2",
            [&tokens.to_string(), worker_id],
        ).map_err(|e| format!("Update error: {e}"))?;

        Ok(())
    }

    pub fn count_workers(&self) -> Result<usize, String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {e}"))?;

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM workers",
            [],
            |row| row.get(0)
        ).map_err(|e| format!("Count error: {e}"))?;

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

/// Resource for controlling autonomous behavior
#[derive(Resource)]
pub struct AutonomySettings {
    pub enabled: bool,
    pub max_concurrent_workers: usize,
    pub assignment_interval_secs: f32,
}

impl Default for AutonomySettings {
    fn default() -> Self {
        Self {
            enabled: false,  // Start disabled
            max_concurrent_workers: 5,
            assignment_interval_secs: 3.0,
        }
    }
}

/// Resource for tracking token usage and budget
#[derive(Resource)]
pub struct TokenBudget {
    pub hourly_limit: u64,
    pub current_period_used: u64,
    pub period_start: chrono::DateTime<chrono::Utc>,
    pub period_duration_hours: i64,
    pub warning_threshold: f32,  // 0.0 to 1.0
}

impl TokenBudget {
    pub fn new(hourly_limit: u64) -> Self {
        Self {
            hourly_limit,
            current_period_used: 0,
            period_start: chrono::Utc::now(),
            period_duration_hours: 1,
            warning_threshold: 0.2,  // Warn at 20% remaining
        }
    }

    #[allow(dead_code)]
    pub fn add_usage(&mut self, tokens: u64) {
        self.current_period_used += tokens;
    }

    pub fn remaining(&self) -> u64 {
        self.hourly_limit.saturating_sub(self.current_period_used)
    }

    pub fn percentage_used(&self) -> f32 {
        if self.hourly_limit == 0 {
            return 0.0;
        }
        (self.current_period_used as f32 / self.hourly_limit as f32) * 100.0
    }

    pub fn percentage_remaining(&self) -> f32 {
        100.0 - self.percentage_used()
    }

    pub fn is_depleted(&self) -> bool {
        self.current_period_used >= self.hourly_limit
    }

    pub fn is_low(&self) -> bool {
        self.percentage_remaining() / 100.0 <= self.warning_threshold
    }

    pub fn time_until_reset(&self) -> chrono::Duration {
        let next_reset = self.period_start + chrono::Duration::hours(self.period_duration_hours);
        next_reset - chrono::Utc::now()
    }

    pub fn should_reset(&self) -> bool {
        chrono::Utc::now() >= self.period_start + chrono::Duration::hours(self.period_duration_hours)
    }

    pub fn reset_period(&mut self) {
        self.current_period_used = 0;
        self.period_start = chrono::Utc::now();
        println!("🔄 Token budget reset! New period started.");
    }

    pub fn estimated_burn_rate_per_hour(&self) -> f32 {
        let elapsed_hours = (chrono::Utc::now() - self.period_start).num_minutes() as f32 / 60.0;

        if elapsed_hours < 0.1 {
            return 0.0;
        }

        self.current_period_used as f32 / elapsed_hours
    }
}

impl Default for TokenBudget {
    fn default() -> Self {
        Self::new(50000)  // Default 50k tokens per hour
    }
}

/// Resource for game statistics displayed in HUD
#[derive(Resource, Default)]
pub struct GameStats {
    pub workers_total: usize,
    pub workers_idle: usize,
    pub workers_working: usize,
    pub tasks_in_progress: usize,
    pub tasks_completed_session: usize,
    pub projects_total: usize,
}

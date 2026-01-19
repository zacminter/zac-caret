use rusqlite::{Connection, Result};
use std::path::Path;
use tauri::{AppHandle, Manager};

pub fn _init(app_handle: &AppHandle) -> Result<()> {
    let app_dir = app_handle.path().app_data_dir()
        .expect("Failed to get app data directory");
    std::fs::create_dir_all(&app_dir).ok();

    let db_path = app_dir.join("zac.db");
    let conn = init_database(&db_path)?;

    // Store connection in app state if needed later
    // For now, just initialize the schema
    drop(conn);
    Ok(())
}

pub fn init_database(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)?;

    conn.execute_batch(r#"
        -- Workers
        CREATE TABLE IF NOT EXISTS workers (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            color_r REAL DEFAULT 0.5,
            color_g REAL DEFAULT 0.5,
            color_b REAL DEFAULT 0.5,
            state TEXT DEFAULT 'idle',
            current_task_id TEXT,
            total_tasks_completed INTEGER DEFAULT 0,
            total_tokens_used INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- Projects
        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL UNIQUE,
            building_theme TEXT DEFAULT 'generic',
            visual_stage INTEGER DEFAULT 0,
            position_x REAL DEFAULT 0.0,
            position_y REAL DEFAULT 0.0,
            position_z REAL DEFAULT 0.0,
            total_missions INTEGER DEFAULT 0,
            completed_missions INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            last_updated DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- Missions (renamed from tasks for M4/M5)
        CREATE TABLE IF NOT EXISTS missions (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            mission_number INTEGER NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT DEFAULT 'not_started',
            dependencies TEXT,
            file_path TEXT,
            assigned_worker_id TEXT,
            started_at DATETIME,
            completed_at DATETIME,
            tokens_used INTEGER DEFAULT 0,
            completion_summary TEXT,
            FOREIGN KEY (project_id) REFERENCES projects(id),
            UNIQUE(project_id, mission_number)
        );

        -- Knowledge base for accumulated learnings
        CREATE TABLE IF NOT EXISTS knowledge_entries (
            id TEXT PRIMARY KEY,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            project_name TEXT,
            project_type TEXT,
            mission_id TEXT,
            task_description TEXT,
            problem_encountered TEXT,
            solution_applied TEXT,
            code_patterns TEXT,
            reasoning TEXT,
            files_modified TEXT,
            tokens_used INTEGER,
            duration_seconds INTEGER,
            worker_id TEXT,
            success BOOLEAN,
            tags TEXT,
            search_keywords TEXT
        );

        -- Index for searching
        CREATE INDEX IF NOT EXISTS idx_knowledge_search
         ON knowledge_entries(search_keywords);

        -- App state (key-value for camera, settings, etc)
        CREATE TABLE IF NOT EXISTS app_state (
            key TEXT PRIMARY KEY,
            value_json TEXT NOT NULL
        );

        -- Zac journal
        CREATE TABLE IF NOT EXISTS zac_journal (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            entry_type TEXT NOT NULL,
            content TEXT NOT NULL,
            related_project_id TEXT,
            related_task_id TEXT
        );

        -- Sessions
        CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            ended_at DATETIME,
            tasks_completed INTEGER DEFAULT 0,
            tokens_used INTEGER DEFAULT 0,
            summary TEXT
        );
    "#)?;

    Ok(conn)
}

pub fn save_state(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO app_state (key, value_json) VALUES (?1, ?2)",
        [key, value],
    )?;
    Ok(())
}

pub fn load_state(conn: &Connection, key: &str) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT value_json FROM app_state WHERE key = ?1")?;
    let result: Result<String> = stmt.query_row([key], |row| row.get(0));
    match result {
        Ok(val) => Ok(Some(val)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

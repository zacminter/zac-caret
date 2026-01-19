use bevy::prelude::*;
use rusqlite::Connection;
use std::path::PathBuf;
use crate::game::project::{Mission, MissionStatus};

/// Resource for managing missions
#[derive(Resource)]
pub struct MissionManager {
    pub db_path: PathBuf,
}

impl MissionManager {
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    pub fn create_mission(&self, mission: &Mission) -> Result<(), String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {}", e))?;

        let deps_json = serde_json::to_string(&mission.dependencies)
            .map_err(|e| format!("JSON error: {}", e))?;

        conn.execute(
            "INSERT INTO missions (id, project_id, mission_number, title, description, status, dependencies, file_path)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            [
                &mission.id,
                &mission.project_id,
                &mission.mission_number.to_string(),
                &mission.title,
                &mission.description,
                mission.status.as_str(),
                &deps_json,
                &mission.file_path.clone().unwrap_or_default(),
            ],
        ).map_err(|e| format!("Insert error: {}", e))?;

        Ok(())
    }

    pub fn load_missions(&self, project_id: &str) -> Result<Vec<Mission>, String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, project_id, mission_number, title, description, status,
                    dependencies, file_path, assigned_worker_id, tokens_used, completion_summary
             FROM missions WHERE project_id = ?1 ORDER BY mission_number ASC"
        ).map_err(|e| format!("Query error: {}", e))?;

        let missions = stmt.query_map([project_id], |row| {
            let deps_str: String = row.get(6)?;
            let dependencies: Vec<u32> = serde_json::from_str(&deps_str).unwrap_or_default();

            Ok(Mission {
                id: row.get(0)?,
                project_id: row.get(1)?,
                mission_number: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                status: MissionStatus::from_str(&row.get::<_, String>(5)?),
                dependencies,
                file_path: row.get(7)?,
                assigned_worker_id: row.get(8)?,
                tokens_used: row.get::<_, i32>(9)? as u32,
                completion_summary: row.get(10)?,
            })
        }).map_err(|e| format!("Map error: {}", e))?;

        let mut result = Vec::new();
        for mission in missions {
            result.push(mission.map_err(|e| format!("Row error: {}", e))?);
        }

        Ok(result)
    }

    pub fn update_mission_status(&self, mission_id: &str, status: MissionStatus, summary: Option<String>, tokens: u32) -> Result<(), String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Database error: {}", e))?;

        conn.execute(
            "UPDATE missions
             SET status = ?1, completion_summary = ?2, tokens_used = ?3, completed_at = CURRENT_TIMESTAMP
             WHERE id = ?4",
            [status.as_str(), &summary.unwrap_or_default(), &tokens.to_string(), mission_id],
        ).map_err(|e| format!("Update error: {}", e))?;

        Ok(())
    }

    pub fn get_available_missions(&self, project_id: &str) -> Result<Vec<Mission>, String> {
        let all_missions = self.load_missions(project_id)?;

        // Get completed mission numbers
        let completed: Vec<u32> = all_missions.iter()
            .filter(|m| m.status == MissionStatus::Completed)
            .map(|m| m.mission_number)
            .collect();

        // Filter available missions
        Ok(all_missions.into_iter()
            .filter(|m| m.is_available(&completed))
            .collect())
    }
}

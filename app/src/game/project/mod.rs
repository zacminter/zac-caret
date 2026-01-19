use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Component representing a software project as a building
#[derive(Component, Debug, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub building_theme: String,
    pub total_missions: u32,
    pub completed_missions: u32,
}

impl Project {
    #[allow(dead_code)]
    pub fn new(name: String, path: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            path,
            building_theme: "generic".to_string(),
            total_missions: 0,
            completed_missions: 0,
        }
    }

    #[allow(dead_code)]
    pub fn completion_percentage(&self) -> f32 {
        if self.total_missions == 0 {
            return 0.0;
        }
        (self.completed_missions as f32 / self.total_missions as f32) * 100.0
    }

    pub fn visual_stage(&self) -> u8 {
        if self.total_missions == 0 {
            return 0;
        }
        // Map completion to stages 0-10
        let ratio = self.completed_missions as f32 / self.total_missions as f32;
        (ratio * 10.0).floor() as u8
    }
}

/// Mission status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MissionStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
    Blocked,
}

impl MissionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            MissionStatus::NotStarted => "not_started",
            MissionStatus::InProgress => "in_progress",
            MissionStatus::Completed => "completed",
            MissionStatus::Failed => "failed",
            MissionStatus::Blocked => "blocked",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "in_progress" => MissionStatus::InProgress,
            "completed" => MissionStatus::Completed,
            "failed" => MissionStatus::Failed,
            "blocked" => MissionStatus::Blocked,
            _ => MissionStatus::NotStarted,
        }
    }
}

/// Mission data structure
#[derive(Debug, Clone)]
pub struct Mission {
    pub id: String,
    pub project_id: String,
    pub mission_number: u32,
    pub title: String,
    pub description: String,
    pub status: MissionStatus,
    pub dependencies: Vec<u32>, // Mission numbers this depends on
    pub file_path: Option<String>,
    #[allow(dead_code)]
    pub assigned_worker_id: Option<String>,
    #[allow(dead_code)]
    pub tokens_used: u32,
    #[allow(dead_code)]
    pub completion_summary: Option<String>,
}

impl Mission {
    #[allow(dead_code)]
    pub fn new(project_id: String, mission_number: u32, title: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            project_id,
            mission_number,
            title,
            description: String::new(),
            status: MissionStatus::NotStarted,
            dependencies: Vec::new(),
            file_path: None,
            assigned_worker_id: None,
            tokens_used: 0,
            completion_summary: None,
        }
    }

    pub fn is_available(&self, completed_missions: &[u32]) -> bool {
        if self.status != MissionStatus::NotStarted {
            return false;
        }
        // Check all dependencies are completed
        self.dependencies.iter().all(|dep| completed_missions.contains(dep))
    }
}

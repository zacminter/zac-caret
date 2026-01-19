use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Worker entity - represents a Claude Code CLI instance
#[derive(Component, Debug, Clone)]
pub struct Worker {
    pub id: String,
    pub name: String,
    pub color: Color,
    pub state: WorkerState,
    pub current_task_id: Option<String>,
    pub total_tasks_completed: u32,
    pub total_tokens_used: u64,
}

impl Worker {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            color: Self::random_color(),
            state: WorkerState::Idle,
            current_task_id: None,
            total_tasks_completed: 0,
            total_tokens_used: 0,
        }
    }

    fn random_color() -> Color {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Color::srgb(
            rng.gen_range(0.3..0.9),
            rng.gen_range(0.3..0.9),
            rng.gen_range(0.3..0.9),
        )
    }
}

/// Worker state machine
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkerState {
    /// At leisure zone, resting
    Idle,

    /// Ready to receive task assignment
    Ready,

    /// Walking to a building
    MovingTo { target: Vec3 },

    /// Executing a mission via Claude CLI
    Working {
        mission_id: String,
        started_at: String,  // ISO timestamp
    },

    /// Post-task reflection (optional in V1)
    Reflecting,

    /// Error state - needs attention
    Crashed {
        error: String,
        last_mission_id: String,
    },
}

impl WorkerState {
    pub fn as_str(&self) -> &str {
        match self {
            WorkerState::Idle => "idle",
            WorkerState::Ready => "ready",
            WorkerState::MovingTo { .. } => "moving",
            WorkerState::Working { .. } => "working",
            WorkerState::Reflecting => "reflecting",
            WorkerState::Crashed { .. } => "crashed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "ready" => WorkerState::Ready,
            "moving" => WorkerState::MovingTo { target: Vec3::ZERO },
            "reflecting" => WorkerState::Reflecting,
            _ => WorkerState::Idle,
        }
    }
}

/// Component for worker visual representation
#[derive(Component)]
pub struct WorkerVisual;

/// Component for worker name display
#[derive(Component)]
pub struct WorkerNameTag;

/// Random name generator
pub struct NameGenerator;

impl NameGenerator {
    const FIRST_NAMES: &'static [&'static str] = &[
        "Alex", "Blake", "Casey", "Dakota", "Ellis", "Finley", "Gray",
        "Harper", "Indigo", "Jordan", "Kennedy", "Logan", "Morgan",
        "Nico", "Oakley", "Parker", "Quinn", "Riley", "Sage", "Taylor",
    ];

    pub fn random_name() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let first = Self::FIRST_NAMES[rng.gen_range(0..Self::FIRST_NAMES.len())];
        format!("{}", first)
    }
}

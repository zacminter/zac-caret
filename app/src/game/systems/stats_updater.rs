use bevy::prelude::*;
use crate::game::resources::GameStats;
use crate::game::worker::{Worker, WorkerState};
use crate::game::project::Project;

/// System to update game statistics
pub fn update_game_stats(
    worker_query: Query<&Worker>,
    project_query: Query<&Project>,
    mut stats: ResMut<GameStats>,
    time: Res<Time>,
    mut last_update: Local<f32>,
) {
    *last_update += time.delta_seconds();

    // Update every second
    if *last_update < 1.0 {
        return;
    }
    *last_update = 0.0;

    stats.workers_total = worker_query.iter().count();

    stats.workers_idle = worker_query.iter()
        .filter(|w| w.state == WorkerState::Idle || w.state == WorkerState::Ready)
        .count();

    stats.workers_working = worker_query.iter()
        .filter(|w| matches!(w.state, WorkerState::Working { .. }))
        .count();

    stats.tasks_completed_session = worker_query.iter()
        .map(|w| w.total_tasks_completed as usize)
        .sum();

    stats.projects_total = project_query.iter().count();
}

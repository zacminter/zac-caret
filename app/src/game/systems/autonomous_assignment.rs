use bevy::prelude::*;
use crate::game::worker::{Worker, WorkerState};
use crate::game::project::Project;
use crate::game::systems::{MissionManager, MovementTarget};
use crate::game::resources::{WorkerManager, AutonomySettings};

/// System to automatically assign idle workers to available missions
pub fn autonomous_task_assignment(
    mut commands: Commands,
    mut worker_query: Query<(Entity, &mut Worker, &Transform)>,
    project_query: Query<(&Project, &Transform)>,
    mission_manager: Res<MissionManager>,
    worker_manager: Res<WorkerManager>,
    autonomy: Res<AutonomySettings>,
    time: Res<Time>,
    mut last_assignment: Local<f32>,
) {
    if !autonomy.enabled {
        return;
    }

    *last_assignment += time.delta_seconds();

    // Check every N seconds
    if *last_assignment < autonomy.assignment_interval_secs {
        return;
    }
    *last_assignment = 0.0;

    // Count active workers
    let active_workers = worker_query.iter()
        .filter(|(_, w, _)| matches!(w.state, WorkerState::Working { .. } | WorkerState::MovingTo { .. }))
        .count();

    if active_workers >= autonomy.max_concurrent_workers {
        // At capacity
        return;
    }

    // Find idle/ready workers
    let mut idle_workers: Vec<_> = worker_query.iter_mut()
        .filter(|(_, w, _)| w.state == WorkerState::Idle || w.state == WorkerState::Ready)
        .collect();

    if idle_workers.is_empty() {
        return;
    }

    // Build priority queue of available missions
    let mut mission_candidates: Vec<(String, String, Vec3, f32)> = Vec::new();

    for (project, project_transform) in project_query.iter() {
        let available_missions = match mission_manager.get_available_missions(&project.id) {
            Ok(missions) if !missions.is_empty() => missions,
            _ => continue,
        };

        for mission in available_missions {
            // Calculate priority score
            let priority = calculate_mission_priority(
                project.completed_missions,
                project.total_missions,
                mission.mission_number,
            );

            mission_candidates.push((
                mission.id.clone(),
                project.id.clone(),
                project_transform.translation,
                priority,
            ));
        }
    }

    // Sort by priority (highest first)
    mission_candidates.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap());

    // Assign workers to top missions
    for (mission_id, _project_id, building_pos, priority) in mission_candidates.iter() {
        if idle_workers.is_empty() {
            break;
        }

        if let Some((worker_entity, mut worker, _)) = idle_workers.pop() {
            // Assign worker to this mission
            commands.entity(worker_entity).insert(MovementTarget::new(*building_pos));

            worker.state = WorkerState::MovingTo { target: *building_pos };
            worker.current_task_id = Some(mission_id.clone());

            let _ = worker_manager.update_worker_state(
                &worker.id,
                &worker.state,
                Some(mission_id)
            );

            println!("🤖 Zac^ AUTO-ASSIGNED worker '{}' to mission (priority: {:.2})",
                     worker.name, priority);
        }
    }
}

/// Calculate mission priority score (higher = more urgent)
fn calculate_mission_priority(
    completed: u32,
    total: u32,
    mission_number: u32,
) -> f32 {
    let mut score = 100.0;

    // Prefer projects closer to completion milestones
    let completion_pct = if total > 0 {
        completed as f32 / total as f32
    } else {
        0.0
    };

    // Projects near stage boundaries get priority boost
    let stage = (completion_pct * 10.0).floor();
    let distance_to_next_stage = ((stage + 1.0) / 10.0) - completion_pct;

    if distance_to_next_stage < 0.1 {
        score += 50.0; // Close to upgrade!
    }

    // Earlier missions slightly preferred (unblock dependencies)
    score -= mission_number as f32 * 0.5;

    score
}

/// System to toggle autonomy with 'Z' key
pub fn toggle_autonomy_keypress(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut autonomy: ResMut<AutonomySettings>,
) {
    if keyboard.just_pressed(KeyCode::KeyZ) {
        autonomy.enabled = !autonomy.enabled;

        if autonomy.enabled {
            println!("🤖 ZAC^ AUTONOMY ENABLED - Foreman is now assigning tasks automatically");
        } else {
            println!("⏸️  ZAC^ AUTONOMY DISABLED - Manual control restored");
        }
    }
}

/// System to show autonomy status
pub fn display_autonomy_status(
    autonomy: Res<AutonomySettings>,
    worker_query: Query<&Worker>,
    time: Res<Time>,
    mut last_display: Local<f32>,
) {
    if !autonomy.enabled {
        return;
    }

    *last_display += time.delta_seconds();

    // Display every 10 seconds
    if *last_display < 10.0 {
        return;
    }
    *last_display = 0.0;

    let idle_count = worker_query.iter()
        .filter(|w| w.state == WorkerState::Idle || w.state == WorkerState::Ready)
        .count();

    let working_count = worker_query.iter()
        .filter(|w| matches!(w.state, WorkerState::Working { .. }))
        .count();

    println!("🤖 Zac^ Status: {} idle, {} working, autonomy {}",
             idle_count, working_count,
             if autonomy.enabled { "ON" } else { "OFF" });
}

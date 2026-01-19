use bevy::prelude::*;
use crate::game::worker::{Worker, WorkerState};
use crate::game::project::Project;
use crate::game::systems::{MissionManager, MovementTarget};
use crate::game::resources::{WorkerManager, CliManagerResource, ProjectManager};
use crate::game::systems::mission_writer::MissionWriter;

/// Temporary: Assign worker to project on 'A' key
pub fn assign_worker_on_keypress(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut worker_query: Query<(Entity, &mut Worker, &Transform)>,
    project_query: Query<(&Project, &Transform)>,
    mission_manager: Res<MissionManager>,
    worker_manager: Res<WorkerManager>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::KeyA) {
        // Find first idle/ready worker
        let worker_result = worker_query.iter_mut()
            .find(|(_, w, _)| w.state == WorkerState::Idle || w.state == WorkerState::Ready)
            .map(|(e, w, t)| (e, w.clone(), t.translation));

        if let Some((worker_entity, worker, _worker_pos)) = worker_result {
            // Find first project with available missions
            for (project, project_transform) in project_query.iter() {
                let available_missions = match mission_manager.get_available_missions(&project.id) {
                    Ok(missions) if !missions.is_empty() => missions,
                    _ => continue,
                };

                let mission = &available_missions[0];

                // Send worker to project building
                commands.entity(worker_entity).insert(MovementTarget::new(project_transform.translation));

                if let Ok((_, mut w, _)) = worker_query.get_mut(worker_entity) {
                    w.state = WorkerState::MovingTo { target: project_transform.translation };
                    w.current_task_id = Some(mission.id.clone());
                }

                // Update database
                let _ = worker_manager.update_worker_state(&worker.id, &WorkerState::MovingTo { target: project_transform.translation }, Some(&mission.id));

                println!("📋 Assigned worker '{}' to mission: {}", worker.name, mission.title);
                println!("   Worker walking to project: {}", project.name);

                return;
            }

            println!("⚠️ No available missions found!");
        } else {
            println!("⚠️ No idle workers available!");
        }
    }
}

/// System to start mission when worker arrives at building
pub fn start_mission_on_arrival(
    mut worker_query: Query<(Entity, &mut Worker, &Transform), Changed<Worker>>,
    project_query: Query<(&Project, &Transform)>,
    mission_manager: Res<MissionManager>,
    worker_manager: Res<WorkerManager>,
    cli_manager: Res<CliManagerResource>,
) {
    for (_entity, mut worker, worker_transform) in worker_query.iter_mut() {
        // Check if worker just became Ready (arrived at destination)
        if worker.state != WorkerState::Ready {
            continue;
        }

        if let Some(mission_id) = &worker.current_task_id {
            // Find which project this mission belongs to
            for (project, project_transform) in project_query.iter() {
                let missions = match mission_manager.load_missions(&project.id) {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                if let Some(mission) = missions.iter().find(|m| &m.id == mission_id) {
                    // Check if worker is near the project
                    let distance = worker_transform.translation.distance(project_transform.translation);

                    if distance < 3.0 {
                        // Start the mission!
                        println!("🎬 Starting mission: {}", mission.title);

                        // Generate/update mission file
                        let mission_file = match MissionWriter::write_mission_file(mission, &project.path) {
                            Ok(path) => path,
                            Err(e) => {
                                eprintln!("Failed to write mission file: {e}");
                                continue;
                            }
                        };

                        // Mark as started
                        let _ = MissionWriter::mark_mission_started(&mission_file, &worker.name);

                        // Spawn Claude CLI process
                        let mut cli_lock = cli_manager.manager.lock().unwrap();
                        match cli_lock.spawn_for_mission(
                            worker.id.clone(),
                            mission.id.clone(),
                            &project.path,
                            &mission_file,
                        ) {
                            Ok(process_id) => {
                                println!("✅ Claude CLI spawned (process: {process_id})");

                                // Update worker state
                                worker.state = WorkerState::Working {
                                    mission_id: mission.id.clone(),
                                    started_at: chrono::Local::now().to_rfc3339(),
                                };

                                let _ = worker_manager.update_worker_state(
                                    &worker.id,
                                    &worker.state,
                                    Some(&mission.id)
                                );

                                // Update mission status in database
                                let _ = mission_manager.update_mission_status(
                                    &mission.id,
                                    crate::game::project::MissionStatus::InProgress,
                                    None,
                                    0,
                                );
                            }
                            Err(e) => {
                                eprintln!("❌ Failed to spawn Claude CLI: {e}");

                                // Reset worker to idle
                                worker.state = WorkerState::Idle;
                                worker.current_task_id = None;
                            }
                        }

                        break;
                    }
                }
            }
        }
    }
}

/// System to check for completed CLI processes
pub fn check_cli_completions(
    mut worker_query: Query<&mut Worker>,
    mission_manager: Res<MissionManager>,
    worker_manager: Res<WorkerManager>,
    project_manager: Res<ProjectManager>,
    cli_manager: Res<CliManagerResource>,
) {
    let completions = {
        let mut cli_lock = cli_manager.manager.lock().unwrap();
        cli_lock.check_completions()
    };

    for completion in completions {
        println!("🎉 Mission completed by worker: {}", completion.worker_id);
        println!("   Duration: {} seconds", completion.duration_secs);
        println!("   Success: {}", completion.success);

        // Extract tokens and summary
        let tokens = completion.extract_tokens();
        let summary = completion.extract_summary();

        println!("   Tokens used: {tokens}");
        println!("   Summary: {summary}");

        // Update mission status
        let status = if completion.success {
            crate::game::project::MissionStatus::Completed
        } else {
            crate::game::project::MissionStatus::Failed
        };

        let _ = mission_manager.update_mission_status(
            &completion.mission_id,
            status.clone(),
            Some(summary.clone()),
            tokens,
        );

        // Update worker stats
        if completion.success {
            let _ = worker_manager.increment_worker_stats(&completion.worker_id, tokens as u64);
        }

        // Update worker state
        for mut worker in worker_query.iter_mut() {
            if worker.id == completion.worker_id {
                worker.state = WorkerState::Idle;
                worker.current_task_id = None;
                worker.total_tasks_completed += 1;
                worker.total_tokens_used += tokens as u64;

                let _ = worker_manager.update_worker_state(
                    &worker.id,
                    &WorkerState::Idle,
                    None,
                );

                println!("   Worker '{}' now idle (total tasks: {})", worker.name, worker.total_tasks_completed);
            }
        }

        // Update project completion count
        if completion.success {
            // Find project for this mission - load all missions to find project_id
            if let Ok(all_missions) = mission_manager.load_missions("") {
                if let Some(mission) = all_missions.iter().find(|m| m.id == completion.mission_id) {
                    // Count completed missions for this project
                    let completed_count = all_missions.iter()
                        .filter(|m| m.project_id == mission.project_id
                                && m.status == crate::game::project::MissionStatus::Completed)
                        .count() as u32;

                    let _ = project_manager.update_mission_count(&mission.project_id, completed_count);

                    let total_project_missions = all_missions.iter()
                        .filter(|m| m.project_id == mission.project_id)
                        .count();

                    println!("   Project progress: {completed_count}/{total_project_missions} missions complete");
                }
            }
        }
    }
}

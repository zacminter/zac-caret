use bevy::prelude::*;
use crate::game::project::Project;
use crate::game::components::StagedBuilding;
use crate::game::resources::ProjectManager;

/// System that checks for mission completions and updates building stages
pub fn track_project_progress(
    mut project_query: Query<(&mut Project, &mut StagedBuilding)>,
    _project_manager: Res<ProjectManager>,
) {
    for (mut project, mut building) in project_query.iter_mut() {
        // Calculate expected stage based on mission completion
        let expected_stage = project.visual_stage();
        let current_stage = building.current_stage.as_u8();

        if expected_stage > current_stage {
            // Upgrade building
            building.set_stage(expected_stage);
            println!("🎉 Project '{}' upgraded to stage {}!", project.name, expected_stage);
        }
    }
}

/// System to sync project data from database periodically
pub fn sync_project_data(
    mut project_query: Query<&mut Project>,
    project_manager: Res<ProjectManager>,
    time: Res<Time>,
    mut last_sync: Local<f32>,
) {
    *last_sync += time.delta_seconds();

    // Sync every 5 seconds
    if *last_sync < 5.0 {
        return;
    }
    *last_sync = 0.0;

    let projects = match project_manager.load_projects() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to sync projects: {}", e);
            return;
        }
    };

    for mut project in project_query.iter_mut() {
        if let Some(db_project) = projects.iter().find(|p| p.id == project.id) {
            project.completed_missions = db_project.completed_missions;
            project.total_missions = db_project.total_missions;
        }
    }
}

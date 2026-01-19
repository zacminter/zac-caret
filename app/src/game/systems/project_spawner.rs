use bevy::prelude::*;
use crate::game::project::Project;
use crate::game::components::{StagedBuilding, BuildingType};
use crate::game::resources::ProjectManager;

/// System to spawn project buildings in the world
pub fn spawn_project_buildings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    project_manager: Res<ProjectManager>,
    existing_projects: Query<&Project>,
) {
    // Load projects from database
    let projects = match project_manager.load_projects() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to load projects: {}", e);
            return;
        }
    };

    // Check which projects are already spawned
    let existing_ids: Vec<String> = existing_projects.iter()
        .map(|p| p.id.clone())
        .collect();

    // Spawn new projects
    for (index, project) in projects.iter().enumerate() {
        if existing_ids.contains(&project.id) {
            continue; // Already spawned
        }

        // Calculate position in spiral pattern
        let position = calculate_building_position(index);

        // Determine visual stage
        let stage = project.visual_stage();

        // Spawn building entity
        let building_entity = commands
            .spawn((
                Project {
                    id: project.id.clone(),
                    name: project.name.clone(),
                    path: project.path.clone(),
                    building_theme: project.building_theme.clone(),
                    total_missions: project.total_missions,
                    completed_missions: project.completed_missions,
                },
                StagedBuilding::new(BuildingType::TownHall, stage),
                SpatialBundle::from_transform(Transform::from_translation(position)),
                Name::new(format!("Project: {}", project.name)),
            ))
            .id();

        // Add visual mesh
        let building_stage = crate::game::entities::building_stage::BuildingStage::from_u8(stage);
        let mesh = building_stage.generate_mesh();
        let color = building_stage.get_color();

        let mesh_entity = commands
            .spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    ..default()
                }),
                ..default()
            })
            .id();

        commands.entity(building_entity).add_child(mesh_entity);

        println!("Spawned project: {} at {:?}", project.name, position);
    }
}

/// Calculate building position in spiral pattern around origin
fn calculate_building_position(index: usize) -> Vec3 {
    let base_distance = 15.0;
    let angle_step = std::f32::consts::PI * 0.618; // Golden angle for nice spacing

    let angle = angle_step * index as f32;
    let distance = base_distance + (index as f32 * 3.0);

    Vec3::new(
        angle.cos() * distance,
        0.0,
        angle.sin() * distance,
    )
}

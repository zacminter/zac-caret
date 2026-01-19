use bevy::prelude::*;
use crate::game::components::StagedBuilding;

/// System to update building visuals when stage changes
pub fn update_building_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &StagedBuilding), Changed<StagedBuilding>>,
    children_query: Query<&Children>,
) {
    for (entity, staged_building) in query.iter() {
        // Remove old mesh children
        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                commands.entity(*child).despawn_recursive();
            }
        }

        // Generate new mesh for current stage
        let mesh = staged_building.current_stage.generate_mesh();
        let color = staged_building.current_stage.get_color();

        // Spawn new mesh as child
        let mesh_entity = commands
            .spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            })
            .id();

        commands.entity(entity).add_child(mesh_entity);
    }
}

/// Startup system to initialize town hall
pub fn spawn_initial_town_hall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use crate::game::entities::town_hall::TownHall as TownHallData;
    use crate::game::systems::TownHall as TownHallProduction;

    // Spawn at stage 4 (complete basic structure) for M2 demo
    let town_hall_entity = TownHallData::spawn(&mut commands, 4);

    // Add production component for M6
    commands.entity(town_hall_entity).insert(TownHallProduction::new());

    // Add initial visual
    let stage = crate::game::entities::building_stage::BuildingStage::from_u8(4);
    let mesh = stage.generate_mesh();
    let color = stage.get_color();

    let mesh_entity = commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: color,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .id();

    commands.entity(town_hall_entity).add_child(mesh_entity);
}

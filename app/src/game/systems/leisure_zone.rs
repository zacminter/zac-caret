use bevy::prelude::*;

/// Marker component for the leisure zone
#[derive(Component)]
pub struct LeisureZone {
    pub center: Vec3,
    pub radius: f32,
}

/// Spawn the leisure zone area
pub fn spawn_leisure_zone(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let center = Vec3::new(-20.0, 0.0, -20.0);
    let radius = 8.0;

    // Visual marker (green circle)
    commands.spawn((
        LeisureZone { center, radius },
        PbrBundle {
            mesh: meshes.add(Circle::new(radius)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgba(0.3, 0.8, 0.3, 0.3),
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_translation(center + Vec3::new(0.0, 0.1, 0.0))
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            ..default()
        },
    ));

    println!("Leisure zone spawned at {center:?}");
}

/// Get a random position within the leisure zone
pub fn random_leisure_position(zone: &LeisureZone) -> Vec3 {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let distance = rng.gen_range(0.0..zone.radius);

    zone.center + Vec3::new(
        angle.cos() * distance,
        0.0,
        angle.sin() * distance,
    )
}

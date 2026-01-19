use bevy::prelude::*;

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane - the meadow
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.55, 0.25), // Grass green
            perceptual_roughness: 0.9,
            ..default()
        }),
        ..default()
    });

    // Directional light (sun)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(50.0, 80.0, 50.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ambient light for softer shadows
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.9, 0.95, 1.0), // Slight blue tint
        brightness: 200.0,
    });

    // Proto Town Hall (debug cube)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(3.0, 4.0, 3.0)),
            material: materials.add(Color::srgb(0.6, 0.4, 0.2)), // Wood brown
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        Name::new("Town Hall"),
    ));
}

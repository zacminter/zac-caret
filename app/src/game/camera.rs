use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

#[derive(Resource)]
pub struct CameraSettings {
    pub pan_speed: f32,
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            pan_speed: 20.0,
            zoom_speed: 5.0,
            min_zoom: 10.0,
            max_zoom: 80.0,
        }
    }
}

pub fn camera_controller(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scroll: EventReader<MouseWheel>,
    settings: Res<CameraSettings>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let mut transform = query.single_mut();
    let delta = time.delta_seconds();

    // WASD panning
    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        transform.translation += direction.normalize() * settings.pan_speed * delta;
    }

    // Scroll zoom
    for event in scroll.read() {
        let zoom_delta = -event.y * settings.zoom_speed;
        let forward = transform.forward();
        let new_pos = transform.translation + forward * zoom_delta;
        
        // Clamp zoom
        if new_pos.y > settings.min_zoom && new_pos.y < settings.max_zoom {
            transform.translation = new_pos;
        }
    }
}

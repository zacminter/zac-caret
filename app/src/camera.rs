use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct CameraSettings {
    pub pan_speed: f32,
    pub zoom_speed: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            pan_speed: 25.0,
            zoom_speed: 10.0,
            min_height: 8.0,
            max_height: 60.0,
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct CameraState {
    pub position: [f32; 3],
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            position: [0.0, 25.0, 35.0],
        }
    }
}

impl CameraState {
    pub fn from_transform(transform: &Transform) -> Self {
        Self {
            position: transform.translation.to_array(),
        }
    }

    pub fn to_transform(&self) -> Transform {
        Transform::from_xyz(self.position[0], self.position[1], self.position[2])
            .looking_at(Vec3::ZERO, Vec3::Y)
    }
}

pub fn spawn_camera_from_state(
    mut commands: Commands,
    state: Res<CameraState>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: state.to_transform(),
            ..default()
        },
        MainCamera,
    ));
}

pub fn camera_pan(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<CameraSettings>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(mut transform) = query.get_single_mut() else { return };

    let delta = time.delta_seconds();
    let speed = settings.pan_speed;

    // Get camera's forward and right vectors (flattened to XZ plane)
    let forward = Vec3::new(transform.forward().x, 0.0, transform.forward().z).normalize_or_zero();
    let right = Vec3::new(transform.right().x, 0.0, transform.right().z).normalize_or_zero();

    let mut movement = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        movement += forward;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        movement -= forward;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        movement -= right;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        movement += right;
    }

    if movement != Vec3::ZERO {
        transform.translation += movement.normalize() * speed * delta;
    }
}

pub fn camera_zoom(
    mut scroll: EventReader<MouseWheel>,
    settings: Res<CameraSettings>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(mut transform) = query.get_single_mut() else { return };

    for event in scroll.read() {
        // Zoom by moving along the camera's forward vector
        let zoom_delta = event.y * settings.zoom_speed;
        let forward = transform.forward();
        let new_pos = transform.translation + forward * zoom_delta;

        // Clamp by height (Y position)
        if new_pos.y >= settings.min_height && new_pos.y <= settings.max_height {
            transform.translation = new_pos;
        }
    }
}

#[derive(Resource)]
pub struct SaveTimer(Timer);

impl Default for SaveTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(5.0, TimerMode::Repeating))
    }
}

pub fn save_camera_state(
    time: Res<Time>,
    mut timer: Local<SaveTimer>,
    query: Query<&Transform, With<MainCamera>>,
    db: Res<crate::Database>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        if let Ok(transform) = query.get_single() {
            let state = CameraState::from_transform(transform);
            if let Ok(json) = serde_json::to_string(&state) {
                if let Ok(conn) = db.0.lock() {
                    let _ = crate::core::database::save_state(&*conn, "camera_state", &json);
                }
            }
        }
    }
}

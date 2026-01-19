use bevy::prelude::*;
use crate::game::worker::{Worker, WorkerState};

/// Component for entities that can move
#[derive(Component)]
pub struct MovementTarget {
    pub destination: Vec3,
    pub speed: f32,
    pub arrival_threshold: f32,
}

impl MovementTarget {
    pub fn new(destination: Vec3) -> Self {
        Self {
            destination,
            speed: 3.0,  // units per second
            arrival_threshold: 0.5,
        }
    }
}

/// System to move workers toward their targets
pub fn move_workers(
    mut commands: Commands,
    mut worker_query: Query<(Entity, &mut Transform, &MovementTarget, &mut Worker)>,
    time: Res<Time>,
) {
    for (entity, mut transform, target, mut worker) in worker_query.iter_mut() {
        let current_pos = transform.translation;
        let target_pos = target.destination;

        // Calculate direction
        let direction = (target_pos - current_pos).normalize_or_zero();

        // Check if arrived
        let distance = current_pos.distance(target_pos);

        if distance <= target.arrival_threshold {
            // Arrived!
            transform.translation = target_pos;
            commands.entity(entity).remove::<MovementTarget>();

            // Update worker state
            if let WorkerState::MovingTo { .. } = worker.state {
                worker.state = WorkerState::Ready;
                println!("Worker '{}' arrived at destination", worker.name);
            }
        } else {
            // Keep moving
            let movement = direction * target.speed * time.delta_seconds();
            transform.translation += movement;

            // Face direction of movement
            if direction.length_squared() > 0.001 {
                let target_rotation = Quat::from_rotation_y(
                    (-direction.z).atan2(direction.x)
                );
                transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
            }
        }
    }
}

/// System to send idle workers to leisure zone
pub fn send_idle_to_leisure(
    mut commands: Commands,
    mut worker_query: Query<(Entity, &mut Worker, &Transform)>,
    leisure_zone_query: Query<&crate::game::systems::leisure_zone::LeisureZone>,
    time: Res<Time>,
    mut last_check: Local<f32>,
) {
    *last_check += time.delta_seconds();

    // Check every 3 seconds
    if *last_check < 3.0 {
        return;
    }
    *last_check = 0.0;

    let leisure_zone = match leisure_zone_query.get_single() {
        Ok(zone) => zone,
        Err(_) => return,
    };

    for (entity, mut worker, transform) in worker_query.iter_mut() {
        if worker.state == WorkerState::Idle {
            // Check if already at leisure zone
            let distance_to_zone = transform.translation.distance(leisure_zone.center);

            if distance_to_zone > leisure_zone.radius {
                // Send to leisure zone
                let target_pos = crate::game::systems::leisure_zone::random_leisure_position(leisure_zone);

                commands.entity(entity).insert(MovementTarget::new(target_pos));
                worker.state = WorkerState::MovingTo { target: target_pos };

                println!("Sending worker '{}' to leisure zone", worker.name);
            }
        }
    }
}

use bevy::prelude::*;
use crate::game::systems::worker_spawner::TownHall;
use crate::game::resources::WorkerManager;

/// Temporary: Spawn worker on 'W' key press
pub fn spawn_worker_on_keypress(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut town_hall_query: Query<&mut TownHall>,
    worker_manager: Res<WorkerManager>,
    time: Res<Time>,
) {
    if keyboard.just_pressed(KeyCode::KeyW) {
        // Check worker limit
        match worker_manager.count_workers() {
            Ok(count) if count >= worker_manager.max_workers => {
                println!("⚠️ Worker limit reached ({}/{})", count, worker_manager.max_workers);
                return;
            }
            Err(e) => {
                eprintln!("Failed to count workers: {}", e);
                return;
            }
            _ => {}
        }

        for mut town_hall in town_hall_query.iter_mut() {
            town_hall.start_worker_production(time.elapsed_seconds_f64());
            println!("🏗️ Started worker production (5 seconds)...");
        }
    }
}

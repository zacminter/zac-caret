use bevy::prelude::*;
use crate::game::worker::{Worker, WorkerVisual, WorkerState, NameGenerator};
use crate::game::resources::WorkerManager;
use crate::game::systems::leisure_zone::LeisureZone;

/// Component for Town Hall
#[derive(Component)]
pub struct TownHall {
    pub worker_production_queue: Vec<WorkerProductionOrder>,
}

/// Worker production order
#[derive(Debug, Clone)]
pub struct WorkerProductionOrder {
    pub started_at: f64,
    pub duration: f32,  // seconds
    pub worker_name: String,
}

impl TownHall {
    pub fn new() -> Self {
        Self {
            worker_production_queue: Vec::new(),
        }
    }

    pub fn start_worker_production(&mut self, time: f64) {
        let name = NameGenerator::random_name();
        self.worker_production_queue.push(WorkerProductionOrder {
            started_at: time,
            duration: 5.0,  // 5 seconds like WC3
            worker_name: name,
        });
    }
}

/// System to process worker production queue
pub fn process_worker_production(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut town_hall_query: Query<(&mut TownHall, &Transform)>,
    worker_manager: Res<WorkerManager>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_seconds_f64();

    for (mut town_hall, th_transform) in town_hall_query.iter_mut() {
        let mut completed_orders = Vec::new();

        for (index, order) in town_hall.worker_production_queue.iter().enumerate() {
            let elapsed = current_time - order.started_at;

            if elapsed >= order.duration as f64 {
                completed_orders.push(index);
            }
        }

        // Process completed orders
        for index in completed_orders.iter().rev() {
            let order = town_hall.worker_production_queue.remove(*index);

            // Create worker in database
            let worker = Worker::new(order.worker_name.clone());
            let color_tuple = (worker.color.to_srgba().red, worker.color.to_srgba().green, worker.color.to_srgba().blue);

            match worker_manager.create_worker(worker.name.clone(), color_tuple) {
                Ok(worker_id) => {
                    // Spawn worker entity
                    let spawn_pos = th_transform.translation + Vec3::new(2.0, 0.0, 2.0);
                    spawn_worker_entity(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        worker,
                        spawn_pos,
                    );

                    println!("✅ Worker '{}' completed training! (ID: {})", order.worker_name, worker_id);
                }
                Err(e) => {
                    eprintln!("Failed to create worker: {}", e);
                }
            }
        }
    }
}

/// Spawn a worker entity in the world
fn spawn_worker_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    worker: Worker,
    position: Vec3,
) {
    commands
        .spawn((
            worker.clone(),
            SpatialBundle::from_transform(Transform::from_translation(position)),
        ))
        .with_children(|parent| {
            // Worker body (capsule)
            parent.spawn((
                WorkerVisual,
                PbrBundle {
                    mesh: meshes.add(Capsule3d::new(0.3, 0.8)),
                    material: materials.add(StandardMaterial {
                        base_color: worker.color,
                        ..default()
                    }),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
                    ..default()
                },
            ));

            // TODO M10: Add name tag text above worker
        });
}

/// System to restore workers from database on startup
pub fn restore_workers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    worker_manager: Res<WorkerManager>,
    leisure_zone_query: Query<&LeisureZone>,
) {
    let workers = match worker_manager.load_workers() {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Failed to load workers: {}", e);
            return;
        }
    };

    let leisure_zone = match leisure_zone_query.get_single() {
        Ok(zone) => zone,
        Err(_) => {
            eprintln!("No leisure zone found - skipping worker restoration");
            return;
        }
    };

    for worker in workers {
        // Spawn at leisure zone
        let position = crate::game::systems::leisure_zone::random_leisure_position(leisure_zone);

        spawn_worker_entity(
            &mut commands,
            &mut meshes,
            &mut materials,
            worker.clone(),
            position,
        );

        println!("Restored worker: {}", worker.name);
    }
}

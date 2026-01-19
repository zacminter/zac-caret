#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use std::path::PathBuf;

mod camera;
mod core;
mod game;
mod agents;
mod ui;

use core::database;

#[derive(Resource)]
pub struct AppPaths {
    pub data_dir: PathBuf,
    pub db_path: PathBuf,
}

impl Default for AppPaths {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let data_dir = home.join("zac-caret").join("data");
        std::fs::create_dir_all(&data_dir).ok();

        Self {
            db_path: data_dir.join("zac.db"),
            data_dir,
        }
    }
}

#[derive(Resource)]
pub struct Database(pub std::sync::Arc<std::sync::Mutex<rusqlite::Connection>>);

fn main() {
    // Initialize paths and database
    let paths = AppPaths::default();
    let conn = database::init_database(&paths.db_path)
        .expect("Failed to initialize database");

    // Load saved camera state
    let camera_state = database::load_state(&conn, "camera_state")
        .ok()
        .flatten()
        .and_then(|s| serde_json::from_str::<camera::CameraState>(&s).ok())
        .unwrap_or_default();

    let db = Database(std::sync::Arc::new(std::sync::Mutex::new(conn)));

    // Create managers
    let project_manager = game::resources::ProjectManager::new(paths.db_path.clone());
    let mission_manager = game::systems::MissionManager::new(paths.db_path.clone());
    let worker_manager = game::resources::WorkerManager::new(paths.db_path.clone());
    let cli_manager = game::resources::CliManagerResource::new(paths.data_dir.clone());

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zac^ Command Center".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(paths)
        .insert_resource(db)
        .insert_resource(camera_state)
        .insert_resource(project_manager)
        .insert_resource(mission_manager)
        .insert_resource(worker_manager)
        .insert_resource(cli_manager)
        .init_resource::<camera::CameraSettings>()
        .add_systems(Startup, (
            game::world::setup_world,
            camera::spawn_camera_from_state,
            game::systems::building_renderer::spawn_initial_town_hall,
            game::systems::spawn_leisure_zone,
            game::systems::spawn_project_buildings,
            game::systems::restore_workers,
            ui::spawn_building_controls,
        ))
        .add_systems(Update, (
            camera::camera_pan,
            camera::camera_zoom,
            camera::save_camera_state,
            game::systems::building_renderer::update_building_visuals,
            game::systems::track_project_progress,
            game::systems::sync_project_data,
            game::systems::process_worker_production,
            game::systems::move_workers,
            game::systems::send_idle_to_leisure,
            game::systems::assign_worker_on_keypress,
            game::systems::start_mission_on_arrival,
            game::systems::check_cli_completions,
            ui::handle_upgrade_button,
            ui::handle_downgrade_button,
            ui::update_stage_display,
            ui::button_hover_system,
            ui::spawn_worker_on_keypress,
        ))
        .run();
}

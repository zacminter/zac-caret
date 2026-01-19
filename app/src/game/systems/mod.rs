pub mod autonomous_assignment;
pub mod building_renderer;
pub mod leisure_zone;
pub mod mission_manager;
pub mod mission_writer;
pub mod movement;
pub mod progress_tracker;
pub mod project_spawner;
pub mod selection;
pub mod stats_display;
pub mod stats_updater;
pub mod task_assignment;
pub mod token_tracker;
pub mod worker_movement;
pub mod worker_spawner;

pub use autonomous_assignment::{autonomous_task_assignment, toggle_autonomy_keypress, display_autonomy_status};
// Note: update_building_visuals is called directly in main.rs, not through re-export
// We keep this commented for future modular use
// pub use building_renderer::update_building_visuals;
#[allow(unused_imports)]
pub use building_renderer::spawn_initial_town_hall;
pub use leisure_zone::spawn_leisure_zone;
#[allow(unused_imports)]
pub use leisure_zone::LeisureZone;
pub use mission_manager::MissionManager;
pub use progress_tracker::{track_project_progress, sync_project_data};
pub use project_spawner::spawn_project_buildings;
pub use stats_display::display_comprehensive_stats;
pub use stats_updater::update_game_stats;
pub use task_assignment::{assign_worker_on_keypress, start_mission_on_arrival, check_cli_completions};
pub use token_tracker::{check_budget_reset, display_budget_warnings, display_budget_status};
pub use worker_movement::{move_workers, send_idle_to_leisure, MovementTarget};
pub use worker_spawner::{process_worker_production, restore_workers, TownHall};

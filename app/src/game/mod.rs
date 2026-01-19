use bevy::prelude::*;

pub mod camera;
pub mod cli;
pub mod components;
pub mod entities;
pub mod project;
pub mod resources;
pub mod systems;
pub mod worker;
pub mod world;

// Note: GameState is used in UI layer, will be re-exported in later phase
#[allow(unused_imports)]
pub use resources::GameState;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<resources::GameState>()
            
            // Startup systems
            .add_systems(Startup, world::setup_world)
            
            // Update systems
            .add_systems(Update, (
                camera::camera_controller,
                systems::selection::handle_selection,
                systems::movement::update_movement,
            ));
    }
}

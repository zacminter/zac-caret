# Zac^ Project Scaffolding Guide
## Getting Started | January 2026

---

## Prerequisites

Before scaffolding, ensure you have:

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup target add wasm32-unknown-unknown  # For potential web builds

# Tauri CLI
cargo install tauri-cli

# Node.js (for Tauri's WebView dev tooling)
# Using nvm recommended
nvm install 20
nvm use 20

# Additional tools
cargo install cargo-watch  # Hot reload during development
cargo install trunk        # For Leptos WebView builds
```

---

## Step 1: Create Project Directory

```bash
# Create the centralized Zac^ directory
mkdir -p ~/zac-caret
cd ~/zac-caret

# Initialize git
git init
echo "target/
node_modules/
*.db
*.db-journal
data/snapshots/
logs/
.DS_Store
" > .gitignore
```

---

## Step 2: Initialize Tauri + Bevy Project

```bash
# Create the Rust project
cargo new app --name zac-caret
cd app

# Initialize Tauri
cargo tauri init
# When prompted:
#   - App name: Zac^
#   - Window title: Zac^ Command Center
#   - Web assets location: ../ui/dist
#   - Dev server URL: http://localhost:1420
#   - Dev command: trunk serve
#   - Build command: trunk build
```

### Update Cargo.toml

Replace `app/Cargo.toml` with:

```toml
[package]
name = "zac-caret"
version = "0.1.0"
edition = "2021"
description = "Gamified AI Agent Orchestration Platform"
authors = ["Zac"]

[dependencies]
# Tauri
tauri = { version = "2", features = ["devtools"] }
tauri-plugin-shell = "2"

# Bevy (3D game engine)
bevy = { version = "0.14", features = ["dynamic_linking"] }

# Database
rusqlite = { version = "0.31", features = ["bundled"] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# HTTP client for Claude API
reqwest = { version = "0.11", features = ["json"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"

# Utilities
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = "0.3"

# Secure storage
keyring = "2"

# Audio
rodio = "0.17"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

---

## Step 3: Create Directory Structure

```bash
cd ~/zac-caret

# Application structure
mkdir -p app/src/{game,ui,core,agents}
mkdir -p app/assets/{models,textures,sounds,fonts}
mkdir -p app/config

# Data directories
mkdir -p data
mkdir -p data/snapshots

# Projects directory (where managed projects live)
mkdir -p projects

# Logs
mkdir -p logs/session
mkdir -p logs/workers

# User config
mkdir -p user

# UI (Leptos WebView)
mkdir -p ui/src
```

### Final Structure

```
~/zac-caret/
├── app/
│   ├── src/
│   │   ├── main.rs              # Entry point
│   │   ├── lib.rs               # Library root
│   │   ├── game/
│   │   │   ├── mod.rs
│   │   │   ├── world.rs         # Bevy world setup
│   │   │   ├── camera.rs        # Camera controller
│   │   │   ├── entities/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── zac.rs       # Zac^ hero unit
│   │   │   │   ├── worker.rs    # Worker entities
│   │   │   │   ├── building.rs  # Project buildings
│   │   │   │   └── town_hall.rs # Central building
│   │   │   ├── systems/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── selection.rs # Click to select
│   │   │   │   ├── movement.rs  # Pathfinding
│   │   │   │   └── animation.rs # Visual effects
│   │   │   └── resources/
│   │   │       ├── mod.rs
│   │   │       └── game_state.rs
│   │   │
│   │   ├── core/
│   │   │   ├── mod.rs
│   │   │   ├── database.rs      # SQLite operations
│   │   │   ├── config.rs        # Settings management
│   │   │   ├── state.rs         # App state machine
│   │   │   └── events.rs        # Event bus
│   │   │
│   │   ├── agents/
│   │   │   ├── mod.rs
│   │   │   ├── orchestrator.rs  # Worker pool management
│   │   │   ├── spawner.rs       # Claude CLI spawner
│   │   │   ├── knowledge.rs     # Knowledge base
│   │   │   └── zac_brain.rs     # Zac^ intelligence
│   │   │
│   │   └── ui/
│   │       ├── mod.rs
│   │       └── ipc.rs           # Tauri IPC handlers
│   │
│   ├── assets/
│   │   ├── models/
│   │   │   └── .gitkeep
│   │   ├── textures/
│   │   │   └── grass.png
│   │   ├── sounds/
│   │   │   └── .gitkeep
│   │   └── fonts/
│   │       └── .gitkeep
│   │
│   ├── config/
│   │   └── default.toml
│   │
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── build.rs
│
├── ui/                          # Leptos WebView
│   ├── src/
│   │   ├── main.rs
│   │   ├── app.rs
│   │   └── components/
│   │       ├── mod.rs
│   │       ├── action_bar.rs
│   │       ├── chat.rs
│   │       ├── journal.rs
│   │       ├── stats_hud.rs
│   │       └── task_list.rs
│   ├── index.html
│   ├── Cargo.toml
│   └── Trunk.toml
│
├── data/
│   ├── zac.db                   # Created at runtime
│   ├── knowledge.db             # Created at runtime
│   └── snapshots/
│
├── projects/                    # Managed projects
│
├── logs/
│   ├── session/
│   ├── workers/
│   └── zac-journal.md
│
├── user/
│   └── settings.toml
│
├── docs/
│   ├── 01_SYSTEM_ARCHITECTURE.md
│   ├── 02_V1_PROJECT_ROADMAP.md
│   └── 03_SCAFFOLDING_GUIDE.md
│
├── .gitignore
└── README.md
```

---

## Step 4: Create Starter Files

### app/src/main.rs

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use tauri::Manager;

mod core;
mod game;
mod agents;
mod ui;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Build Tauri app with Bevy
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize database
            let app_handle = app.handle().clone();
            core::database::init(&app_handle)?;
            
            // Start Bevy in a separate thread
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                App::new()
                    .add_plugins(DefaultPlugins.set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Zac^ Command Center".into(),
                            resolution: (1280., 720.).into(),
                            ..default()
                        }),
                        ..default()
                    }))
                    .insert_resource(game::resources::TauriHandle(handle))
                    .add_plugins(game::GamePlugin)
                    .run();
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ui::ipc::select_entity,
            ui::ipc::send_chat_message,
            ui::ipc::get_game_stats,
            ui::ipc::toggle_autonomy,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### app/src/game/mod.rs

```rust
use bevy::prelude::*;

pub mod camera;
pub mod entities;
pub mod systems;
pub mod resources;
mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<resources::GameState>()
            
            // Startup systems
            .add_systems(Startup, (
                world::setup_world,
                world::spawn_town_hall,
            ))
            
            // Update systems
            .add_systems(Update, (
                camera::camera_controller,
                systems::selection::handle_selection,
                systems::movement::update_movement,
            ));
    }
}
```

### app/src/game/world.rs

```rust
use bevy::prelude::*;
use super::entities::town_hall::TownHall;

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)), // Grass green
        ..default()
    });

    // Sunlight
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(50.0, 50.0, 50.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 30.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn spawn_town_hall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Simple cube for now - will be replaced with proper model
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(3.0, 4.0, 3.0)),
            material: materials.add(Color::rgb(0.6, 0.4, 0.2)), // Wood brown
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        TownHall::default(),
        Name::new("Town Hall"),
    ));
}
```

### app/src/game/camera.rs

```rust
use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};

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
```

### app/src/game/entities/town_hall.rs

```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct TownHall {
    pub level: u8,
    pub worker_capacity: u8,
}

impl Default for TownHall {
    fn default() -> Self {
        Self {
            level: 1,
            worker_capacity: 5,
        }
    }
}

impl TownHall {
    pub fn max_workers(&self) -> u8 {
        // Level 1: 5 workers, +2 per level
        5 + (self.level - 1) * 2
    }
    
    pub fn upgrade_requirements(&self) -> TownHallUpgradeReq {
        match self.level {
            1 => TownHallUpgradeReq { projects: 1, workers: 2 },
            2 => TownHallUpgradeReq { projects: 2, workers: 4 },
            3 => TownHallUpgradeReq { projects: 3, workers: 6 },
            _ => TownHallUpgradeReq { projects: self.level as usize, workers: self.level as usize * 2 },
        }
    }
}

pub struct TownHallUpgradeReq {
    pub projects: usize,
    pub workers: usize,
}
```

### app/config/default.toml

```toml
[general]
autonomy_enabled = true
max_concurrent_workers = 10
reflection_enabled = true

[knowledge]
injection_enabled = true
max_injection_tokens = 1000
relevance_threshold = 0.7

[visuals]
camera_speed = 20.0
zoom_min = 10.0
zoom_max = 80.0
show_worker_names = true

[audio]
master_volume = 0.8
notification_sounds = true
ambient_sounds = true

[tokens]
hourly_budget = 50000
warning_threshold = 0.2

[demo]
enabled = false
simulation_speed = 1.0
```

### user/settings.toml

```toml
# User settings (overrides defaults)
# This file is created/modified by the application

[general]
# autonomy_enabled = true

[tokens]
# hourly_budget = 50000
```

---

## Step 5: Initialize UI (Leptos)

```bash
cd ~/zac-caret/ui

# Create Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "zac-caret-ui"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = { version = "0.6", features = ["csr"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Window", "Document"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[profile.release]
opt-level = "z"
lto = true
EOF

# Create Trunk.toml
cat > Trunk.toml << 'EOF'
[build]
target = "index.html"
dist = "dist"

[watch]
watch = ["src", "index.html"]

[serve]
port = 1420
EOF

# Create index.html
cat > index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Zac^ UI</title>
    <link data-trunk rel="css" href="style.css">
</head>
<body>
    <div id="app"></div>
    <link data-trunk rel="rust" data-wasm-opt="z"/>
</body>
</html>
EOF

# Create basic styles
cat > style.css << 'EOF'
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: system-ui, -apple-system, sans-serif;
    background: transparent;
    color: #e0e0e0;
}

#app {
    width: 100vw;
    height: 100vh;
    pointer-events: none;
}

.stats-hud {
    position: fixed;
    top: 10px;
    right: 10px;
    background: rgba(0, 0, 0, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    padding: 12px 16px;
    pointer-events: auto;
    font-size: 14px;
    min-width: 200px;
}

.action-bar {
    position: fixed;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    background: rgba(0, 0, 0, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.3);
    border-radius: 8px;
    padding: 10px;
    display: flex;
    gap: 8px;
    pointer-events: auto;
}

.action-button {
    width: 48px;
    height: 48px;
    background: rgba(60, 60, 60, 0.9);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 20px;
    transition: all 0.2s;
}

.action-button:hover {
    background: rgba(80, 80, 80, 0.9);
    border-color: rgba(255, 255, 255, 0.4);
}

.chat-panel {
    position: fixed;
    bottom: 90px;
    left: 50%;
    transform: translateX(-50%);
    width: 500px;
    background: rgba(0, 0, 0, 0.85);
    border: 1px solid rgba(255, 255, 255, 0.3);
    border-radius: 8px;
    pointer-events: auto;
}

.chat-messages {
    max-height: 300px;
    overflow-y: auto;
    padding: 12px;
}

.chat-input {
    display: flex;
    border-top: 1px solid rgba(255, 255, 255, 0.2);
    padding: 8px;
}

.chat-input input {
    flex: 1;
    background: rgba(40, 40, 40, 0.9);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    padding: 8px 12px;
    color: #fff;
    font-size: 14px;
}

.chat-input button {
    margin-left: 8px;
    padding: 8px 16px;
    background: #4a9eff;
    border: none;
    border-radius: 4px;
    color: #fff;
    cursor: pointer;
}
EOF
```

### ui/src/main.rs

```rust
use leptos::*;

mod app;
mod components;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <app::App/> });
}
```

### ui/src/app.rs

```rust
use leptos::*;
use crate::components::{stats_hud::StatsHud, action_bar::ActionBar};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <StatsHud/>
        <ActionBar/>
    }
}
```

---

## Step 6: Build and Run

```bash
# From the app directory
cd ~/zac-caret/app

# Development mode (with hot reload)
cargo tauri dev

# Production build
cargo tauri build
```

---

## Step 7: Verify Setup

Checklist:
- [ ] `cargo tauri dev` launches without errors
- [ ] 3D window shows green terrain plane
- [ ] Brown cube (Town Hall) visible at center
- [ ] Camera pans with WASD
- [ ] Camera zooms with scroll wheel
- [ ] Stats HUD visible in corner
- [ ] No console errors

---

## Next Steps

With scaffolding complete, proceed to:

1. **Milestone 1 tasks** in the V1 Project Roadmap
2. Start with camera persistence (save/load position)
3. Add entity selection system
4. Wire up IPC between Bevy and WebView

The foundation is laid. Time to build your command center.

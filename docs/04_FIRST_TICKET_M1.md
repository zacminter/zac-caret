# Implementation Ticket: M1 - Foundation
## First Steps to Break Ground

---

## Overview

This ticket covers the initial setup to get a working Tauri + Bevy application with basic 3D rendering and camera controls. By the end, you'll have a green meadow with a camera you can move around.

**Estimated Time:** 2-3 days
**Dependencies:** None (this is the starting point)

---

## Pre-Work Checklist

Before coding, ensure your environment is ready:

```bash
# Verify Rust
rustc --version  # Should be 1.75+

# Verify Tauri CLI
cargo tauri --version  # Should be 2.x

# Verify Node
node --version  # Should be 18+

# Create project directory
mkdir -p ~/zac-caret
cd ~/zac-caret
git init
```

---

## Task 1.1: Initialize Tauri Project

**Goal:** Create the basic Tauri application shell.

```bash
cd ~/zac-caret
cargo new app --name zac-caret
cd app
cargo tauri init
```

When prompted during `tauri init`:
- **App name:** `zac-caret`
- **Window title:** `Zac^ Command Center`
- **Web assets:** `../ui/dist`
- **Dev URL:** `http://localhost:1420`
- **Dev command:** `trunk serve`
- **Build command:** `trunk build`

**Verify:** `cargo build` completes without errors.

---

## Task 1.2: Add Bevy Dependency

**Goal:** Configure Bevy with appropriate features for development.

Update `app/Cargo.toml`:

```toml
[package]
name = "zac-caret"
version = "0.1.0"
edition = "2021"

[dependencies]
# Core
tauri = { version = "2", features = ["devtools"] }
tauri-plugin-shell = "2"
bevy = { version = "0.14", features = ["dynamic_linking"] }

# Essential utilities
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
rusqlite = { version = "0.31", features = ["bundled"] }

[build-dependencies]
tauri-build = { version = "2", features = [] }

# Fast compile times in dev
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

**Note:** `dynamic_linking` feature dramatically speeds up Bevy compile times during development. Remove for production builds.

**Verify:** `cargo build` pulls Bevy and compiles.

---

## Task 1.3: Create Basic 3D Scene

**Goal:** Render a flat terrain plane with lighting.

Create `app/src/main.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zac^ Command Center".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_world)
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane - the meadow
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.35, 0.55, 0.25), // Grass green
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
        color: Color::rgb(0.9, 0.95, 1.0), // Slight blue tint
        brightness: 200.0,
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 25.0, 35.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Debug: spawn a cube so we can see something
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
        material: materials.add(Color::rgb(0.8, 0.5, 0.3)),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });
}
```

**Verify:** `cargo run` shows a green plane with a brown cube and nice lighting.

---

## Task 1.4: Implement Camera Controller

**Goal:** Pan with WASD/arrows, zoom with scroll wheel.

Create `app/src/camera.rs`:

```rust
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 25.0, 35.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
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
```

Update `app/src/main.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zac^ Command Center".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<camera::CameraSettings>()
        .add_systems(Startup, (setup_world, camera::spawn_camera))
        .add_systems(Update, (camera::camera_pan, camera::camera_zoom))
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.35, 0.55, 0.25),
            perceptual_roughness: 0.9,
            ..default()
        }),
        ..default()
    });

    // Sun
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

    // Ambient
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.9, 0.95, 1.0),
        brightness: 200.0,
    });

    // Test cube (will become Town Hall)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(3.0, 4.0, 3.0)),
        material: materials.add(Color::rgb(0.6, 0.4, 0.2)),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..default()
    });
}
```

**Verify:** 
- WASD moves camera across the terrain
- Scroll wheel zooms in/out
- Camera doesn't go through the ground or too far up

---

## Task 1.5: Set Up WebView (Placeholder)

**Goal:** Create minimal Leptos UI that compiles.

```bash
mkdir -p ~/zac-caret/ui/src
cd ~/zac-caret/ui
```

Create `ui/Cargo.toml`:

```toml
[package]
name = "zac-caret-ui"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = { version = "0.6", features = ["csr"] }
console_error_panic_hook = "0.1"
wasm-bindgen = "0.2"
```

Create `ui/Trunk.toml`:

```toml
[build]
target = "index.html"
dist = "dist"

[watch]
watch = ["src", "index.html"]

[serve]
port = 1420
```

Create `ui/index.html`:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Zac^ UI</title>
    <link data-trunk rel="css" href="style.css">
</head>
<body>
    <link data-trunk rel="rust" data-wasm-opt="z"/>
</body>
</html>
```

Create `ui/style.css`:

```css
* { margin: 0; padding: 0; box-sizing: border-box; }
body { 
    font-family: system-ui, sans-serif;
    background: transparent;
    color: #e0e0e0;
}
.placeholder {
    position: fixed;
    bottom: 20px;
    right: 20px;
    background: rgba(0,0,0,0.7);
    padding: 10px 15px;
    border-radius: 6px;
    font-size: 12px;
}
```

Create `ui/src/main.rs`:

```rust
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {
        <div class="placeholder">
            "Zac^ UI Loading..."
        </div>
    });
}
```

**Verify:** `cd ui && trunk serve` compiles and serves at localhost:1420.

---

## Task 1.6: Establish IPC Bridge (Stub)

**Goal:** Understand the IPC pattern. Full implementation in M2.

Tauri IPC allows the WebView (Leptos) to call Rust functions and receive events. For now, just note the pattern:

```rust
// In Rust (app side):
#[tauri::command]
fn get_game_stats() -> GameStats {
    GameStats { workers: 0, tasks: 0 }
}

// Register in main:
.invoke_handler(tauri::generate_handler![get_game_stats])

// In WebView (JS/Leptos):
// invoke("get_game_stats").then(stats => console.log(stats))
```

**Verify:** Conceptual understanding. Implementation in M2.

---

## Task 1.7: Create Directory Structure

**Goal:** Set up the full directory layout per architecture doc.

```bash
cd ~/zac-caret

# App structure
mkdir -p app/src/{game,core,agents,ui}
mkdir -p app/src/game/{entities,systems,resources}
mkdir -p app/assets/{models,textures,sounds,fonts}
mkdir -p app/config

# Data
mkdir -p data/snapshots

# Projects
mkdir -p projects

# Logs
mkdir -p logs/{session,workers}

# User
mkdir -p user

# Docs (copy the generated docs here)
mkdir -p docs
```

Create placeholder mod.rs files:

```bash
echo "// Game module" > app/src/game/mod.rs
echo "// Entity definitions" > app/src/game/entities/mod.rs
echo "// Game systems" > app/src/game/systems/mod.rs
echo "// Game resources" > app/src/game/resources/mod.rs
echo "// Core functionality" > app/src/core/mod.rs
echo "// Agent orchestration" > app/src/agents/mod.rs
echo "// UI/IPC handlers" > app/src/ui/mod.rs
```

**Verify:** Directory structure matches architecture doc.

---

## Task 1.8: Initialize SQLite Database

**Goal:** Create database with initial schema.

Create `app/src/core/database.rs`:

```rust
use rusqlite::{Connection, Result};
use std::path::Path;

pub fn init_database(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)?;
    
    conn.execute_batch(r#"
        -- Workers
        CREATE TABLE IF NOT EXISTS workers (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            appearance_json TEXT,
            state TEXT NOT NULL DEFAULT 'idle',
            specialty_scores_json TEXT DEFAULT '{}',
            total_tasks_completed INTEGER DEFAULT 0,
            total_tokens_used INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- Projects
        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL UNIQUE,
            building_theme TEXT,
            visual_stage INTEGER DEFAULT 0,
            milestone_progress_json TEXT DEFAULT '{}',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- Tasks
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            project_id TEXT REFERENCES projects(id),
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'available',
            assigned_worker_id TEXT REFERENCES workers(id),
            milestone_index INTEGER,
            dependencies_json TEXT DEFAULT '[]',
            started_at DATETIME,
            completed_at DATETIME,
            tokens_used INTEGER,
            completion_summary TEXT
        );

        -- App state (key-value for camera, settings, etc)
        CREATE TABLE IF NOT EXISTS app_state (
            key TEXT PRIMARY KEY,
            value_json TEXT NOT NULL
        );

        -- Zac journal
        CREATE TABLE IF NOT EXISTS zac_journal (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            entry_type TEXT NOT NULL,
            content TEXT NOT NULL,
            related_project_id TEXT,
            related_task_id TEXT
        );

        -- Sessions
        CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            ended_at DATETIME,
            tasks_completed INTEGER DEFAULT 0,
            tokens_used INTEGER DEFAULT 0,
            summary TEXT
        );
    "#)?;

    Ok(conn)
}

pub fn save_state(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO app_state (key, value_json) VALUES (?1, ?2)",
        [key, value],
    )?;
    Ok(())
}

pub fn load_state(conn: &Connection, key: &str) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT value_json FROM app_state WHERE key = ?1")?;
    let result: Result<String> = stmt.query_row([key], |row| row.get(0));
    match result {
        Ok(val) => Ok(Some(val)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}
```

Update `app/src/core/mod.rs`:

```rust
pub mod database;
```

**Verify:** Import and call `init_database` from main. Check that `zac.db` file is created.

---

## Task 1.9: Implement Camera State Persistence

**Goal:** Save camera position on exit, restore on startup.

Update `app/src/main.rs` to integrate database and persistence:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use std::path::PathBuf;

mod camera;
mod core;

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
pub struct Database(pub rusqlite::Connection);

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
        .insert_resource(Database(conn))
        .insert_resource(camera_state)
        .init_resource::<camera::CameraSettings>()
        .add_systems(Startup, (setup_world, camera::spawn_camera_from_state))
        .add_systems(Update, (
            camera::camera_pan, 
            camera::camera_zoom,
            camera::save_camera_state,
        ))
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.35, 0.55, 0.25),
            perceptual_roughness: 0.9,
            ..default()
        }),
        ..default()
    });

    // Sun
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

    // Ambient
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.9, 0.95, 1.0),
        brightness: 200.0,
    });

    // Proto Town Hall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(3.0, 4.0, 3.0)),
            material: materials.add(Color::rgb(0.6, 0.4, 0.2)),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        Name::new("Town Hall"),
    ));
}
```

Update `camera.rs` to add persistence:

```rust
// Add to camera.rs

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
                let _ = crate::core::database::save_state(&db.0, "camera_state", &json);
            }
        }
    }
}
```

**Verify:** 
- Move camera to a new position
- Wait 5+ seconds
- Close and reopen app
- Camera should be at the same position

---

## Completion Checklist

- [ ] Tauri project initialized
- [ ] Bevy renders 3D scene with terrain
- [ ] Camera pan works (WASD/arrows)
- [ ] Camera zoom works (scroll)
- [ ] UI WebView compiles (placeholder)
- [ ] Directory structure created
- [ ] SQLite database initializes with schema
- [ ] Camera position saves every 5 seconds
- [ ] Camera position restores on app restart

---

## Definition of Done

Milestone 1 is complete when:

1. `cargo run` from `~/zac-caret/app` launches the application
2. A green meadow terrain is visible
3. A brown cube (proto Town Hall) sits in the center
4. WASD pans the camera smoothly
5. Mouse scroll zooms in/out with proper limits
6. Database exists at `~/zac-caret/data/zac.db`
7. Camera position persists across restarts
8. All code compiles without warnings

---

## Next Milestone

**M2: Town Hall** - Create the central building entity with selection highlighting, implement the Action Bar UI that appears when selected, and establish the IPC bridge between Bevy and the WebView.

---

## Troubleshooting

**Bevy compiles slowly:**
- Ensure `dynamic_linking` feature is enabled
- Use `cargo watch -x run` for faster iteration

**Camera feels wrong:**
- Adjust `CameraSettings` pan_speed (default 25.0)
- Adjust zoom_speed (default 10.0)
- Check min/max height bounds

**Database not persisting:**
- Check that `~/zac-caret/data/` directory exists
- Verify write permissions
- Check console for rusqlite errors

**WebView doesn't load:**
- Ensure `trunk serve` is running on port 1420
- Check browser console for WASM errors
- Verify Trunk.toml configuration

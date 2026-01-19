# Zac^ Command Center - Technical Architecture

**Version:** 1.0
**Date:** 2026-01-19
**Target Audience:** Developers, Contributors, Technical Users

---

## Table of Contents

1. [System Overview](#system-overview)
2. [Architecture](#architecture)
3. [Technology Stack](#technology-stack)
4. [Data Flow](#data-flow)
5. [Core Systems](#core-systems)
6. [Database Schema](#database-schema)
7. [Entity-Component-System Design](#entity-component-system-design)
8. [Implementation Details](#implementation-details)
9. [Performance Considerations](#performance-considerations)
10. [Future Enhancements](#future-enhancements)

---

## System Overview

Zac^ is a real-time 3D application built with Bevy ECS (Entity-Component-System) that orchestrates Claude Code CLI workers to autonomously complete software development tasks. It bridges game engine architecture with AI agent management to create a unique productivity tool.

### Core Capabilities

- **Multi-Project Management**: Manages multiple software projects simultaneously
- **Visual Progress Tracking**: 3D buildings that evolve through 10 stages based on completion
- **Autonomous Worker Assignment**: AI-driven task allocation with priority scoring
- **Budget Management**: Token usage tracking with hourly limits and automatic resets
- **Session Persistence**: All state persists to SQLite and resumes on restart
- **Real-Time Monitoring**: Live statistics and event logging

---

## Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────────────────┐
│                     Bevy Application                        │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │   Main Loop │──│   Systems    │──│    Resources     │  │
│  │   (Update)  │  │ (Game Logic) │  │   (Managers)     │  │
│  └─────────────┘  └──────────────┘  └──────────────────┘  │
│         │                 │                    │           │
│         └─────────────────┼────────────────────┘           │
│                           │                                │
│  ┌─────────────────────────────────────────────────────┐  │
│  │              Entities (ECS)                         │  │
│  │  ┌──────────┐  ┌─────────┐  ┌────────────┐       │  │
│  │  │ Projects │  │ Workers │  │ Town Hall  │  ...  │  │
│  │  └──────────┘  └─────────┘  └────────────┘       │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          │
                          │ SQLite
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                    Database (zac.db)                        │
│  projects | missions | workers | app_state | sessions      │
└─────────────────────────────────────────────────────────────┘
                          │
                          │ Subprocess
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                  Claude Code CLI                            │
│             (One process per worker)                        │
└─────────────────────────────────────────────────────────────┘
```

### Key Architectural Decisions

1. **Bevy ECS Over Traditional OOP**: Enables data-oriented design for performance
2. **SQLite for Persistence**: Simple, reliable, no external dependencies
3. **Subprocess Management**: Each worker = separate Claude CLI process
4. **Console-First UI**: V1 focuses on functionality over GUI polish
5. **Primitive 3D Graphics**: Cubes/capsules before custom models
6. **Synchronous Game Loop**: 60 FPS target, blocking operations avoided

---

## Technology Stack

### Core Technologies

| Component | Technology | Version | Purpose |
|-----------|-----------|---------|---------|
| **Language** | Rust | 1.70+ | Systems programming, safety, performance |
| **Game Engine** | Bevy | 0.14 | ECS architecture, 3D rendering, input |
| **Database** | SQLite | 3.x | State persistence via rusqlite |
| **UI Framework** | Tauri | 2.x | Desktop app shell (minimal use in V1) |
| **AI Integration** | Claude Code CLI | Latest | Subprocess execution of tasks |
| **Serialization** | serde_json | 1.x | JSON for complex data types |

### Dependencies (Cargo.toml)

```toml
[dependencies]
bevy = "0.14"
rusqlite = { version = "0.31", features = ["bundled"] }
tauri = { version = "2", features = ["shell-open"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
chrono = "0.4"
rand = "0.8"
dirs = "5.0"
```

---

## Data Flow

### Startup Sequence

```
1. main.rs entry point
   └─> Initialize AppPaths (~/zac-caret/data)
   └─> Create/open database (init_database)
   └─> Load camera state from DB
   └─> Create resource managers (Project, Mission, Worker, CLI)
   └─> Create autonomy settings, token budget, game stats
   └─> Build Bevy App with plugins
   └─> Register startup systems:
       ├─> setup_world (ground plane, lighting)
       ├─> spawn_camera_from_state
       ├─> spawn_initial_town_hall
       ├─> spawn_leisure_zone
       ├─> spawn_project_buildings (from DB)
       └─> restore_workers (from DB)
   └─> Register update systems (see below)
   └─> Run Bevy event loop
```

### Update Loop (Every Frame)

```
┌─ Camera Systems ────────────────────────┐
│ • camera_pan (WASD input)               │
│ • camera_zoom (scroll input)            │
│ • save_camera_state (throttled)         │
└─────────────────────────────────────────┘
           │
           ▼
┌─ Building Systems ──────────────────────┐
│ • update_building_visuals               │
│ • track_project_progress                │
│ • sync_project_data (DB ↔ ECS)          │
└─────────────────────────────────────────┘
           │
           ▼
┌─ Worker Systems ────────────────────────┐
│ • process_worker_production             │
│ • move_workers (to targets)             │
│ • send_idle_to_leisure                  │
│ • assign_worker_on_keypress ('A' key)   │
│ • start_mission_on_arrival              │
│ • check_cli_completions                 │
└─────────────────────────────────────────┘
           │
           ▼
┌─ Autonomy Systems ──────────────────────┐
│ • autonomous_task_assignment            │
│ • toggle_autonomy_keypress ('Z' key)    │
│ • display_autonomy_status               │
└─────────────────────────────────────────┘
           │
           ▼
┌─ Budget Systems ────────────────────────┐
│ • check_budget_reset (hourly)           │
│ • display_budget_warnings               │
│ • display_budget_status                 │
└─────────────────────────────────────────┘
           │
           ▼
┌─ Stats Systems ─────────────────────────┐
│ • update_game_stats (every second)      │
│ • display_comprehensive_stats ('S' key) │
└─────────────────────────────────────────┘
```

### Task Execution Flow

```
1. Worker in Idle state at leisure zone
2. Assignment triggers (manual 'A' or autonomous)
   └─> Find available mission (no deps, not started)
   └─> Update worker state to Moving
   └─> Add MovementTarget component
3. move_workers system
   └─> Move toward target at 3 units/sec
   └─> Check if reached (distance < 0.5)
4. start_mission_on_arrival
   └─> Worker state → Working
   └─> Update mission status → InProgress
   └─> Generate mission .md file
   └─> Spawn Claude CLI subprocess
   └─> Store process in CliManagerResource
5. check_cli_completions (every frame)
   └─> Poll subprocess status
   └─> If complete:
       ├─> Parse token usage from output
       ├─> Update mission status → Completed
       ├─> Update worker stats
       ├─> Increment project completion
       ├─> Trigger building upgrade if needed
       ├─> Add tokens to budget
       └─> Worker state → Idle
6. send_idle_to_leisure
   └─> Move worker back to leisure zone
7. Repeat
```

---

## Core Systems

### 1. Project System

**Files:**
- `src/game/project/mod.rs` - Data structures
- `src/game/resources.rs` - ProjectManager
- `src/game/systems/project_spawner.rs` - Spawning logic
- `src/game/systems/progress_tracker.rs` - Stage upgrades

**Components:**
```rust
#[derive(Component)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub building_theme: String,
    pub total_missions: u32,
    pub completed_missions: u32,
}
```

**Key Functions:**
- `visual_stage()`: Maps completion % to stages 0-10
- `spawn_project_buildings()`: Arranges buildings in spiral pattern using golden angle (137.5°)

**Spiral Placement Algorithm:**
```rust
fn golden_angle_spiral(index: usize, base_distance: f32) -> (f32, f32) {
    let golden_angle = 137.5_f32.to_radians();
    let angle = index as f32 * golden_angle;
    let distance = base_distance + (index as f32 * distance_increment);
    (angle.cos() * distance, angle.sin() * distance)
}
```

---

### 2. Mission System

**Files:**
- `src/game/project/mod.rs` - Mission struct
- `src/game/systems/mission_manager.rs` - CRUD operations
- `src/game/systems/mission_writer.rs` - File generation

**Components:**
```rust
pub struct Mission {
    pub id: String,
    pub project_id: String,
    pub mission_number: u32,
    pub title: String,
    pub description: String,
    pub status: MissionStatus,
    pub dependencies: Vec<u32>,
    pub file_path: Option<String>,
}
```

**Mission File Format:**
```markdown
# Mission M1: Setup Project

**Project:** MyApp
**Mission Number:** 1
**Status:** in_progress
**Dependencies:** None

## Description
Initialize the repository and setup basic structure.

## Tasks
- [ ] Create README.md
- [ ] Setup .gitignore
- [ ] Initialize package.json

## Completion Criteria
- Repository has all basic files
- Project can be cloned and run

---
Started: 2026-01-19 10:30:00
Assigned: Alex (worker-abc-123)
```

---

### 3. Worker System

**Files:**
- `src/game/worker/mod.rs` - Worker component and state machine
- `src/game/resources.rs` - WorkerManager
- `src/game/systems/worker_spawner.rs` - Production queue
- `src/game/systems/worker_movement.rs` - Movement logic

**State Machine:**
```rust
pub enum WorkerState {
    Idle,                          // At leisure zone
    Ready,                         // Available for assignment
    Moving { target: Vec3 },       // Walking to destination
    Working { mission_id: String }, // Executing Claude CLI
    Crashed,                       // Error state
}
```

**Production Queue:**
```rust
pub struct WorkerProductionOrder {
    pub started_at: Instant,
    pub duration: Duration,  // 5 seconds
}

// Town Hall has Vec<WorkerProductionOrder>
// Processed every frame, spawns when duration elapsed
```

**Name Generation:**
```rust
const NAMES: &[&str] = &[
    "Alex", "Blake", "Casey", "Dakota", "Ellis", "Finley",
    "Gray", "Harper", "Indigo", "Jordan", "Kennedy", "Logan",
    "Morgan", "Nico", "Oakley", "Parker", "Quinn", "Riley",
    "Sage", "Taylor"
];
```

---

### 4. Claude CLI Integration

**Files:**
- `src/game/cli/mod.rs` - Subprocess management
- `src/game/resources.rs` - CliManagerResource wrapper
- `src/game/systems/task_assignment.rs` - Execution logic

**Process Management:**
```rust
pub struct ClaudeProcess {
    pub worker_id: String,
    pub mission_id: String,
    pub child: Child,  // std::process::Child
    pub started_at: Instant,
}

// Spawned with:
Command::new("claude")
    .arg("code")
    .arg("--task")
    .arg(mission_file)
    .current_dir(project_path)
    .spawn()
```

**Completion Detection:**
```rust
// Poll every frame
match process.child.try_wait() {
    Ok(Some(status)) => {
        // Process complete
        let output = read_stdout(&process);
        let tokens = extract_token_usage(&output);
        // Update mission, worker, budget
    }
    Ok(None) => {
        // Still running
    }
    Err(e) => {
        // Error, mark worker as Crashed
    }
}
```

**Token Extraction:**
```rust
// Parse Claude CLI output for patterns like:
// "Tokens used: 2,345"
// "Total tokens: 2345"
fn extract_token_usage(output: &str) -> u32 {
    // Regex or string search for token count
    // Fallback to 0 if not found
}
```

---

### 5. Autonomy System

**Files:**
- `src/game/systems/autonomous_assignment.rs` - Assignment logic
- `src/game/resources.rs` - AutonomySettings

**Decision Algorithm:**
```rust
fn calculate_priority(project: &Project, mission: &Mission) -> f32 {
    let mut priority = 0.0;

    // Factor 1: Stage boundary proximity
    let current_stage = project.visual_stage();
    let next_completion = project.completed_missions + 1;
    let next_stage = (next_completion * 10) / project.total_missions;

    if next_stage > current_stage {
        priority += 10.0;  // Crossing stage boundary
    }

    // Factor 2: Project age (older = higher priority)
    priority += project.created_timestamp / 1000.0;

    // Factor 3: Mission number (earlier = higher priority)
    priority -= mission.mission_number as f32 * 0.1;

    priority
}
```

**Assignment Loop:**
```rust
// Every 3 seconds (configurable)
if autonomy.enabled {
    let idle_workers = query_idle_workers();
    let available_missions = query_available_missions();

    // Respect max concurrent
    if working_workers.len() >= autonomy.max_concurrent {
        return;
    }

    // Check budget
    if token_budget.remaining() < 1000 {
        return;  // Too low
    }

    // Sort missions by priority
    available_missions.sort_by(|a, b| {
        priority(b).partial_cmp(&priority(a))
    });

    // Assign highest priority
    if let Some(worker) = idle_workers.first() {
        if let Some(mission) = available_missions.first() {
            assign(worker, mission);
        }
    }
}
```

---

### 6. Budget System

**Files:**
- `src/game/systems/token_tracker.rs` - Tracking and warnings
- `src/game/resources.rs` - TokenBudget

**Budget Structure:**
```rust
pub struct TokenBudget {
    pub hourly_limit: u64,
    pub current_period_used: u64,
    pub period_start: DateTime<Utc>,
    pub period_duration_hours: i64,
    pub warning_threshold: f32,  // 0.2 = 20%
}
```

**Reset Logic:**
```rust
fn check_budget_reset(mut budget: ResMut<TokenBudget>) {
    let now = Utc::now();
    let elapsed = now.signed_duration_since(budget.period_start);

    if elapsed.num_hours() >= budget.period_duration_hours {
        budget.current_period_used = 0;
        budget.period_start = now;
        println!("🔄 Token budget reset!");
    }
}
```

**Warning System:**
```rust
fn display_budget_warnings(
    budget: Res<TokenBudget>,
    mut last_warning: Local<Option<Instant>>,
) {
    let remaining_pct = budget.remaining() as f32 / budget.hourly_limit as f32;

    if remaining_pct <= budget.warning_threshold {
        // Warn every 2 minutes
        if should_warn(&mut last_warning) {
            println!("⚠️  Token budget low: {}%", remaining_pct * 100.0);
        }
    }
}
```

---

### 7. Stats System

**Files:**
- `src/game/systems/stats_updater.rs` - Calculation
- `src/game/systems/stats_display.rs` - Rendering

**Stats Resource:**
```rust
pub struct GameStats {
    pub total_workers: u32,
    pub idle_workers: u32,
    pub working_workers: u32,
    pub total_missions: u32,
    pub available_missions: u32,
    pub in_progress_missions: u32,
    pub completed_missions: u32,
    pub active_projects: u32,
    pub average_completion: f32,
}
```

**Update Frequency:**
```rust
// Every 1 second
let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);

fn update_game_stats(
    time: Res<Time>,
    mut timer: Local<Timer>,
    mut stats: ResMut<GameStats>,
    workers: Query<&Worker>,
    missions: Res<MissionManager>,
    projects: Query<&Project>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    // Recalculate all stats
    stats.total_workers = workers.iter().count() as u32;
    stats.idle_workers = workers.iter()
        .filter(|w| matches!(w.state, WorkerState::Idle))
        .count() as u32;
    // ... etc
}
```

---

## Database Schema

### Tables

**projects**
```sql
CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE,
    building_theme TEXT DEFAULT 'generic',
    visual_stage INTEGER DEFAULT 0,
    position_x REAL DEFAULT 0.0,
    position_y REAL DEFAULT 0.0,
    position_z REAL DEFAULT 0.0,
    total_missions INTEGER DEFAULT 0,
    completed_missions INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_updated DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**missions**
```sql
CREATE TABLE missions (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    mission_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT DEFAULT 'not_started',
    dependencies TEXT,  -- JSON array
    file_path TEXT,
    assigned_worker_id TEXT,
    started_at DATETIME,
    completed_at DATETIME,
    tokens_used INTEGER DEFAULT 0,
    completion_summary TEXT,
    FOREIGN KEY (project_id) REFERENCES projects(id),
    UNIQUE(project_id, mission_number)
);
```

**workers**
```sql
CREATE TABLE workers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    color_r REAL DEFAULT 0.5,
    color_g REAL DEFAULT 0.5,
    color_b REAL DEFAULT 0.5,
    state TEXT DEFAULT 'idle',
    current_task_id TEXT,
    total_tasks_completed INTEGER DEFAULT 0,
    total_tokens_used INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**app_state**
```sql
CREATE TABLE app_state (
    key TEXT PRIMARY KEY,
    value_json TEXT NOT NULL
);

-- Examples:
-- key='camera_state', value='{"x":10,"y":5,"z":10,...}'
-- key='token_budget', value='{"used":12000,"period_start":"..."}'
```

**sessions**
```sql
CREATE TABLE sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    ended_at DATETIME,
    tasks_completed INTEGER DEFAULT 0,
    tokens_used INTEGER DEFAULT 0,
    summary TEXT
);
```

**knowledge_entries** (Placeholder for V1.1+)
```sql
CREATE TABLE knowledge_entries (
    id TEXT PRIMARY KEY,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    project_name TEXT,
    mission_id TEXT,
    task_description TEXT,
    problem_encountered TEXT,
    solution_applied TEXT,
    code_patterns TEXT,
    reasoning TEXT,
    files_modified TEXT,
    tokens_used INTEGER,
    success BOOLEAN,
    tags TEXT,
    search_keywords TEXT
);

CREATE INDEX idx_knowledge_search ON knowledge_entries(search_keywords);
```

---

## Entity-Component-System Design

### Entities

| Entity Type | Components | Purpose |
|-------------|-----------|---------|
| **Town Hall** | `TownHall`, `Transform`, `SpatialBundle` | Worker production |
| **Project Building** | `Project`, `StagedBuilding`, `Transform`, `SpatialBundle`, visual children | Visual representation of project |
| **Worker** | `Worker`, `Transform`, `SpatialBundle`, optional `MovementTarget` | Claude CLI instance |
| **Leisure Zone** | `LeisureZone`, `Transform`, mesh | Idle worker gathering point |
| **Camera** | `Camera3d`, `Transform`, `CameraState` | Player viewpoint |
| **Ground** | `Transform`, `Mesh`, `Material` | World floor |

### Resources (Global State)

| Resource | Purpose |
|----------|---------|
| `AppPaths` | File system paths |
| `Database` | SQLite connection wrapper |
| `ProjectManager` | Project CRUD operations |
| `MissionManager` | Mission CRUD operations |
| `WorkerManager` | Worker CRUD operations |
| `CliManagerResource` | Active subprocess tracking |
| `AutonomySettings` | Autonomy configuration |
| `TokenBudget` | Budget tracking |
| `GameStats` | Real-time statistics |
| `CameraState` | Camera position/target |

### Systems (Logic)

Systems operate on components via queries:

```rust
// Example: Move workers toward targets
fn move_workers(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MovementTarget, &Worker)>,
) {
    for (mut transform, target, _worker) in query.iter_mut() {
        let direction = (target.position - transform.translation).normalize();
        let move_speed = 3.0;
        let movement = direction * move_speed * time.delta_seconds();
        transform.translation += movement;
    }
}
```

---

## Implementation Details

### Camera System

**File:** `src/camera.rs`

**Features:**
- WASD pan controls
- Scroll wheel zoom
- Automatic state persistence to DB (throttled to every 2 seconds)
- Restore on startup

**Code:**
```rust
pub fn camera_pan(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();
    let speed = 10.0;
    let delta = time.delta_seconds() * speed;

    if keyboard.pressed(KeyCode::KeyW) {
        transform.translation.z -= delta;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        transform.translation.z += delta;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        transform.translation.x -= delta;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        transform.translation.x += delta;
    }
}
```

---

### Building Renderer

**File:** `src/game/systems/building_renderer.rs`

**Visual Stages:**
```rust
fn stage_to_dimensions(stage: u8) -> (f32, f32, f32) {
    match stage {
        0 => (2.0, 1.0, 2.0),    // Small cube
        1 => (2.5, 2.0, 2.5),
        2 => (3.0, 3.0, 3.0),
        3 => (3.0, 4.0, 3.0),
        4 => (3.0, 5.0, 3.0),
        5 => (3.0, 7.0, 3.0),    // Mid-height
        6 => (3.0, 9.0, 3.0),
        7 => (3.5, 11.0, 3.5),
        8 => (4.0, 13.0, 4.0),
        9 => (4.0, 15.0, 4.0),
        10 => (4.0, 18.0, 4.0),  // Skyscraper
        _ => (2.0, 1.0, 2.0),
    }
}
```

**Update System:**
```rust
fn update_building_visuals(
    query: Query<(&StagedBuilding, &Children), Changed<StagedBuilding>>,
    mut mesh_query: Query<(&mut Transform, &mut Handle<Mesh>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (building, children) in query.iter() {
        let (width, height, depth) = stage_to_dimensions(building.current_stage.as_u8());

        // Update child mesh
        for &child in children.iter() {
            if let Ok((mut transform, mut mesh_handle)) = mesh_query.get_mut(child) {
                *mesh_handle = meshes.add(Cuboid::new(width, height, depth));
                // Position so base is at ground level
                transform.translation.y = height / 2.0;
            }
        }
    }
}
```

---

### Leisure Zone

**File:** `src/game/systems/leisure_zone.rs`

**Position:** (-20, 0, -20) northwest of Town Hall
**Visual:** Green circle mesh at ground level
**Purpose:** Idle worker gathering point

**Spawning:**
```rust
pub fn spawn_leisure_zone(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        LeisureZone,
        PbrBundle {
            mesh: meshes.add(Circle::new(5.0)),
            material: materials.add(Color::srgb(0.2, 0.8, 0.3)),
            transform: Transform::from_xyz(-20.0, 0.01, -20.0)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
            ..default()
        },
    ));
}
```

---

## Performance Considerations

### Frame Rate Target

- **Target:** 60 FPS
- **Acceptable:** 30 FPS with 10 workers
- **Stress Test:** 20 workers max

### Optimization Strategies

1. **Query Caching**: Use `Query::iter()` efficiently
2. **System Ordering**: Critical systems run first
3. **Throttled Updates**: Stats update every 1s, not every frame
4. **Lazy Database Writes**: Batch updates where possible
5. **Process Polling**: Only check completions, don't block

### Memory Profile

- **Baseline:** ~200 MB (empty world)
- **10 Workers:** ~350 MB
- **20 Workers:** ~500 MB
- **Database:** ~10 MB (after 1000 missions)

### CPU Usage

- **Idle:** 5-10% (rendering only)
- **Active (5 workers):** 15-25%
- **Heavy (10 workers):** 30-50%

---

## Future Enhancements

### V1.1 - UI Polish

- Persistent Bevy UI HUD with stats
- Worker name tags (floating text)
- Building selection and info panel
- Help screen (F1)
- FPS counter (F3)
- Settings panel

### V1.2 - Audio & Effects

- Background music
- SFX for events (spawn, complete, upgrade)
- Particle effects on building upgrades
- Better worker animations

### V2.0 - Advanced Features

- Zac^ chat interface (conversational control)
- Project creation wizard
- Worker specialties/personalities
- Knowledge base integration
- Vector search for similar problems
- Multi-machine worker pools
- Custom building themes

### V3.0 - Enterprise

- Multi-user support
- Web dashboard
- Metrics export
- CI/CD integration
- Webhook notifications
- API for external tools

---

## Development Setup

### Clone and Build

```bash
git clone <repo-url>
cd zac-caret/app
cargo build
cargo run
```

### Development Mode

```bash
# Hot reload (requires cargo-watch)
cargo watch -x run

# With debug logging
RUST_LOG=debug cargo run

# Release build
cargo build --release
```

### Database Inspection

```bash
sqlite3 ~/zac-caret/data/zac.db

.tables
.schema projects
SELECT * FROM workers;
```

### Debugging

```bash
# Verbose Bevy logs
RUST_LOG=bevy=debug,zac_caret=trace cargo run

# Performance profiling
cargo flamegraph
```

---

## Testing Strategy

### V1.0 Testing Approach

- **Unit Tests:** Pure functions in `core/` modules
- **Integration Tests:** Database operations
- **Manual Testing:** Visual systems and gameplay
- **Smoke Tests:** App launches without errors

### Future Testing

- Property-based testing for state machines
- Bevy system testing with mock worlds
- Subprocess mocking for CI
- Load testing with 50+ workers

---

## Contributing

### Code Style

- Follow Rust conventions (rustfmt)
- Use clippy for linting
- Document public APIs with `///`
- Keep functions <100 lines

### PR Guidelines

1. One feature per PR
2. Include tests where applicable
3. Update documentation
4. No breaking changes without discussion

### Areas for Contribution

- UI/UX improvements
- Performance optimization
- New building themes
- Audio system integration
- Knowledge base implementation

---

## Conclusion

Zac^ demonstrates how game engine architecture (ECS) can be applied to productivity tools. By treating projects as entities and tasks as behaviors, it creates an intuitive, visual interface for complex AI orchestration.

The system is designed for extensibility: new building types, worker specialties, and autonomy strategies can be added without core refactoring.

**Next Steps for Developers:**

1. Read `docs/01_SYSTEM_ARCHITECTURE.md` for design rationale
2. Explore `src/game/` for implementation details
3. Run the app and observe behavior
4. Modify autonomy algorithm for experimentation
5. Contribute UI polish features

---

**Happy Hacking! 🦀**

*Zac^ Command Center - Built with Rust + Bevy*

# Zac^ System Architecture Document
## Version 1.0 | January 2026

---

## 1. Executive Summary

Zac^ is a **gamified AI agent orchestration platform** that transforms project portfolio management into an RTS-inspired experience. Users command Claude-powered workers through a 3D game interface, managing multiple software projects as evolving buildings in a persistent digital settlement.

**Core Value Proposition:**
- Visualize project progress as growing 3D structures
- Autonomously orchestrate multiple Claude Code CLI instances
- Accumulate institutional knowledge through a learning AI foreman (Zac^)
- Gamify productivity to maintain motivation across complex, multi-project workflows

---

## 2. System Overview

### 2.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Zac^ Desktop Application                          │
│                              (Tauri Shell)                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────┐      ┌─────────────────────────────────────┐  │
│  │     Game World (Bevy)   │      │      UI Layer (WebView/Leptos)      │  │
│  │                         │      │                                     │  │
│  │  • 3D Scene Rendering   │◄────►│  • Chat Interface                   │  │
│  │  • Zac^ Hero Unit       │ IPC  │  • Journal/Idea Log                 │  │
│  │  • Worker Entities      │      │  • Task Lists                       │  │
│  │  • Building Models      │      │  • Stats HUD                        │  │
│  │  • Camera Controls      │      │  • Settings Panel                   │  │
│  │  • Leisure Zone         │      │  • Notifications                    │  │
│  └───────────┬─────────────┘      └──────────────┬──────────────────────┘  │
│              │                                    │                         │
│              └────────────┬───────────────────────┘                         │
│                           │                                                 │
│              ┌────────────▼────────────┐                                    │
│              │      Core Engine        │                                    │
│              │                         │                                    │
│              │  • State Manager        │                                    │
│              │  • Event Bus            │                                    │
│              │  • Persistence Layer    │                                    │
│              └────────────┬────────────┘                                    │
│                           │                                                 │
├───────────────────────────┼─────────────────────────────────────────────────┤
│                           │                                                 │
│  ┌────────────────────────▼────────────────────────┐                        │
│  │              Zac^ Intelligence Core             │                        │
│  │                                                 │                        │
│  │  • Decision Engine (task assignment logic)     │                        │
│  │  • Knowledge Base (accumulated learnings)      │                        │
│  │  • Pattern Analyzer (your work style)          │                        │
│  │  • Dependency Resolver (task sequencing)       │                        │
│  │  • Question Queue (stores Qs for you)          │                        │
│  └────────────────────────┬────────────────────────┘                        │
│                           │                                                 │
│  ┌────────────────────────▼────────────────────────┐                        │
│  │              Agent Orchestrator                  │                        │
│  │                                                 │                        │
│  │  • Worker Pool Manager                         │                        │
│  │  • Claude CLI Spawner                          │                        │
│  │  • Task Queue & Scheduler                      │                        │
│  │  • Performance Monitor (CPU/Token tracking)    │                        │
│  │  • Context Snapshot Manager                    │                        │
│  └─────────────────────────────────────────────────┘                        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    │               │               │
                    ▼               ▼               ▼
           ┌──────────────┐ ┌─────────────┐ ┌──────────────┐
           │  Local FS    │ │   SQLite    │ │ Claude API   │
           │              │ │             │ │              │
           │ • Projects   │ │ • State     │ │ • Workers    │
           │ • .md files  │ │ • Memory    │ │ • Zac^ brain │
           │ • Assets     │ │ • History   │ │              │
           └──────────────┘ └─────────────┘ └──────────────┘
```

### 2.2 Technology Stack

| Component | Technology | Justification |
|-----------|------------|---------------|
| Application Shell | **Tauri 2.x** | Lightweight (~50MB), Rust-native, secure IPC |
| Game Engine | **Bevy 0.14+** | Pure Rust, ECS architecture, excellent 3D support |
| UI Framework | **Leptos** (in WebView) | Rust-native, reactive, fast compilation |
| Database | **SQLite** (via rusqlite) | Embedded, zero-config, portable |
| Vector Search | **sqlite-vss** | Semantic search for knowledge base |
| API Client | **reqwest** | Async HTTP for Claude API calls |
| Process Management | **tokio** | Async runtime for CLI orchestration |
| Serialization | **serde** | JSON/TOML for configs and state |
| Audio | **rodio** | Cross-platform sound effects |

### 2.3 Directory Structure

```
~/zac-caret/
├── app/                          # Application binary and assets
│   ├── assets/
│   │   ├── models/               # 3D models (.gltf)
│   │   ├── textures/             # Textures and sprites
│   │   ├── sounds/               # Sound effects
│   │   └── fonts/                # UI fonts
│   └── config/
│       └── default.toml          # Default settings
│
├── data/
│   ├── zac.db                    # SQLite: state, memory, history
│   ├── knowledge.db              # SQLite + VSS: accumulated learnings
│   └── snapshots/                # Worker context snapshots
│
├── projects/                     # All managed projects live here
│   ├── moon-scratch/
│   │   ├── PROJECT_ROADMAP.md
│   │   ├── .zac/                 # Zac^ project metadata
│   │   │   ├── config.toml
│   │   │   ├── tasks.json
│   │   │   └── building.json     # Visual state
│   │   └── [project files...]
│   │
│   └── sumo-bets/
│       └── [same structure...]
│
├── logs/
│   ├── session/                  # Per-session logs
│   ├── workers/                  # Per-worker task logs
│   └── zac-journal.md            # Zac^'s running journal
│
└── user/
    ├── settings.toml             # User preferences
    └── api-key                   # Symlink to OS keychain reference
```

---

## 3. Core Components

### 3.1 Game World (Bevy)

The game world renders the RTS-style interface using Bevy's ECS (Entity-Component-System) architecture.

#### 3.1.1 Entity Types

| Entity | Components | Description |
|--------|------------|-------------|
| **Zac^** | `Transform`, `HeroUnit`, `Selectable`, `ActionBar`, `Journal` | The player's avatar/foreman |
| **Worker** | `Transform`, `WorkerState`, `Selectable`, `TaskAssignment`, `Personality` | Claude CLI instances |
| **Building** | `Transform`, `Project`, `MilestoneProgress`, `VisualStage`, `Selectable` | Project representations |
| **TownHall** | `Transform`, `TownHallLevel`, `WorkerCapacity`, `Selectable` | Central hub |
| **LeisureZone** | `Transform`, `Activities`, `OccupantList` | Worker rest area |
| **Terrain** | `Transform`, `Mesh`, `Material` | Ground, water, mountains |

#### 3.1.2 Visual Stages System

Buildings evolve through 10 visual stages based on milestone completion:

```rust
struct VisualStage {
    stage: u8,                    // 0-9
    base_model: Handle<Scene>,    // Core geometry
    attachments: Vec<Attachment>, // Modular additions
    particles: Option<ParticleEffect>,
}

struct Attachment {
    model: Handle<Scene>,
    position: Vec3,
    unlocked_at_milestone: u8,
}
```

**Example: Moon Scratch Rocket**
- Stage 0: Foundation pad
- Stage 1: Rocket body frame
- Stage 2: Body with ticket logo
- Stage 3: Fins attached
- Stage 4: Windshield
- Stage 5: Engine section
- Stage 6: Fuel tanks
- Stage 7: Launch tower
- Stage 8: Smoke/steam effects
- Stage 9: Full launch-ready with gantry

#### 3.1.3 Camera System

```rust
struct CameraController {
    target: Vec3,
    distance: f32,        // Zoom level
    pitch: f32,           // Vertical angle (clamped)
    yaw: f32,             // Horizontal rotation
    pan_speed: f32,
    zoom_speed: f32,
}

// Persisted across sessions
struct CameraState {
    position: Vec3,
    target: Vec3,
    distance: f32,
}
```

Controls mirror Starcraft 2:
- **WASD / Arrow Keys / Edge Pan**: Move camera
- **Mouse Wheel**: Zoom in/out
- **Middle Mouse Drag**: Rotate view
- **Double-click minimap** (V2): Jump to location

### 3.2 UI Layer (WebView + Leptos)

The UI runs in Tauri's WebView, communicating with Bevy via IPC.

#### 3.2.1 UI Panels

| Panel | Trigger | Contents |
|-------|---------|----------|
| **Action Bar** | Select any entity | Context-sensitive buttons (WC3 style) |
| **Chat Interface** | Click Zac^ chat button | Claude-powered command input |
| **Journal** | Click Zac^ journal button | Timestamped learnings and observations |
| **Idea Log** | Click Zac^ idea button | Suggestions, patterns, questions for you |
| **Task List** | Click building | Available/active/completed tasks |
| **Worker Status** | Click worker | State, last task summary, specialty |
| **Stats HUD** | Always visible (corner) | Workers, tasks, tokens, projections |
| **Settings** | Hotkey or menu | Preferences, API key, autonomy toggle |

#### 3.2.2 Stats HUD Layout

```
┌─────────────────────────────────────┐
│ 👷 5/8 Workers Active               │
│ 📋 12 Tasks in Progress             │
│ ✅ 47 Tasks Completed (session)     │
│ 🪙 ~2,340 tokens/hr burn rate       │
│ 🍖 Food: ████████░░ 78%             │
│    (resets in 4h 23m)               │
└─────────────────────────────────────┘
```

### 3.3 Zac^ Intelligence Core

Zac^ is the autonomous AI foreman—a persistent Claude instance that manages workers, learns your patterns, and grows with your projects.

#### 3.3.1 Decision Engine

```rust
struct DecisionEngine {
    autonomy_enabled: bool,
    task_queue: PriorityQueue<Task>,
    dependency_graph: DependencyGraph,
    worker_pool: Vec<WorkerRef>,
}

impl DecisionEngine {
    /// Continuously recalculates optimal task assignments
    async fn optimization_loop(&mut self) {
        loop {
            if self.autonomy_enabled {
                self.resolve_dependencies();
                self.assign_available_tasks();
                self.handle_idle_workers();
                self.check_worker_health();
            }
            sleep(Duration::from_secs(5)).await;
        }
    }
    
    /// Determines which tasks can run in parallel vs sequentially
    fn resolve_dependencies(&mut self) {
        // Analyze PROJECT_ROADMAP.md for task dependencies
        // Identify file conflicts (two tasks touching same file)
        // Build execution DAG
    }
    
    /// Assigns tasks to workers based on:
    /// - Worker specialties (accumulated from history)
    /// - Task dependencies (what's unblocked)
    /// - System load (CPU/token budget)
    fn assign_available_tasks(&mut self) {
        for worker in self.idle_workers() {
            if let Some(task) = self.next_optimal_task(&worker) {
                self.assign(worker, task);
            } else {
                self.send_to_leisure(worker);
            }
        }
    }
}
```

#### 3.3.2 Knowledge Base

Accumulated learnings stored with semantic search capability:

```rust
struct KnowledgeEntry {
    id: Uuid,
    timestamp: DateTime<Utc>,
    project: String,
    task_type: String,           // "frontend", "rust", "solana", etc.
    problem: String,             // What was the challenge
    solution: String,            // How it was solved
    embedding: Vec<f32>,         // 1536-dim vector for semantic search
    tokens_used: u32,
    success: bool,
}

impl KnowledgeBase {
    /// Called when a worker completes a task
    async fn learn(&mut self, completion: TaskCompletion) {
        let entry = self.create_entry(completion);
        let embedding = self.embed(&entry.problem, &entry.solution).await;
        self.store(entry.with_embedding(embedding));
    }
    
    /// Called when a worker starts a task
    async fn recall(&self, task: &Task) -> Vec<KnowledgeEntry> {
        let query_embedding = self.embed(&task.description, "").await;
        self.search_similar(query_embedding, limit: 5, threshold: 0.7)
    }
}
```

**Context Injection Flow:**
1. Worker receives new task
2. Knowledge base searched for relevant past solutions
3. Up to 1000 tokens of relevant context injected into worker's starting prompt
4. Worker executes task with accumulated wisdom

#### 3.3.3 Pattern Analyzer

Tracks your decision patterns to become a better assistant:

```rust
struct PatternAnalyzer {
    decision_log: Vec<Decision>,
    work_sessions: Vec<SessionMetrics>,
    preferences: InferredPreferences,
}

struct Decision {
    timestamp: DateTime<Utc>,
    context: String,           // What was happening
    choice: String,            // What you decided
    outcome: Option<Outcome>,  // How it turned out
}

struct InferredPreferences {
    preferred_task_order: Vec<String>,      // "tests first" vs "impl first"
    break_patterns: Duration,                // How often you context-switch
    review_thoroughness: f32,                // Quick glance vs detailed review
    naming_conventions: HashMap<String, String>,
}
```

### 3.4 Agent Orchestrator

Manages the pool of Claude Code CLI worker instances.

#### 3.4.1 Worker Lifecycle

```
┌─────────┐    Spawn     ┌─────────┐    Assign    ┌─────────┐
│  Idle   │─────────────►│ Ready   │─────────────►│ Working │
│(Leisure)│              │         │              │         │
└────┬────┘              └─────────┘              └────┬────┘
     │                                                 │
     │              ┌─────────┐                        │
     │   Reflect    │Reflecting│    Complete           │
     │◄─────────────│         │◄───────────────────────┘
     │              └─────────┘
     │                   │
     │                   │ Snapshot saved
     │◄──────────────────┘
```

#### 3.4.2 Worker State Machine

```rust
enum WorkerState {
    Idle { location: LeisureActivity },
    Ready { awaiting_assignment: bool },
    Working { 
        task: TaskRef,
        started_at: DateTime<Utc>,
        cli_process: ProcessHandle,
    },
    Reflecting {
        last_task: TaskCompletion,
        reflection_prompt: String,
    },
    Crashed {
        error: String,
        last_task: TaskRef,
        recovery_attempts: u8,
    },
}

struct Worker {
    id: Uuid,
    name: String,                    // "John", "Sarah", etc.
    appearance: WorkerAppearance,    // Visual customization
    state: WorkerState,
    specialty_scores: HashMap<String, f32>,  // Accumulated expertise
    total_tasks_completed: u32,
    total_tokens_used: u64,
    personality: Personality,        // Cosmetic flavor
}
```

#### 3.4.3 Claude CLI Integration

```rust
struct CliSpawner {
    working_dir: PathBuf,
    api_key: SecureString,
    max_concurrent: usize,
    active_processes: HashMap<Uuid, Child>,
}

impl CliSpawner {
    async fn spawn_worker(&mut self, worker: &Worker, task: &Task) -> Result<Child> {
        // Build the task prompt with knowledge injection
        let context = self.knowledge_base.recall(task).await;
        let prompt = self.build_prompt(task, context);
        
        // Write task file
        let task_file = self.write_task_md(task, &prompt)?;
        
        // Spawn claude code CLI
        let child = Command::new("claude")
            .args(["--dangerously-skip-permissions"])
            .current_dir(&task.project_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        
        // Monitor for completion
        self.monitor_process(worker.id, child, task).await
    }
    
    async fn monitor_process(&self, worker_id: Uuid, mut child: Child, task: &Task) {
        // Stream stdout for progress updates
        // Detect completion or errors
        // Generate task completion summary
        // Trigger reflection if enabled
    }
}
```

#### 3.4.4 Task Resume System

When app restarts, workers resume interrupted tasks:

```rust
struct TaskCheckpoint {
    task_id: Uuid,
    worker_id: Uuid,
    started_at: DateTime<Utc>,
    last_activity: DateTime<Utc>,
    progress_markers: Vec<String>,    // Completed subtasks
    context_snapshot: String,         // Compressed context
}

impl AgentOrchestrator {
    async fn restore_session(&mut self) -> Result<()> {
        for checkpoint in self.db.load_active_checkpoints()? {
            let worker = self.get_worker(checkpoint.worker_id)?;
            let task = self.get_task(checkpoint.task_id)?;
            
            // Resume with context
            self.resume_task(worker, task, checkpoint).await?;
        }
        Ok(())
    }
}
```

### 3.5 Persistence Layer

#### 3.5.1 SQLite Schema

```sql
-- Core state
CREATE TABLE workers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    appearance_json TEXT,
    state TEXT NOT NULL,
    specialty_scores_json TEXT,
    total_tasks_completed INTEGER DEFAULT 0,
    total_tokens_used INTEGER DEFAULT 0,
    personality_json TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE,
    building_theme TEXT,
    visual_stage INTEGER DEFAULT 0,
    milestone_progress_json TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    conquered_at DATETIME  -- NULL if created fresh
);

CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    project_id TEXT REFERENCES projects(id),
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,  -- 'available', 'blocked', 'in_progress', 'completed'
    assigned_worker_id TEXT REFERENCES workers(id),
    milestone_index INTEGER,
    dependencies_json TEXT,
    started_at DATETIME,
    completed_at DATETIME,
    tokens_used INTEGER,
    completion_summary TEXT
);

CREATE TABLE task_checkpoints (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id),
    worker_id TEXT REFERENCES workers(id),
    last_activity DATETIME,
    progress_markers_json TEXT,
    context_snapshot TEXT
);

-- Zac^ memory
CREATE TABLE zac_journal (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    entry_type TEXT NOT NULL,  -- 'observation', 'learning', 'decision', 'question'
    content TEXT NOT NULL,
    related_project_id TEXT REFERENCES projects(id),
    related_task_id TEXT REFERENCES tasks(id)
);

CREATE TABLE zac_ideas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    idea TEXT NOT NULL,
    context TEXT,
    status TEXT DEFAULT 'pending',  -- 'pending', 'accepted', 'dismissed'
    related_project_id TEXT REFERENCES projects(id)
);

CREATE TABLE zac_questions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    question TEXT NOT NULL,
    context TEXT,
    answered BOOLEAN DEFAULT FALSE,
    answer TEXT
);

-- Knowledge base (separate DB with vector extension)
CREATE TABLE knowledge_entries (
    id TEXT PRIMARY KEY,
    timestamp DATETIME,
    project TEXT,
    task_type TEXT,
    problem TEXT,
    solution TEXT,
    tokens_used INTEGER,
    success BOOLEAN
);

-- Vector embeddings stored via sqlite-vss virtual table
CREATE VIRTUAL TABLE knowledge_embeddings USING vss0(
    embedding(1536)
);

-- Session history
CREATE TABLE sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at DATETIME,
    ended_at DATETIME,
    tasks_completed INTEGER,
    tokens_used INTEGER,
    milestones_reached INTEGER,
    summary TEXT
);

-- App state
CREATE TABLE app_state (
    key TEXT PRIMARY KEY,
    value_json TEXT
);
-- Stores: camera_position, town_hall_level, autonomy_enabled, etc.
```

#### 3.5.2 File-Based State

Some state lives in project folders for portability:

**`projects/moon-scratch/.zac/config.toml`**
```toml
[project]
name = "Moon Scratch"
theme = "rocket"
created_at = "2026-01-15T10:30:00Z"

[building]
visual_stage = 4
position = { x = 10.0, y = 0.0, z = -5.0 }

[milestones]
total = 10
completed = 4
current = "Implement prize distribution"
```

**`projects/moon-scratch/.zac/tasks.json`**
```json
{
  "tasks": [
    {
      "id": "task-001",
      "title": "Implement symbol weighting",
      "milestone_index": 2,
      "status": "completed",
      "completed_at": "2026-01-14T15:42:00Z"
    },
    {
      "id": "task-002", 
      "title": "Prize distribution logic",
      "milestone_index": 3,
      "status": "in_progress",
      "dependencies": ["task-001"]
    }
  ]
}
```

---

## 4. Data Flows

### 4.1 Project Creation Flow

```
User clicks "Start New Project" on Zac^
            │
            ▼
┌─────────────────────────────────────┐
│  Chat Interface Opens               │
│  User describes project vision      │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Zac^ processes via Claude API      │
│  Generates:                         │
│  • PROJECT_ROADMAP.md               │
│  • 10 milestones with tasks         │
│  • Suggested building theme         │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  User reviews, edits, or reruns     │
└─────────────┬───────────────────────┘
              │ Accept
              ▼
┌─────────────────────────────────────┐
│  Zac^ scaffolds:                    │
│  ~/zac-caret/projects/{name}/       │
│  ├── PROJECT_ROADMAP.md             │
│  ├── .zac/config.toml               │
│  └── .zac/tasks.json                │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Building entity spawns in world    │
│  30-second "foundation" animation   │
│  Workers become assignable          │
└─────────────────────────────────────┘
```

### 4.2 Project Conquering Flow

```
User clicks "Conquer Project" on Zac^
            │
            ▼
┌─────────────────────────────────────┐
│  File picker / path input opens     │
│  User selects existing project dir  │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Zac^ analyzes project structure    │
│  Looks for existing roadmap or      │
│  prompts user to provide/create one │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  .zac/ folder created in project    │
│  User confirms milestone status     │
│  Suggests building theme            │
└─────────────┬───────────────────────┘
              │ Confirm
              ▼
┌─────────────────────────────────────┐
│  Building spawns at current stage   │
│  based on completed milestones      │
└─────────────────────────────────────┘
```

### 4.3 Task Execution Flow

```
Zac^ identifies available task
(or user manually assigns)
            │
            ▼
┌─────────────────────────────────────┐
│  Knowledge Base queried             │
│  Relevant past solutions retrieved  │
│  (up to 1000 tokens)                │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Task .md file generated            │
│  Includes: task description,        │
│  context, relevant knowledge,       │
│  success criteria                   │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Claude Code CLI spawned            │
│  Worker visual walks to building    │
│  Working animation starts           │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  CLI executes task                  │
│  Progress streamed to Zac^          │
│  Checkpoint saved periodically      │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Task completes                     │
│  Status log generated               │
│  Knowledge base updated             │
│  Worker enters Reflection state     │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  If milestone complete:             │
│  • 30-second upgrade animation      │
│  • Building visual evolves          │
│  • Sound effect plays               │
└─────────────────────────────────────┘
```

### 4.4 Worker Reflection Flow

```
Worker completes task
            │
            ▼
┌─────────────────────────────────────┐
│  Reflection prompt generated:       │
│  "What did you learn from this      │
│   task? What patterns emerged?      │
│   What would you do differently?"   │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Small Claude API call (~500 tok)   │
│  Worker "meditates" visually        │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Reflection stored in:              │
│  • Worker's personal notes          │
│  • Knowledge base (if valuable)     │
│  • Specialty scores updated         │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Worker returns to Idle             │
│  Zac^ may immediately reassign      │
│  or send to Leisure Zone            │
└─────────────────────────────────────┘
```

---

## 5. IPC Protocol

Communication between Bevy (game world) and WebView (UI) via Tauri IPC.

### 5.1 Event Types

```rust
// Bevy → WebView
enum GameEvent {
    EntitySelected { entity_type: String, id: Uuid, data: Value },
    EntityDeselected,
    WorkerStateChanged { worker_id: Uuid, new_state: WorkerState },
    TaskCompleted { task_id: Uuid, summary: String },
    MilestoneReached { project_id: Uuid, milestone: u8 },
    NotificationCreated { notification: Notification },
    StatsUpdated { stats: GameStats },
}

// WebView → Bevy
enum UiCommand {
    SelectEntity { id: Uuid },
    MoveCamera { target: Vec3 },
    AssignWorkerToTask { worker_id: Uuid, task_id: Uuid },
    CreateProject { vision: String },
    ConquerProject { path: PathBuf },
    ToggleAutonomy { enabled: bool },
    PauseAllWorkers,
    ResumeAllWorkers,
    SendWorkerToLeisure { worker_id: Uuid },
    ChatWithZac { message: String },
    AcceptIdea { idea_id: Uuid },
    AnswerQuestion { question_id: Uuid, answer: String },
}
```

### 5.2 Message Format

```json
{
  "type": "WorkerStateChanged",
  "payload": {
    "worker_id": "550e8400-e29b-41d4-a716-446655440000",
    "new_state": {
      "type": "Working",
      "task": "task-002",
      "started_at": "2026-01-15T14:30:00Z"
    }
  },
  "timestamp": "2026-01-15T14:30:01Z"
}
```

---

## 6. Security Considerations

### 6.1 API Key Storage

- Stored in OS keychain (never in plaintext files)
- Retrieved at runtime, held in memory as `SecureString`
- Never logged or transmitted except to Claude API

### 6.2 Project Isolation

- Each Claude CLI instance runs in its project directory
- No cross-project file access without explicit configuration
- Worker processes sandboxed to project scope

### 6.3 User Data

- All data stored locally (no cloud sync in V1)
- Knowledge base contains work summaries, not source code
- Session logs can be purged via settings

---

## 7. Performance Considerations

### 7.1 Resource Budgets

| Resource | Budget | Monitoring |
|----------|--------|------------|
| RAM (app) | 200-400 MB | Stats HUD |
| RAM (per worker) | 50-100 MB | Worker count limit |
| CPU (idle) | <5% | Background optimization |
| CPU (active) | 10-30% | Worker throttling |
| Tokens/hour | User-defined | Food supply meter |

### 7.2 Optimization Strategies

- **Lazy loading**: 3D models loaded on-demand
- **LOD system**: Distant buildings use simpler geometry
- **Event batching**: UI updates throttled to 10/sec
- **Knowledge pruning**: Old, low-value entries archived
- **Worker pooling**: CLI processes reused when possible

---

## 8. Demo Mode

For testing without API costs:

```rust
struct DemoMode {
    enabled: bool,
    mock_workers: Vec<MockWorker>,
    simulated_progress: f32,
}

impl MockWorker {
    fn simulate_task(&mut self, task: &Task) {
        // Fake progress over time
        // Generate mock completion summaries
        // No actual Claude API calls
    }
}
```

Activated via settings or `--demo` flag.

---

## 9. Future Considerations (V2+)

Architectural hooks for future features:

- **Multi-machine networking**: Worker pool spans multiple computers
- **Unit types**: Scout, Merchant, Military (different Claude configurations)
- **Research tree**: Unlockable Zac^ capabilities
- **Map expansion**: Procedural terrain generation
- **Collaborative mode**: Multiple human operators
- **Cloud sync**: Optional backup/sync of knowledge base

---

## 10. Appendix

### 10.1 Glossary

| Term | Definition |
|------|------------|
| **Zac^** | The AI foreman hero unit and your digital twin |
| **Worker** | A Claude Code CLI instance represented as a game character |
| **Building** | A 3D structure representing a software project |
| **Milestone** | A major project checkpoint that triggers visual upgrades |
| **Knowledge Base** | Accumulated learnings from all task completions |
| **Food Supply** | Token budget visualization (hunger = burn rate) |
| **Conquest** | Importing an existing project into Zac^ management |

### 10.2 File Formats

**PROJECT_ROADMAP.md structure:**
```markdown
# Project Name

## Overview
Brief description of the project.

## Milestones

### Milestone 1: Foundation
- [ ] Task 1.1: Description
- [ ] Task 1.2: Description

### Milestone 2: Core Features
- [ ] Task 2.1: Description
  - Depends on: Task 1.1
- [ ] Task 2.2: Description

[... up to Milestone 10]

## Building Theme
Suggested: Rocket ship (evolves from launchpad to full rocket)

## Notes
Additional context for Zac^ and workers.
```

### 10.3 Configuration Reference

**settings.toml:**
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
camera_speed = 10.0
zoom_min = 5.0
zoom_max = 50.0
show_worker_names = true

[audio]
master_volume = 0.8
notification_sounds = true
ambient_sounds = true

[tokens]
hourly_budget = 50000
warning_threshold = 0.2  # Warn at 20% remaining

[demo]
enabled = false
simulation_speed = 1.0
```

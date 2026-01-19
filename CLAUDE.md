# CLAUDE.md - Zac^ Development Context

## Project Overview

Zac^ is a gamified AI agent orchestration platform—an RTS-inspired desktop app for managing software projects. Built with Tauri + Bevy (Rust), it visualizes projects as evolving 3D buildings with Claude Code CLI workers.

## Tech Stack

- **App Shell:** Tauri 2.x
- **Game Engine:** Bevy 0.14 (Rust)
- **UI Framework:** Leptos (Rust → WASM, runs in Tauri WebView)
- **Database:** SQLite (rusqlite)
- **API:** Claude API (reqwest)
- **Platform:** Windows + WSL (development), cross-platform target

## Project Structure

```
~/zac-caret/
├── app/                    # Tauri + Bevy application
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── camera.rs       # Camera controller
│   │   ├── game/           # Bevy game systems
│   │   ├── core/           # Database, config, state
│   │   ├── agents/         # Worker orchestration
│   │   └── ui/             # IPC handlers
│   ├── assets/             # Models, textures, sounds
│   └── Cargo.toml
├── ui/                     # Leptos WebView UI
├── data/                   # SQLite databases, snapshots
├── projects/               # Managed project folders
├── docs/                   # Architecture & roadmap
└── logs/                   # Session and worker logs
```

## Key Architectural Decisions

1. **Hybrid Rendering:** Bevy handles 3D world, Tauri WebView handles text UI
2. **Primitive 3D First:** Start with cubes/capsules, replace with models later
3. **Knowledge Accumulation:** Workers learn from past tasks via vector search
4. **Centralized Structure:** All projects under ~/zac-caret/projects/
5. **OS Keychain:** API keys stored securely, never in config files
6. **Session Resume:** Tasks checkpoint and resume across app restarts

## Current Development Phase

Check `docs/02_V1_PROJECT_ROADMAP.md` for current milestone. Each milestone has:
- Specific tasks with clear success criteria
- Dependencies on previous milestones
- Visual stage upgrades for the "Command Tower" building

## Code Conventions

### Rust
- Use `thiserror` for custom errors, `anyhow` for propagation
- Prefer `Result<T>` returns over panics
- Document public APIs with `///` doc comments
- Use Bevy's ECS patterns: Components for data, Systems for logic

### Bevy Specifics
- Components are plain structs with `#[derive(Component)]`
- Systems are functions that take `Query<>`, `Res<>`, `Commands`
- Use `Name` component for debugging entity identification
- Keep `dynamic_linking` feature in dev, remove for release

### File Naming
- Rust: `snake_case.rs`
- Components: singular (`worker.rs` not `workers.rs`)
- Systems: verb-based (`handle_selection.rs`, `update_movement.rs`)

## Common Commands

```bash
# Development
cd ~/zac-caret/app
cargo run                    # Run app
cargo watch -x run           # Hot reload

# UI Development  
cd ~/zac-caret/ui
trunk serve                  # Serve WebView at :1420

# Full Tauri Dev
cd ~/zac-caret/app
cargo tauri dev              # Run with WebView integration

# Database Reset
rm ~/zac-caret/data/zac.db   # Delete and restart app to recreate
```

## Important Files to Reference

- `docs/01_SYSTEM_ARCHITECTURE.md` - Full technical spec
- `docs/02_V1_PROJECT_ROADMAP.md` - Milestone tasks
- `app/src/core/database.rs` - Schema definitions
- `app/Cargo.toml` - Dependencies

## Testing Approach

- **Demo Mode:** Mock workers without API calls (use `--demo` flag)
- **Unit Tests:** For pure functions in `core/` and `agents/`
- **Manual Testing:** For Bevy systems (visual verification)

## What NOT to Do

- Don't store API keys in files—use OS keychain
- Don't create separate CSS/JS files—inline in Leptos components
- Don't over-engineer V1—follow the milestone scope strictly
- Don't use `println!`—use `tracing` macros (`info!`, `debug!`, `warn!`)
- Don't panic on recoverable errors—propagate with `?`

## Asking for Clarification

If requirements are unclear, check these docs in order:
1. `docs/01_SYSTEM_ARCHITECTURE.md` - Technical decisions
2. `docs/02_V1_PROJECT_ROADMAP.md` - Feature scope
3. This chat history - Design rationale and Q&A

## Entity Quick Reference

| Entity | Key Components | Purpose |
|--------|---------------|---------|
| Zac^ | `HeroUnit`, `Selectable`, `ActionBar` | Player's foreman avatar |
| Worker | `WorkerState`, `TaskAssignment`, `Personality` | Claude CLI instance |
| Building | `Project`, `MilestoneProgress`, `VisualStage` | Project representation |
| TownHall | `TownHallLevel`, `WorkerCapacity` | Central hub |

## IPC Events (Bevy ↔ WebView)

```rust
// Bevy → WebView
GameEvent::EntitySelected { entity_type, id, data }
GameEvent::WorkerStateChanged { worker_id, new_state }
GameEvent::TaskCompleted { task_id, summary }

// WebView → Bevy  
UiCommand::AssignWorkerToTask { worker_id, task_id }
UiCommand::ChatWithZac { message }
UiCommand::ToggleAutonomy { enabled }
```

## Remember

This is a **productivity tool first**, game second. Every feature should make managing projects easier or more motivating. No fluff, no time-wasting animations, no blocking dialogs.

The goal: Zac^ eventually manages itself while you focus on high-level decisions.

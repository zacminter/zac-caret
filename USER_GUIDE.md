# Zac^ Command Center - User Guide

**Version:** 1.0
**Date:** 2026-01-19
**Status:** Production Ready

---

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Core Concepts](#core-concepts)
4. [Controls Reference](#controls-reference)
5. [Understanding the Interface](#understanding-the-interface)
6. [Workflow Examples](#workflow-examples)
7. [Configuration](#configuration)
8. [Troubleshooting](#troubleshooting)

---

## Introduction

Zac^ is a gamified AI agent orchestration platform that turns software project management into an RTS-inspired desktop experience. It visualizes your projects as evolving 3D buildings and manages Claude Code CLI workers to autonomously complete tasks.

### What Zac^ Does

- **Manages Multiple Projects**: Each project is represented as a visual building
- **Autonomous Task Execution**: AI workers complete missions without micromanagement
- **Budget Management**: Tracks token usage to prevent API overuse
- **Progress Visualization**: Buildings evolve through 10 visual stages as missions complete
- **Session Persistence**: All state saves automatically and resumes on restart

---

## Getting Started

### Prerequisites

- Rust toolchain installed
- Claude Code CLI installed and configured
- Linux, macOS, or Windows with WSL
- 4GB RAM minimum, 8GB recommended

### Installation

```bash
# Clone the repository
cd ~/zac-caret

# Build the application
cd app
cargo build --release

# Run Zac^
cargo run --release
```

### First Launch

On first launch, Zac^ will:

1. Create database at `~/zac-caret/data/zac.db`
2. Initialize with default settings
3. Spawn the Town Hall at world center
4. Display the main 3D view

**Initial View:**
- Green ground plane
- Town Hall building at center (Stage 0)
- Leisure zone (green circle) northwest of Town Hall
- Empty world ready for projects

---

## Core Concepts

### 1. Projects as Buildings

Each software project is visualized as a 3D building that evolves through 10 stages:

- **Stage 0**: Simple cube (0% complete)
- **Stage 5**: Mid-height tower (50% complete)
- **Stage 10**: Full skyscraper (100% complete)

Buildings automatically upgrade as missions are completed.

### 2. Missions

Missions are tasks within a project:
- Each project has multiple missions
- Missions can depend on other missions
- Status: Not Started → In Progress → Completed
- Generated as `.md` files in `projects/<name>/missions/`

### 3. Workers

Workers are Claude Code CLI instances:
- Represented as colored capsules
- Have unique names (Alex, Blake, Casey, etc.)
- Track tasks completed and tokens used
- States: Idle, Ready, Moving, Working, Crashed

**Worker Lifecycle:**
1. Spawned from Town Hall (5-second production time)
2. Idle in leisure zone (green circle)
3. Assigned to mission (manual or autonomous)
4. Walk to project building
5. Execute mission via Claude CLI
6. Return to leisure zone when complete

### 4. Autonomy (Zac^ Foreman)

When enabled, Zac^ autonomously assigns idle workers to missions:
- Checks every 3 seconds for available workers
- Prioritizes missions near stage boundaries
- Respects max concurrent workers limit (default: 5)
- Stops when token budget depleted

### 5. Token Budget

Hourly token usage limits:
- Default: 50,000 tokens/hour
- Tracks usage across all workers
- Warns at 20% remaining
- Blocks new assignments when depleted
- Auto-resets every hour

---

## Controls Reference

### Keyboard Controls

| Key | Action | Description |
|-----|--------|-------------|
| **W** | Spawn Worker | Starts 5-second production at Town Hall |
| **A** | Assign Task | Manually assigns idle worker to available mission |
| **Z** | Toggle Autonomy | Enable/disable Zac^ autonomous assignments |
| **S** | Show Stats | Display comprehensive statistics overlay |
| **WASD** | Camera Pan | Move camera around the world |
| **ESC** | Quit | Exit application |

### Mouse Controls

| Action | Control |
|--------|---------|
| **Zoom In/Out** | Scroll Wheel |
| **Pan Camera** | Middle Mouse Drag (alternative to WASD) |

---

## Understanding the Interface

### Main 3D View

**Ground Plane:** Green grid representing the world

**Town Hall:** Central building where workers are produced
- Always at world center (0, 0, 0)
- Has production queue UI

**Project Buildings:** Spiral pattern around Town Hall
- First project: 15 units away
- Each subsequent project: +3 units further
- Arranged using golden angle (137.5°) for even distribution

**Leisure Zone:** Green circular area northwest (-20, 0, -20)
- Where idle workers rest
- Workers automatically return here after tasks

**Workers:** Colored capsules moving around the world
- Color randomly assigned on spawn
- Move at 3 units/second
- Visible during all states

### Stats Display (Press 'S')

When you press 'S', a comprehensive stats overlay appears:

```
═══════════════════════════════════════════
          ZAC^ COMMAND CENTER
═══════════════════════════════════════════

👷 WORKERS
  Total: 5
  Idle: 2
  Working: 3

📋 MISSIONS
  Total: 24
  Available: 8
  In Progress: 3
  Completed: 13

🏗️  PROJECTS
  Active: 3
  Total Missions: 24
  Average Completion: 54%

💰 TOKEN BUDGET
  Used: 12,450 / 50,000 (24.9%)
  Remaining: 37,550
  [████████░░] 24.9%

  Burn Rate: 8,300 tokens/hour
  Time Until Reset: 45m 23s
  Status: ✅ Healthy

⚙️  AUTONOMY
  Status: ✅ ENABLED
  Max Concurrent: 5
  Check Interval: 3s
═══════════════════════════════════════════
Press 'S' again to hide
```

### Console Output

Real-time events logged to console:

```
🎮 Zac^ Command Center Started
📦 Loaded 3 projects from database
👷 Worker restored: Alex (ID: abc-123)
🔨 Worker spawned: Blake
🚀 Worker 'Blake' assigned to mission M1 for project 'MyApp'
🚶 Worker 'Blake' walking to project building...
⚡ Starting Claude CLI for worker 'Blake'
✅ Mission M1 completed! Tokens used: 2,345
🎉 Project 'MyApp' upgraded to stage 6!
```

---

## Workflow Examples

### Example 1: Manual Task Execution

**Scenario:** You want to manually control worker assignments.

**Steps:**

1. **Launch Zac^**
   ```bash
   cargo run
   ```

2. **Spawn a Worker**
   - Press **W** key
   - Wait 5 seconds for production
   - Worker appears at Town Hall, walks to leisure zone

3. **Assign Mission**
   - Press **A** key
   - Zac^ finds first available mission
   - Worker walks to project building
   - Claude CLI starts execution

4. **Monitor Progress**
   - Watch console for status updates
   - Worker stays at building during execution
   - Returns to leisure zone when complete

5. **Check Results**
   - Press **S** to view stats
   - See updated completion counts
   - Building visually upgraded if stage boundary crossed

---

### Example 2: Autonomous Operation

**Scenario:** Let Zac^ manage workers automatically for 1 hour.

**Steps:**

1. **Spawn Multiple Workers**
   - Press **W** five times (one per second)
   - Wait for all to complete production
   - All workers idle in leisure zone

2. **Enable Autonomy**
   - Press **Z** key
   - Console shows: "⚙️  Autonomy ENABLED"
   - Zac^ starts checking every 3 seconds

3. **Observe Autonomous Behavior**
   - Workers auto-assigned to missions
   - Prioritizes missions near stage boundaries
   - Max 5 workers concurrent (configurable)
   - Stops when budget low

4. **Monitor Budget**
   - Press **S** periodically to check stats
   - Watch token usage climb
   - See burn rate calculation
   - Get warning at 80% used (20% remaining)

5. **Depletion Handling**
   - When budget depleted: "⚠️  Token budget depleted!"
   - No new assignments until reset
   - Workers return to leisure zone
   - Budget auto-resets after 1 hour

---

### Example 3: Adding New Projects

**Scenario:** Add a new project to manage.

**Steps:**

1. **Create Project Directory**
   ```bash
   mkdir -p ~/zac-caret/projects/my-new-project
   cd ~/zac-caret/projects/my-new-project
   git init  # Optional: Git repo
   ```

2. **Add to Database**
   ```bash
   sqlite3 ~/zac-caret/data/zac.db
   ```

   ```sql
   INSERT INTO projects (id, name, path, total_missions, completed_missions)
   VALUES (
     'proj-123',
     'My New Project',
     '/home/user/zac-caret/projects/my-new-project',
     10,
     0
   );
   ```

3. **Add Missions**
   ```sql
   INSERT INTO missions (id, project_id, mission_number, title, description, status)
   VALUES
     ('m1', 'proj-123', 1, 'Setup Project', 'Initialize repo', 'not_started'),
     ('m2', 'proj-123', 2, 'Add README', 'Create README.md', 'not_started');
   -- Add more missions...
   ```

4. **Restart Zac^**
   - Quit (ESC) and relaunch
   - New project building spawns in spiral
   - Missions available for assignment

---

## Configuration

### Token Budget Settings

Edit in code: `src/game/resources.rs`

```rust
impl Default for TokenBudget {
    fn default() -> Self {
        Self {
            hourly_limit: 50_000,  // Adjust here
            warning_threshold: 0.2, // Warn at 20% remaining
            period_duration_hours: 1,
            // ...
        }
    }
}
```

### Autonomy Settings

Edit in code: `src/game/resources.rs`

```rust
impl Default for AutonomySettings {
    fn default() -> Self {
        Self {
            enabled: false,  // Start disabled
            max_concurrent_workers: 5,  // Adjust here
            assignment_interval_secs: 3.0,
        }
    }
}
```

### Worker Settings

Edit in code: `src/game/resources.rs`

```rust
impl WorkerManager {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
            max_workers: 20,  // Adjust here
        }
    }
}
```

### Camera Settings

Camera position saves automatically to database.
Reset by deleting: `~/zac-caret/data/zac.db` (WARNING: Deletes all state!)

---

## Troubleshooting

### Issue: Workers Not Moving

**Symptoms:** Workers spawn but don't move to leisure zone

**Solutions:**
1. Check console for errors
2. Verify leisure zone spawned (green circle northwest)
3. Restart application

---

### Issue: No Missions Available

**Symptoms:** Press 'A' but no assignment happens

**Solutions:**
1. Check database has missions: `SELECT * FROM missions WHERE status='not_started';`
2. Verify missions have no blocking dependencies
3. Add missions manually via SQL

---

### Issue: Claude CLI Not Starting

**Symptoms:** Workers reach building but nothing happens

**Solutions:**
1. Verify Claude Code CLI installed: `claude --version`
2. Check API keys configured: `claude configure`
3. Check console for subprocess errors
4. Verify mission file paths exist

---

### Issue: Token Budget Not Resetting

**Symptoms:** Budget stays depleted past 1 hour

**Solutions:**
1. Check system time is correct
2. Restart application (forces recalculation)
3. Manually reset in database:
   ```sql
   DELETE FROM app_state WHERE key='token_budget';
   ```

---

### Issue: Building Not Upgrading

**Symptoms:** Missions complete but building stays same stage

**Solutions:**
1. Check `completed_missions` in database matches reality
2. Verify `total_missions` is set correctly
3. Trigger manual sync by restarting app

---

### Issue: Application Won't Start

**Symptoms:** Immediate crash or errors on launch

**Solutions:**
1. Check Rust version: `rustc --version` (need 1.70+)
2. Rebuild: `cargo clean && cargo build`
3. Check dependencies: `cargo update`
4. Delete database (last resort): `rm ~/zac-caret/data/zac.db`

---

## Performance Tips

### For Large Projects (20+ missions)

- Keep max concurrent workers at 5 or lower
- Monitor FPS with console output
- Consider chunking missions into phases

### For Many Projects (10+)

- Buildings spiral outward, may need to zoom out
- Use camera WASD controls to navigate
- Consider filtering inactive projects (future feature)

### For Long Sessions (4+ hours)

- Monitor token budget closely
- Adjust hourly limit if needed
- Use manual mode for critical tasks
- Enable autonomy for background work

---

## Advanced Usage

### Custom Mission Priorities

Missions are auto-prioritized by:
1. Projects near stage boundaries (e.g., 49% → 50% = stage upgrade)
2. Oldest not-started missions
3. Missions with no dependencies

**Future:** Manual priority field in database

### Multi-Session Management

- Each session tracked in `sessions` table
- Resume interrupted sessions on restart
- View session history:
  ```sql
  SELECT * FROM sessions ORDER BY started_at DESC;
  ```

### Knowledge Base (Placeholder)

`knowledge_entries` table exists for future learning:
- Will store solutions from past missions
- Enable workers to learn from experience
- Semantic search for similar problems

**Current Status:** Not yet implemented (V1.1+)

---

## Keyboard Shortcuts Summary

```
W       → Spawn Worker
A       → Assign Task
Z       → Toggle Autonomy
S       → Show Stats
WASD    → Pan Camera
Scroll  → Zoom Camera
ESC     → Quit
```

---

## Next Steps

After mastering the basics:

1. **Experiment with Autonomy**: Enable and observe decision-making
2. **Monitor Token Usage**: Understand your burn rate
3. **Add Real Projects**: Integrate with your actual codebases
4. **Optimize Worker Count**: Find your ideal balance
5. **Customize Settings**: Tune budgets and limits to your workflow

---

## Support & Feedback

- **Issues**: GitHub issues page
- **Docs**: See `docs/` directory for architecture details
- **Contributing**: PRs welcome for V1.1 features

---

**Happy Building! 🚀**

*Zac^ Command Center - Autonomous AI Orchestration*

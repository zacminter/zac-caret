# How Zac^ Works - User Guide
**Version:** 0.5 (M1-M5 Complete)
**Last Updated:** 2026-01-19

---

## What Is Zac^?

Zac^ is a **gamified AI agent orchestration platform** - an RTS-inspired desktop application for managing software projects. It visualizes your projects as evolving 3D buildings, with AI workers (Claude Code CLI instances) completing tasks to upgrade them.

**Current Status:** Foundation complete. You can visualize projects as buildings that evolve based on mission completion.

---

## Getting Started

### Prerequisites
- Rust 1.70+ installed
- Linux/WSL environment (Windows support in progress)
- ~200MB disk space

### Installation
```bash
cd ~/zac-caret/app
cargo build --release
```

### First Run
```bash
cargo run
```

You'll see:
- A green meadow (ground plane)
- A wooden building at the center (Town Hall)
- A UI panel in the top-right corner
- Smooth lighting

---

## Current Features

### 1. Camera Controls
**Pan Camera:**
- W / Up Arrow: Move forward
- S / Down Arrow: Move backward
- A / Left Arrow: Move left
- D / Right Arrow: Move right

**Zoom:**
- Mouse Scroll Up: Zoom in (min height: 8 units)
- Mouse Scroll Down: Zoom out (max height: 60 units)

**Persistence:**
- Camera position saves every 5 seconds
- Restores when you restart the app

---

### 2. Building Stages (10 Stages)

Buildings evolve through 10 visual stages based on project completion:

| Stage | Name | Size | Color | Completion |
|-------|------|------|-------|------------|
| 0 | Empty | Tiny marker | Dirt brown | 0% |
| 1 | Foundation | Flat base | Stone gray | 10% |
| 2 | Frame | Frame structure | Wood | 20% |
| 3 | Walls & Roof | Basic building | Light wood | 30% |
| 4 | Complete | Finished | Finished wood | 40% |
| 5 | Enhanced | Larger | Polished | 50% |
| 6 | Second Floor | Multi-story | Refined | 60% |
| 7 | Tower | Tall tower | Noble | 70% |
| 8 | Decorated | Ornate | Elegant | 80% |
| 9 | Grand | Impressive | Majestic | 90% |
| 10 | Monument | Maximum | Golden accents | 100% |

---

### 3. UI Controls

**Town Hall Controls Panel** (Top-Right):
- Title: "Town Hall Controls"
- Stage Display: Shows current stage (e.g., "Stage: 4")
- Green Button: "Upgrade Stage" - Makes building larger
- Red Button: "Downgrade Stage" - Makes building smaller

**Interaction:**
- Click buttons to manually change stage
- Building mesh regenerates in real-time
- Colors and sizes update automatically

---

### 4. Multi-Project System

**How It Works:**
1. Projects are stored in SQLite database
2. Each project has:
   - Unique ID
   - Name
   - File path
   - Total missions count
   - Completed missions count
3. Projects spawn as buildings in a **spiral pattern**
4. Position calculated using **golden angle** (aesthetically pleasing)
5. Visual stage calculated as: `(completed / total) * 10`

**Adding a Project (Manual - via SQLite):**
```bash
# While app is running
sqlite3 ~/zac-caret/data/zac.db << SQL
INSERT INTO projects (id, name, path, total_missions, completed_missions)
VALUES (
  'my-project-1',
  'My Cool Project',
  '/home/user/my-project',
  20,
  8
);
SQL
```

**Result:**
- Wait 5 seconds (auto-sync runs)
- New building spawns at calculated position
- Stage = (8 / 20) * 10 = 4 (Complete)

---

### 5. Mission Progress Tracking

**How Missions Work:**
- Each project has numbered missions (1, 2, 3, ...)
- Missions can depend on other missions
- Status: not_started, in_progress, completed, failed, blocked
- Stored in `missions` table in database

**Updating Progress:**
```bash
# Simulate mission completion
sqlite3 ~/zac-caret/data/zac.db << SQL
UPDATE projects
SET completed_missions = 15
WHERE id = 'my-project-1';
SQL
```

**Result:**
- Wait 5 seconds
- Building auto-upgrades to stage 7 (75% complete)
- Console prints: 🎉 Project 'My Cool Project' upgraded to stage 7!

---

## Database Structure

**Location:** `~/zac-caret/data/zac.db`

**Key Tables:**

### projects
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

### missions
```sql
CREATE TABLE missions (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    mission_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT DEFAULT 'not_started',
    dependencies TEXT,  -- JSON array of mission numbers
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

### app_state
```sql
CREATE TABLE app_state (
    key TEXT PRIMARY KEY,
    value_json TEXT NOT NULL
);
```

Stores camera position as JSON.

---

## System Architecture

### Bevy ECS Pattern

**Components:**
- `StagedBuilding` - Building evolution state
- `Project` - Project data (name, missions, etc.)
- `Name` - Debug labels

**Resources:**
- `ProjectManager` - Database CRUD for projects
- `MissionManager` - Database CRUD for missions
- `CameraSettings` - Pan/zoom speed config
- `CameraState` - Serializable camera position

**Systems (Update Loop):**
- `camera_pan` - WASD movement
- `camera_zoom` - Mouse scroll
- `camera_save_state` - Persists every 5s
- `update_building_visuals` - Regenerates mesh on stage change
- `track_project_progress` - Auto-upgrades buildings
- `sync_project_data` - Syncs DB every 5s
- `handle_upgrade_button` - UI button handling
- `handle_downgrade_button` - UI button handling
- `update_stage_display` - Updates UI text
- `button_hover_system` - Button visual feedback

**Systems (Startup):**
- `setup_world` - Creates ground + lighting
- `spawn_camera_from_state` - Restores camera
- `spawn_initial_town_hall` - Creates Town Hall
- `spawn_project_buildings` - Loads projects from DB
- `spawn_building_controls` - Creates UI panel

---

## Performance

### Frame Rate
- **Target:** 60 FPS
- **Typical:** 60 FPS (with 1-10 buildings)
- **Low:** 30-45 FPS (with 50+ buildings)

### Memory
- **Base:** ~100MB
- **Per Building:** ~1MB
- **Database:** <10MB typically

### Build Times
- **Debug:** ~6s incremental, ~60s clean
- **Release:** ~5min clean build

---

## Troubleshooting

### App Won't Start
```bash
# Check if database is locked
rm ~/zac-caret/data/zac.db-shm
rm ~/zac-caret/data/zac.db-wal

# Recreate database
rm ~/zac-caret/data/zac.db
cargo run
```

### Buildings Don't Spawn
1. Check database has projects:
```bash
sqlite3 ~/zac-caret/data/zac.db "SELECT * FROM projects;"
```

2. Check console for errors:
```bash
cargo run 2>&1 | grep -i error
```

### Building Doesn't Upgrade
- Wait 5 seconds (sync interval)
- Check `completed_missions` was actually updated
- Restart app to force sync

### Camera Position Lost
- Check `app_state` table:
```bash
sqlite3 ~/zac-caret/data/zac.db "SELECT * FROM app_state WHERE key = 'camera_state';"
```

---

## Limitations (Current Version)

### Not Yet Implemented
- ❌ Worker spawning
- ❌ Task assignment
- ❌ Claude CLI integration
- ❌ Autonomous mode (Zac^ as foreman)
- ❌ Token tracking
- ❌ Sound effects
- ❌ Particle effects
- ❌ Project creation UI (manual SQL only)
- ❌ Mission file generation

### Known Issues
- No error messages in UI (check console)
- Cannot delete projects from UI
- No confirmation dialogs
- Building collision not implemented
- Camera can go underground

---

## File Locations

### Executables
- **Debug:** `target/debug/zac-caret`
- **Release:** `target/release/zac-caret`

### Data
- **Database:** `~/zac-caret/data/zac.db`
- **Logs:** stderr output (not saved to file yet)

### Source Code
- **Main:** `src/main.rs`
- **Camera:** `src/camera.rs`
- **Database:** `src/core/database.rs`
- **Game Logic:** `src/game/`
- **UI:** `src/ui/`

---

## Example Workflow

### Scenario: Visualizing 3 Projects

```bash
# 1. Start fresh
rm ~/zac-caret/data/zac.db
cargo build
cargo run &

# 2. Add 3 projects
sqlite3 ~/zac-caret/data/zac.db << SQL
INSERT INTO projects (id, name, path, total_missions, completed_missions) VALUES
('proj-a', 'Project Alpha', '/tmp/alpha', 10, 2),
('proj-b', 'Project Beta', '/tmp/beta', 20, 10),
('proj-c', 'Project Gamma', '/tmp/gamma', 15, 14);
SQL

# 3. Wait 5 seconds - buildings appear!

# 4. Simulate work on Project Beta
sleep 10
sqlite3 ~/zac-caret/data/zac.db << SQL
UPDATE projects SET completed_missions = 15 WHERE id = 'proj-b';
SQL

# 5. Wait 5 seconds - Project Beta building upgrades!
```

**Expected Result:**
- Town Hall in center (stage 4)
- Project Alpha: Stage 2 (20% done)
- Project Beta: Stage 5 (50% done) → upgrades to stage 7 (75%)
- Project Gamma: Stage 9 (93% done)

All positioned in spiral around Town Hall.

---

## Advanced Usage

### Custom Building Themes (Future)
Currently all projects use "generic" theme. Future versions will support:
- Medieval castle
- Modern office
- Cyberpunk tower
- Fantasy treehouse

### Mission Dependencies Example

```sql
INSERT INTO missions (id, project_id, mission_number, title, dependencies) VALUES
('m1', 'proj-a', 1, 'Setup Repository', '[]'),
('m2', 'proj-a', 2, 'Write Tests', '[1]'),      -- Depends on M1
('m3', 'proj-a', 3, 'Implement Feature', '[1]'),
('m4', 'proj-a', 4, 'Integration', '[2,3]');    -- Depends on M2 AND M3
```

Mission 4 won't be available until both M2 and M3 are complete.

---

## Technical Details

### Rendering
- **Engine:** Bevy 0.14
- **Graphics:** PBR (Physically Based Rendering)
- **Lighting:** Directional + Ambient
- **Materials:** StandardMaterial with base color

### Input Handling
- **Keyboard:** ButtonInput<KeyCode> resource
- **Mouse:** MouseWheel events
- **UI:** Bevy UI with Interaction component

### Persistence
- **Format:** JSON (camera state)
- **Storage:** SQLite TEXT column
- **Frequency:** Every 5 seconds

---

## Contributing

### To Add a New Feature
1. Create new system in `src/game/systems/`
2. Register in `src/game/systems/mod.rs`
3. Add to Update or Startup in `src/main.rs`
4. Test with `cargo run`

### To Add a New Component
1. Create in `src/game/components/`
2. Derive `Component` from bevy
3. Use in systems via Query

---

## FAQ

**Q: Why is there a Town Hall and other buildings?**
A: Town Hall is the default building. Other buildings represent your actual software projects.

**Q: Can I delete a project?**
A: Not from UI yet. Use SQL:
```sql
DELETE FROM projects WHERE id = 'your-project-id';
```

**Q: Why spiral positioning?**
A: Golden angle (137.5°) creates aesthetically pleasing, non-overlapping layout that scales indefinitely.

**Q: What happens if I have 100 projects?**
A: They'll all spawn in a spiral. Performance may degrade (not optimized yet).

**Q: Can I change building colors?**
A: Not directly. Colors are determined by BuildingStage. Future versions will support themes.

---

## Next Steps

See `COMPREHENSIVE_STATUS.md` for:
- What's coming in M6-M10
- Estimated completion times
- Implementation roadmap

---

**Enjoy exploring Zac^!** 🎮🏗️✨

# Zac^ Quick Start Guide

## Running the Application

```bash
cd ~/zac-caret/app
cargo run
```

**First launch:** The application will create a database at `~/zac-caret/data/zac.db`

---

## Basic Controls

| Key | Action |
|-----|--------|
| `W` | Spawn a new worker (5-second production timer) |
| `A` | Assign idle worker to available mission (manual mode) |
| `Z` | Toggle autonomous assignment ON/OFF |
| `S` | Display comprehensive statistics |
| `WASD` | Pan camera |
| `Mouse Scroll` | Zoom camera |

---

## Quick Workflow

### 1. Add a Test Project

```bash
# Create a test project directory
mkdir -p /tmp/test-project-alpha

# Add to database
sqlite3 ~/zac-caret/data/zac.db << SQL
INSERT INTO projects (id, name, path, total_missions, completed_missions)
VALUES ('test-1', 'Test Project Alpha', '/tmp/test-project-alpha', 10, 0);

INSERT INTO missions (id, project_id, mission_number, title, description, status, dependencies)
VALUES
('m1', 'test-1', 1, 'Setup Foundation', 'Initialize project', 'not_started', '[]'),
('m2', 'test-1', 2, 'Core Features', 'Implement main functionality', 'not_started', '[1]'),
('m3', 'test-1', 3, 'Testing', 'Write tests', 'not_started', '[2]');
SQL
```

### 2. Spawn Workers

- Press `W` three times
- Wait 5 seconds each (production time)
- Workers appear as colored capsules
- Workers automatically move to leisure zone (green circle)

### 3. Manual Mode

- Press `A` to assign an idle worker to a mission
- Worker walks to building
- Mission starts automatically when worker arrives
- Claude Code CLI executes the task
- Worker returns to idle when complete

### 4. Autonomous Mode

- Press `Z` to enable Zac^ autonomy
- Console shows: "🤖 ZAC^ AUTONOMY ENABLED..."
- Idle workers automatically get assigned every 3 seconds
- Up to 5 workers can work concurrently (configurable)
- Press `Z` again to disable

### 5. Monitor Progress

- Press `S` to see detailed stats:
  - Worker counts (total, idle, working)
  - Task progress
  - Token budget status with progress bar
  - Burn rate and time until reset
  - Autonomy status

---

## Understanding the Display

### Console Messages

**Worker spawning:**
```
✅ Worker 'Casey' completed training! (ID: uuid)
```

**Auto-assignment:**
```
🤖 Zac^ AUTO-ASSIGNED worker 'Jordan' to mission (priority: 125.5)
```

**Mission execution:**
```
🎬 Starting mission: Core Features
🚀 Spawning Claude CLI for mission: /path/to/M02.md
✅ Claude CLI spawned (process: uuid)
```

**Mission completion:**
```
🎉 Mission completed by worker: worker-uuid
   Duration: 45 seconds
   Success: true
   Tokens used: 1250
```

**Budget warnings:**
```
⚠️  TOKEN BUDGET LOW: 18.5% remaining (9250 tokens left)
   Resets in: 0h 35m
```

```
🚫 TOKEN BUDGET DEPLETED - No new tasks will start
   Resets in: 0h 15m
```

**Stats display (press 'S'):**
```
╔════════════════════════════════════════╗
║        ZAC^ COMMAND CENTER STATS       ║
╠════════════════════════════════════════╣
║ 👷 WORKERS                             ║
║   Total: 5                             ║
║   Idle:  2                             ║
║   Working: 3                           ║
║                                        ║
║ 📋 TASKS                               ║
║   In Progress: 3                       ║
║   Completed (session): 12              ║
║                                        ║
║ 💰 TOKEN BUDGET                        ║
║   Used: 25000/50000 (50.0%)            ║
║   Food: ██████████░░░░░░░░░░ 50%       ║
║   Resets in: 0h 30m                    ║
║   Burn rate: 1200 tok/hr               ║
╚════════════════════════════════════════╝
```

---

## Visual Indicators

### On Screen
- **Green ground plane:** World terrain
- **Colored building at center:** Town Hall (worker production)
- **Green circle:** Leisure zone (workers rest here when idle)
- **Colored capsules:** Workers (each has unique color)
- **Additional buildings:** Projects (appear in spiral pattern)

### Building Evolution
- Buildings grow as missions complete
- Visual stages: 0-10
- Stage based on completion percentage (e.g., 3/10 missions = stage 3)
- Upgrades happen automatically

---

## Common Tasks

### Spawn More Workers
```
Press 'W' multiple times
Wait 5 seconds each
Maximum: 20 workers (configurable)
```

### Check System Status
```
Press 'S' for full stats
Look for autonomy status
Check token budget remaining
See idle vs working counts
```

### Add More Projects
```bash
sqlite3 ~/zac-caret/data/zac.db << SQL
INSERT INTO projects (id, name, path, total_missions)
VALUES ('my-project', 'My Project Name', '/path/to/project', 10);

INSERT INTO missions (id, project_id, mission_number, title, status, dependencies)
VALUES ('m1', 'my-project', 1, 'Mission Title', 'not_started', '[]');
SQL
```

### View Database
```bash
# All projects
sqlite3 ~/zac-caret/data/zac.db "SELECT name, completed_missions, total_missions FROM projects;"

# All workers
sqlite3 ~/zac-caret/data/zac.db "SELECT name, state, total_tasks_completed FROM workers;"

# All missions
sqlite3 ~/zac-caret/data/zac.db "SELECT mission_number, title, status FROM missions;"
```

---

## Troubleshooting

### Workers don't spawn
- Check console for errors
- Verify Town Hall spawned (should see building at center)
- Wait full 5 seconds after pressing 'W'
- Check worker count limit (max 20 by default)

### Workers don't move
- Verify leisure zone spawned (green circle should be visible)
- Check console for "Sending worker to leisure zone" messages
- Workers only move when idle

### Missions don't start
- Ensure Claude Code CLI is installed: `which claude-code`
- Check project path exists: `ls /path/to/project`
- Verify mission dependencies are met
- Check token budget isn't depleted (press 'S')

### Autonomy not working
- Verify it's enabled: press 'Z' and look for "ENABLED" message
- Check budget isn't depleted: press 'S'
- Ensure there are idle workers
- Ensure there are available missions (no unmet dependencies)
- Check max concurrent workers limit not reached

### Budget depleted
- Wait for period reset (check remaining time with 'S')
- Or manually reset:
  ```bash
  sqlite3 ~/zac-caret/data/zac.db << SQL
  -- This would require adding budget persistence first
  -- For now, restart the app (budget resets to default)
  SQL
  ```

---

## Configuration

### Adjust Token Limit
Edit `src/game/resources.rs` and change:
```rust
impl Default for TokenBudget {
    fn default() -> Self {
        Self::new(50000)  // Change this value
    }
}
```

### Adjust Max Concurrent Workers
Edit `src/game/resources.rs`:
```rust
impl Default for AutonomySettings {
    fn default() -> Self {
        Self {
            enabled: false,
            max_concurrent_workers: 5,  // Change this value
            assignment_interval_secs: 3.0,
        }
    }
}
```

### Adjust Worker Limit
Edit `src/game/resources.rs`:
```rust
impl WorkerManager {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
            max_workers: 20,  // Change this value
        }
    }
}
```

After changes, rebuild:
```bash
cd ~/zac-caret/app
cargo build
```

---

## Tips & Best Practices

1. **Start small:** Test with 2-3 workers before scaling up
2. **Monitor budget:** Press 'S' regularly to check token usage
3. **Use autonomy:** Enable with 'Z' for hands-free operation
4. **Check mission dependencies:** Ensure missions are properly sequenced
5. **Watch console:** Important events are logged there
6. **Restart if stuck:** App restores all state from database

---

## Performance Notes

- **Smooth operation:** Up to 10 workers
- **Usable:** Up to 15 workers
- **Max limit:** 20 workers (may experience slowdown)

If performance degrades:
- Reduce max_concurrent_workers
- Spawn fewer workers
- Close other applications

---

## Need Help?

**Check logs:**
```bash
# Application should log to console
# Future: logs/ directory for persistent logs
```

**Reset everything:**
```bash
# Backup first!
cp ~/zac-caret/data/zac.db ~/zac-caret/data/zac.db.backup

# Delete database (will recreate on next launch)
rm ~/zac-caret/data/zac.db

# Restart app
cd ~/zac-caret/app && cargo run
```

**Read documentation:**
- `MILESTONES_M4_M10_COMPLETE.md` - Comprehensive technical docs
- `M8_M9_COMPLETE.txt` - Autonomy and budget system details
- Milestone guides in `~/moon-scratch/` - Original implementation guides

---

**Happy building! 🚀**

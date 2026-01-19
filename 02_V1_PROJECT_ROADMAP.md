# Zac^ V1 Project Roadmap
## Building the Builder | January 2026

---

## Overview

This roadmap defines the 10 milestones for building Zac^ V1—a gamified AI agent orchestration platform. Each milestone unlocks core functionality and corresponds to a visual evolution of the project.

**Building Theme:** Medieval Command Tower
- Evolves from a simple wooden watchtower to a grand stone command center
- Represents the growing capability to oversee and command your digital workforce

---

## Milestone 1: Foundation
**Visual Stage:** Wooden stakes marking the ground, surveyor's tent

### Objective
Establish the project structure, toolchain, and basic Tauri + Bevy integration.

### Tasks
- [ ] **1.1** Initialize Tauri 2.x project with Rust backend
- [ ] **1.2** Add Bevy as a dependency, configure windowing
- [ ] **1.3** Create basic 3D scene: flat terrain plane with grass texture
- [ ] **1.4** Implement camera controller (pan, zoom, rotate)
- [ ] **1.5** Set up WebView for UI (Leptos scaffolding)
- [ ] **1.6** Establish IPC bridge between Bevy and WebView
- [ ] **1.7** Create directory structure per architecture doc
- [ ] **1.8** Initialize SQLite database with schema
- [ ] **1.9** Implement basic app state persistence (camera position)

### Success Criteria
- App launches showing 3D terrain
- Camera controls work smoothly
- State persists across restarts

---

## Milestone 2: Town Hall
**Visual Stage:** Wooden scaffolding and foundation stones

### Objective
Create the central Town Hall building and basic selection system.

### Tasks
- [ ] **2.1** Create primitive Town Hall model (textured cube + flag)
- [ ] **2.2** Implement entity selection system (click to select)
- [ ] **2.3** Display selection indicator (outline or circle)
- [ ] **2.4** Create Action Bar UI component (appears on selection)
- [ ] **2.5** Town Hall shows basic stats when selected
- [ ] **2.6** Implement Town Hall level system (10 levels, visual changes)
- [ ] **2.7** Store Town Hall state in database
- [ ] **2.8** First-launch experience: Zac^ places Town Hall

### Success Criteria
- Town Hall renders in world
- Clicking shows Action Bar with stats
- Position persists across restarts

---

## Milestone 3: Zac^ Hero Unit
**Visual Stage:** Wooden watchtower frame erected

### Objective
Bring Zac^ to life as the controllable hero unit.

### Tasks
- [ ] **3.1** Create Zac^ character model (capsule body, sphere head, distinct color)
- [ ] **3.2** Implement pathfinding/movement system
- [ ] **3.3** Click-to-move for Zac^
- [ ] **3.4** Zac^ Action Bar with buttons: Chat, Journal, Ideas, Pause
- [ ] **3.5** Create Chat Interface UI (Claude-style input box)
- [ ] **3.6** Wire Chat to Claude API for responses
- [ ] **3.7** API key setup flow (first launch prompt, OS keychain storage)
- [ ] **3.8** Zac^ idle animations (looking around, subtle movement)
- [ ] **3.9** Zac^ walks to buildings when inspecting

### Success Criteria
- Zac^ moves around the map via click commands
- Chat interface sends messages to Claude and displays responses
- API key securely stored and retrieved

---

## Milestone 4: Workers
**Visual Stage:** Watchtower with basic roof, ladder

### Objective
Implement worker entities that can be spawned and visualized.

### Tasks
- [ ] **4.1** Create Worker model (simpler than Zac^, randomized colors)
- [ ] **4.2** Worker name generator (random first names)
- [ ] **4.3** Spawn workers from Town Hall (button in Action Bar)
- [ ] **4.4** Worker states: Idle, Ready, Working, Reflecting
- [ ] **4.5** Visual state indicators (icons above head, animations)
- [ ] **4.6** Worker selection shows status panel
- [ ] **4.7** Workers walk to Leisure Zone when idle
- [ ] **4.8** Create Leisure Zone area with activity spots
- [ ] **4.9** Workers choose random activities when resting
- [ ] **4.10** Worker persistence (survive app restart)

### Success Criteria
- Can spawn multiple workers
- Workers show different states visually
- Idle workers hang out in Leisure Zone
- Workers persist across sessions

---

## Milestone 5: Project Buildings
**Visual Stage:** Stone foundation added to watchtower

### Objective
Create and visualize project buildings that evolve with milestones.

### Tasks
- [ ] **5.1** Building entity system with visual stages (0-9)
- [ ] **5.2** Primitive building models (cube base, modular attachments)
- [ ] **5.3** "Start New Project" flow: chat → .md generation
- [ ] **5.4** PROJECT_ROADMAP.md template and parser
- [ ] **5.5** Project folder scaffolding on creation
- [ ] **5.6** Building theme suggestions from Claude
- [ ] **5.7** Building placement system (auto-place near Town Hall)
- [ ] **5.8** "Conquer Project" flow: file picker, import existing
- [ ] **5.9** Building upgrade animation (30-second timer, particles)
- [ ] **5.10** Building selection shows project stats and tasks

### Success Criteria
- Can create new project via chat with Zac^
- Building spawns and can be selected
- Conquering existing project works
- Milestone completion triggers visual upgrade

---

## Milestone 6: Task System
**Visual Stage:** Second story added to tower

### Objective
Implement the task management system that drives worker assignments.

### Tasks
- [ ] **6.1** Task data model (id, status, dependencies, assignment)
- [ ] **6.2** Parse tasks from PROJECT_ROADMAP.md
- [ ] **6.3** Task list UI when building selected
- [ ] **6.4** Task status indicators (available, blocked, in_progress, completed)
- [ ] **6.5** Manual task assignment: click worker → right-click building → select task
- [ ] **6.6** Task dependency resolver (identify what's unblocked)
- [ ] **6.7** Task .md file generation for Claude Code CLI
- [ ] **6.8** Multi-task queuing per worker
- [ ] **6.9** Task completion detection and status update
- [ ] **6.10** Milestone progress tracking (tasks → milestone → visual upgrade)

### Success Criteria
- Tasks display correctly from PROJECT_ROADMAP.md
- Can manually assign worker to task
- Task completion updates building progress

---

## Milestone 7: Claude Integration
**Visual Stage:** Tower gains stone walls, arrow slits

### Objective
Wire up actual Claude Code CLI execution for worker tasks.

### Tasks
- [ ] **7.1** Claude Code CLI spawner (tokio process management)
- [ ] **7.2** Task prompt builder (description + context + success criteria)
- [ ] **7.3** Worker visual transitions when assigned (walks to building, works)
- [ ] **7.4** Progress streaming from CLI stdout
- [ ] **7.5** Task completion summary generation
- [ ] **7.6** Worker status log on completion (duration, tokens, summary)
- [ ] **7.7** Error handling and crash recovery
- [ ] **7.8** Auto-restart crashed workers
- [ ] **7.9** "Wounded" worker notification on repeated failures
- [ ] **7.10** Task checkpointing for session resume

### Success Criteria
- Assigning task spawns real Claude CLI
- CLI executes and completes task
- Worker shows working animation during execution
- Crashed tasks auto-recover
- Tasks resume after app restart

---

## Milestone 8: Zac^ Autonomy
**Visual Stage:** Watchtower roof upgraded, flag flying

### Objective
Make Zac^ an autonomous manager that assigns tasks without intervention.

### Tasks
- [ ] **8.1** Autonomy toggle (pause/resume button)
- [ ] **8.2** Idle worker detection loop
- [ ] **8.3** Automatic task assignment logic
- [ ] **8.4** Task dependency-aware scheduling
- [ ] **8.5** Parallel vs sequential task analysis (file conflict detection)
- [ ] **8.6** Optimal path calculation for multi-project workloads
- [ ] **8.7** Worker reassignment when tasks unblock
- [ ] **8.8** System load monitoring (CPU, memory)
- [ ] **8.9** Dynamic worker throttling based on performance
- [ ] **8.10** Zac^ decision logging (why did he assign X to Y?)

### Success Criteria
- With autonomy on, Zac^ keeps workers busy
- No file conflicts between parallel workers
- System doesn't overload
- Can see why Zac^ made decisions

---

## Milestone 9: Knowledge & Learning
**Visual Stage:** Tower gains second flag, ornate windows

### Objective
Implement the accumulated knowledge system that makes workers smarter over time.

### Tasks
- [ ] **9.1** Knowledge base schema and storage
- [ ] **9.2** Embedding generation for task completions (Claude API)
- [ ] **9.3** sqlite-vss integration for vector search
- [ ] **9.4** Knowledge injection on task start (relevant past solutions)
- [ ] **9.5** Per-project toggle for knowledge injection
- [ ] **9.6** Worker reflection system (post-task meditation prompt)
- [ ] **9.7** Reflection storage and integration
- [ ] **9.8** Worker specialty scoring (track expertise areas)
- [ ] **9.9** Zac^ Journal implementation (learnings, observations)
- [ ] **9.10** Zac^ Idea Log (suggestions based on patterns)
- [ ] **9.11** Zac^ Question Queue (stores Qs for user)

### Success Criteria
- Workers receive relevant context from past tasks
- Reflections generate and store
- Worker specialties visible and influence assignment
- Zac^ Journal and Idea Log populate with content

---

## Milestone 10: Polish & Production
**Visual Stage:** Grand stone command tower with banners

### Objective
Complete the V1 experience with HUD, audio, and quality-of-life features.

### Tasks
- [ ] **10.1** Stats HUD (workers, tasks, tokens, food supply)
- [ ] **10.2** Token tracking and burn rate calculation
- [ ] **10.3** Food supply projection visualization
- [ ] **10.4** Notification system through Zac^
- [ ] **10.5** Sound effects (task complete, milestone, warning, ambient)
- [ ] **10.6** Session summary on app close
- [ ] **10.7** Session history log (on-demand view)
- [ ] **10.8** Demo mode (mock workers, no API calls)
- [ ] **10.9** Settings panel (all configurable options)
- [ ] **10.10** Performance optimization pass
- [ ] **10.11** Bug fixes and edge case handling
- [ ] **10.12** Documentation and README

### Success Criteria
- All HUD elements display accurately
- Sound effects enhance experience without annoyance
- Demo mode works completely offline
- App is stable and performant
- Ready for daily use

---

## Dependency Graph

```
M1 ─────► M2 ─────► M3 ─────► M4 ─────► M5
Foundation  Town    Zac^    Workers  Buildings
            Hall
                      │         │         │
                      └─────────┴─────────┘
                                │
                                ▼
                              M6
                            Tasks
                                │
                                ▼
                              M7
                           Claude CLI
                                │
                                ▼
                              M8
                           Autonomy
                                │
                                ▼
                              M9
                           Knowledge
                                │
                                ▼
                              M10
                            Polish
```

---

## Estimated Timeline

| Milestone | Estimated Effort | Cumulative |
|-----------|------------------|------------|
| M1: Foundation | 2-3 days | Week 1 |
| M2: Town Hall | 2-3 days | Week 1 |
| M3: Zac^ Hero | 3-4 days | Week 2 |
| M4: Workers | 3-4 days | Week 2-3 |
| M5: Buildings | 4-5 days | Week 3 |
| M6: Tasks | 4-5 days | Week 4 |
| M7: Claude CLI | 5-6 days | Week 5 |
| M8: Autonomy | 4-5 days | Week 6 |
| M9: Knowledge | 5-6 days | Week 7 |
| M10: Polish | 5-7 days | Week 8 |

**Total Estimated:** 6-8 weeks of focused development

---

## Risk Factors

| Risk | Mitigation |
|------|------------|
| Bevy learning curve | Start with primitives, upgrade models later |
| Claude CLI integration complexity | Build mock mode first, wire real CLI incrementally |
| Vector search performance | Start with simple keyword search, add embeddings later |
| Scope creep | Strictly enforce V1 boundary, log V2 ideas separately |
| Token costs during development | Use demo mode, set low budgets |

---

## Success Metrics for V1

- [ ] Can manage 3+ projects simultaneously
- [ ] Workers complete tasks autonomously for 1+ hours
- [ ] Knowledge injection demonstrably improves task quality
- [ ] App remains responsive with 10 workers active
- [ ] Session resume works reliably
- [ ] Daily driver for real project work

---

## Notes

This roadmap is itself managed by Zac^ once M5 is complete. Meta-inception achieved.

Building theme for this project: **Medieval Command Tower**
- Represents growing command and control capability
- Stone = stability, permanence
- Height = oversight, vision
- Flags = achievement, pride

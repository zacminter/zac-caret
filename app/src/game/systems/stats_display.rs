use bevy::prelude::*;
use crate::game::resources::{GameStats, TokenBudget, AutonomySettings};

/// System to display comprehensive stats
pub fn display_comprehensive_stats(
    stats: Res<GameStats>,
    token_budget: Res<TokenBudget>,
    autonomy: Res<AutonomySettings>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyS) {
        println!("\n╔════════════════════════════════════════╗");
        println!("║        ZAC^ COMMAND CENTER STATS       ║");
        println!("╠════════════════════════════════════════╣");

        // Workers
        println!("║ 👷 WORKERS                             ║");
        println!("║   Total: {:2}                           ║", stats.workers_total);
        println!("║   Idle:  {:2}                           ║", stats.workers_idle);
        println!("║   Working: {:2}                         ║", stats.workers_working);

        // Tasks
        println!("║                                        ║");
        println!("║ 📋 TASKS                               ║");
        println!("║   In Progress: {:2}                     ║", stats.tasks_in_progress);
        println!("║   Completed (session): {:3}            ║", stats.tasks_completed_session);

        // Projects
        println!("║                                        ║");
        println!("║ 🏗️  PROJECTS                           ║");
        println!("║   Total: {:2}                           ║", stats.projects_total);

        // Token Budget
        println!("║                                        ║");
        println!("║ 💰 TOKEN BUDGET                        ║");
        println!("║   Used: {}/{} ({:.1}%)      ║",
                 token_budget.current_period_used,
                 token_budget.hourly_limit,
                 token_budget.percentage_used());

        let remaining_pct = token_budget.percentage_remaining();
        let bar_length = 20;
        let filled = ((remaining_pct / 100.0) * bar_length as f32) as usize;
        let empty = bar_length - filled;

        let bar = format!("{}{}",
                         "█".repeat(filled),
                         "░".repeat(empty));

        println!("║   Food: {bar} {remaining_pct:.0}%       ║");

        let time_until_reset = token_budget.time_until_reset();
        let hours = time_until_reset.num_hours();
        let minutes = time_until_reset.num_minutes() % 60;

        println!("║   Resets in: {hours}h {minutes}m                  ║");

        let burn_rate = token_budget.estimated_burn_rate_per_hour();
        println!("║   Burn rate: {burn_rate:.0} tok/hr             ║");

        // Autonomy
        println!("║                                        ║");
        println!("║ 🤖 ZAC^ AUTONOMY                       ║");
        println!("║   Status: {}                      ║",
                 if autonomy.enabled { "ENABLED " } else { "DISABLED" });
        println!("║   Max Concurrent: {}                   ║", autonomy.max_concurrent_workers);

        println!("╚════════════════════════════════════════╝\n");
    }
}

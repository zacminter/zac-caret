use bevy::prelude::*;
use crate::game::resources::TokenBudget;

/// System to check for budget reset
pub fn check_budget_reset(
    mut token_budget: ResMut<TokenBudget>,
    time: Res<Time>,
    mut last_check: Local<f32>,
) {
    *last_check += time.delta_seconds();

    // Check every 60 seconds
    if *last_check < 60.0 {
        return;
    }
    *last_check = 0.0;

    if token_budget.should_reset() {
        token_budget.reset_period();
    }
}

/// System to display budget warnings
pub fn display_budget_warnings(
    token_budget: Res<TokenBudget>,
    time: Res<Time>,
    mut last_warning: Local<f32>,
    mut warning_shown: Local<bool>,
) {
    if token_budget.is_low() && !*warning_shown {
        println!("⚠️  TOKEN BUDGET LOW: {:.1}% remaining ({} tokens left)",
                 token_budget.percentage_remaining(),
                 token_budget.remaining());

        let time_until_reset = token_budget.time_until_reset();
        let hours = time_until_reset.num_hours();
        let minutes = time_until_reset.num_minutes() % 60;

        println!("   Resets in: {hours}h {minutes}m");

        *warning_shown = true;
    }

    if token_budget.is_depleted() {
        *last_warning += time.delta_seconds();

        // Remind every 5 minutes
        if *last_warning >= 300.0 {
            *last_warning = 0.0;

            println!("🚫 TOKEN BUDGET DEPLETED - No new tasks will start");

            let time_until_reset = token_budget.time_until_reset();
            let hours = time_until_reset.num_hours();
            let minutes = time_until_reset.num_minutes() % 60;

            println!("   Resets in: {hours}h {minutes}m");
        }
    }

    // Reset warning flag when budget recovers
    if !token_budget.is_low() && *warning_shown {
        *warning_shown = false;
    }
}

/// System to display budget status
pub fn display_budget_status(
    token_budget: Res<TokenBudget>,
    time: Res<Time>,
    mut last_display: Local<f32>,
) {
    *last_display += time.delta_seconds();

    // Display every 2 minutes
    if *last_display < 120.0 {
        return;
    }
    *last_display = 0.0;

    let burn_rate = token_budget.estimated_burn_rate_per_hour();

    println!("💰 Token Budget: {}/{} used ({:.1}%)",
             token_budget.current_period_used,
             token_budget.hourly_limit,
             token_budget.percentage_used());

    println!("   Burn rate: {burn_rate:.0} tokens/hour");

    if burn_rate > 0.0 {
        let hours_until_depleted = token_budget.remaining() as f32 / burn_rate;
        println!("   Estimated depletion: {hours_until_depleted:.1} hours at current rate");
    }
}

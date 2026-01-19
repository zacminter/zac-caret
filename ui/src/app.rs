use leptos::*;
use crate::components::{stats_hud::StatsHud, action_bar::ActionBar};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <StatsHud/>
        <ActionBar/>
    }
}

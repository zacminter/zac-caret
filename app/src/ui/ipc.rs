use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameStats {
    pub workers: u32,
    pub tasks: u32,
}

// IPC command handlers - stubs for M1, will be fully implemented in M2
#[allow(dead_code)]
#[tauri::command]
pub fn select_entity(_entity_id: String) -> Result<(), String> {
    // Placeholder - will be implemented in M2
    Ok(())
}

#[allow(dead_code)]
#[tauri::command]
pub fn send_chat_message(_message: String) -> Result<String, String> {
    // Placeholder - will be implemented in M2
    Ok("Received".to_string())
}

#[allow(dead_code)]
#[tauri::command]
pub fn get_game_stats() -> Result<GameStats, String> {
    // Placeholder - will be implemented in M2
    Ok(GameStats {
        workers: 0,
        tasks: 0,
    })
}

#[allow(dead_code)]
#[tauri::command]
pub fn toggle_autonomy(_enabled: bool) -> Result<(), String> {
    // Placeholder - will be implemented in M2
    Ok(())
}

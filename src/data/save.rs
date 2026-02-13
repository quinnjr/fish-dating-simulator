//! Save/load functionality using JSON files.

use std::path::PathBuf;

use crate::data::PlayerState;

fn save_path() -> PathBuf {
    let dir = dirs_next::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("fish-dating-simulator");
    std::fs::create_dir_all(&dir).ok();
    dir.join("save.json")
}

/// Save the player state to disk.
pub fn save_game(state: &PlayerState) -> Result<(), String> {
    let path = save_path();
    let json = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;
    tracing::info!("Game saved to {}", path.display());
    Ok(())
}

/// Load the player state from disk.
pub fn load_game() -> Option<PlayerState> {
    let path = save_path();
    if !path.exists() {
        return None;
    }
    let json = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&json).ok()
}

/// Check if a save file exists.
pub fn save_exists() -> bool {
    save_path().exists()
}

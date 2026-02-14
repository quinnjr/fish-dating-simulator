//! Game data types and state management.

pub mod dialogues;
pub mod save;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::achievements::UnlockedAchievements;
use crate::plugins::FishRegistry;

/// Unique fish identity.
///
/// Built-in fish use the named variants. Plugin fish use `Plugin(id_string)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum FishId {
    Bubbles,
    Marina,
    Gill,
    /// A plugin-defined fish, identified by its unique string ID.
    Plugin(String),
}

#[allow(dead_code)]
impl FishId {
    /// All built-in fish IDs (does not include plugins).
    pub const BUILTIN: [FishId; 3] = [FishId::Bubbles, FishId::Marina, FishId::Gill];

    /// Get all fish IDs including plugins.
    pub fn all_with_plugins(registry: &FishRegistry) -> Vec<FishId> {
        let mut all: Vec<FishId> = Self::BUILTIN.to_vec();
        for id in registry.plugin_ids() {
            all.push(FishId::Plugin(id.clone()));
        }
        all
    }

    /// Whether this is a plugin fish.
    pub fn is_plugin(&self) -> bool {
        matches!(self, FishId::Plugin(_))
    }

    pub fn name_with_registry(&self, registry: &FishRegistry) -> String {
        match self {
            FishId::Bubbles => "Bubbles".to_string(),
            FishId::Marina => "Marina".to_string(),
            FishId::Gill => "Gill".to_string(),
            FishId::Plugin(id) => registry
                .get(id)
                .map(|f| f.name.clone())
                .unwrap_or_else(|| id.clone()),
        }
    }

    pub fn species_with_registry(&self, registry: &FishRegistry) -> String {
        match self {
            FishId::Bubbles => "Clownfish".to_string(),
            FishId::Marina => "Swordfish".to_string(),
            FishId::Gill => "Pufferfish".to_string(),
            FishId::Plugin(id) => registry
                .get(id)
                .map(|f| f.species.clone())
                .unwrap_or_else(|| "Fish".to_string()),
        }
    }

    pub fn description_with_registry(&self, registry: &FishRegistry) -> String {
        match self {
            FishId::Bubbles => crate::ascii_art::BUBBLES_DESC.to_string(),
            FishId::Marina => crate::ascii_art::MARINA_DESC.to_string(),
            FishId::Gill => crate::ascii_art::GILL_DESC.to_string(),
            FishId::Plugin(id) => registry
                .get(id)
                .map(|f| f.description.clone())
                .unwrap_or_else(|| "A mysterious fish.".to_string()),
        }
    }

    /// Difficulty of catching this fish (0.0 = easy, 1.0 = hard).
    pub fn difficulty_with_registry(&self, registry: &FishRegistry) -> f32 {
        match self {
            FishId::Bubbles => 0.3,
            FishId::Marina => 0.6,
            FishId::Gill => 0.45,
            FishId::Plugin(id) => registry
                .get(id)
                .map(|f| f.difficulty)
                .unwrap_or(0.5),
        }
    }

    /// Which pond index this fish appears in.
    pub fn pond_index(&self) -> usize {
        match self {
            FishId::Bubbles => 0,
            FishId::Marina => 1,
            FishId::Gill => 2,
            // Plugin fish ponds are dynamically assigned after built-in ponds
            FishId::Plugin(_) => usize::MAX,
        }
    }

    /// The fish's color for rendering.
    pub fn color_with_registry(&self, registry: &FishRegistry) -> [f32; 4] {
        match self {
            FishId::Bubbles => crate::render::Colors::ORANGE,
            FishId::Marina => crate::render::Colors::LIGHT_BLUE,
            FishId::Gill => crate::render::Colors::GREEN,
            FishId::Plugin(id) => registry
                .get(id)
                .map(|f| f.color)
                .unwrap_or(crate::render::Colors::WHITE),
        }
    }

    // ── Legacy convenience methods (for code that doesn't have registry access) ──

    pub fn name(&self) -> &str {
        match self {
            FishId::Bubbles => "Bubbles",
            FishId::Marina => "Marina",
            FishId::Gill => "Gill",
            FishId::Plugin(id) => id.as_str(),
        }
    }

    pub fn species(&self) -> &str {
        match self {
            FishId::Bubbles => "Clownfish",
            FishId::Marina => "Swordfish",
            FishId::Gill => "Pufferfish",
            FishId::Plugin(_) => "Fish",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            FishId::Bubbles => crate::ascii_art::BUBBLES_DESC,
            FishId::Marina => crate::ascii_art::MARINA_DESC,
            FishId::Gill => crate::ascii_art::GILL_DESC,
            FishId::Plugin(_) => "A mysterious fish.",
        }
    }

    pub fn difficulty(&self) -> f32 {
        match self {
            FishId::Bubbles => 0.3,
            FishId::Marina => 0.6,
            FishId::Gill => 0.45,
            FishId::Plugin(_) => 0.5,
        }
    }

    pub fn color(&self) -> [f32; 4] {
        match self {
            FishId::Bubbles => crate::render::Colors::ORANGE,
            FishId::Marina => crate::render::Colors::LIGHT_BLUE,
            FishId::Gill => crate::render::Colors::GREEN,
            FishId::Plugin(_) => crate::render::Colors::WHITE,
        }
    }
}

/// Size of a caught fish.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FishSize {
    Small,
    Medium,
    Large,
}

impl FishSize {
    pub fn label(&self) -> &'static str {
        match self {
            FishSize::Small => "Small",
            FishSize::Medium => "Medium",
            FishSize::Large => "Large",
        }
    }
}

/// A fish the player has caught.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaughtFish {
    pub id: FishId,
    pub caught_at: String,
    pub size: FishSize,
}

/// Relationship level descriptions.
pub fn relationship_label(score: i32) -> &'static str {
    match score {
        ..=0 => "Stranger",
        1..=5 => "Acquaintance",
        6..=15 => "Friend",
        16..=25 => "Close Friend",
        26..=40 => "Romantic Interest",
        41.. => "Soulmate",
    }
}

/// The complete player state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub fish_collection: Vec<CaughtFish>,
    pub relationship_scores: HashMap<FishId, i32>,
    pub date_counts: HashMap<FishId, u32>,
    pub current_day: u32,
    pub dates_completed: u32,
    /// Locally tracked achievement unlocks.
    #[serde(default)]
    pub achievements: UnlockedAchievements,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            fish_collection: Vec::new(),
            relationship_scores: HashMap::new(),
            date_counts: HashMap::new(),
            current_day: 1,
            dates_completed: 0,
            achievements: UnlockedAchievements::default(),
        }
    }
}

impl PlayerState {
    pub fn has_caught(&self, fish_id: &FishId) -> bool {
        self.fish_collection.iter().any(|f| f.id == *fish_id)
    }

    pub fn catch_count(&self, fish_id: &FishId) -> usize {
        self.fish_collection.iter().filter(|f| f.id == *fish_id).count()
    }

    pub fn relationship(&self, fish_id: &FishId) -> i32 {
        self.relationship_scores.get(fish_id).copied().unwrap_or(0)
    }

    pub fn add_affection(&mut self, fish_id: FishId, amount: i32) {
        let score = self.relationship_scores.entry(fish_id).or_insert(0);
        *score = (*score + amount).max(0);
    }

    pub fn date_count(&self, fish_id: &FishId) -> u32 {
        self.date_counts.get(fish_id).copied().unwrap_or(0)
    }

    pub fn increment_date_count(&mut self, fish_id: FishId) {
        let count = self.date_counts.entry(fish_id).or_insert(0);
        *count += 1;
    }

    pub fn add_catch(&mut self, fish_id: FishId, pond_name: &str, size: FishSize) {
        self.fish_collection.push(CaughtFish {
            id: fish_id,
            caught_at: pond_name.to_string(),
            size,
        });
    }

    /// Check if the player has won (soulmate with any fish).
    pub fn has_won(&self) -> bool {
        self.relationship_scores.values().any(|&s| s >= 41)
    }

    /// Get the fish the player is closest to (if any).
    pub fn closest_fish(&self) -> Option<(FishId, i32)> {
        self.relationship_scores
            .iter()
            .max_by_key(|(_, score)| **score)
            .map(|(id, score)| (id.clone(), *score))
    }
}

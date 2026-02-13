//! Game data types and state management.

pub mod dialogues;
pub mod save;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Unique fish identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FishId {
    Bubbles,
    Marina,
    Gill,
}

impl FishId {
    pub const ALL: [FishId; 3] = [FishId::Bubbles, FishId::Marina, FishId::Gill];

    pub fn name(&self) -> &'static str {
        match self {
            FishId::Bubbles => "Bubbles",
            FishId::Marina => "Marina",
            FishId::Gill => "Gill",
        }
    }

    pub fn species(&self) -> &'static str {
        match self {
            FishId::Bubbles => "Clownfish",
            FishId::Marina => "Swordfish",
            FishId::Gill => "Pufferfish",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            FishId::Bubbles => crate::ascii_art::BUBBLES_DESC,
            FishId::Marina => crate::ascii_art::MARINA_DESC,
            FishId::Gill => crate::ascii_art::GILL_DESC,
        }
    }

    /// Difficulty of catching this fish (0.0 = easy, 1.0 = hard).
    pub fn difficulty(&self) -> f32 {
        match self {
            FishId::Bubbles => 0.3,
            FishId::Marina => 0.6,
            FishId::Gill => 0.45,
        }
    }

    /// Which pond index this fish appears in.
    pub fn pond_index(&self) -> usize {
        match self {
            FishId::Bubbles => 0,
            FishId::Marina => 1,
            FishId::Gill => 2,
        }
    }

    /// The fish's color for rendering.
    pub fn color(&self) -> [f32; 4] {
        match self {
            FishId::Bubbles => crate::render::Colors::ORANGE,
            FishId::Marina => crate::render::Colors::LIGHT_BLUE,
            FishId::Gill => crate::render::Colors::GREEN,
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
    pub current_day: u32,
    pub dates_completed: u32,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            fish_collection: Vec::new(),
            relationship_scores: HashMap::new(),
            current_day: 1,
            dates_completed: 0,
        }
    }
}

impl PlayerState {
    pub fn has_caught(&self, fish_id: FishId) -> bool {
        self.fish_collection.iter().any(|f| f.id == fish_id)
    }

    pub fn catch_count(&self, fish_id: FishId) -> usize {
        self.fish_collection.iter().filter(|f| f.id == fish_id).count()
    }

    pub fn relationship(&self, fish_id: FishId) -> i32 {
        self.relationship_scores.get(&fish_id).copied().unwrap_or(0)
    }

    pub fn add_affection(&mut self, fish_id: FishId, amount: i32) {
        let score = self.relationship_scores.entry(fish_id).or_insert(0);
        *score = (*score + amount).max(0);
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
            .map(|(id, score)| (*id, *score))
    }
}

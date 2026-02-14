//! Fish character rendering helpers.

use crate::ascii_art;
use crate::data::FishId;
use crate::plugins::FishRegistry;

/// Get the large ASCII art for a fish by ID and mood.
pub fn fish_art<'a>(id: &FishId, affection: i32, registry: &'a FishRegistry) -> String {
    match id {
        FishId::Bubbles => {
            if affection > 20 {
                ascii_art::BUBBLES_HAPPY.to_string()
            } else if affection > 10 {
                ascii_art::BUBBLES_ART.to_string()
            } else {
                ascii_art::BUBBLES_SHY.to_string()
            }
        }
        FishId::Marina => {
            if affection > 20 {
                ascii_art::MARINA_HAPPY.to_string()
            } else if affection > 10 {
                ascii_art::MARINA_ART.to_string()
            } else {
                ascii_art::MARINA_ANGRY.to_string()
            }
        }
        FishId::Gill => {
            if affection > 20 {
                ascii_art::GILL_ART.to_string()
            } else if affection > 10 {
                ascii_art::GILL_SHY.to_string()
            } else {
                ascii_art::GILL_PUFFED.to_string()
            }
        }
        FishId::Plugin(plugin_id) => {
            if let Some(fish) = registry.get(plugin_id) {
                fish.art_for_affection(affection).to_string()
            } else {
                "  ><(((o>".to_string()
            }
        }
    }
}

/// Get the date location art for a fish.
pub fn date_scene_art(id: &FishId, registry: &FishRegistry) -> String {
    match id {
        FishId::Bubbles => ascii_art::CORAL_CAFE.to_string(),
        FishId::Marina => ascii_art::MOONLIT_REEF.to_string(),
        FishId::Gill => ascii_art::SUNKEN_SHIP.to_string(),
        FishId::Plugin(plugin_id) => {
            registry.get(plugin_id)
                .map(|f| f.date_scene_art.clone())
                .unwrap_or_else(|| "  ~~~~~~~~\n  ~ ~ ~ ~ ~\n  ~~~~~~~~".to_string())
        }
    }
}

/// Get the date location name for a fish.
pub fn date_location(id: &FishId, registry: &FishRegistry) -> String {
    match id {
        FishId::Bubbles => "Coral Cafe".to_string(),
        FishId::Marina => "Moonlit Reef".to_string(),
        FishId::Gill => "Sunken Ship".to_string(),
        FishId::Plugin(plugin_id) => {
            registry.get(plugin_id)
                .map(|f| f.date_location.clone())
                .unwrap_or_else(|| "The Deep".to_string())
        }
    }
}

/// Get the small fish art for the fishing minigame.
pub fn fish_small_art(id: &FishId, registry: &FishRegistry) -> String {
    match id {
        FishId::Bubbles => ascii_art::BUBBLES_SMALL.to_string(),
        FishId::Marina => ascii_art::MARINA_SMALL.to_string(),
        FishId::Gill => ascii_art::GILL_SMALL.to_string(),
        FishId::Plugin(plugin_id) => {
            registry.get(plugin_id)
                .map(|f| f.art_small.clone())
                .unwrap_or_else(|| "><>".to_string())
        }
    }
}

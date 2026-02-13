//! Fish character rendering helpers.

use crate::ascii_art;
use crate::data::FishId;

/// Get the large ASCII art for a fish by ID and mood.
pub fn fish_art(id: FishId, affection: i32) -> &'static str {
    match id {
        FishId::Bubbles => {
            if affection > 20 {
                ascii_art::BUBBLES_HAPPY
            } else if affection > 10 {
                ascii_art::BUBBLES_ART
            } else {
                ascii_art::BUBBLES_SHY
            }
        }
        FishId::Marina => {
            if affection > 20 {
                ascii_art::MARINA_HAPPY
            } else if affection > 10 {
                ascii_art::MARINA_ART
            } else {
                ascii_art::MARINA_ANGRY
            }
        }
        FishId::Gill => {
            if affection > 20 {
                ascii_art::GILL_ART
            } else if affection > 10 {
                ascii_art::GILL_SHY
            } else {
                ascii_art::GILL_PUFFED
            }
        }
    }
}

/// Get the date location art for a fish.
pub fn date_scene_art(id: FishId) -> &'static str {
    match id {
        FishId::Bubbles => ascii_art::CORAL_CAFE,
        FishId::Marina => ascii_art::MOONLIT_REEF,
        FishId::Gill => ascii_art::SUNKEN_SHIP,
    }
}

/// Get the date location name for a fish.
pub fn date_location(id: FishId) -> &'static str {
    match id {
        FishId::Bubbles => "Coral Cafe",
        FishId::Marina => "Moonlit Reef",
        FishId::Gill => "Sunken Ship",
    }
}

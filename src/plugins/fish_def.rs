//! Fish definition data structure for plugin fish.
//!
//! A `FishDef` holds all the data that defines a dateable fish character,
//! whether built-in or loaded from a Rhai plugin script.

use sable_dialogue::prelude::*;
use sable_dialogue::dialogue::DialogueBuilder;
use sable_dialogue::node::Choice as DChoice;

/// Complete definition of a dateable fish character.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FishDef {
    /// Unique plugin ID (e.g., "coral_seahorse"). Used as the key in the registry.
    pub id: String,
    /// Display name (e.g., "Coral").
    pub name: String,
    /// Species (e.g., "Seahorse").
    pub species: String,
    /// Short description shown in menus.
    pub description: String,
    /// Difficulty of catching this fish (0.0 = easy, 1.0 = hard).
    pub difficulty: f32,
    /// RGBA color for rendering.
    pub color: [f32; 4],

    // ── ASCII art ──────────────────────────────────────────────────
    /// Art shown at high affection (> 20).
    pub art_happy: String,
    /// Art shown at medium affection (> 10).
    pub art_neutral: String,
    /// Art shown at low affection (<= 10).
    pub art_sad: String,
    /// Small inline art used during fishing minigame.
    pub art_small: String,

    // ── Date location ──────────────────────────────────────────────
    /// Name of the date location (e.g., "Kelp Garden").
    pub date_location: String,
    /// ASCII art for the date scene background.
    pub date_scene_art: String,
    /// Name of the fishing pond where this fish can be caught.
    pub pond_name: String,

    // ── Dialogues ──────────────────────────────────────────────────
    /// Dialogue trees for dates (rotated by date number).
    pub dialogues: Vec<DialogueTree>,
}

impl FishDef {
    /// Get the appropriate art based on affection level.
    pub fn art_for_affection(&self, affection: i32) -> &str {
        if affection > 20 {
            &self.art_happy
        } else if affection > 10 {
            &self.art_neutral
        } else {
            &self.art_sad
        }
    }

    /// Build a dialogue tree for a given date number.
    pub fn dialogue_for_date(&self, date_number: u32) -> DialogueTree {
        if self.dialogues.is_empty() {
            // Fallback: generate a minimal dialogue
            return Self::fallback_dialogue(&self.name);
        }
        let idx = (date_number as usize) % self.dialogues.len();
        self.dialogues[idx].clone()
    }

    /// Generate a simple fallback dialogue when no dialogues are defined.
    /// Public so other modules can use it for unknown plugin fish.
    pub fn fallback_dialogue_for(name: &str) -> DialogueTree {
        Self::fallback_dialogue(name)
    }

    fn fallback_dialogue(name: &str) -> DialogueTree {
        let speaker_id = name.to_lowercase();
        DialogueBuilder::new("start")
            .title(&format!("Date with {}", name))
            .speaker(Speaker::new(&speaker_id, name))
            .speaker(Speaker::new("player", "You"))
            .node(DialogueNode::Text {
                id: "start".into(),
                speaker: Some(speaker_id.clone()),
                emotion: None,
                text: format!(
                    "Hi there! I'm {}. Thanks for taking me out!",
                    name
                ),
                text_key: None,
                next_node: Some("q1".into()),
                actions: Vec::new(),
                voice_clip: None,
            })
            .node(DialogueNode::Choice {
                id: "q1".into(),
                prompt: Some(format!("{} smiles at you.", name)),
                speaker: None,
                choices: vec![
                    DChoice::new("This is nice!", "ending")
                        .sets("affection", 3_i32),
                    DChoice::new("Let's do this again sometime.", "ending")
                        .sets("affection", 2_i32),
                ],
            })
            .node(DialogueNode::Text {
                id: "ending".into(),
                speaker: Some(speaker_id),
                emotion: None,
                text: format!("I had a great time! See you around!"),
                text_key: None,
                next_node: Some("end".into()),
                actions: Vec::new(),
                voice_clip: None,
            })
            .node(DialogueNode::end("end"))
            .build_unchecked()
    }
}

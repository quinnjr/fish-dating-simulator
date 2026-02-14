//! Dialogue definition types for Rhai scripts.
//!
//! Provides a simplified dialogue builder that Rhai scripts can use to
//! construct dialogue trees without needing the full sable-dialogue API.

use rhai::{Map, Array};
use sable_dialogue::prelude::*;
use sable_dialogue::dialogue::DialogueBuilder;
use sable_dialogue::node::Choice as DChoice;

/// A simplified dialogue definition that can be constructed from Rhai.
/// Converted to a `DialogueTree` via `to_dialogue_tree()`.
#[derive(Debug, Clone)]
pub struct DialogueDef {
    pub title: String,
    pub speakers: Vec<(String, String)>,
    pub nodes: Vec<NodeDef>,
}

/// A simplified node definition.
#[derive(Debug, Clone)]
pub enum NodeDef {
    Text {
        id: String,
        speaker: String,
        text: String,
        next: String,
    },
    Choice {
        id: String,
        prompt: String,
        options: Vec<ChoiceOptionDef>,
    },
    End {
        id: String,
    },
}

/// A simplified choice option.
#[derive(Debug, Clone)]
pub struct ChoiceOptionDef {
    pub text: String,
    pub next: String,
    pub affection: i32,
}

impl DialogueDef {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            speakers: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn add_speaker(&mut self, id: &str, display_name: &str) {
        self.speakers.push((id.to_string(), display_name.to_string()));
    }

    pub fn add_text(&mut self, id: &str, speaker: &str, text: &str, next: &str) {
        self.nodes.push(NodeDef::Text {
            id: id.to_string(),
            speaker: speaker.to_string(),
            text: text.to_string(),
            next: next.to_string(),
        });
    }

    pub fn add_choice(&mut self, id: &str, prompt: &str, options: Vec<ChoiceOptionDef>) {
        self.nodes.push(NodeDef::Choice {
            id: id.to_string(),
            prompt: prompt.to_string(),
            options,
        });
    }

    pub fn add_end(&mut self, id: &str) {
        self.nodes.push(NodeDef::End {
            id: id.to_string(),
        });
    }

    /// Convert this definition into a sable-dialogue `DialogueTree`.
    pub fn to_dialogue_tree(&self) -> DialogueTree {
        let start_node = self.nodes.first().map(|n| match n {
            NodeDef::Text { id, .. } => id.as_str(),
            NodeDef::Choice { id, .. } => id.as_str(),
            NodeDef::End { id } => id.as_str(),
        }).unwrap_or("start");

        let mut builder = DialogueBuilder::new(start_node)
            .title(&self.title);

        for (id, display_name) in &self.speakers {
            builder = builder.speaker(Speaker::new(id, display_name));
        }

        for node in &self.nodes {
            match node {
                NodeDef::Text { id, speaker, text, next } => {
                    builder = builder.node(DialogueNode::Text {
                        id: id.clone(),
                        speaker: Some(speaker.clone()),
                        emotion: None,
                        text: text.clone(),
                        text_key: None,
                        next_node: Some(next.clone()),
                        actions: Vec::new(),
                        voice_clip: None,
                    });
                }
                NodeDef::Choice { id, prompt, options } => {
                    let choices: Vec<DChoice> = options.iter().map(|opt| {
                        let mut choice = DChoice::new(&opt.text, &opt.next);
                        if opt.affection != 0 {
                            choice = choice.sets("affection", opt.affection);
                        }
                        choice
                    }).collect();

                    builder = builder.node(DialogueNode::Choice {
                        id: id.clone(),
                        prompt: Some(prompt.clone()),
                        speaker: None,
                        choices,
                    });
                }
                NodeDef::End { id } => {
                    builder = builder.node(DialogueNode::end(id));
                }
            }
        }

        builder.build_unchecked()
    }
}

/// Parse an array of choice options from Rhai.
/// Each option can be a map with keys: text, next, affection
pub fn parse_choice_options(arr: &Array) -> Vec<ChoiceOptionDef> {
    arr.iter().filter_map(|item| {
        if let Some(map) = item.clone().try_cast::<Map>() {
            let text = map.get("text")?.clone().into_string().ok()?;
            let next = map.get("next")?.clone().into_string().ok()?;
            let affection = map.get("affection")
                .and_then(|v| v.as_int().ok())
                .unwrap_or(0) as i32;
            Some(ChoiceOptionDef { text, next, affection })
        } else {
            None
        }
    }).collect()
}

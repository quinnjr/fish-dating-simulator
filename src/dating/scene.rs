//! Date scene with dialogue integration.

use sable_dialogue::prelude::*;
use winit::keyboard::KeyCode;

use crate::data::dialogues;
use crate::data::FishId;
use crate::dating::fish;
use crate::game::GameScreen;
use crate::render::{Colors, GameRenderer};
use crate::ui;
use crate::ui::menu::SelectionMenu;

/// State for an active date scene.
pub struct DatingState {
    pub fish_id: FishId,
    runner: DialogueRunner,
    /// Current text being displayed.
    current_text: String,
    /// Current speaker name.
    current_speaker: String,
    /// Choices menu (if in choice mode).
    choice_menu: Option<SelectionMenu>,
    /// Accumulated affection gained during this date.
    affection_gained: i32,
    /// Whether the date has ended.
    ended: bool,
    /// Typewriter effect progress.
    typewriter_pos: usize,
    typewriter_timer: f32,
}

impl DatingState {
    pub fn new(fish_id: FishId, date_number: u32) -> Self {
        let tree = dialogues::build_dialogue(fish_id, date_number);
        let runner = DialogueRunner::new(tree);

        let mut state = Self {
            fish_id,
            runner,
            current_text: String::new(),
            current_speaker: String::new(),
            choice_menu: None,
            affection_gained: 0,
            ended: false,
            typewriter_pos: 0,
            typewriter_timer: 0.0,
        };
        state.sync_state();
        state
    }

    /// Synchronize rendering state from the dialogue runner.
    fn sync_state(&mut self) {
        // Drain events for affection tracking
        while let Some(event) = self.runner.poll_event() {
            if let DialogueEvent::VariableChanged { name, new_value, .. } = event {
                if name == "affection" {
                    if let Ok(val) = new_value.parse::<i32>() {
                        self.affection_gained += val;
                    }
                }
            }
        }

        match self.runner.current() {
            Some(DialogueState::Text {
                speaker, text, ..
            }) => {
                self.current_speaker = speaker
                    .map(|s| s.display_name().to_string())
                    .unwrap_or_default();
                self.current_text = text;
                self.choice_menu = None;
                self.typewriter_pos = 0;
                self.typewriter_timer = 0.0;
            }
            Some(DialogueState::Choices {
                prompt, choices, ..
            }) => {
                self.current_text = prompt.unwrap_or_default();
                self.current_speaker = String::new();
                let items: Vec<String> = choices
                    .iter()
                    .map(|c| c.text.clone())
                    .collect();
                self.choice_menu = Some(SelectionMenu::new(items));
                self.typewriter_pos = 0;
                self.typewriter_timer = 0.0;
            }
            Some(DialogueState::End) | None => {
                self.ended = true;
            }
            _ => {}
        }
    }

    /// Get the affection gained during this date.
    #[allow(dead_code)]
    pub fn affection_gained(&self) -> i32 {
        self.affection_gained
    }

    pub fn update(&mut self, dt: f32, key: Option<KeyCode>) -> Option<GameScreen> {
        // Typewriter effect
        self.typewriter_timer += dt;
        let chars_per_sec = 30.0;
        self.typewriter_pos = (self.typewriter_timer * chars_per_sec) as usize;

        if self.ended {
            if let Some(KeyCode::Enter | KeyCode::Space) = key {
                return Some(GameScreen::DateResult {
                    fish_id: self.fish_id,
                    affection: self.affection_gained,
                });
            }
            return None;
        }

        if let Some(k) = key {
            if let Some(ref mut menu) = self.choice_menu {
                match k {
                    KeyCode::ArrowUp | KeyCode::KeyW => menu.move_up(),
                    KeyCode::ArrowDown | KeyCode::KeyS => menu.move_down(),
                    KeyCode::Enter | KeyCode::Space => {
                        let idx = menu.selected_index();
                        let _ = self.runner.select_choice(idx);
                        self.sync_state();
                    }
                    _ => {}
                }
            } else {
                // Text node - advance on Enter/Space
                match k {
                    KeyCode::Enter | KeyCode::Space => {
                        // If typewriter not done, skip to end
                        if self.typewriter_pos < self.current_text.len() {
                            self.typewriter_pos = self.current_text.len();
                        } else {
                            let _ = self.runner.advance();
                            self.sync_state();
                        }
                    }
                    KeyCode::Escape => {
                        return Some(GameScreen::DateResult {
                            fish_id: self.fish_id,
                            affection: self.affection_gained,
                        });
                    }
                    _ => {}
                }
            }
        }

        None
    }

    pub fn render(&self, renderer: &mut GameRenderer, affection_total: i32, _time: f32) {
        let location = fish::date_location(self.fish_id);
        renderer.draw_centered(
            &format!("=== Date at {} ===", location),
            1.0,
            Colors::PINK,
        );

        // Scene art
        let scene_art = fish::date_scene_art(self.fish_id);
        renderer.draw_multiline_centered(scene_art, 3.0, Colors::LIGHT_BLUE);

        // Fish art on the left side
        let fish_art = fish::fish_art(self.fish_id, affection_total);
        renderer.draw_multiline_at_grid(fish_art, 3.0, 3.0, self.fish_id.color());

        // Hearts
        let cols = renderer.screen_cols() as usize;
        ui::draw_hearts(
            renderer,
            (cols / 2 - 8) as f32,
            12.0,
            affection_total + self.affection_gained,
            5,
        );

        if self.ended {
            renderer.draw_centered("Date over!", 14.0, Colors::YELLOW);
            renderer.draw_centered(
                &format!("Affection gained: +{}", self.affection_gained),
                15.0,
                Colors::PINK,
            );
            renderer.draw_centered("[Enter] Continue", 17.0, Colors::WHITE);
            return;
        }

        // Dialogue box
        let box_row = 14.0;
        let box_width = 56;
        let box_col = ((cols.saturating_sub(box_width)) / 2) as f32;

        ui::draw_box(renderer, box_col, box_row, box_width, 8, Colors::WHITE);

        // Speaker name
        if !self.current_speaker.is_empty() {
            renderer.draw_at_grid(
                &format!(" {} ", self.current_speaker),
                box_col + 2.0,
                box_row,
                self.fish_id.color(),
            );
        }

        if let Some(ref menu) = self.choice_menu {
            // Show prompt
            if !self.current_text.is_empty() {
                let display_text = truncate_to_width(&self.current_text, box_width - 4);
                renderer.draw_at_grid(&display_text, box_col + 2.0, box_row + 1.0, Colors::GRAY);
            }
            // Show choices
            menu.draw(renderer, box_col + 2.0, box_row + 3.0);
        } else {
            // Show text with typewriter effect
            let visible = &self.current_text[..self.current_text.len().min(self.typewriter_pos)];
            let wrapped = word_wrap(visible, box_width - 4);
            for (i, line) in wrapped.iter().enumerate() {
                renderer.draw_at_grid(line, box_col + 2.0, box_row + 1.0 + i as f32, Colors::WHITE);
            }

            // Show "press enter" prompt
            if self.typewriter_pos >= self.current_text.len() {
                renderer.draw_at_grid(
                    "[Enter]",
                    box_col + (box_width as f32) - 10.0,
                    box_row + 7.0,
                    Colors::DARK_GRAY,
                );
            }
        }
    }
}

/// Simple word wrapping.
fn word_wrap(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() > max_width {
            lines.push(current_line.clone());
            current_line = word.to_string();
        } else {
            current_line.push(' ');
            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

/// Truncate text to fit within a given width.
fn truncate_to_width(text: &str, max_width: usize) -> String {
    if text.len() <= max_width {
        text.to_string()
    } else {
        format!("{}...", &text[..max_width - 3])
    }
}

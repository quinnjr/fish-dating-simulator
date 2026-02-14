//! Fishing hook timing minigame.

use rand::Rng;
use winit::keyboard::KeyCode;

use crate::ascii_art;
use crate::data::{FishId, FishSize};
use crate::dating::fish as fish_helpers;
use crate::game::GameScreen;
use crate::plugins::FishRegistry;
use crate::render::{Colors, GameRenderer};
use crate::ui;

/// Width of the catch bar in characters.
const BAR_WIDTH: usize = 40;

/// Phases of the minigame.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Phase {
    Casting,
    Waiting,
    Reeling,
    Result,
}

pub struct MinigameState {
    fish_id: FishId,
    pond_index: usize,
    phase: Phase,
    /// Timer for phase transitions.
    timer: f32,
    /// Position of the cursor on the bar (0.0 to 1.0).
    cursor_pos: f32,
    /// Direction of cursor movement.
    cursor_dir: f32,
    /// Speed of the cursor.
    cursor_speed: f32,
    /// Start of the catch zone (0.0 to 1.0).
    zone_start: f32,
    /// Width of the catch zone (0.0 to 1.0).
    zone_width: f32,
    /// Whether the catch was successful.
    caught: bool,
    /// Fish size if caught.
    fish_size: FishSize,
    /// Wait duration before fish bites.
    wait_duration: f32,
}

impl MinigameState {
    pub fn new(fish_id: FishId, pond_index: usize) -> Self {
        let mut rng = rand::thread_rng();
        let difficulty = fish_id.difficulty(); // Uses legacy method

        // Zone width inversely proportional to difficulty
        let zone_width = 0.25 - difficulty * 0.15; // 0.10 to 0.25
        let zone_start = rng.r#gen::<f32>() * (1.0 - zone_width);

        // Cursor speed proportional to difficulty
        let cursor_speed = 0.5 + difficulty * 1.0;

        Self {
            fish_id,
            pond_index,
            phase: Phase::Casting,
            timer: 0.0,
            cursor_pos: 0.0,
            cursor_dir: 1.0,
            cursor_speed,
            zone_start,
            zone_width,
            caught: false,
            fish_size: FishSize::Medium,
            wait_duration: rng.r#gen::<f32>() * 2.0 + 1.0,
        }
    }

    pub fn update(&mut self, dt: f32, key: Option<KeyCode>) -> Option<GameScreen> {
        self.timer += dt;

        match self.phase {
            Phase::Casting => {
                if self.timer > 1.5 {
                    self.phase = Phase::Waiting;
                    self.timer = 0.0;
                }
            }
            Phase::Waiting => {
                if self.timer > self.wait_duration {
                    self.phase = Phase::Reeling;
                    self.timer = 0.0;
                }
            }
            Phase::Reeling => {
                // Move cursor back and forth
                self.cursor_pos += self.cursor_dir * self.cursor_speed * dt;
                if self.cursor_pos >= 1.0 {
                    self.cursor_pos = 1.0;
                    self.cursor_dir = -1.0;
                } else if self.cursor_pos <= 0.0 {
                    self.cursor_pos = 0.0;
                    self.cursor_dir = 1.0;
                }

                // Check for space press
                if key == Some(KeyCode::Space) || key == Some(KeyCode::Enter) {
                    let in_zone = self.cursor_pos >= self.zone_start
                        && self.cursor_pos <= self.zone_start + self.zone_width;
                    self.caught = in_zone;

                    if in_zone {
                        // Determine fish size based on accuracy
                        let zone_center = self.zone_start + self.zone_width / 2.0;
                        let accuracy = 1.0 - ((self.cursor_pos - zone_center).abs() / (self.zone_width / 2.0));
                        self.fish_size = if accuracy > 0.8 {
                            FishSize::Large
                        } else if accuracy > 0.4 {
                            FishSize::Medium
                        } else {
                            FishSize::Small
                        };
                    }

                    self.phase = Phase::Result;
                    self.timer = 0.0;
                }

                // Timeout after 5 seconds
                if self.timer > 5.0 {
                    self.caught = false;
                    self.phase = Phase::Result;
                    self.timer = 0.0;
                }
            }
            Phase::Result => {
                if let Some(k) = key {
                    match k {
                        KeyCode::Enter | KeyCode::Space => {
                            if self.caught {
                                return Some(GameScreen::CatchResult {
                                    fish_id: self.fish_id.clone(),
                                    pond_index: self.pond_index,
                                    size: self.fish_size,
                                });
                            } else {
                                // Try again or go back
                                return Some(GameScreen::FishingPondSelect);
                            }
                        }
                        KeyCode::Escape => {
                            return Some(GameScreen::FishingPondSelect);
                        }
                        _ => {}
                    }
                }
            }
        }

        None
    }

    pub fn render(&self, renderer: &mut GameRenderer, time: f32, registry: &FishRegistry) {
        let fish_name = self.fish_id.name_with_registry(registry);
        let pond_name = if self.pond_index < ascii_art::POND_NAMES.len() {
            ascii_art::POND_NAMES[self.pond_index].to_string()
        } else {
            // Plugin fish pond
            registry.pond_names()
                .get(self.pond_index - ascii_art::POND_NAMES.len())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Unknown Pond".to_string())
        };

        renderer.draw_centered(
            &format!("=== Fishing at {} ===", pond_name),
            1.0,
            Colors::CYAN,
        );

        match self.phase {
            Phase::Casting => {
                renderer.draw_multiline_centered(ascii_art::CASTING_ART, 4.0, Colors::WHITE);
                renderer.draw_centered("Casting...", 14.0, Colors::YELLOW);
            }
            Phase::Waiting => {
                renderer.draw_multiline_centered(ascii_art::CASTING_ART, 4.0, Colors::WHITE);

                // Animated dots
                let dots = ".".repeat(((time * 3.0) as usize % 4) + 1);
                renderer.draw_centered(
                    &format!("Waiting for a bite{}", dots),
                    14.0,
                    Colors::GRAY,
                );
            }
            Phase::Reeling => {
                renderer.draw_multiline_centered(ascii_art::FISH_ON_LINE, 4.0, Colors::YELLOW);

                renderer.draw_centered("! FISH ON THE LINE !", 13.0, Colors::RED);

                // Draw the catch bar
                self.draw_catch_bar(renderer, 15.0);

                // Fish swimming
                let small_art = fish_helpers::fish_small_art(&self.fish_id, registry);
                let fish_col = 2.0 + self.cursor_pos * (BAR_WIDTH as f32 - 8.0);
                let cols = renderer.screen_cols() as usize;
                let bar_start = (cols.saturating_sub(BAR_WIDTH)) / 2;
                renderer.draw_at_grid(
                    &small_art,
                    bar_start as f32 + fish_col,
                    17.0,
                    self.fish_id.color(),
                );

                renderer.draw_centered("[SPACE] Hook the fish!", 20.0, Colors::GREEN);
            }
            Phase::Result => {
                if self.caught {
                    renderer.draw_multiline_centered(ascii_art::CATCH_SUCCESS, 4.0, Colors::GREEN);
                    renderer.draw_centered(
                        &format!("You caught {} ({})!", fish_name, self.fish_size.label()),
                        12.0,
                        Colors::YELLOW,
                    );
                    renderer.draw_centered("[Enter] Continue", 14.0, Colors::WHITE);
                } else {
                    renderer.draw_multiline_centered(ascii_art::CATCH_FAIL, 4.0, Colors::RED);
                    renderer.draw_centered("The fish got away...", 12.0, Colors::GRAY);
                    renderer.draw_centered("[Enter] Try Again  [Esc] Back", 14.0, Colors::WHITE);
                }
            }
        }
    }

    fn draw_catch_bar(&self, renderer: &mut GameRenderer, row: f32) {
        let cols = renderer.screen_cols() as usize;
        let bar_start = (cols.saturating_sub(BAR_WIDTH)) / 2;

        // Build the bar string character by character
        let mut bar = String::with_capacity(BAR_WIDTH);
        bar.push('[');

        let inner = BAR_WIDTH - 2;
        let zone_start_idx = (self.zone_start * inner as f32) as usize;
        let zone_end_idx = ((self.zone_start + self.zone_width) * inner as f32) as usize;
        let cursor_idx = (self.cursor_pos * (inner - 1) as f32) as usize;

        for i in 0..inner {
            if i == cursor_idx {
                bar.push('|');
            } else if i >= zone_start_idx && i <= zone_end_idx {
                bar.push('#');
            } else {
                bar.push('-');
            }
        }
        bar.push(']');

        // Draw the full bar at the base color
        renderer.draw_at_grid(&bar, bar_start as f32, row, Colors::DARK_GRAY);

        // Overdraw the zone in green
        let zone_str: String = (0..inner)
            .map(|i| {
                if i >= zone_start_idx && i <= zone_end_idx {
                    '#'
                } else {
                    ' '
                }
            })
            .collect();
        renderer.draw_at_grid(
            &format!(" {}", zone_str),
            bar_start as f32,
            row,
            Colors::GREEN,
        );

        // Overdraw the cursor in yellow
        let cursor_str: String = (0..inner)
            .map(|i| if i == cursor_idx { '|' } else { ' ' })
            .collect();
        renderer.draw_at_grid(
            &format!(" {}", cursor_str),
            bar_start as f32,
            row,
            Colors::YELLOW,
        );

        // Labels
        ui::draw_progress_bar(renderer, bar_start as f32, row + 1.0, BAR_WIDTH, self.cursor_pos, Colors::CYAN, Colors::DARK_GRAY);
    }
}

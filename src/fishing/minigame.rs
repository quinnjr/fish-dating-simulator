//! Tug-of-war fishing minigame.
//!
//! The player must keep a line indicator centered on a meter by pressing
//! A (left) and D (right) while the fish fights back with random tugs.
//! Staying centered fills a reel-in progress bar. Drifting too far to
//! the edges risks the line snapping.

use rand::Rng;
use winit::keyboard::KeyCode;

use crate::ascii_art;
use crate::data::{FishId, FishSize};
use crate::dating::fish as fish_helpers;
use crate::game::GameScreen;
use crate::plugins::FishRegistry;
use crate::render::{Colors, GameRenderer};

/// Width of the tug-of-war meter in characters.
const METER_WIDTH: usize = 50;

/// How far from center (0.0–1.0) the line can drift before it snaps.
const SNAP_THRESHOLD: f32 = 1.0;

/// Center zone half-width — staying within this zone reels in the fish.
const CENTER_ZONE: f32 = 0.2;

/// How much reel progress is needed to land the fish (seconds in zone).
const REEL_TARGET: f32 = 5.0;

/// How much reel progress drains per second when outside the center zone.
const REEL_DRAIN_RATE: f32 = 0.3;

/// Player input force per key press.
const PLAYER_FORCE: f32 = 1.8;

/// Damping applied to line velocity each frame (friction).
const VELOCITY_DAMPING: f32 = 3.0;

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

    // ── Tug-of-war state (active during Reeling) ──

    /// Line position: 0.0 = center, -1.0 = full left, 1.0 = full right.
    line_pos: f32,
    /// Line velocity (used for momentum / smoothing).
    line_vel: f32,
    /// Reel-in progress (0.0 to REEL_TARGET).
    reel_progress: f32,
    /// Whether the catch was successful.
    caught: bool,
    /// Fish size if caught (determined by accuracy).
    fish_size: FishSize,
    /// Wait duration before fish bites.
    wait_duration: f32,

    // ── Fish AI ──

    /// Base aggressiveness (0.0–1.0). Higher = stronger tugs.
    fish_aggression: f32,
    /// Current fish force direction (-1 or 1).
    fish_dir: f32,
    /// Current fish force strength.
    fish_force: f32,
    /// Timer until next fish behavior change.
    fish_change_timer: f32,
    /// How erratic the fish is (shorter change intervals).
    fish_erratic: f32,
    /// Per-frame tension animation offset.
    tension_shake: f32,

    // ── Input tracking ──

    /// Whether A is currently held.
    holding_left: bool,
    /// Whether D is currently held.
    holding_right: bool,
}

impl MinigameState {
    pub fn new(fish_id: FishId, pond_index: usize) -> Self {
        let mut rng = rand::thread_rng();
        let difficulty = fish_id.difficulty();

        // Fish personality derived from difficulty
        let fish_aggression = 0.3 + difficulty * 0.7; // 0.3 to 1.0
        let fish_erratic = 0.3 + difficulty * 0.5;

        Self {
            fish_id,
            pond_index,
            phase: Phase::Casting,
            timer: 0.0,
            line_pos: 0.0,
            line_vel: 0.0,
            reel_progress: 0.0,
            caught: false,
            fish_size: FishSize::Medium,
            wait_duration: rng.r#gen::<f32>() * 2.0 + 1.0,
            fish_aggression,
            fish_dir: if rng.r#gen::<bool>() { 1.0 } else { -1.0 },
            fish_force: fish_aggression * 0.5,
            fish_change_timer: rng.r#gen::<f32>() * 0.5 + 0.3,
            fish_erratic,
            tension_shake: 0.0,
            holding_left: false,
            holding_right: false,
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
                self.update_reeling(dt, key);
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

        // Escape always exits during active phases
        if self.phase != Phase::Result {
            if key == Some(KeyCode::Escape) {
                return Some(GameScreen::FishingPondSelect);
            }
        }

        None
    }

    fn update_reeling(&mut self, dt: f32, key: Option<KeyCode>) {
        let mut rng = rand::thread_rng();

        // ── Process input ──
        if let Some(k) = key {
            match k {
                KeyCode::KeyA | KeyCode::ArrowLeft => self.holding_left = true,
                KeyCode::KeyD | KeyCode::ArrowRight => self.holding_right = true,
                _ => {}
            }
        }

        // Calculate player force from held keys
        let mut player_impulse: f32 = 0.0;
        if self.holding_left {
            player_impulse -= PLAYER_FORCE;
        }
        if self.holding_right {
            player_impulse += PLAYER_FORCE;
        }
        // Reset holds each frame (keys are press-only, not held in this engine)
        self.holding_left = false;
        self.holding_right = false;

        // ── Update fish AI ──
        self.fish_change_timer -= dt;
        if self.fish_change_timer <= 0.0 {
            // Fish changes behavior
            let base_interval = 0.8 - self.fish_erratic * 0.5; // 0.3 to 0.65s
            self.fish_change_timer = rng.r#gen::<f32>() * base_interval + 0.15;

            // Randomize direction and strength
            let surge_chance: f32 = rng.r#gen();
            if surge_chance < 0.2 {
                // Big surge — sudden strong pull
                self.fish_dir = if rng.r#gen::<bool>() { 1.0 } else { -1.0 };
                self.fish_force = self.fish_aggression * (1.2 + rng.r#gen::<f32>() * 0.8);
                self.tension_shake = 0.5;
            } else if surge_chance < 0.5 {
                // Direction swap with moderate force
                self.fish_dir = -self.fish_dir;
                self.fish_force = self.fish_aggression * (0.4 + rng.r#gen::<f32>() * 0.5);
            } else {
                // Gentle adjustment
                self.fish_force = self.fish_aggression * (0.2 + rng.r#gen::<f32>() * 0.4);
                // Slight random drift
                self.fish_dir += (rng.r#gen::<f32>() - 0.5) * 0.4;
                self.fish_dir = self.fish_dir.clamp(-1.0, 1.0);
            }

            // Fish tends to pull away from center (self-preservation)
            if self.line_pos.abs() < 0.15 {
                self.fish_dir = if rng.r#gen::<bool>() { 1.0 } else { -1.0 };
                self.fish_force *= 1.3;
            }
        }

        // ── Apply forces ──
        let fish_accel = self.fish_dir * self.fish_force;
        self.line_vel += (fish_accel + player_impulse) * dt;

        // Damping
        self.line_vel -= self.line_vel * VELOCITY_DAMPING * dt;

        // Integrate position
        self.line_pos += self.line_vel * dt;

        // Tension shake decay
        self.tension_shake *= (1.0 - 4.0 * dt).max(0.0);

        // ── Reel progress ──
        let dist_from_center = self.line_pos.abs();
        if dist_from_center < CENTER_ZONE {
            // In the sweet spot — reel in!
            let efficiency = 1.0 - (dist_from_center / CENTER_ZONE);
            self.reel_progress += efficiency * dt;
        } else {
            // Outside center — progress drains slowly
            self.reel_progress = (self.reel_progress - REEL_DRAIN_RATE * dt).max(0.0);
        }

        // ── Win/lose conditions ──
        if self.reel_progress >= REEL_TARGET {
            // Fish caught! Determine size by how centered the player stayed.
            let avg_accuracy = self.reel_progress / self.timer.max(0.1);
            self.fish_size = if avg_accuracy > 0.85 {
                FishSize::Large
            } else if avg_accuracy > 0.5 {
                FishSize::Medium
            } else {
                FishSize::Small
            };
            self.caught = true;
            self.phase = Phase::Result;
            self.timer = 0.0;
            return;
        }

        if dist_from_center >= SNAP_THRESHOLD {
            // Line snapped!
            self.caught = false;
            self.phase = Phase::Result;
            self.timer = 0.0;
            return;
        }

        // Timeout safety (30 seconds max)
        if self.timer > 30.0 {
            self.caught = false;
            self.phase = Phase::Result;
            self.timer = 0.0;
        }
    }

    pub fn render(&self, renderer: &mut GameRenderer, time: f32, registry: &FishRegistry) {
        let fish_name = self.fish_id.name_with_registry(registry);
        let pond_name = if self.pond_index < ascii_art::POND_NAMES.len() {
            ascii_art::POND_NAMES[self.pond_index].to_string()
        } else {
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
                let dots = ".".repeat(((time * 3.0) as usize % 4) + 1);
                renderer.draw_centered(
                    &format!("Waiting for a bite{}", dots),
                    14.0,
                    Colors::GRAY,
                );
            }
            Phase::Reeling => {
                self.render_reeling(renderer, time, &fish_name, registry);
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
                    let msg = if self.line_pos.abs() >= SNAP_THRESHOLD {
                        "The line snapped!"
                    } else {
                        "The fish got away..."
                    };
                    renderer.draw_centered(msg, 12.0, Colors::GRAY);
                    renderer.draw_centered("[Enter] Try Again  [Esc] Back", 14.0, Colors::WHITE);
                }
            }
        }
    }

    fn render_reeling(
        &self,
        renderer: &mut GameRenderer,
        time: f32,
        fish_name: &str,
        registry: &FishRegistry,
    ) {
        let cols = renderer.screen_cols();

        // ── Header ──
        renderer.draw_multiline_centered(ascii_art::FISH_ON_LINE, 3.0, Colors::YELLOW);

        let alert_flash = (time * 6.0).sin() * 0.3 + 0.7;
        renderer.draw_centered(
            "! FISH ON THE LINE !",
            11.0,
            [1.0, 0.3, 0.3, alert_flash],
        );

        // ── Tug-of-war meter ──
        let meter_row = 13.0;
        self.draw_tug_meter(renderer, meter_row, time);

        // ── Tension indicator ──
        let tension = self.line_pos.abs() / SNAP_THRESHOLD;
        let tension_label = if tension > 0.8 {
            "!!! EXTREME TENSION !!!"
        } else if tension > 0.6 {
            "!! HIGH TENSION !!"
        } else if tension > 0.35 {
            "~ Moderate tension ~"
        } else {
            "Line is steady"
        };
        let tension_color = if tension > 0.8 {
            [1.0, 0.1, 0.1, (time * 8.0).sin().abs()]
        } else if tension > 0.6 {
            Colors::ORANGE
        } else if tension > 0.35 {
            Colors::YELLOW
        } else {
            Colors::GREEN
        };
        renderer.draw_centered(tension_label, meter_row + 2.0, tension_color);

        // ── Reel progress bar ──
        let progress_row = meter_row + 4.0;
        let progress = (self.reel_progress / REEL_TARGET).clamp(0.0, 1.0);
        renderer.draw_centered("REEL PROGRESS", progress_row, Colors::WHITE);
        let bar_width = 40_usize;
        let bar_col = (cols as usize).saturating_sub(bar_width) / 2;
        crate::ui::draw_progress_bar(
            renderer,
            bar_col as f32,
            progress_row + 1.0,
            bar_width,
            progress,
            Colors::CYAN,
            Colors::DARK_GRAY,
        );
        let pct_str = format!("{}%", (progress * 100.0) as u32);
        renderer.draw_centered(&pct_str, progress_row + 2.0, Colors::CYAN);

        // ── Animated fish ──
        let fish_row = progress_row + 4.0;
        let small_art = fish_helpers::fish_small_art(&self.fish_id, registry);

        // Fish visual position tracks the line position + wiggle
        let wiggle = (time * 4.0).sin() * 0.5;
        let fish_visual_x = cols / 2.0 + self.line_pos * (METER_WIDTH as f32 / 2.0 - 4.0) + wiggle;
        renderer.draw_at_grid(
            &small_art,
            fish_visual_x - 2.0,
            fish_row,
            self.fish_id.color(),
        );

        // Animated water below fish
        let wave = if ((time * 3.0) as i32) % 2 == 0 {
            "~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~"
        } else {
            " ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~"
        };
        renderer.draw_centered(wave, fish_row + 1.0, [0.2, 0.3, 0.7, 0.4]);

        // ── Controls hint ──
        renderer.draw_centered(
            &format!("Reel in {}!", fish_name),
            fish_row + 3.0,
            Colors::WHITE,
        );
        renderer.draw_centered(
            "[A/Left] Pull left  [D/Right] Pull right  [Esc] Cut line",
            fish_row + 4.0,
            Colors::DARK_GRAY,
        );
    }

    /// Draw the centered tug-of-war meter.
    fn draw_tug_meter(&self, renderer: &mut GameRenderer, row: f32, time: f32) {
        let cols = renderer.screen_cols() as usize;
        let bar_start = cols.saturating_sub(METER_WIDTH) / 2;
        let inner = METER_WIDTH - 2;
        let half = inner / 2;

        // Center zone boundaries (in bar-character indices)
        let zone_chars = (CENTER_ZONE * half as f32) as usize;
        let zone_left = half - zone_chars;
        let zone_right = half + zone_chars;

        // Line indicator position (mapped from -1..1 to 0..inner-1)
        let shake = if self.tension_shake > 0.05 {
            (time * 40.0).sin() * self.tension_shake * 2.0
        } else {
            0.0
        };
        let mapped = ((self.line_pos + shake * 0.02) * 0.5 + 0.5).clamp(0.0, 1.0);
        let cursor_idx = (mapped * (inner - 1) as f32) as usize;

        // ── Draw danger zone markers ──
        let danger_left = "<<< SNAP";
        let danger_right = "SNAP >>>";
        let tension = self.line_pos.abs() / SNAP_THRESHOLD;
        let danger_alpha = if tension > 0.6 {
            (time * 6.0).sin().abs()
        } else {
            0.3
        };
        renderer.draw_at_grid(
            danger_left,
            (bar_start as f32) - danger_left.len() as f32 - 1.0,
            row,
            [1.0, 0.2, 0.2, danger_alpha],
        );
        renderer.draw_at_grid(
            danger_right,
            (bar_start + METER_WIDTH) as f32 + 1.0,
            row,
            [1.0, 0.2, 0.2, danger_alpha],
        );

        // ── Build base bar ──
        let mut bar = String::with_capacity(METER_WIDTH);
        bar.push('[');
        for i in 0..inner {
            if i == cursor_idx {
                bar.push('|');
            } else if i == half {
                bar.push('+');
            } else if i >= zone_left && i <= zone_right {
                bar.push('=');
            } else {
                bar.push('-');
            }
        }
        bar.push(']');
        renderer.draw_at_grid(&bar, bar_start as f32, row, Colors::DARK_GRAY);

        // ── Overdraw center zone in green ──
        let zone_overlay: String = (0..inner)
            .map(|i| {
                if i >= zone_left && i <= zone_right {
                    if i == half { '+' } else { '=' }
                } else {
                    ' '
                }
            })
            .collect();
        renderer.draw_at_grid(
            &format!(" {}", zone_overlay),
            bar_start as f32,
            row,
            Colors::GREEN,
        );

        // ── Overdraw cursor ──
        let in_zone = cursor_idx >= zone_left && cursor_idx <= zone_right;
        let cursor_color = if in_zone {
            Colors::CYAN
        } else if tension > 0.7 {
            [1.0, 0.2, 0.2, 1.0]
        } else {
            Colors::YELLOW
        };
        let cursor_overlay: String = (0..inner)
            .map(|i| if i == cursor_idx { '|' } else { ' ' })
            .collect();
        renderer.draw_at_grid(
            &format!(" {}", cursor_overlay),
            bar_start as f32,
            row,
            cursor_color,
        );

        // ── Center marker label ──
        renderer.draw_centered("v CENTER v", row - 1.0, [0.5, 0.8, 0.5, 0.6]);
    }
}

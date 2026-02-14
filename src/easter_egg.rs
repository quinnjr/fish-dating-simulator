//! Easter egg: cult_papa captures the moon and fights it with swords.
//!
//! Triggered by typing "moon" on the main menu. Plays a multi-phase
//! cinematic ASCII animation of cult_papa lassoing the moon out of the
//! sky and engaging it in an epic sword duel.

use winit::keyboard::KeyCode;

use crate::ascii_art;
use crate::game::GameScreen;
use crate::render::{Colors, GameRenderer};

/// Size of cult_papa's face in grid cells (matches the 4-line ASCII head).
const FACE_SIZE: f32 = 4.0;

/// Phases of the moon battle sequence.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Phase {
    /// Night sky with the moon. cult_papa gazes up.
    Stargazing,
    /// cult_papa throws a lasso at the moon.
    Lasso,
    /// The moon is captured and dragged down.
    Capture,
    /// The moon falls and lands.
    MoonFalls,
    /// Both draw swords.
    DrawSwords,
    /// Clash sequence (cycles through 3 clash frames).
    Clash(u8),
    /// cult_papa wins.
    Victory,
}

/// Secret key sequence detector for "moon".
pub struct SecretSequence {
    /// The keys we're looking for (M, O, O, N).
    target: &'static [KeyCode],
    /// How far the player has progressed.
    progress: usize,
}

impl SecretSequence {
    pub fn new() -> Self {
        Self {
            target: &[KeyCode::KeyM, KeyCode::KeyO, KeyCode::KeyO, KeyCode::KeyN],
            progress: 0,
        }
    }

    /// Feed a key press. Returns `true` when the full sequence is matched.
    pub fn feed(&mut self, key: KeyCode) -> bool {
        if key == self.target[self.progress] {
            self.progress += 1;
            if self.progress >= self.target.len() {
                self.progress = 0;
                return true;
            }
        } else {
            // Reset, but check if this key starts a new match
            self.progress = if key == self.target[0] { 1 } else { 0 };
        }
        false
    }

    /// Reset the detector.
    pub fn reset(&mut self) {
        self.progress = 0;
    }
}

/// State for the cult_papa vs Moon easter egg.
pub struct MoonBattleState {
    phase: Phase,
    phase_timer: f32,
    total_time: f32,
    /// Number of clash cycles completed.
    clash_cycles: u8,
    /// Whether the player has dismissed the scene.
    _skip_requested: bool,
    /// Shake offset for impact frames.
    shake: f32,
    /// Set to true the frame victory is first reached.
    victory_just_reached: bool,
}

impl MoonBattleState {
    pub fn new() -> Self {
        Self {
            phase: Phase::Stargazing,
            phase_timer: 0.0,
            total_time: 0.0,
            clash_cycles: 0,
            _skip_requested: false,
            shake: 0.0,
            victory_just_reached: false,
        }
    }

    /// Returns true once when the victory phase is first entered.
    /// Subsequent calls return false.
    pub fn take_victory_flag(&mut self) -> bool {
        if self.victory_just_reached {
            self.victory_just_reached = false;
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, dt: f32, key: Option<KeyCode>) -> Option<GameScreen> {
        self.total_time += dt;
        self.phase_timer += dt;
        self.shake *= 0.9; // Decay shake

        // Escape exits at any time
        if let Some(KeyCode::Escape) = key {
            return Some(GameScreen::MainMenu);
        }

        // Any key (Enter, Space, arrow keys) advances to the next phase
        if let Some(k) = key {
            match k {
                KeyCode::Enter | KeyCode::Space | KeyCode::ArrowRight | KeyCode::ArrowDown => {
                    if self.phase == Phase::Victory {
                        return Some(GameScreen::MainMenu);
                    }
                    self.advance_phase();
                }
                _ => {}
            }
        }

        None
    }

    fn advance_phase(&mut self) {
        self.phase_timer = 0.0;
        self.phase = match self.phase {
            Phase::Stargazing => Phase::Lasso,
            Phase::Lasso => Phase::Capture,
            Phase::Capture => {
                self.shake = 1.0;
                Phase::MoonFalls
            }
            Phase::MoonFalls => Phase::DrawSwords,
            Phase::DrawSwords => {
                self.shake = 1.0;
                Phase::Clash(0)
            }
            Phase::Clash(n) => {
                self.shake = 1.0;
                self.clash_cycles += 1;
                if self.clash_cycles >= 4 {
                    self.victory_just_reached = true;
                    Phase::Victory
                } else {
                    Phase::Clash((n + 1) % 3)
                }
            }
            Phase::Victory => Phase::Victory,
        };
    }

    pub fn render(&self, renderer: &mut GameRenderer, time: f32) {
        // Screen shake offset
        let shake_x = if self.shake > 0.05 {
            (time * 50.0).sin() * self.shake * 0.5
        } else {
            0.0
        };
        let shake_y = if self.shake > 0.05 {
            (time * 37.0).cos() * self.shake * 0.3
        } else {
            0.0
        };

        match self.phase {
            Phase::Stargazing => self.render_stargazing(renderer, time, shake_x, shake_y),
            Phase::Lasso => self.render_lasso(renderer, time, shake_x, shake_y),
            Phase::Capture => self.render_capture(renderer, time, shake_x, shake_y),
            Phase::MoonFalls => self.render_moon_falls(renderer, time, shake_x, shake_y),
            Phase::DrawSwords => self.render_draw_swords(renderer, time, shake_x, shake_y),
            Phase::Clash(frame) => self.render_clash(renderer, time, frame, shake_x, shake_y),
            Phase::Victory => self.render_victory(renderer, time, shake_x, shake_y),
        }

        // Phase indicator
        if self.phase != Phase::Victory {
            renderer.draw_centered(
                "[Enter] Next  [Esc] Skip",
                28.0,
                Colors::DARK_GRAY,
            );
        }
    }

    fn render_stargazing(&self, renderer: &mut GameRenderer, time: f32, sx: f32, sy: f32) {
        // Twinkling night sky (stars only — moon drawn separately with glow)
        let twinkle = (time * 2.0).sin() * 0.3 + 0.7;
        let star_color = [0.8, 0.8, 1.0, twinkle];
        renderer.draw_multiline_centered(ascii_art::STARS_ONLY, 1.0 + sy, star_color);

        // Moon with animated glow — single multiline block for alignment
        let glow = (time * 1.5).sin() * 0.1 + 0.9;
        let moon_color = [1.0, 1.0, 0.8, glow];
        renderer.draw_multiline_centered(ascii_art::MOON_FACE, 2.0 + sy, moon_color);

        // cult_papa standing below, looking up
        renderer.draw_multiline_centered(ascii_art::CULT_PAPA_STANDING, 14.0 + sy, Colors::WHITE);

        // Overlay cult_papa face on head (lines 1-4, centered)
        let cols = renderer.screen_cols();
        renderer.draw_cult_papa_face(
            cols / 2.0 - FACE_SIZE / 2.0,
            16.5 - FACE_SIZE / 2.0 + sy,
            FACE_SIZE,
            Colors::WHITE,
        );

        // Dramatic text
        renderer.draw_centered(
            "cult_papa gazes at the moon...",
            24.0,
            [0.7, 0.7, 0.9, 1.0],
        );
    }

    fn render_lasso(&self, renderer: &mut GameRenderer, time: f32, sx: f32, sy: f32) {
        // Stars dim
        let star_color = [0.5, 0.5, 0.7, 0.5];
        renderer.draw_multiline_centered(ascii_art::STARS_ONLY, 1.0 + sy, star_color);

        // Lasso scene — cult_papa throwing
        renderer.draw_multiline_centered(
            ascii_art::CULT_PAPA_LASSO,
            12.0 + sy,
            Colors::WHITE,
        );

        // Overlay cult_papa face on head (shifted left of center for lasso pose)
        let cols = renderer.screen_cols();
        renderer.draw_cult_papa_face(
            cols / 2.0 - 7.5 - FACE_SIZE / 2.0,
            14.5 - FACE_SIZE / 2.0 + sy,
            FACE_SIZE,
            Colors::WHITE,
        );

        // Lasso rope animation
        let lasso_progress = (self.phase_timer * 0.4).min(1.0);
        let rope_col = cols / 2.0 + 6.0 + sx;
        for i in 0..(lasso_progress * 8.0) as usize {
            let wobble = (time * 5.0 + i as f32).sin() * 0.3;
            renderer.draw_at_grid(
                "|",
                rope_col + wobble,
                12.0 - i as f32 + sy,
                Colors::YELLOW,
            );
        }

        // Moon at top — single aligned block
        let moon_color = if lasso_progress > 0.7 {
            [1.0, 1.0, 0.5, 1.0]
        } else {
            [1.0, 1.0, 0.8, 0.9]
        };
        renderer.draw_multiline_centered(ascii_art::MOON_FACE, 2.0 + sy, moon_color);

        renderer.draw_centered(
            "\"Get over here!\"",
            24.0,
            [1.0, 0.8, 0.2, 1.0],
        );
    }

    fn render_capture(&self, renderer: &mut GameRenderer, time: f32, sx: f32, sy: f32) {
        let star_color = [0.4, 0.4, 0.6, 0.4];
        renderer.draw_multiline_centered(ascii_art::STARS_ONLY, 1.0 + sy, star_color);

        // cult_papa in capture pose
        renderer.draw_multiline_centered(
            ascii_art::CULT_PAPA_CAPTURE,
            10.0 + sy,
            Colors::WHITE,
        );

        // Overlay cult_papa face
        let cols = renderer.screen_cols();
        renderer.draw_cult_papa_face(
            cols / 2.0 - 5.0 - FACE_SIZE / 2.0,
            12.5 - FACE_SIZE / 2.0 + sy,
            FACE_SIZE,
            Colors::WHITE,
        );

        // Moon being pulled down (animated)
        let pull_progress = (self.phase_timer * 0.4).min(1.0);
        let moon_row = 2.0 + pull_progress * 5.0;
        let struggle = (time * 8.0).sin() * 0.5;
        let panic_color = [1.0, 0.9, 0.3, 1.0];
        let moon_col = cols / 2.0 + 4.0 + struggle + sx;
        renderer.draw_multiline_at_grid(
            ascii_art::MOON_FACE_PANIC,
            moon_col,
            moon_row + sy,
            panic_color,
        );

        renderer.draw_centered(
            "The moon struggles but cult_papa's grip is iron!",
            24.0,
            [0.9, 0.7, 0.2, 1.0],
        );
    }

    fn render_moon_falls(&self, renderer: &mut GameRenderer, time: f32, _sx: f32, sy: f32) {
        // The sky goes dark without the moon
        let dark_color = [0.3, 0.3, 0.5, 0.3];
        renderer.draw_multiline_centered(ascii_art::STARS_ONLY, 1.0 + sy, dark_color);

        // Moon falling animation
        renderer.draw_multiline_centered(
            ascii_art::MOON_FALLING,
            4.0 + sy,
            [1.0, 0.9, 0.3, 1.0],
        );

        // Impact effect near the end
        let progress = self.phase_timer / 2.0;
        if progress > 0.7 {
            let flash = ((time * 20.0).sin() * 0.5 + 0.5).min(1.0);
            renderer.draw_centered(
                "*** CRASH ***",
                16.0 + sy,
                [1.0, 1.0, flash, 1.0],
            );
            renderer.draw_centered(
                "The ground shakes!",
                17.0 + sy,
                [0.8, 0.4, 0.2, 1.0],
            );
        }

        renderer.draw_centered(
            "\"You're mine now, moon.\"",
            24.0,
            [1.0, 0.5, 0.5, 1.0],
        );
    }

    fn render_draw_swords(&self, renderer: &mut GameRenderer, time: f32, sx: f32, sy: f32) {
        let cols = renderer.screen_cols();
        let left_col = cols / 2.0 - 22.0 + sx;
        let right_col = cols / 2.0 + 8.0 + sx;

        // cult_papa with sword
        renderer.draw_multiline_at_grid(
            ascii_art::CULT_PAPA_SWORD,
            left_col,
            6.0 + sy,
            Colors::WHITE,
        );

        // Overlay cult_papa face
        renderer.draw_cult_papa_face(
            left_col + 6.0 - FACE_SIZE / 2.0,
            8.5 - FACE_SIZE / 2.0 + sy,
            FACE_SIZE,
            Colors::WHITE,
        );

        // Moon with sword
        renderer.draw_multiline_at_grid(
            ascii_art::MOON_SWORD,
            right_col,
            6.0 + sy,
            [1.0, 1.0, 0.6, 1.0],
        );

        // Dramatic text
        let flash = (time * 4.0).sin() * 0.4 + 0.6;
        renderer.draw_centered(
            "///  PREPARE YOURSELF  \\\\\\",
            3.0 + sy,
            [1.0, 0.3, 0.3, flash],
        );

        let gleam = (time * 6.0).sin() * 0.5 + 0.5;
        renderer.draw_centered(
            "*  SHING!  *",
            18.0 + sy,
            [0.8, 0.8, 1.0, gleam],
        );

        renderer.draw_centered(
            "\"En garde, celestial body!\"",
            24.0,
            [0.7, 0.9, 1.0, 1.0],
        );
    }

    fn render_clash(
        &self,
        renderer: &mut GameRenderer,
        time: f32,
        frame: u8,
        sx: f32,
        sy: f32,
    ) {
        let clash_art = match frame {
            0 => ascii_art::DUEL_CLASH_1,
            1 => ascii_art::DUEL_CLASH_2,
            _ => ascii_art::DUEL_CLASH_3,
        };

        // Draw the clash scene
        let papa_color = [1.0, 0.95, 0.9, 1.0];
        renderer.draw_multiline_centered(clash_art, 5.0 + sy, papa_color);

        // Overlay cult_papa face (left side of clash art)
        let cols = renderer.screen_cols();
        renderer.draw_cult_papa_face(
            cols / 2.0 - 11.0 - FACE_SIZE / 2.0 + sx,
            7.5 - FACE_SIZE / 2.0 + sy,
            FACE_SIZE,
            papa_color,
        );

        // Spark particles
        for i in 0..5_usize {
            let angle = time * 3.0 + i as f32 * 1.2;
            let radius = self.phase_timer * 4.0;
            let spark_x = cols / 2.0 + angle.cos() * radius + sx;
            let spark_y = 10.0 + angle.sin() * radius * 0.5 + sy;
            if spark_y > 0.0 && spark_y < 28.0 {
                let spark_chars = ["*", "+", ".", "x", "o"];
                let spark_color = [
                    1.0,
                    0.8 + (i as f32 * 0.04),
                    0.2,
                    (1.0 - self.phase_timer / 1.5).max(0.0),
                ];
                renderer.draw_at_grid(
                    spark_chars[i % spark_chars.len()],
                    spark_x,
                    spark_y,
                    spark_color,
                );
            }
        }

        // Impact text
        let impact_texts = ["CLANG!", "SLASH!", "PARRY!"];
        let text = impact_texts[frame as usize % impact_texts.len()];
        let text_flash = ((time * 10.0).sin() * 0.5 + 0.5).min(1.0);
        renderer.draw_centered(
            text,
            20.0 + sy,
            [1.0, text_flash, 0.2, 1.0],
        );

        // Combo counter
        let combo = self.clash_cycles + 1;
        renderer.draw_centered(
            &format!("COMBO x{}", combo),
            22.0 + sy,
            [1.0, 0.5, 0.0, 1.0],
        );
    }

    fn render_victory(&self, renderer: &mut GameRenderer, time: f32, sx: f32, sy: f32) {
        // Stars return brighter
        let twinkle = (time * 2.0).sin() * 0.2 + 0.8;
        let star_color = [0.9, 0.9, 1.0, twinkle];
        renderer.draw_multiline_centered(ascii_art::STARS_ONLY, 1.0 + sy, star_color);

        // Victory pose
        let gold = [1.0, 0.85, 0.0, 1.0];
        renderer.draw_multiline_centered(ascii_art::CULT_PAPA_VICTORY, 8.0 + sy, gold);

        // Overlay cult_papa face
        let cols = renderer.screen_cols();
        renderer.draw_cult_papa_face(
            cols / 2.0 - 5.5 - FACE_SIZE / 2.0,
            11.5 - FACE_SIZE / 2.0 + sy,
            FACE_SIZE,
            Colors::WHITE,
        );

        // Celebration particles
        for i in 0..8_usize {
            let x = cols / 2.0 + (time * 1.5 + i as f32 * 0.8).sin() * 15.0 + sx;
            let y = 2.0 + (time * 1.2 + i as f32 * 1.1).cos().abs() * 6.0 + sy;
            let particle = if i % 2 == 0 { "*" } else { "+" };
            let color = match i % 4 {
                0 => Colors::YELLOW,
                1 => Colors::CYAN,
                2 => Colors::PINK,
                _ => Colors::GREEN,
            };
            renderer.draw_at_grid(particle, x, y, color);
        }

        // Victory text with rainbow cycling
        let hue = time * 2.0;
        let r = (hue.sin() * 0.5 + 0.5).min(1.0);
        let g = ((hue + 2.094).sin() * 0.5 + 0.5).min(1.0);
        let b = ((hue + 4.189).sin() * 0.5 + 0.5).min(1.0);
        renderer.draw_centered(
            "cult_papa has conquered the moon!",
            20.0 + sy,
            [r, g, b, 1.0],
        );

        renderer.draw_centered(
            "The tides themselves bow to cult_papa.",
            22.0 + sy,
            [0.7, 0.7, 0.9, 0.8],
        );

        renderer.draw_centered("[Enter] Return", 26.0, Colors::DARK_GRAY);
    }
}

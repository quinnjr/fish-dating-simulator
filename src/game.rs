//! Top-level game state machine and screen management.

use winit::keyboard::KeyCode;

use crate::ascii_art;
use crate::data::{FishId, FishSize, PlayerState, relationship_label};
use crate::data::save;
use crate::dating::DatingState;
use crate::fishing::{MinigameState, PondSelectState};
use crate::render::{Colors, GameRenderer};
use crate::ui;
use crate::ui::menu::SelectionMenu;

/// All possible game screens.
pub enum GameScreen {
    MainMenu,
    FishingPondSelect,
    FishingMinigame(MinigameState),
    CatchResult {
        fish_id: FishId,
        pond_index: usize,
        size: FishSize,
    },
    FishCollection,
    DateSelect,
    Dating(DatingState),
    DateResult {
        fish_id: FishId,
        affection: i32,
    },
    GameOver,
}

/// The complete game state.
pub struct Game {
    pub screen: GameScreen,
    pub player: PlayerState,
    pub time: f32,
    // Screen-specific sub-states
    menu: SelectionMenu,
    pond_state: Option<PondSelectState>,
    date_select_menu: Option<SelectionMenu>,
    collection_scroll: usize,
}

impl Game {
    pub fn new() -> Self {
        let player = save::load_game().unwrap_or_default();
        let has_save = save::save_exists();

        let menu_items = if has_save {
            vec![
                "Go Fishing".to_string(),
                "Go on a Date".to_string(),
                "Fish Collection".to_string(),
                "New Game".to_string(),
                "Quit".to_string(),
            ]
        } else {
            vec![
                "Go Fishing".to_string(),
                "Quit".to_string(),
            ]
        };

        Self {
            screen: GameScreen::MainMenu,
            player,
            time: 0.0,
            menu: SelectionMenu::new(menu_items),
            pond_state: None,
            date_select_menu: None,
            collection_scroll: 0,
        }
    }

    /// Rebuild the main menu based on current state.
    fn rebuild_menu(&mut self) {
        let has_fish = !self.player.fish_collection.is_empty();
        let mut items = vec!["Go Fishing".to_string()];
        if has_fish {
            items.push("Go on a Date".to_string());
            items.push("Fish Collection".to_string());
        }
        items.push("Save Game".to_string());
        items.push("Quit".to_string());
        self.menu = SelectionMenu::new(items);
    }

    pub fn update(&mut self, dt: f32, key: Option<KeyCode>) {
        self.time += dt;

        let transition = match &mut self.screen {
            GameScreen::MainMenu => self.update_main_menu(key),
            GameScreen::FishingPondSelect => {
                if let Some(ref mut state) = self.pond_state {
                    if let Some(k) = key {
                        state.update(k)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            GameScreen::FishingMinigame(state) => state.update(dt, key),
            GameScreen::CatchResult { .. } => self.update_catch_result(key),
            GameScreen::FishCollection => self.update_collection(key),
            GameScreen::DateSelect => self.update_date_select(key),
            GameScreen::Dating(state) => state.update(dt, key),
            GameScreen::DateResult { .. } => self.update_date_result(key),
            GameScreen::GameOver => self.update_game_over(key),
        };

        if let Some(new_screen) = transition {
            self.transition_to(new_screen);
        }
    }

    fn transition_to(&mut self, screen: GameScreen) {
        match &screen {
            GameScreen::MainMenu => {
                self.rebuild_menu();
            }
            GameScreen::FishingPondSelect => {
                self.pond_state = Some(PondSelectState::new());
            }
            GameScreen::DateSelect => {
                let dateable: Vec<String> = FishId::ALL
                    .iter()
                    .filter(|f| self.player.has_caught(**f))
                    .map(|f| {
                        let score = self.player.relationship(*f);
                        let label = relationship_label(score);
                        format!("{} ({}) - {} [{}]", f.name(), f.species(), label, score)
                    })
                    .collect();
                if dateable.is_empty() {
                    // No fish to date, go back
                    self.rebuild_menu();
                    self.screen = GameScreen::MainMenu;
                    return;
                }
                self.date_select_menu = Some(SelectionMenu::new(dateable));
            }
            GameScreen::CatchResult {
                fish_id,
                pond_index,
                size,
            } => {
                let pond_name = ascii_art::POND_NAMES[*pond_index];
                self.player.add_catch(*fish_id, pond_name, *size);
                // Give a small affection bonus for catching
                self.player.add_affection(*fish_id, 1);
                let _ = save::save_game(&self.player);
            }
            GameScreen::DateResult {
                fish_id,
                affection,
            } => {
                self.player.add_affection(*fish_id, *affection);
                self.player.increment_date_count(*fish_id);
                self.player.dates_completed += 1;
                self.player.current_day += 1;
                let _ = save::save_game(&self.player);
            }
            _ => {}
        }
        self.screen = screen;
    }

    fn update_main_menu(&mut self, key: Option<KeyCode>) -> Option<GameScreen> {
        let k = key?;
        match k {
            KeyCode::ArrowUp | KeyCode::KeyW => {
                self.menu.move_up();
                None
            }
            KeyCode::ArrowDown | KeyCode::KeyS => {
                self.menu.move_down();
                None
            }
            KeyCode::Enter | KeyCode::Space => {
                let selected = &self.menu.items[self.menu.selected_index()];
                match selected.as_str() {
                    "Go Fishing" => Some(GameScreen::FishingPondSelect),
                    "Go on a Date" => Some(GameScreen::DateSelect),
                    "Fish Collection" => Some(GameScreen::FishCollection),
                    "Save Game" => {
                        let _ = save::save_game(&self.player);
                        None
                    }
                    "New Game" => {
                        self.player = PlayerState::default();
                        let _ = save::save_game(&self.player);
                        self.rebuild_menu();
                        None
                    }
                    "Quit" => {
                        std::process::exit(0);
                    }
                    _ => None,
                }
            }
            KeyCode::Escape => {
                std::process::exit(0);
            }
            _ => None,
        }
    }

    fn update_catch_result(&mut self, key: Option<KeyCode>) -> Option<GameScreen> {
        if let Some(KeyCode::Enter | KeyCode::Space) = key {
            if self.player.has_won() {
                return Some(GameScreen::GameOver);
            }
            return Some(GameScreen::MainMenu);
        }
        None
    }

    fn update_collection(&mut self, key: Option<KeyCode>) -> Option<GameScreen> {
        match key? {
            KeyCode::Escape | KeyCode::Enter => Some(GameScreen::MainMenu),
            KeyCode::ArrowUp | KeyCode::KeyW => {
                self.collection_scroll = self.collection_scroll.saturating_sub(1);
                None
            }
            KeyCode::ArrowDown | KeyCode::KeyS => {
                self.collection_scroll += 1;
                None
            }
            _ => None,
        }
    }

    fn update_date_select(&mut self, key: Option<KeyCode>) -> Option<GameScreen> {
        let k = key?;
        if let Some(ref mut menu) = self.date_select_menu {
            match k {
                KeyCode::ArrowUp | KeyCode::KeyW => {
                    menu.move_up();
                    None
                }
                KeyCode::ArrowDown | KeyCode::KeyS => {
                    menu.move_down();
                    None
                }
                KeyCode::Enter | KeyCode::Space => {
                    let idx = menu.selected_index();
                    let dateable: Vec<FishId> = FishId::ALL
                        .iter()
                        .filter(|f| self.player.has_caught(**f))
                        .copied()
                        .collect();
                    if let Some(&fish_id) = dateable.get(idx) {
                        let date_num = self.player.date_count(fish_id);
                        Some(GameScreen::Dating(DatingState::new(fish_id, date_num)))
                    } else {
                        None
                    }
                }
                KeyCode::Escape => Some(GameScreen::MainMenu),
                _ => None,
            }
        } else {
            None
        }
    }

    fn update_date_result(&mut self, key: Option<KeyCode>) -> Option<GameScreen> {
        if let Some(KeyCode::Enter | KeyCode::Space) = key {
            if self.player.has_won() {
                return Some(GameScreen::GameOver);
            }
            return Some(GameScreen::MainMenu);
        }
        None
    }

    fn update_game_over(&mut self, key: Option<KeyCode>) -> Option<GameScreen> {
        if let Some(KeyCode::Enter | KeyCode::Space) = key {
            self.player = PlayerState::default();
            let _ = save::save_game(&self.player);
            return Some(GameScreen::MainMenu);
        }
        None
    }

    pub fn render(&self, renderer: &mut GameRenderer) {
        match &self.screen {
            GameScreen::MainMenu => self.render_main_menu(renderer),
            GameScreen::FishingPondSelect => {
                if let Some(ref state) = self.pond_state {
                    state.render(renderer, self.time);
                }
            }
            GameScreen::FishingMinigame(state) => state.render(renderer, self.time),
            GameScreen::CatchResult {
                fish_id,
                size,
                ..
            } => self.render_catch_result(renderer, *fish_id, *size),
            GameScreen::FishCollection => self.render_collection(renderer),
            GameScreen::DateSelect => self.render_date_select(renderer),
            GameScreen::Dating(state) => {
                let affection = self.player.relationship(state.fish_id);
                state.render(renderer, affection, self.time);
            }
            GameScreen::DateResult { fish_id, affection } => {
                self.render_date_result(renderer, *fish_id, *affection);
            }
            GameScreen::GameOver => self.render_game_over(renderer),
        }
    }

    fn render_main_menu(&self, renderer: &mut GameRenderer) {
        // Animated title with color cycling
        let hue = (self.time * 0.5).sin() * 0.5 + 0.5;
        let title_color = [0.0 + hue * 0.3, 0.8 + hue * 0.2, 1.0, 1.0];
        renderer.draw_multiline_centered(ascii_art::TITLE_ART, 1.0, title_color);

        // Subtitle with gentle pulse
        let pulse = (self.time * 2.0).sin() * 0.2 + 0.8;
        renderer.draw_centered(
            ascii_art::SUBTITLE,
            14.0,
            [1.0, 1.0, 0.0, pulse],
        );

        // Animated swimming fish
        let fish_x_offset = (self.time * 1.5).sin() * 3.0;
        let cols = renderer.screen_cols();
        let fish_col = (cols / 2.0 - 5.0 + fish_x_offset) as f32;
        renderer.draw_at_grid(
            ascii_art::BUBBLES_SMALL,
            fish_col,
            16.0,
            Colors::ORANGE,
        );
        renderer.draw_at_grid(
            ascii_art::MARINA_SMALL,
            fish_col + 15.0 - fish_x_offset * 0.5,
            17.0,
            Colors::LIGHT_BLUE,
        );
        renderer.draw_at_grid(
            ascii_art::GILL_SMALL,
            fish_col + 5.0 + fish_x_offset * 0.7,
            18.0,
            Colors::GREEN,
        );

        // Animated water line
        let wave = if ((self.time * 3.0) as i32) % 2 == 0 {
            "~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~"
        } else {
            " ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~"
        };
        renderer.draw_centered(wave, 19.0, [0.2, 0.3, 0.7, 0.6]);

        // Menu
        self.menu.draw_centered(renderer, 21.0);

        // Status bar
        let day = self.player.current_day;
        let fish_count = self.player.fish_collection.len();
        let dates = self.player.dates_completed;
        renderer.draw_centered(
            &format!("Day {} | Fish: {} | Dates: {}", day, fish_count, dates),
            27.0,
            Colors::DARK_GRAY,
        );

        // Controls hint
        renderer.draw_centered(
            "[Arrow Keys] Navigate  [Enter] Select  [Esc] Quit",
            29.0,
            [0.3, 0.3, 0.3, 0.5],
        );
    }

    fn render_catch_result(&self, renderer: &mut GameRenderer, fish_id: FishId, size: FishSize) {
        renderer.draw_centered("=== CATCH! ===", 2.0, Colors::GREEN);

        renderer.draw_multiline_centered(ascii_art::CATCH_SUCCESS, 4.0, Colors::YELLOW);

        let art = crate::dating::fish::fish_art(fish_id, 0);
        renderer.draw_multiline_centered(art, 11.0, fish_id.color());

        renderer.draw_centered(
            &format!("You caught {} ({})!", fish_id.name(), fish_id.species()),
            19.0,
            Colors::WHITE,
        );
        renderer.draw_centered(
            &format!("Size: {}", size.label()),
            20.0,
            Colors::YELLOW,
        );
        renderer.draw_centered(
            &format!("Total {}: {}", fish_id.name(), self.player.catch_count(fish_id)),
            21.0,
            Colors::GRAY,
        );

        renderer.draw_centered("[Enter] Continue", 24.0, Colors::DARK_GRAY);
    }

    fn render_collection(&self, renderer: &mut GameRenderer) {
        renderer.draw_centered("=== FISH COLLECTION ===", 1.0, Colors::CYAN);

        if self.player.fish_collection.is_empty() {
            renderer.draw_centered("No fish caught yet! Go fishing!", 10.0, Colors::GRAY);
            renderer.draw_centered("[Enter/Esc] Back", 12.0, Colors::DARK_GRAY);
            return;
        }

        let mut row = 3.0;
        for fish_id in &FishId::ALL {
            let count = self.player.catch_count(*fish_id);
            if count == 0 {
                continue;
            }

            let score = self.player.relationship(*fish_id);
            let label = relationship_label(score);

            renderer.draw_centered(
                &format!(
                    "{} ({}) - Caught: {} - {}: {}",
                    fish_id.name(),
                    fish_id.species(),
                    count,
                    label,
                    score,
                ),
                row,
                fish_id.color(),
            );

            // Mini hearts
            let cols = renderer.screen_cols() as usize;
            ui::draw_hearts(renderer, (cols / 2 - 8) as f32, row + 1.0, score, 5);

            row += 3.0;
        }

        renderer.draw_centered("[Enter/Esc] Back", row + 2.0, Colors::DARK_GRAY);
    }

    fn render_date_select(&self, renderer: &mut GameRenderer) {
        renderer.draw_centered("=== CHOOSE A DATE ===", 1.0, Colors::PINK);
        renderer.draw_centered(
            "Select a fish to take on a date:",
            3.0,
            Colors::WHITE,
        );

        if let Some(ref menu) = self.date_select_menu {
            menu.draw_centered(renderer, 5.0);

            // Show selected fish preview
            let dateable: Vec<FishId> = FishId::ALL
                .iter()
                .filter(|f| self.player.has_caught(**f))
                .copied()
                .collect();
            if let Some(&fish_id) = dateable.get(menu.selected_index()) {
                let score = self.player.relationship(fish_id);
                let art = crate::dating::fish::fish_art(fish_id, score);
                renderer.draw_multiline_centered(art, 10.0, fish_id.color());

                let loc = crate::dating::fish::date_location(fish_id);
                renderer.draw_centered(
                    &format!("Date location: {}", loc),
                    18.0,
                    Colors::LIGHT_BLUE,
                );
            }
        }

        renderer.draw_centered("[Enter] Go on date  [Esc] Back", 20.0, Colors::DARK_GRAY);
    }

    fn render_date_result(&self, renderer: &mut GameRenderer, fish_id: FishId, affection: i32) {
        renderer.draw_centered("=== DATE COMPLETE ===", 2.0, Colors::PINK);

        let art = crate::dating::fish::fish_art(fish_id, self.player.relationship(fish_id));
        renderer.draw_multiline_centered(art, 5.0, fish_id.color());

        let total = self.player.relationship(fish_id);
        let label = relationship_label(total);

        renderer.draw_centered(
            &format!("Date with {} finished!", fish_id.name()),
            13.0,
            Colors::WHITE,
        );
        renderer.draw_centered(
            &format!("Affection gained: +{}", affection),
            14.0,
            if affection > 5 {
                Colors::GREEN
            } else if affection > 2 {
                Colors::YELLOW
            } else {
                Colors::RED
            },
        );
        renderer.draw_centered(
            &format!("Relationship: {} ({})", label, total),
            15.0,
            Colors::PINK,
        );

        let cols = renderer.screen_cols() as usize;
        ui::draw_hearts(renderer, (cols / 2 - 8) as f32, 17.0, total, 5);

        renderer.draw_centered("[Enter] Continue", 19.0, Colors::DARK_GRAY);
    }

    fn render_game_over(&self, renderer: &mut GameRenderer) {
        renderer.draw_centered("=== CONGRATULATIONS! ===", 3.0, Colors::YELLOW);

        if let Some((fish_id, score)) = self.player.closest_fish() {
            let art = crate::dating::fish::fish_art(fish_id, score);
            renderer.draw_multiline_centered(art, 6.0, fish_id.color());

            renderer.draw_centered(
                &format!("You and {} are soulmates!", fish_id.name()),
                14.0,
                Colors::PINK,
            );
            renderer.draw_centered(
                &format!("Final affection: {}", score),
                15.0,
                Colors::WHITE,
            );
        }

        renderer.draw_centered(
            "Thank you for playing cult_papa Fish Dating Simulator!",
            18.0,
            Colors::CYAN,
        );
        renderer.draw_centered("[Enter] New Game", 20.0, Colors::DARK_GRAY);
    }
}

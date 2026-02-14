//! Top-level game state machine and screen management.

use winit::keyboard::KeyCode;

use crate::ascii_art;
use crate::data::{FishId, FishSize, PlayerState, relationship_label};
use crate::data::save;
use crate::dating::DatingState;
use crate::dating::fish as fish_helpers;
use crate::easter_egg::{MoonBattleState, SecretSequence};
use crate::fishing::{MinigameState, PondSelectState};
use crate::plugins::FishRegistry;
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
    /// Secret: cult_papa captures and fights the moon.
    MoonBattle(MoonBattleState),
}

/// The complete game state.
pub struct Game {
    pub screen: GameScreen,
    pub player: PlayerState,
    pub time: f32,
    pub registry: FishRegistry,
    // Screen-specific sub-states
    menu: SelectionMenu,
    pond_state: Option<PondSelectState>,
    date_select_menu: Option<SelectionMenu>,
    collection_scroll: usize,
    /// Tracks the secret "moon" key sequence on the main menu.
    moon_secret: SecretSequence,
}

impl Game {
    pub fn new(registry: FishRegistry) -> Self {
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
            registry,
            menu: SelectionMenu::new(menu_items),
            pond_state: None,
            date_select_menu: None,
            collection_scroll: 0,
            moon_secret: SecretSequence::new(),
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
            GameScreen::MoonBattle(state) => state.update(dt, key),
        };

        if let Some(new_screen) = transition {
            self.transition_to(new_screen);
        }
    }

    fn transition_to(&mut self, screen: GameScreen) {
        match &screen {
            GameScreen::MainMenu => {
                self.rebuild_menu();
                self.moon_secret.reset();
            }
            GameScreen::FishingPondSelect => {
                self.pond_state = Some(PondSelectState::new(&self.registry));
            }
            GameScreen::DateSelect => {
                let all_fish = FishId::all_with_plugins(&self.registry);
                let dateable: Vec<String> = all_fish
                    .iter()
                    .filter(|f| self.player.has_caught(f))
                    .map(|f| {
                        let score = self.player.relationship(f);
                        let label = relationship_label(score);
                        let name = f.name_with_registry(&self.registry);
                        let species = f.species_with_registry(&self.registry);
                        format!("{} ({}) - {} [{}]", name, species, label, score)
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
                let pond_name = if *pond_index < ascii_art::POND_NAMES.len() {
                    ascii_art::POND_NAMES[*pond_index].to_string()
                } else {
                    self.registry.pond_names()
                        .get(*pond_index - ascii_art::POND_NAMES.len())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "Unknown Pond".to_string())
                };
                self.player.add_catch(fish_id.clone(), &pond_name, *size);
                // Give a small affection bonus for catching
                self.player.add_affection(fish_id.clone(), 1);
                let _ = save::save_game(&self.player);
            }
            GameScreen::DateResult {
                fish_id,
                affection,
            } => {
                self.player.add_affection(fish_id.clone(), *affection);
                self.player.increment_date_count(fish_id.clone());
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

        // Feed every key press to the secret "moon" detector
        if self.moon_secret.feed(k) {
            return Some(GameScreen::MoonBattle(MoonBattleState::new()));
        }

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
                    let all_fish = FishId::all_with_plugins(&self.registry);
                    let dateable: Vec<FishId> = all_fish
                        .into_iter()
                        .filter(|f| self.player.has_caught(f))
                        .collect();
                    if let Some(fish_id) = dateable.get(idx) {
                        let date_num = self.player.date_count(fish_id);
                        Some(GameScreen::Dating(DatingState::new(
                            fish_id.clone(),
                            date_num,
                            &self.registry,
                        )))
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
                    state.render(renderer, self.time, &self.registry);
                }
            }
            GameScreen::FishingMinigame(state) => state.render(renderer, self.time, &self.registry),
            GameScreen::CatchResult {
                fish_id,
                size,
                ..
            } => self.render_catch_result(renderer, fish_id, *size),
            GameScreen::FishCollection => self.render_collection(renderer),
            GameScreen::DateSelect => self.render_date_select(renderer),
            GameScreen::Dating(state) => {
                let affection = self.player.relationship(&state.fish_id);
                state.render(renderer, affection, self.time, &self.registry);
            }
            GameScreen::DateResult { fish_id, affection } => {
                self.render_date_result(renderer, fish_id, *affection);
            }
            GameScreen::GameOver => self.render_game_over(renderer),
            GameScreen::MoonBattle(state) => state.render(renderer, self.time),
        }
    }

    fn render_main_menu(&self, renderer: &mut GameRenderer) {
        // Title art (18 lines starting at row 1)
        let title_lines = ascii_art::TITLE_ART.lines().count() as f32;
        let hue = (self.time * 0.5).sin() * 0.5 + 0.5;
        let title_color = [0.0 + hue * 0.3, 0.8 + hue * 0.2, 1.0, 1.0];
        renderer.draw_multiline_centered(ascii_art::TITLE_ART, 1.0, title_color);

        // Subtitle just below the title art
        let subtitle_row = 1.0 + title_lines;
        let pulse = (self.time * 2.0).sin() * 0.2 + 0.8;
        renderer.draw_centered(
            ascii_art::SUBTITLE,
            subtitle_row,
            [1.0, 1.0, 0.0, pulse],
        );

        // Animated swimming fish below subtitle
        let fish_start = subtitle_row + 2.0;
        let fish_x_offset = (self.time * 1.5).sin() * 3.0;
        let cols = renderer.screen_cols();
        let fish_col = (cols / 2.0 - 5.0 + fish_x_offset) as f32;
        renderer.draw_at_grid(
            ascii_art::BUBBLES_SMALL,
            fish_col,
            fish_start,
            Colors::ORANGE,
        );
        renderer.draw_at_grid(
            ascii_art::MARINA_SMALL,
            fish_col + 15.0 - fish_x_offset * 0.5,
            fish_start + 1.0,
            Colors::LIGHT_BLUE,
        );
        renderer.draw_at_grid(
            ascii_art::GILL_SMALL,
            fish_col + 5.0 + fish_x_offset * 0.7,
            fish_start + 2.0,
            Colors::GREEN,
        );

        // Animated water line
        let wave_row = fish_start + 3.0;
        let wave = if ((self.time * 3.0) as i32) % 2 == 0 {
            "~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~"
        } else {
            " ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~~ ~"
        };
        renderer.draw_centered(wave, wave_row, [0.2, 0.3, 0.7, 0.6]);

        // Menu below the water line with a gap
        let menu_row = wave_row + 2.0;
        self.menu.draw_centered(renderer, menu_row);

        // Everything below the menu flows from the menu bottom
        let menu_end = menu_row + self.menu.items.len() as f32;

        // Plugin count indicator
        let mut info_row = menu_end + 1.0;
        if self.registry.count() > 0 {
            renderer.draw_centered(
                &format!("Plugins: {} fish loaded", self.registry.count()),
                info_row,
                Colors::PURPLE,
            );
            info_row += 1.0;
        }

        // Status bar
        info_row += 1.0;
        let day = self.player.current_day;
        let fish_count = self.player.fish_collection.len();
        let dates = self.player.dates_completed;
        renderer.draw_centered(
            &format!("Day {} | Fish: {} | Dates: {}", day, fish_count, dates),
            info_row,
            Colors::DARK_GRAY,
        );

        // Controls hint
        renderer.draw_centered(
            "[Arrow Keys] Navigate  [Enter] Select  [Esc] Quit",
            info_row + 2.0,
            [0.3, 0.3, 0.3, 0.5],
        );
    }

    fn render_catch_result(&self, renderer: &mut GameRenderer, fish_id: &FishId, size: FishSize) {
        renderer.draw_centered("=== CATCH! ===", 2.0, Colors::GREEN);

        renderer.draw_multiline_centered(ascii_art::CATCH_SUCCESS, 4.0, Colors::YELLOW);

        let art = fish_helpers::fish_art(fish_id, 0, &self.registry);
        renderer.draw_multiline_centered(&art, 11.0, fish_id.color());

        let name = fish_id.name_with_registry(&self.registry);
        let species = fish_id.species_with_registry(&self.registry);
        renderer.draw_centered(
            &format!("You caught {} ({})!", name, species),
            19.0,
            Colors::WHITE,
        );
        renderer.draw_centered(
            &format!("Size: {}", size.label()),
            20.0,
            Colors::YELLOW,
        );
        renderer.draw_centered(
            &format!("Total {}: {}", name, self.player.catch_count(fish_id)),
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
        let all_fish = FishId::all_with_plugins(&self.registry);
        for fish_id in &all_fish {
            let count = self.player.catch_count(fish_id);
            if count == 0 {
                continue;
            }

            let score = self.player.relationship(fish_id);
            let label = relationship_label(score);
            let name = fish_id.name_with_registry(&self.registry);
            let species = fish_id.species_with_registry(&self.registry);

            renderer.draw_centered(
                &format!(
                    "{} ({}) - Caught: {} - {}: {}",
                    name,
                    species,
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
            let all_fish = FishId::all_with_plugins(&self.registry);
            let dateable: Vec<&FishId> = all_fish
                .iter()
                .filter(|f| self.player.has_caught(f))
                .collect();
            if let Some(fish_id) = dateable.get(menu.selected_index()) {
                let score = self.player.relationship(fish_id);
                let art = fish_helpers::fish_art(fish_id, score, &self.registry);
                renderer.draw_multiline_centered(&art, 10.0, fish_id.color());

                let loc = fish_helpers::date_location(fish_id, &self.registry);
                renderer.draw_centered(
                    &format!("Date location: {}", loc),
                    18.0,
                    Colors::LIGHT_BLUE,
                );
            }
        }

        renderer.draw_centered("[Enter] Go on date  [Esc] Back", 20.0, Colors::DARK_GRAY);
    }

    fn render_date_result(&self, renderer: &mut GameRenderer, fish_id: &FishId, affection: i32) {
        renderer.draw_centered("=== DATE COMPLETE ===", 2.0, Colors::PINK);

        let art = fish_helpers::fish_art(fish_id, self.player.relationship(fish_id), &self.registry);
        renderer.draw_multiline_centered(&art, 5.0, fish_id.color());

        let total = self.player.relationship(fish_id);
        let label = relationship_label(total);
        let name = fish_id.name_with_registry(&self.registry);

        renderer.draw_centered(
            &format!("Date with {} finished!", name),
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
            let art = fish_helpers::fish_art(&fish_id, score, &self.registry);
            renderer.draw_multiline_centered(&art, 6.0, fish_id.color());

            let name = fish_id.name_with_registry(&self.registry);
            renderer.draw_centered(
                &format!("You and {} are soulmates!", name),
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

//! Pond selection screen.

use winit::keyboard::KeyCode;

use crate::ascii_art;
use crate::data::FishId;
use crate::game::GameScreen;
use crate::render::{Colors, GameRenderer};
use crate::ui::menu::SelectionMenu;

pub struct PondSelectState {
    menu: SelectionMenu,
}

impl PondSelectState {
    pub fn new() -> Self {
        Self {
            menu: SelectionMenu::new(
                ascii_art::POND_NAMES
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
        }
    }

    pub fn update(&mut self, key: KeyCode) -> Option<GameScreen> {
        match key {
            KeyCode::ArrowUp | KeyCode::KeyW => {
                self.menu.move_up();
                None
            }
            KeyCode::ArrowDown | KeyCode::KeyS => {
                self.menu.move_down();
                None
            }
            KeyCode::Enter | KeyCode::Space => {
                let pond_idx = self.menu.selected_index();
                // Find which fish lives in this pond
                let fish = FishId::ALL
                    .iter()
                    .find(|f| f.pond_index() == pond_idx)
                    .copied()
                    .unwrap_or(FishId::Bubbles);
                Some(GameScreen::FishingMinigame(
                    crate::fishing::MinigameState::new(fish, pond_idx),
                ))
            }
            KeyCode::Escape => Some(GameScreen::MainMenu),
            _ => None,
        }
    }

    pub fn render(&self, renderer: &mut GameRenderer, time: f32) {
        renderer.draw_centered("=== CHOOSE A FISHING SPOT ===", 1.0, Colors::CYAN);

        // Animated pond scene
        renderer.draw_multiline_centered(ascii_art::POND_SCENE, 3.0, Colors::LIGHT_BLUE);

        // Animate water
        let wave_offset = ((time * 2.0).sin() * 2.0) as i32;
        let wave = if wave_offset > 0 { "~~ " } else { " ~~" };
        renderer.draw_centered(
            &wave.repeat(15),
            16.0,
            [0.2, 0.4, 0.8, 0.5],
        );

        // Pond selection
        renderer.draw_centered("Select a pond:", 18.0, Colors::WHITE);
        self.menu.draw_centered(renderer, 20.0);

        // Fish hint for selected pond
        let pond_idx = self.menu.selected_index();
        if let Some(fish) = FishId::ALL.iter().find(|f| f.pond_index() == pond_idx) {
            let hint = format!("Rumor has it {} ({}) swims here...", fish.name(), fish.species());
            renderer.draw_centered(&hint, 24.0, Colors::GRAY);
        }

        renderer.draw_centered("[Enter] Cast  [Esc] Back", 26.0, Colors::DARK_GRAY);
    }
}

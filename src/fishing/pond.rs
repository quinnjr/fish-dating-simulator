//! Pond selection screen.

use winit::keyboard::KeyCode;

use crate::ascii_art;
use crate::data::FishId;
use crate::game::GameScreen;
use crate::plugins::FishRegistry;
use crate::render::{Colors, GameRenderer};
use crate::ui::menu::SelectionMenu;

pub struct PondSelectState {
    menu: SelectionMenu,
    /// Mapping from menu index to FishId.
    fish_map: Vec<FishId>,
}

impl PondSelectState {
    pub fn new(registry: &FishRegistry) -> Self {
        let mut pond_names: Vec<String> = ascii_art::POND_NAMES
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut fish_map: Vec<FishId> = Vec::new();

        // Map built-in ponds to fish
        for fish_id in &FishId::BUILTIN {
            fish_map.push(fish_id.clone());
        }

        // Add plugin fish ponds
        for plugin_id in registry.plugin_ids() {
            if let Some(fish) = registry.get(plugin_id) {
                pond_names.push(fish.pond_name.clone());
                fish_map.push(FishId::Plugin(plugin_id.clone()));
            }
        }

        Self {
            menu: SelectionMenu::new(pond_names),
            fish_map,
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
                if let Some(fish_id) = self.fish_map.get(pond_idx) {
                    Some(GameScreen::FishingMinigame(
                        crate::fishing::MinigameState::new(fish_id.clone(), pond_idx),
                    ))
                } else {
                    None
                }
            }
            KeyCode::Escape => Some(GameScreen::MainMenu),
            _ => None,
        }
    }

    pub fn render(&self, renderer: &mut GameRenderer, time: f32, registry: &FishRegistry) {
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
        if let Some(fish_id) = self.fish_map.get(pond_idx) {
            let name = fish_id.name_with_registry(registry);
            let species = fish_id.species_with_registry(registry);
            let hint = format!("Rumor has it {} ({}) swims here...", name, species);
            renderer.draw_centered(&hint, 24.0, Colors::GRAY);
        }

        renderer.draw_centered("[Enter] Cast  [Esc] Back", 26.0, Colors::DARK_GRAY);
    }
}

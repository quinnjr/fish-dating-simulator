//! Menu selection components.

use crate::render::{Colors, GameRenderer};

/// A simple selectable menu.
pub struct SelectionMenu {
    pub items: Vec<String>,
    pub selected: usize,
}

impl SelectionMenu {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items,
            selected: 0,
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.items.len() {
            self.selected += 1;
        }
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }

    pub fn draw(&self, renderer: &mut GameRenderer, col: f32, start_row: f32) {
        for (i, item) in self.items.iter().enumerate() {
            let is_selected = i == self.selected;
            let prefix = if is_selected { "> " } else { "  " };
            let color = if is_selected {
                Colors::YELLOW
            } else {
                Colors::WHITE
            };
            let text = format!("{}{}", prefix, item);
            renderer.draw_at_grid(&text, col, start_row + i as f32, color);
        }
    }

    pub fn draw_centered(&self, renderer: &mut GameRenderer, start_row: f32) {
        for (i, item) in self.items.iter().enumerate() {
            let is_selected = i == self.selected;
            let prefix = if is_selected { "> " } else { "  " };
            let color = if is_selected {
                Colors::YELLOW
            } else {
                Colors::WHITE
            };
            let text = format!("{}{}", prefix, item);
            renderer.draw_centered(&text, start_row + i as f32, color);
        }
    }
}

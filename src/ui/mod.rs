//! Reusable UI components for menus, boxes, and bars.

pub mod menu;

use crate::render::{Colors, GameRenderer};

/// Draw a bordered box at grid position with given dimensions.
pub fn draw_box(
    renderer: &mut GameRenderer,
    col: f32,
    row: f32,
    width: usize,
    height: usize,
    color: [f32; 4],
) {
    let inner_w = width.saturating_sub(2);
    // Top border
    let top = format!("+{}+", "-".repeat(inner_w));
    renderer.draw_at_grid(&top, col, row, color);

    // Middle
    for i in 1..height.saturating_sub(1) {
        let mid = format!("|{}|", " ".repeat(inner_w));
        renderer.draw_at_grid(&mid, col, row + i as f32, color);
    }

    // Bottom border
    if height > 1 {
        let bot = format!("+{}+", "-".repeat(inner_w));
        renderer.draw_at_grid(&bot, col, row + (height - 1) as f32, color);
    }
}

/// Draw a centered bordered box.
pub fn draw_centered_box(
    renderer: &mut GameRenderer,
    row: f32,
    width: usize,
    height: usize,
    color: [f32; 4],
) {
    let cols = renderer.screen_cols() as usize;
    let col = (cols.saturating_sub(width)) / 2;
    draw_box(renderer, col as f32, row, width, height, color);
}

/// Draw a progress bar at grid position.
pub fn draw_progress_bar(
    renderer: &mut GameRenderer,
    col: f32,
    row: f32,
    width: usize,
    progress: f32,
    fill_color: [f32; 4],
    empty_color: [f32; 4],
) {
    let inner = width.saturating_sub(2);
    let filled = (progress.clamp(0.0, 1.0) * inner as f32) as usize;
    let empty = inner - filled;

    renderer.draw_at_grid("[", col, row, Colors::WHITE);
    renderer.draw_at_grid(
        &"#".repeat(filled),
        col + 1.0,
        row,
        fill_color,
    );
    renderer.draw_at_grid(
        &"-".repeat(empty),
        col + 1.0 + filled as f32,
        row,
        empty_color,
    );
    renderer.draw_at_grid("]", col + 1.0 + inner as f32, row, Colors::WHITE);
}

/// Draw affection hearts.
pub fn draw_hearts(
    renderer: &mut GameRenderer,
    col: f32,
    row: f32,
    score: i32,
    max_hearts: i32,
) {
    let full_hearts = (score / 10).min(max_hearts);
    let mut x = col;
    for i in 0..max_hearts {
        if i < full_hearts {
            renderer.draw_at_grid("<3", x, row, Colors::RED);
        } else {
            renderer.draw_at_grid("<3", x, row, Colors::DARK_GRAY);
        }
        x += 3.0;
    }
}

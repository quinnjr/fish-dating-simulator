//! Steam achievements and in-game achievement tracking.
//!
//! Defines all achievements, checks unlock conditions against game state,
//! persists locally, and syncs with Steam when connected.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::data::{FishId, FishSize, PlayerState};
use crate::plugins::FishRegistry;
use crate::render::{Colors, GameRenderer};

// ── Achievement Identifiers ──────────────────────────────────────────────────

/// All known achievement API names.
pub mod ids {
    pub const FIRST_CATCH: &str = "ACH_FIRST_CATCH";
    pub const CATCH_ALL: &str = "ACH_CATCH_ALL";
    pub const FIRST_DATE: &str = "ACH_FIRST_DATE";
    pub const DATE_10: &str = "ACH_DATE_10";
    pub const SOULMATE: &str = "ACH_SOULMATE";
    pub const ALL_FRIENDS: &str = "ACH_ALL_FRIENDS";
    pub const BIG_CATCH: &str = "ACH_BIG_CATCH";
    pub const MOON_BATTLE: &str = "ACH_MOON_BATTLE";
    pub const MOON_VICTORY: &str = "ACH_MOON_VICTORY";
    pub const DAY_30: &str = "ACH_DAY_30";
    pub const CATCH_50: &str = "ACH_CATCH_50";
    pub const PLUGIN_FISH: &str = "ACH_PLUGIN_FISH";
}

/// Human-readable metadata for an achievement.
#[allow(dead_code)]
struct AchievementDef {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    hidden: bool,
}

/// The full catalog of achievements.
const ACHIEVEMENTS: &[AchievementDef] = &[
    AchievementDef {
        id: ids::FIRST_CATCH,
        name: "Gone Fishin'",
        description: "Catch your first fish.",
        hidden: false,
    },
    AchievementDef {
        id: ids::CATCH_ALL,
        name: "Gotta Catch 'Em All",
        description: "Catch every species of fish.",
        hidden: false,
    },
    AchievementDef {
        id: ids::FIRST_DATE,
        name: "Testing the Waters",
        description: "Go on your first date.",
        hidden: false,
    },
    AchievementDef {
        id: ids::DATE_10,
        name: "Serial Dater",
        description: "Go on 10 dates.",
        hidden: false,
    },
    AchievementDef {
        id: ids::SOULMATE,
        name: "Fish Soulmate",
        description: "Reach soulmate status with any fish.",
        hidden: false,
    },
    AchievementDef {
        id: ids::ALL_FRIENDS,
        name: "Social Butterfly...fish",
        description: "Become friends with every fish.",
        hidden: false,
    },
    AchievementDef {
        id: ids::BIG_CATCH,
        name: "The Big One",
        description: "Catch a large fish.",
        hidden: false,
    },
    AchievementDef {
        id: ids::MOON_BATTLE,
        name: "That's No Moon...",
        description: "Discover the secret moon battle.",
        hidden: true,
    },
    AchievementDef {
        id: ids::MOON_VICTORY,
        name: "Moonslayer",
        description: "Defeat the moon in combat.",
        hidden: true,
    },
    AchievementDef {
        id: ids::DAY_30,
        name: "Dedicated Angler",
        description: "Play for 30 days.",
        hidden: false,
    },
    AchievementDef {
        id: ids::CATCH_50,
        name: "Fish Hoarder",
        description: "Catch 50 fish total.",
        hidden: false,
    },
    AchievementDef {
        id: ids::PLUGIN_FISH,
        name: "Modding Community",
        description: "Catch a plugin fish.",
        hidden: false,
    },
];

// ── Toast Notification ───────────────────────────────────────────────────────

/// A transient on-screen notification when an achievement unlocks.
struct Toast {
    /// Achievement display name.
    name: String,
    /// Achievement description.
    description: String,
    /// Seconds remaining before the toast disappears.
    timer: f32,
}

const TOAST_DURATION: f32 = 4.0;
const TOAST_FADE_START: f32 = 1.0;

// ── Achievement Tracker ──────────────────────────────────────────────────────

/// Persistent set of unlocked achievement IDs (stored in save file).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnlockedAchievements {
    pub ids: HashSet<String>,
}

/// Runtime achievement tracker that checks conditions and manages toasts.
pub struct AchievementTracker {
    /// Optional Steam client (None if Steam unavailable).
    steam: Option<sable_steam::SteamClient>,
    /// Active toast notifications to display.
    toasts: Vec<Toast>,
}

impl AchievementTracker {
    /// Create a new tracker, optionally connecting to Steam.
    pub fn new() -> Self {
        let steam = Self::try_init_steam();
        Self {
            steam,
            toasts: Vec::new(),
        }
    }

    /// Attempt to initialize the Steam client. Returns None on failure.
    fn try_init_steam() -> Option<sable_steam::SteamClient> {
        // Use Spacewar test app ID for development.
        // Replace with real app ID when publishing.
        let config = sable_steam::SteamConfig::new(480).optional_client().skip_ownership_check();
        match sable_steam::SteamClient::init(config) {
            Ok(client) => {
                tracing::info!("Steam achievements connected");
                Some(client)
            }
            Err(e) => {
                tracing::info!("Steam not available ({}), achievements tracked locally", e);
                None
            }
        }
    }

    /// Run Steam callbacks (should be called once per frame).
    pub fn run_callbacks(&self) {
        if let Some(ref steam) = self.steam {
            steam.run_callbacks();
        }
    }

    /// Unlock an achievement if it hasn't been unlocked yet.
    /// Adds a toast and syncs to Steam when applicable.
    fn unlock(
        &mut self,
        id: &str,
        unlocked: &mut UnlockedAchievements,
    ) {
        if unlocked.ids.contains(id) {
            return;
        }

        unlocked.ids.insert(id.to_string());
        tracing::info!("Achievement unlocked: {}", id);

        // Sync with Steam
        if let Some(ref steam) = self.steam {
            let _ = steam.achievements().unlock(id);
        }

        // Find display metadata and create toast
        if let Some(def) = ACHIEVEMENTS.iter().find(|a| a.id == id) {
            self.toasts.push(Toast {
                name: def.name.to_string(),
                description: def.description.to_string(),
                timer: TOAST_DURATION,
            });
        }
    }

    // ── Condition Checks ─────────────────────────────────────────────────

    /// Check all state-driven achievements. Call after game state changes.
    ///
    /// Takes a snapshot of the relevant player fields to avoid borrow conflicts
    /// with `player.achievements`.
    pub fn check_state(
        &mut self,
        player: &mut PlayerState,
        registry: &FishRegistry,
    ) {
        // Snapshot the data we need so we can mutably borrow achievements.
        let collection_len = player.fish_collection.len();
        let dates_completed = player.dates_completed;
        let current_day = player.current_day;
        let has_won = player.has_won();

        let all_fish = FishId::all_with_plugins(registry);
        let all_caught = !all_fish.is_empty() && all_fish.iter().all(|f| player.has_caught(f));
        let all_friends = !all_fish.is_empty() && all_fish.iter().all(|f| player.relationship(f) >= 6);
        let has_plugin_catch = player.fish_collection.iter().any(|c| c.id.is_plugin());

        let unlocked = &mut player.achievements;

        // Catch achievements
        if collection_len > 0 {
            self.unlock(ids::FIRST_CATCH, unlocked);
        }
        if collection_len >= 50 {
            self.unlock(ids::CATCH_50, unlocked);
        }
        if all_caught {
            self.unlock(ids::CATCH_ALL, unlocked);
        }
        if has_plugin_catch {
            self.unlock(ids::PLUGIN_FISH, unlocked);
        }

        // Date achievements
        if dates_completed >= 1 {
            self.unlock(ids::FIRST_DATE, unlocked);
        }
        if dates_completed >= 10 {
            self.unlock(ids::DATE_10, unlocked);
        }
        if has_won {
            self.unlock(ids::SOULMATE, unlocked);
        }
        if all_friends {
            self.unlock(ids::ALL_FRIENDS, unlocked);
        }

        // Day achievements
        if current_day >= 30 {
            self.unlock(ids::DAY_30, unlocked);
        }
    }

    /// Unlock the "big catch" achievement when a large fish is caught.
    pub fn on_catch_size(
        &mut self,
        size: FishSize,
        unlocked: &mut UnlockedAchievements,
    ) {
        if matches!(size, FishSize::Large) {
            self.unlock(ids::BIG_CATCH, unlocked);
        }
    }

    /// Unlock the moon battle discovery achievement.
    pub fn on_moon_battle_started(
        &mut self,
        unlocked: &mut UnlockedAchievements,
    ) {
        self.unlock(ids::MOON_BATTLE, unlocked);
    }

    /// Unlock the moon victory achievement.
    pub fn on_moon_victory(
        &mut self,
        unlocked: &mut UnlockedAchievements,
    ) {
        self.unlock(ids::MOON_VICTORY, unlocked);
    }

    // ── Toast Rendering ──────────────────────────────────────────────────

    /// Update toast timers. Call once per frame with delta time.
    pub fn update(&mut self, dt: f32) {
        for toast in &mut self.toasts {
            toast.timer -= dt;
        }
        self.toasts.retain(|t| t.timer > 0.0);
    }

    /// Render active toasts in the top-right corner.
    pub fn render_toasts(&self, renderer: &mut GameRenderer) {
        let cols = renderer.screen_cols();

        for (i, toast) in self.toasts.iter().enumerate() {
            let alpha = if toast.timer < TOAST_FADE_START {
                toast.timer / TOAST_FADE_START
            } else {
                1.0
            };

            let row = 1.0 + i as f32 * 3.0;

            // Background bar
            let bar = "========================================";
            let bar_col = cols - bar.len() as f32 - 1.0;
            renderer.draw_at_grid(bar, bar_col, row, [0.2, 0.2, 0.3, alpha * 0.8]);
            renderer.draw_at_grid(bar, bar_col, row + 2.0, [0.2, 0.2, 0.3, alpha * 0.8]);

            // Trophy + name
            let header = format!(" * ACHIEVEMENT UNLOCKED * ");
            renderer.draw_at_grid(
                &header,
                cols - header.len() as f32 - 1.0,
                row,
                [1.0, 0.85, 0.0, alpha],
            );

            // Achievement name
            let name_line = format!(" {}", toast.name);
            renderer.draw_at_grid(
                &name_line,
                cols - name_line.len() as f32 - 1.0,
                row + 1.0,
                [1.0, 1.0, 1.0, alpha],
            );

            // Description
            let desc_line = format!(" {}", toast.description);
            renderer.draw_at_grid(
                &desc_line,
                cols - desc_line.len() as f32 - 1.0,
                row + 2.0,
                [0.7, 0.7, 0.7, alpha * 0.9],
            );
        }
    }

    /// Get the total number of achievements.
    pub fn total_count() -> usize {
        ACHIEVEMENTS.len()
    }

    /// Get the number of unlocked achievements.
    pub fn unlocked_count(unlocked: &UnlockedAchievements) -> usize {
        unlocked.ids.len()
    }

    /// Render a full achievement list (for a future achievements screen).
    #[allow(dead_code)]
    pub fn render_list(
        renderer: &mut GameRenderer,
        unlocked: &UnlockedAchievements,
        start_row: f32,
    ) {
        let header = format!(
            "=== ACHIEVEMENTS ({}/{}) ===",
            unlocked.ids.len(),
            ACHIEVEMENTS.len()
        );
        renderer.draw_centered(&header, start_row, Colors::YELLOW);

        for (i, def) in ACHIEVEMENTS.iter().enumerate() {
            let row = start_row + 2.0 + i as f32 * 2.0;
            let is_unlocked = unlocked.ids.contains(def.id);

            if is_unlocked {
                let line = format!("[x] {} - {}", def.name, def.description);
                renderer.draw_centered(&line, row, Colors::GREEN);
            } else if def.hidden {
                renderer.draw_centered("[ ] ???", row, Colors::DARK_GRAY);
            } else {
                let line = format!("[ ] {} - {}", def.name, def.description);
                renderer.draw_centered(&line, row, Colors::GRAY);
            }
        }
    }
}

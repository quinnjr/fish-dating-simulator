//! Plugin system for loading custom fish characters via Rhai scripts.
//!
//! Place `.rhai` files in the `plugins/` directory to add new dateable fish.
//! Each script defines a fish character with art, stats, and dialogue trees
//! using the Rhai scripting API.

pub mod dialogue_def;
pub mod fish_def;
pub mod loader;
pub mod registry;

pub use fish_def::FishDef;
pub use registry::FishRegistry;

use std::path::PathBuf;

/// Load all plugins from the default plugins directory.
pub fn load_all_plugins() -> FishRegistry {
    let mut registry = FishRegistry::new();

    // Look for plugins directory relative to the executable / cwd
    let plugin_dirs = [
        PathBuf::from("plugins"),
        PathBuf::from("./plugins"),
    ];

    for dir in &plugin_dirs {
        if dir.exists() {
            loader::load_plugins(dir, &mut registry);
            break;
        }
    }

    if registry.count() > 0 {
        tracing::info!("Loaded {} plugin fish total", registry.count());
    }

    registry
}

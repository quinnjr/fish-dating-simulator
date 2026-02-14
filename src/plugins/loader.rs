//! Rhai script loader for fish plugins.
//!
//! Sets up the Rhai engine with the fish plugin API and loads `.rhai` scripts
//! from the `plugins/` directory.

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use rhai::{Engine, Dynamic, Map, Array, CustomType, TypeBuilder};

use super::dialogue_def::{DialogueDef, parse_choice_options};
use super::fish_def::FishDef;
use super::registry::FishRegistry;

/// Load all `.rhai` plugins from the given directory into the registry.
pub fn load_plugins(plugins_dir: &Path, registry: &mut FishRegistry) {
    if !plugins_dir.exists() {
        tracing::info!("No plugins directory found at {:?}, skipping plugin loading", plugins_dir);
        return;
    }

    let entries = match std::fs::read_dir(plugins_dir) {
        Ok(entries) => entries,
        Err(e) => {
            tracing::warn!("Failed to read plugins directory: {:?}", e);
            return;
        }
    };

    let mut scripts: Vec<std::path::PathBuf> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "rhai"))
        .collect();

    scripts.sort();

    if scripts.is_empty() {
        tracing::info!("No .rhai plugin scripts found in {:?}", plugins_dir);
        return;
    }

    tracing::info!("Found {} plugin script(s) in {:?}", scripts.len(), plugins_dir);

    for script_path in &scripts {
        load_single_plugin(script_path, registry);
    }
}

/// Load a single `.rhai` plugin script.
fn load_single_plugin(path: &Path, registry: &mut FishRegistry) {
    let filename = path.file_name().unwrap_or_default().to_string_lossy();
    tracing::info!("Loading plugin: {}", filename);

    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to read plugin {}: {:?}", filename, e);
            return;
        }
    };

    // Create a shared vec to collect registered fish from the script
    let registered: Rc<RefCell<Vec<FishDef>>> = Rc::new(RefCell::new(Vec::new()));

    let engine = create_engine(registered.clone());

    match engine.eval::<()>(&source) {
        Ok(()) => {
            let fish_defs = registered.borrow();
            if fish_defs.is_empty() {
                tracing::warn!("Plugin {} didn't register any fish", filename);
            }
            for fish in fish_defs.iter() {
                registry.register(fish.clone());
            }
        }
        Err(e) => {
            tracing::error!("Error in plugin {}: {}", filename, e);
        }
    }
}

/// Create a Rhai engine with all the fish plugin API functions registered.
fn create_engine(registered: Rc<RefCell<Vec<FishDef>>>) -> Engine {
    let mut engine = Engine::new();

    // Register the DialogueDef custom type
    engine.build_type::<DialogueDef>();

    // ── Dialogue builder functions ─────────────────────────────────────

    // new_dialogue(title) -> DialogueDef
    engine.register_fn("new_dialogue", |title: &str| -> DialogueDef {
        DialogueDef::new(title)
    });

    // dialogue.speaker(id, display_name)
    engine.register_fn("speaker", |d: &mut DialogueDef, id: &str, display_name: &str| {
        d.add_speaker(id, display_name);
    });

    // dialogue.text(id, speaker, text, next)
    engine.register_fn("text", |d: &mut DialogueDef, id: &str, speaker: &str, text: &str, next: &str| {
        d.add_text(id, speaker, text, next);
    });

    // dialogue.choice(id, prompt, options_array)
    // options_array is an array of maps: #{ text: "...", next: "...", affection: N }
    engine.register_fn("choice", |d: &mut DialogueDef, id: &str, prompt: &str, options: Array| {
        let opts = parse_choice_options(&options);
        d.add_choice(id, prompt, opts);
    });

    // dialogue.end(id)
    engine.register_fn("end", |d: &mut DialogueDef, id: &str| {
        d.add_end(id);
    });

    // ── Fish registration ──────────────────────────────────────────────

    // register_fish(map) - takes a Rhai map and registers a fish
    let reg = registered.clone();
    engine.register_fn("register_fish", move |fish_map: Map| {
        match parse_fish_def(&fish_map) {
            Ok(fish) => {
                reg.borrow_mut().push(fish);
            }
            Err(e) => {
                // We can't easily return errors in Rhai registered fns,
                // so we log and continue
                eprintln!("[plugin error] Failed to register fish: {}", e);
            }
        }
    });

    // Set max operations to prevent infinite loops in plugins
    engine.set_max_operations(100_000);

    engine
}

/// Parse a Rhai Map into a FishDef.
fn parse_fish_def(map: &Map) -> Result<FishDef, String> {
    let get_str = |key: &str| -> Result<String, String> {
        map.get(key)
            .ok_or_else(|| format!("missing required field '{}'", key))?
            .clone()
            .into_string()
            .map_err(|_| format!("field '{}' must be a string", key))
    };

    let get_str_or = |key: &str, default: &str| -> String {
        map.get(key)
            .and_then(|v| v.clone().into_string().ok())
            .unwrap_or_else(|| default.to_string())
    };

    let id = get_str("id")?;
    let name = get_str("name")?;
    let species = get_str("species")?;
    let description = get_str_or("description", "A mysterious fish.");
    let difficulty = map.get("difficulty")
        .and_then(|v| {
            if let Ok(f) = v.as_float() {
                Some(f as f32)
            } else if let Ok(i) = v.as_int() {
                Some(i as f32)
            } else {
                None
            }
        })
        .unwrap_or(0.5);

    let color = parse_color(map.get("color")).unwrap_or([1.0, 1.0, 1.0, 1.0]);

    let art_happy = get_str_or("art_happy", "  ><(((o>");
    let art_neutral = get_str_or("art_neutral", "  ><(((o>");
    let art_sad = get_str_or("art_sad", "  ><(((o>");
    let art_small = get_str_or("art_small", "><>");

    let date_location = get_str_or("date_location", "The Deep");
    let date_scene_art = get_str_or("date_scene_art", "  ~~~~~~~~\n  ~ ~ ~ ~ ~\n  ~~~~~~~~");
    let pond_name = get_str_or("pond_name", &format!("{}'s Pond", name));

    // Parse dialogues array
    let dialogues = if let Some(dates_val) = map.get("dates") {
        if let Some(dates_arr) = dates_val.clone().try_cast::<Array>() {
            dates_arr.iter()
                .filter_map(|d| {
                    d.clone().try_cast::<DialogueDef>()
                        .map(|def| def.to_dialogue_tree())
                })
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    Ok(FishDef {
        id,
        name,
        species,
        description,
        difficulty,
        color,
        art_happy,
        art_neutral,
        art_sad,
        art_small,
        date_location,
        date_scene_art,
        pond_name,
        dialogues,
    })
}

/// Parse an RGBA color from a Rhai array [r, g, b, a] or [r, g, b].
fn parse_color(val: Option<&Dynamic>) -> Option<[f32; 4]> {
    let val = val?;
    let arr = val.clone().try_cast::<Array>()?;

    let get_f32 = |idx: usize| -> f32 {
        arr.get(idx).and_then(|v| {
            if let Ok(f) = v.as_float() {
                Some(f as f32)
            } else if let Ok(i) = v.as_int() {
                Some(i as f32)
            } else {
                None
            }
        }).unwrap_or(1.0)
    };

    if arr.len() >= 3 {
        let r = get_f32(0);
        let g = get_f32(1);
        let b = get_f32(2);
        let a = if arr.len() >= 4 { get_f32(3) } else { 1.0 };
        Some([r, g, b, a])
    } else {
        None
    }
}

// Implement CustomType for DialogueDef so Rhai can work with it
impl CustomType for DialogueDef {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("DialogueDef");
    }
}

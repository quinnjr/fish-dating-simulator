//! Fish plugin registry.
//!
//! Stores all plugin fish definitions and provides lookup methods.

use std::collections::HashMap;

use super::fish_def::FishDef;

/// Central registry of all plugin fish characters.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct FishRegistry {
    /// Fish definitions indexed by plugin ID.
    fish: HashMap<String, FishDef>,
    /// Ordered list of plugin IDs (for deterministic iteration).
    order: Vec<String>,
}

#[allow(dead_code)]
impl FishRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new plugin fish. Returns true if successful, false if ID already taken.
    pub fn register(&mut self, fish: FishDef) -> bool {
        if self.fish.contains_key(&fish.id) {
            tracing::warn!("Plugin fish '{}' already registered, skipping duplicate", fish.id);
            return false;
        }
        let id = fish.id.clone();
        tracing::info!(
            "Registered plugin fish: {} ({}) - {}",
            fish.name, fish.species, fish.id
        );
        self.fish.insert(id.clone(), fish);
        self.order.push(id);
        true
    }

    /// Get a fish definition by plugin ID.
    pub fn get(&self, id: &str) -> Option<&FishDef> {
        self.fish.get(id)
    }

    /// Get all registered plugin fish IDs in registration order.
    pub fn plugin_ids(&self) -> &[String] {
        &self.order
    }

    /// Get all registered fish definitions in registration order.
    pub fn all_fish(&self) -> Vec<&FishDef> {
        self.order.iter().filter_map(|id| self.fish.get(id)).collect()
    }

    /// Number of registered plugin fish.
    pub fn count(&self) -> usize {
        self.fish.len()
    }

    /// Get all pond names from plugin fish (for adding to the pond selection).
    pub fn pond_names(&self) -> Vec<&str> {
        self.order
            .iter()
            .filter_map(|id| self.fish.get(id))
            .map(|f| f.pond_name.as_str())
            .collect()
    }

    /// Find a plugin fish by its pond name.
    pub fn fish_by_pond(&self, pond_name: &str) -> Option<&FishDef> {
        self.fish.values().find(|f| f.pond_name == pond_name)
    }
}

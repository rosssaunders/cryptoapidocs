use std::collections::HashMap;
use std::sync::OnceLock;
use inventory;

use super::ApiProcessor;

// Define a struct to hold processor registration info
#[derive(Clone)]
pub struct ProcessorRegistration {
    pub name: &'static str,
    pub create_fn: fn() -> Box<dyn ApiProcessor + Send + Sync>,
}

// Create an inventory collector for processor registrations
inventory::collect!(ProcessorRegistration);

// Global registry that will be lazily initialized
static REGISTRY: OnceLock<HashMap<&'static str, fn() -> Box<dyn ApiProcessor + Send + Sync>>> = OnceLock::new();

// Initialize the registry from inventory items
fn init_registry() -> HashMap<&'static str, fn() -> Box<dyn ApiProcessor + Send + Sync>> {
    let mut map = HashMap::new();
    for registration in inventory::iter::<ProcessorRegistration> {
        map.insert(registration.name, registration.create_fn);
    }
    map
}

// Get the global registry, initializing it if needed
pub fn get_registry() -> &'static HashMap<&'static str, fn() -> Box<dyn ApiProcessor + Send + Sync>> {
    REGISTRY.get_or_init(init_registry)
}

// Function to create all processors of a specific type (spot or futures)
pub fn create_processors_by_type(is_spot: bool) -> Vec<Box<dyn ApiProcessor + Send + Sync>> {
    get_registry()
        .iter()
        .filter(|(name, _)| {
            let is_spot_processor = name.contains("spot");
            is_spot == is_spot_processor
        })
        .map(|(_, create_fn)| create_fn())
        .collect()
} 
use std::collections::HashMap;
use std::sync::Arc;

use super::r#trait::ProviderAdapter;

pub struct ProviderRegistry {
    adapters: HashMap<String, Arc<dyn ProviderAdapter>>,
    default: String,
}

impl ProviderRegistry {
    pub fn new(default: &str) -> Self {
        Self {
            adapters: HashMap::new(),
            default: default.to_string(),
        }
    }

    pub fn register(&mut self, adapter: Arc<dyn ProviderAdapter>) {
        self.adapters.insert(adapter.name().to_string(), adapter);
    }

    /// Resolve by explicit name, or fall back to the default.
    pub fn resolve(&self, name: Option<&str>) -> Option<Arc<dyn ProviderAdapter>> {
        let key = name.unwrap_or(&self.default);
        self.adapters.get(key).cloned()
    }
}

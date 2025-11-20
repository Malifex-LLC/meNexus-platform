// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use dashmap::DashMap;
use std::sync::Arc;
use synapse_core::{
    CoreError,
    ports::modules::{Module, ModuleRegistry},
};

pub struct InMemoryModuleRegistry {
    inner: dashmap::DashMap<String, std::sync::Arc<dyn Module>>,
}

impl InMemoryModuleRegistry {
    pub fn new() -> Self {
        Self {
            inner: DashMap::new(),
        }
    }
}

impl ModuleRegistry for InMemoryModuleRegistry {
    fn register(&self, module: Arc<dyn Module>) -> Result<(), CoreError> {
        self.inner.insert(module.kind()?, module);
        Ok(())
    }
    fn get(&self, k: &str) -> Option<Arc<dyn Module>> {
        self.inner.get(k).map(|e| e.clone())
    }
    fn installed_kinds(&self) -> Vec<String> {
        self.inner.iter().map(|e| e.key().clone()).collect()
    }
}

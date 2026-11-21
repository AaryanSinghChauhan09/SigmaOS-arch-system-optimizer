// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-system-optimizer: arch system optimizer component model.
//!
//! Real implementation: optimizers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Optimizer {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchSystemOptimizerError {
    EmptyOptimizerName,
    DuplicateOptimizer(String),
    EmptyEventSubject,
}

impl fmt::Display for ArchSystemOptimizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyOptimizerName => write!(f, "optimizer name must not be empty"),
            Self::DuplicateOptimizer(id) => write!(f, "duplicate optimizer {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ArchSystemOptimizerError {}

pub struct ArchSystemOptimizer {
    pub name: String,
    optimizers: BTreeMap<String, Optimizer>,
    events: BTreeMap<String, Event>,
}

impl ArchSystemOptimizer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            optimizers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_optimizer(&mut self, item: Optimizer) -> Result<(), ArchSystemOptimizerError> {
        if item.name.trim().is_empty() {
            return Err(ArchSystemOptimizerError::EmptyOptimizerName);
        }
        let id = item.id.clone();
        if self.optimizers.contains_key(&id) {
            return Err(ArchSystemOptimizerError::DuplicateOptimizer(id.clone()));
        }
        self.optimizers.insert(id, item);
        Ok(())
    }

    pub fn optimizer_count(&self) -> usize {
        self.optimizers.len()
    }

    pub fn get_optimizer(&self, id: &str) -> Option<&Optimizer> {
        self.optimizers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ArchSystemOptimizerError> {
        if event.subject.trim().is_empty() {
            return Err(ArchSystemOptimizerError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_optimizer(id: &str, name: &str) -> Optimizer {
        Optimizer {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ArchSystemOptimizer::new("ArchSystemOptimizer");
        assert_eq!(m.name, "ArchSystemOptimizer");
        assert_eq!(m.optimizer_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_optimizer() {
        let mut m = ArchSystemOptimizer::new("ArchSystemOptimizer");
        m.add_optimizer(sample_optimizer("a1", "Alpha")).unwrap();
        assert_eq!(m.optimizer_count(), 1);
        assert!(m.get_optimizer("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ArchSystemOptimizer::new("ArchSystemOptimizer");
        assert!(m.add_optimizer(sample_optimizer("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ArchSystemOptimizer::new("ArchSystemOptimizer");
        m.add_optimizer(sample_optimizer("x", "One")).unwrap();
        assert!(m.add_optimizer(sample_optimizer("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ArchSystemOptimizer::new("ArchSystemOptimizer");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

/// Module extracted from the SigmaOS-system monolith.
pub mod optimizer;

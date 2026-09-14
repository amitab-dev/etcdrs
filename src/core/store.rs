use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

const DEFAULT_KEY_EXPIRATION_IN_MS: u64 = 900000;

#[derive(Debug, Clone)]
pub struct Unit {
    pub value: String,
    pub expiration: Instant,
}

#[derive(Debug, Clone)]
pub struct Store {
    pub entities: HashMap<String, Unit>,
}

impl Unit {
    pub fn is_expired(&self) -> bool {
        if self.expiration < Instant::now() {
            return true;
        }
        false
    }
}

impl Store {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, expiration_in_ms: Option<u64>) -> String {
        let expires_at = expiration_in_ms.unwrap_or(DEFAULT_KEY_EXPIRATION_IN_MS);
        let ttl = Instant::now() + Duration::from_millis(expires_at);
        self.entities.insert(
            key,
            Unit {
                value,
                expiration: ttl,
            },
        );
        "OK".to_string()
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        if let Some(unit) = self.entities.get(key) {
            if unit.is_expired() {
                self.entities.remove(key);
                return None;
            }
        }
        self.entities.get(key).map(|unit| &unit.value)
    }

    pub fn del(&mut self, key: &str) {
        self.entities.remove(key);
    }
}

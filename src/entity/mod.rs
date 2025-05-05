use std::{any::Any, u64};

use rand::{rng, Rng};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

/// Since all entities are stored a JSON, all types need to implement ``Deserialize`` and ``Serialize`` from the
/// ``serde`` library.
pub struct Entity {
    pub id: u64,
    pub contents: Value,
}

impl Entity {
    /// Entity ID
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Create a new entity
    pub fn create<T: Any + Serialize>(a: T) -> Self {
        let mut rng = rng();
        let id = rng.random_range(0..u64::MAX);
        return Entity {
            id: id,
            contents: serde_json::to_value(&a).unwrap(),
        };
    }

    /// Get the original entity value and type
    pub fn get<T: DeserializeOwned>(&self) -> T {
        let og: Result<T, serde_json::Error> = serde_json::from_value(self.contents.clone());
        return og.unwrap();
    }

    pub fn recreate<T: Any + Serialize>(id: u64, entity: T) -> Self {
        return Entity {
            id,
            contents: serde_json::to_value(&entity).unwrap(),
        };
    }
}

use indexmap::IndexMap;
use serde::{
    Serialize,
    Deserialize,
};

use crate::property::Property;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Properties(IndexMap<String, Property>);

// Deref, DerefMut にするかは要検討
impl Properties {
    pub fn new() -> Self {
        Properties(IndexMap::new())
    }

    pub fn insert(
        &mut self,
        key: impl Into<String>,
        property: Property,
    ) {
        self.0.insert(key.into(), property);
    }
}

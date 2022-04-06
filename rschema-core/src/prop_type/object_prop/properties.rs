use indexmap::IndexMap;
use serde::{
    Serialize,
    Deserialize,
};

use super::Property;

/// Properties map of an object type property.
/// 
/// This is a map with entries of names and properties for each field of the structure and structure variant.
/// 
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

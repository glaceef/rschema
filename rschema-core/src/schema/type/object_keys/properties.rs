use indexmap::IndexMap;
use serde::Serialize;

use std::ops::{
    Deref,
    DerefMut,
};

use super::Property;

type PropertiesMap = IndexMap<String, Property>;

/// Properties map of an object type property.
/// 
/// This is a map with entries of names and properties for each field of the structure and structure variant.
/// 
#[derive(Debug, Default, Serialize)]
pub struct Properties(PropertiesMap);

impl Deref for Properties {
    type Target = PropertiesMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Properties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Properties {
    pub fn new() -> Self {
        Properties(IndexMap::new())
    }
}

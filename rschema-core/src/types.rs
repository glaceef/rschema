use indexmap::IndexMap;

use crate::Property;

pub type Properties = IndexMap<String, Property>;

pub type Required = &'static[&'static str];

use serde::{
    Serialize,
    Deserialize,
};

mod properties;
mod property;

pub use properties::Properties;
pub use property::Property;

/// Attributes for object type properties.
/// 
/// Structs and struct-type variants of enums are correspond to `ObjectProp`.
/// 
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ObjectProp {
    pub properties: Properties,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,

    pub additional_properties: bool,
}

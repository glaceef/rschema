use serde::Serialize;

mod additional_properties;
mod properties;
mod property;

pub use additional_properties::AdditionalProperties;
pub use properties::Properties;
pub use property::Property;

/// Attributes for object type properties.
/// 
/// Structs and struct-type variants of enums are correspond to `ObjectKeys`.
/// 
#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ObjectKeys {
    pub properties: Properties,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,

    pub additional_properties: Box<AdditionalProperties>,
}

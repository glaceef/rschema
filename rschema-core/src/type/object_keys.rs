use serde::Serialize;

mod additional_properties;
mod properties;
mod property;

pub use additional_properties::AdditionalProperties;
pub use properties::Properties;
pub use property::Property;

/// Keywords for an object type property.
/// 
/// Structs and struct-type variants are correspond to `ObjectKeys`.
/// 
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectKeys {
    pub properties: Properties,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,

    pub additional_properties: Box<AdditionalProperties>,
}

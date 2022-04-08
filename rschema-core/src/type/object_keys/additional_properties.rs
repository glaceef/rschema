use serde::Serialize;

use crate::Type;

/// Whether or not the object type property accepts additional properties, or what kind of properties it accepts.
/// 
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum AdditionalProperties {
    /// For objects with fixed properties, such as struct.
    Boolean(bool),

    /// For objects with undefined properties, such as HashMap.
    Complex(Type),
}

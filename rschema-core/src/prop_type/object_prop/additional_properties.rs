use serde::Serialize;

use crate::PropType;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum AdditionalProperties {
    /// For objects with fixed properties, such as struct.
    Boolean(bool),

    /// For objects with undefined properties, such as HashMap.
    Complex(PropType),
}

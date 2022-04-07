use serde::{
    Serialize,
    Deserialize,
};

use crate::PropType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdditionalProperties {
    /// For objects with fixed properties, such as struct.
    Boolean(bool),

    /// For objects with undefined properties, such as HashMap.
    Complex(PropType),
}

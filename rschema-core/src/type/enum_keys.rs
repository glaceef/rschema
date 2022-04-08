use serde::Serialize;

use super::Type;

/// Attributes for enum type (a kind of array type) properties.
/// 
/// Only an array of enum struct corresponds to `EnumKeys`.
/// 
#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct EnumKeys {
    pub any_of: Vec<Type>,
}

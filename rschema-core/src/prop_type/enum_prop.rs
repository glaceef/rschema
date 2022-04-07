use serde::Serialize;

use super::PropType;

/// Attributes for enum type (a kind of array type) properties.
/// 
/// Only an array of enum struct corresponds to `EnumProp`.
/// 
#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct EnumProp {
    pub any_of: Vec<PropType>,
}

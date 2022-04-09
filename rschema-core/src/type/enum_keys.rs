use serde::Serialize;

use super::Type;

/// Keywords for an enum type (a kind of array type) property.
/// 
/// Only an array of enum struct corresponds to `EnumKeys`.
/// 
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumKeys {
    pub any_of: Vec<Type>,
}

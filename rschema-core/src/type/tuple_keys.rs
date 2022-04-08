use serde::Serialize;

use super::Type;

/// Attributes for tuple type (a kind of array type) properties.
/// 
/// Tuple type is a kind of array type, which has ordered items.
/// Tuples and tuple structs are correspond to `TupleKeys`, additional properties are not allowed.
/// 
#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct TupleKeys {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Type>,
}

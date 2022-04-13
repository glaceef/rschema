use serde::Serialize;

use super::Type;

/// Keywords for a tuple type (a kind of array type) property.
/// 
/// Tuple type is a kind of array type, which has ordered items.
/// Tuples and tuple structs are correspond to `TupleKeys`, additional properties are not allowed.
/// 
#[derive(Debug, Serialize)]
pub struct TupleKeys {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Type>,
}

use serde::Serialize;

use super::PropType;

/// Attributes for tuple type (a kind of array type) properties.
/// 
/// Tuple type is a kind of array type, which has ordered items.
/// Tuples and tuple structs are correspond to `TupleProp`, additional properties are not allowed.
/// 
#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct TupleProp {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PropType>,
}

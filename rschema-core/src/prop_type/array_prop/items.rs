use serde::Serialize;

use crate::prop_type::PropType;

/// Items of an array type or a tuple type property.
/// 
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Items {
    /// For an array type property with a single type items.
    Single(PropType),

    /// For an array type property with ordered items, like a tuple struct.
    Tuple(Vec<PropType>),
}
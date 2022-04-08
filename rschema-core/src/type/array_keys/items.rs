use serde::Serialize;

use crate::Type;

/// Items of an array type or a tuple type property.
/// 
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Items {
    /// For an array type property with a single type items.
    Single(Type),

    /// For an array type property with ordered items, like a tuple struct.
    Tuple(Vec<Type>),
}
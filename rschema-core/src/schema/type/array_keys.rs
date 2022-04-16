use serde::Serialize;

use crate::is_falsy;

mod items;
pub use items::Items;

/// Keywords for an array type property.
/// 
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrayKeys {
    pub items: Box<Items>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,

    #[serde(skip_serializing_if = "is_falsy")]
    pub unique_items: Option<bool>,
}

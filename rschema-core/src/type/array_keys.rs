use serde::Serialize;

mod items;
pub use items::Items;

/// Attributes for array type properties.
/// 
#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ArrayKeys {
    pub items: Box<Items>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,

    #[serde(skip_serializing_if = "is_falsy")]
    pub unique_items: Option<bool>,
}

fn is_falsy(b: &Option<bool>) -> bool {
    *b != Some(true)
}

use serde::{
    Serialize,
    Deserialize,
};

mod items;
pub use items::Items;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ArrayProp {
    pub items: Box<Items>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u64>,
}

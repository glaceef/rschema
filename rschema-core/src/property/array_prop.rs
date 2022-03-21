use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ArrayProp {
    #[serde(skip_serializing_if = "Option::is_none")]
    min_items: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_items: Option<u64>,
}
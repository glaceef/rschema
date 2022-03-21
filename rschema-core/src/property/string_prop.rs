use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct StringProp {
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
}
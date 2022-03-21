use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct NumericProp {
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    multiple_of: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exclusive_minimum: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exclusive_maximum: Option<u64>,
}
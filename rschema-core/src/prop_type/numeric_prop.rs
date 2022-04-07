use serde::{
    Serialize,
    Deserialize,
};

/// Attributes for numeric type properties.
/// 
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct NumericProp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<i64>,
}

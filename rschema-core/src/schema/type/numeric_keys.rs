use serde::Serialize;

/// Keywords for a numeric type property.
/// 
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NumericKeys {
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

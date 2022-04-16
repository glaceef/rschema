use serde::Serialize;

/// Keywords for a string type property.
/// 
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StringKeys {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "enum")]
    pub enm: Vec<String>,
}
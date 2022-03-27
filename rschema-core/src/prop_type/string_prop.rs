use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct StringProp {
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
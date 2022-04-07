use darling::FromMeta;
use serde::Serialize;

#[derive(Debug, FromMeta, Serialize)]
pub struct Field {
    /* common */
    pub title: String,
    #[darling(default)]
    pub description: Option<String>,

    #[darling(default)]
    #[darling(rename = "type")]
    #[serde(rename = "type")]
    ty: Option<String>,

    /* type: string */
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u64>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /* type: number */
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<i64>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<i64>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<i64>,

    /* type: array */
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
}

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

    /* type: number */
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<u64>, // 整数？
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>, // minimumを含めるか否か
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>, // maximumを含めるか否か

    /* type: array */
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<u64>,
    #[darling(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u64>,

    /* type: object */
    // #[darling(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub additional_properties: Option<bool>,
}

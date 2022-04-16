use serde::Serialize;

use crate::{
    Type,
    is_falsy,
};

/// One of the properties of an object type property.
/// 
#[derive(Debug, Serialize)]
pub struct Property {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(skip_serializing_if = "is_falsy")]
    pub deprecated: Option<bool>,

    #[serde(flatten)]
    pub ty: Type,
}

use serde::{
    Serialize,
    Deserialize,
};

use crate::PropType;

/// One of the properties of an object type property.
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(flatten)]
    pub ty: PropType,
}

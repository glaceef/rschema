use serde::{
    Serialize,
    Deserialize,
};

use crate::prop_type::PropType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    pub title: String,
    pub description: Option<String>,

    // #[serde(rename = "type")]
    #[serde(flatten)]
    pub ty: PropType,
}

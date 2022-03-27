use serde::{
    Serialize,
    Deserialize,
};

use super::PropType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct TupleProp {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PropType>,
}

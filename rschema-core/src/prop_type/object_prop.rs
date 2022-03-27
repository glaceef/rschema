use serde::{
    Serialize,
    Deserialize,
};

use crate::Properties;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ObjectProp {
    pub properties: Properties,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,

    pub additional_properties: bool,
}

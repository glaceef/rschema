use serde::{
    Serialize,
    Deserialize,
};

use crate::Properties;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ObjectProp {
    #[serde(default)]
    pub properties: Properties,

    #[serde(default)]
    pub required: Vec<String>,

    #[serde(default)]
    pub additional_properties: bool,
}

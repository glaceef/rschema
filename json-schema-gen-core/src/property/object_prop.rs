use serde::{
    Serialize,
    Deserialize,
};

use crate::Properties;

#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all(serialize = "camelCase"))]
pub struct ObjectProp {
    // #[serde(skip_serializing_if = "Option::is_none")]
    // properties: Option<Properties>,

    #[serde(default)]
    properties: Properties,
}

impl ObjectProp {
    pub fn set_properties(&mut self, properties: Properties) {
        self.properties = properties;
    }
}
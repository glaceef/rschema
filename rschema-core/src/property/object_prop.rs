use serde::{
    Serialize,
    Deserialize,
};

use crate::types::Properties;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ObjectProp {
    #[serde(default)]
    properties: Properties,

    #[serde(default)]
    required: Vec<String>,

    #[serde(default)]
    additional_properties: bool,
}

impl ObjectProp {
    pub fn set_properties(&mut self, properties: Properties) {
        self.properties = properties;
    }

    pub fn set_required(&mut self, required: Vec<String>) {
        self.required = required;
    }

    pub fn set_additional_properties(&mut self, additional_properties: bool) {
        self.additional_properties = additional_properties;
    }
}
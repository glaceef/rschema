use serde::Serialize;

use crate::{
    Properties,
    ToProperties,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    title: String,

    #[serde(rename = "type")]
    ty: String, // object

    properties: Properties,

    required: &'static[&'static str],

    additional_properties: bool,
}

impl Schema {
    pub fn new<T: ToProperties>(title: &str) -> Self {
        Schema {
            title: title.into(),
            ty: "object".into(),
            properties: T::to_properties(),
            required: T::REQUIRED,
            additional_properties: T::ADDITIONAL_PROPERTIES,
        }
    }

    pub fn to_string(&self) -> Result<String, String> {
        match serde_json::to_string(self) {
             Ok(v) => Ok(v),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn to_string_pretty(&self) -> Result<String, String> {
        match serde_json::to_string_pretty(self) {
             Ok(v) => Ok(v),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}

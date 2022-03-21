use serde::Serialize;

use crate::{
    Properties,
    Schematic,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    title: String,

    #[serde(rename = "type")]
    ty: String, // object

    properties: Properties,

    required: Vec<String>,

    additional_properties: bool,
}

impl Schema {
    pub fn new<T: Schematic>(title: &str) -> Self {
        Schema {
            title: title.into(),
            ty: "object".into(),
            properties: T::properties(),
            required: T::required(),
            additional_properties: T::additional_properties(),
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

    pub fn write<T, P>(&self, path: P) -> std::io::Result<()>
    where
        T: Schematic,
        P: AsRef<std::path::Path>,
    {
        let self_str = self.to_string().unwrap();
        std::fs::write(path, self_str)
    }

    pub fn write_pretty<T, P>(&self, path: P) -> std::io::Result<()>
    where
        T: Schematic,
        P: AsRef<std::path::Path>,
    {
        let self_str = self.to_string_pretty().unwrap();
        std::fs::write(path, self_str)
    }
}

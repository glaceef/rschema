use serde::Serialize;

use std::{
    fs,
    io,
    path::Path,
};

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

    pub fn write(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let self_str = self.to_string().unwrap();
        fs::write(path, self_str)
    }

    pub fn write_pretty(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let self_str = self.to_string_pretty().unwrap();
        fs::write(path, self_str)
    }
}

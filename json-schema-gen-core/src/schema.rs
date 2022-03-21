use serde::Serialize;

use crate::{
    Properties,
    ToProperties,
};

#[derive(Debug, Default)]
pub struct SchemaBuilder {
    pub title: String,
    pub required: Vec<String>,
    pub additional_properties: bool,
}

impl SchemaBuilder {
    pub fn new(title: &str) -> Self {
        SchemaBuilder {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn build<T: ToProperties>(self) -> Result<Schema, String> {
        let SchemaBuilder {
            title,
            required,
            additional_properties,
        } = self;

        Ok(Schema {
            title,
            ty: "object".into(),
            properties: T::to_properties(),
            required,
            additional_properties,
        })
    }

    pub fn required(mut self, required: Vec<String>) -> Self {
        self.required = required;
        self
    }

    pub fn additional_properties(mut self, b: bool) -> Self {
        self.additional_properties = b;
        self
    }

    pub fn with_required(&mut self, required: Vec<String>) -> &mut Self {
        self.required = required;
        self
    }

    pub fn with_additional_properties(&mut self, b: bool) -> &mut Self {
        self.additional_properties = b;
        self
    }
}

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

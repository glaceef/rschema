use serde::Serialize;

use crate::{
    Properties,
    ToProperties,
};

#[derive(Debug, Default)]
pub struct SchemaBuilder {
    pub title: String,
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
            additional_properties,
        } = self;

        Ok(Schema {
            title,
            ty: "object".into(),
            properties: T::to_properties(),
            // required: T::required(),
            required: T::REQUIRED,
            additional_properties,
        })
    }

    pub fn additional_properties(mut self, b: bool) -> Self {
        self.additional_properties = b;
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

    required: &'static[&'static str],

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
